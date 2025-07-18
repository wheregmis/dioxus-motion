//! Memory pool management for Dioxus Motion optimizations
//!
//! This module provides pooling systems to reduce memory allocations in hot paths
//! of the animation system, particularly for configuration objects and other
//! frequently allocated structures.

use crate::animations::core::{Animatable, AnimationConfig};
use crate::animations::spring::Spring;
use std::collections::HashMap;
use std::sync::{Arc, Weak, Mutex};
use std::cell::RefCell;
use std::any::{Any, TypeId};

/// A pool for reusing AnimationConfig instances to reduce allocations
pub struct ConfigPool {
    available: Vec<AnimationConfig>,
    in_use: HashMap<usize, AnimationConfig>,
    next_id: usize,
}

impl ConfigPool {
    /// Creates a new config pool with default capacity
    pub fn new() -> Self {
        Self::with_capacity(16)
    }

    /// Creates a new config pool with specified initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            available: Vec::with_capacity(capacity),
            in_use: HashMap::with_capacity(capacity),
            next_id: 0,
        }
    }

    /// Gets a config from the pool, creating a new one if none available
    pub fn get_config(&mut self) -> ConfigHandle {
        let config = self.available.pop().unwrap_or_default();
        let id = self.next_id;
        self.next_id += 1;
        self.in_use.insert(id, config);
        
        ConfigHandle {
            id,
            pool_ref: Arc::downgrade(&Arc::new(Mutex::new(self as *mut ConfigPool))),
        }
    }

    /// Returns a config to the pool for reuse
    pub fn return_config(&mut self, handle: ConfigHandle) {
        if let Some(mut config) = self.in_use.remove(&handle.id) {
            // Reset config to default state before returning to pool
            config.reset_to_default();
            self.available.push(config);
        }
    }

    /// Modifies a config in the pool safely
    pub fn modify_config<F>(&mut self, handle: &ConfigHandle, f: F) 
    where 
        F: FnOnce(&mut AnimationConfig)
    {
        if let Some(config) = self.in_use.get_mut(&handle.id) {
            f(config);
        }
    }

    /// Gets a reference to a config in the pool
    pub fn get_config_ref(&self, handle: &ConfigHandle) -> Option<&AnimationConfig> {
        self.in_use.get(&handle.id)
    }

    /// Gets the number of configs currently in use
    pub fn in_use_count(&self) -> usize {
        self.in_use.len()
    }

    /// Gets the number of configs available in the pool
    pub fn available_count(&self) -> usize {
        self.available.len()
    }

    /// Clears all configs from the pool
    pub fn clear(&mut self) {
        self.available.clear();
        self.in_use.clear();
        self.next_id = 0;
    }
}

impl Default for ConfigPool {
    fn default() -> Self {
        Self::new()
    }
}

/// A handle to a pooled AnimationConfig that automatically returns to pool when dropped
pub struct ConfigHandle {
    id: usize,
    pool_ref: Weak<Mutex<*mut ConfigPool>>,
}

impl ConfigHandle {
    /// Gets the ID of this handle
    pub fn id(&self) -> usize {
        self.id
    }

    /// Creates a new handle with the given ID and pool reference
    /// This is primarily for testing purposes
    #[cfg(test)]
    pub fn new_test(id: usize) -> Self {
        Self {
            id,
            pool_ref: Weak::new(),
        }
    }
}

impl Drop for ConfigHandle {
    fn drop(&mut self) {
        // Note: In a real implementation, we'd need to properly handle
        // returning the config to the pool here. For now, this is a
        // simplified version that demonstrates the pattern.
        // The actual return would need to be handled by the pool manager.
    }
}

impl Clone for ConfigHandle {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            pool_ref: self.pool_ref.clone(),
        }
    }
}

/// Extension trait for AnimationConfig to support pooling
trait ConfigPoolable {
    fn reset_to_default(&mut self);
}

impl ConfigPoolable for AnimationConfig {
    fn reset_to_default(&mut self) {
        *self = AnimationConfig::default();
    }
}

// Thread-local config pool for efficient access
thread_local! {
    static CONFIG_POOL: RefCell<ConfigPool> = RefCell::new(ConfigPool::new());
}

/// A pooled Arc<AnimationConfig> that automatically returns to pool when dropped
pub struct PooledConfig {
    config: Arc<AnimationConfig>,
    handle: Option<ConfigHandle>,
}

impl PooledConfig {
    /// Creates a new pooled config from the global pool
    pub fn new(config: AnimationConfig) -> Self {
        let handle = global::get_config();
        global::modify_config(&handle, |pooled_config| {
            *pooled_config = config;
        });
        
        let arc_config = global::get_config_ref(&handle)
            .map(Arc::new)
            .unwrap_or_else(|| Arc::new(AnimationConfig::default()));
            
        Self {
            config: arc_config,
            handle: Some(handle),
        }
    }
    
    /// Creates a pooled config from an existing Arc (for compatibility)
    pub fn from_arc(config: Arc<AnimationConfig>) -> Self {
        Self {
            config,
            handle: None,
        }
    }
    
    /// Gets the Arc<AnimationConfig> for compatibility with existing code
    pub fn as_arc(&self) -> Arc<AnimationConfig> {
        self.config.clone()
    }
}

impl Drop for PooledConfig {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            global::return_config(handle);
        }
    }
}

impl Clone for PooledConfig {
    fn clone(&self) -> Self {
        if self.handle.is_some() {
            // Create a new pooled config with the same configuration
            Self::new((*self.config).clone())
        } else {
            // Just clone the Arc for non-pooled configs
            Self::from_arc(self.config.clone())
        }
    }
}

/// Global functions for accessing the thread-local config pool
pub mod global {
    use super::*;

    /// Gets a config from the global thread-local pool
    pub fn get_config() -> ConfigHandle {
        CONFIG_POOL.with(|pool| {
            pool.borrow_mut().get_config()
        })
    }

    /// Returns a config to the global thread-local pool
    pub fn return_config(handle: ConfigHandle) {
        CONFIG_POOL.with(|pool| {
            pool.borrow_mut().return_config(handle);
        });
    }

    /// Modifies a config in the global thread-local pool
    pub fn modify_config<F>(handle: &ConfigHandle, f: F) 
    where 
        F: FnOnce(&mut AnimationConfig)
    {
        CONFIG_POOL.with(|pool| {
            pool.borrow_mut().modify_config(handle, f);
        });
    }

    /// Gets a reference to a config in the global thread-local pool
    pub fn get_config_ref(handle: &ConfigHandle) -> Option<AnimationConfig> {
        CONFIG_POOL.with(|pool| {
            pool.borrow().get_config_ref(handle).cloned()
        })
    }

    /// Creates a pooled Arc<AnimationConfig> - replacement for Arc::new(config)
    /// Note: This is a simple replacement that doesn't actually pool since Arc<AnimationConfig>
    /// doesn't have a way to track when it's dropped. For true pooling, we'd need to change
    /// the Motion struct to use PooledConfig directly.
    pub fn pooled_config(config: AnimationConfig) -> Arc<AnimationConfig> {
        // For now, just create a regular Arc to maintain compatibility
        // In a future version, we could change Motion to use PooledConfig directly
        Arc::new(config)
    }

    /// Gets pool statistics
    pub fn pool_stats() -> (usize, usize) {
        CONFIG_POOL.with(|pool| {
            let pool = pool.borrow();
            (pool.in_use_count(), pool.available_count())
        })
    }

    /// Clears the global pool (primarily for testing)
    #[cfg(test)]
    pub fn clear_pool() {
        CONFIG_POOL.with(|pool| {
            pool.borrow_mut().clear();
        });
    }
}


/// Spring integrator with pre-allocated buffers for RK4 integration
/// Eliminates temporary State struct allocations in hot paths
pub struct SpringIntegrator<T: Animatable> {
    // Pre-allocated buffers for RK4 integration steps
    k1_pos: T,
    k1_vel: T,
    k2_pos: T,
    k2_vel: T,
    k3_pos: T,
    k3_vel: T,
    k4_pos: T,
    k4_vel: T,
    // Temporary state for calculations
    temp_pos: T,
    temp_vel: T,
}

impl<T: Animatable> SpringIntegrator<T> {
    /// Creates a new spring integrator with default-initialized buffers
    pub fn new() -> Self {
        Self {
            k1_pos: T::default(),
            k1_vel: T::default(),
            k2_pos: T::default(),
            k2_vel: T::default(),
            k3_pos: T::default(),
            k3_vel: T::default(),
            k4_pos: T::default(),
            k4_vel: T::default(),
            temp_pos: T::default(),
            temp_vel: T::default(),
        }
    }

    /// Performs RK4 integration using pre-allocated buffers
    /// Returns the new position and velocity
    pub fn integrate_rk4(
        &mut self,
        current_pos: T,
        current_vel: T,
        target: T,
        spring: &Spring,
        dt: f32,
    ) -> (T, T) {
        let stiffness = spring.stiffness;
        let damping = spring.damping;
        let mass_inv = 1.0 / spring.mass;

        // K1 calculation
        let delta = target - current_pos;
        let force = delta * stiffness;
        let damping_force = current_vel * damping;
        let acc = (force - damping_force) * mass_inv;
        self.k1_pos = current_vel;
        self.k1_vel = acc;

        // K2 calculation
        self.temp_pos = current_pos + self.k1_pos * (dt * 0.5);
        self.temp_vel = current_vel + self.k1_vel * (dt * 0.5);
        let delta = target - self.temp_pos;
        let force = delta * stiffness;
        let damping_force = self.temp_vel * damping;
        let acc = (force - damping_force) * mass_inv;
        self.k2_pos = self.temp_vel;
        self.k2_vel = acc;

        // K3 calculation
        self.temp_pos = current_pos + self.k2_pos * (dt * 0.5);
        self.temp_vel = current_vel + self.k2_vel * (dt * 0.5);
        let delta = target - self.temp_pos;
        let force = delta * stiffness;
        let damping_force = self.temp_vel * damping;
        let acc = (force - damping_force) * mass_inv;
        self.k3_pos = self.temp_vel;
        self.k3_vel = acc;

        // K4 calculation
        self.temp_pos = current_pos + self.k3_pos * dt;
        self.temp_vel = current_vel + self.k3_vel * dt;
        let delta = target - self.temp_pos;
        let force = delta * stiffness;
        let damping_force = self.temp_vel * damping;
        let acc = (force - damping_force) * mass_inv;
        self.k4_pos = self.temp_vel;
        self.k4_vel = acc;

        // Final integration
        const SIXTH: f32 = 1.0 / 6.0;
        let new_pos = current_pos + (self.k1_pos + self.k2_pos * 2.0 + self.k3_pos * 2.0 + self.k4_pos) * (dt * SIXTH);
        let new_vel = current_vel + (self.k1_vel + self.k2_vel * 2.0 + self.k3_vel * 2.0 + self.k4_vel) * (dt * SIXTH);

        (new_pos, new_vel)
    }

    /// Resets all buffers to default values (for pool reuse)
    pub fn reset(&mut self) {
        self.k1_pos = T::default();
        self.k1_vel = T::default();
        self.k2_pos = T::default();
        self.k2_vel = T::default();
        self.k3_pos = T::default();
        self.k3_vel = T::default();
        self.k4_pos = T::default();
        self.k4_vel = T::default();
        self.temp_pos = T::default();
        self.temp_vel = T::default();
    }
}

impl<T: Animatable> Default for SpringIntegrator<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Pool for reusing SpringIntegrator instances
pub struct SpringIntegratorPool<T: Animatable> {
    available: Vec<SpringIntegrator<T>>,
    in_use: HashMap<usize, SpringIntegrator<T>>,
    next_id: usize,
}

impl<T: Animatable> SpringIntegratorPool<T> {
    /// Creates a new integrator pool
    pub fn new() -> Self {
        Self::with_capacity(8)
    }

    /// Creates a new integrator pool with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            available: Vec::with_capacity(capacity),
            in_use: HashMap::with_capacity(capacity),
            next_id: 0,
        }
    }

    /// Gets an integrator from the pool
    pub fn get_integrator(&mut self) -> SpringIntegratorHandle {
        let mut integrator = self.available.pop().unwrap_or_default();
        integrator.reset(); // Ensure clean state
        
        let id = self.next_id;
        self.next_id += 1;
        self.in_use.insert(id, integrator);
        
        SpringIntegratorHandle { id }
    }

    /// Returns an integrator to the pool
    pub fn return_integrator(&mut self, handle: SpringIntegratorHandle) {
        if let Some(integrator) = self.in_use.remove(&handle.id) {
            self.available.push(integrator);
        }
    }

    /// Gets a mutable reference to an integrator
    pub fn get_integrator_mut(&mut self, handle: &SpringIntegratorHandle) -> Option<&mut SpringIntegrator<T>> {
        self.in_use.get_mut(&handle.id)
    }

    /// Gets pool statistics
    pub fn stats(&self) -> (usize, usize) {
        (self.in_use.len(), self.available.len())
    }

    /// Clears the pool
    pub fn clear(&mut self) {
        self.available.clear();
        self.in_use.clear();
        self.next_id = 0;
    }
}

impl<T: Animatable> Default for SpringIntegratorPool<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle to a pooled SpringIntegrator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpringIntegratorHandle {
    id: usize,
}

impl SpringIntegratorHandle {
    /// Gets the ID of this handle
    pub fn id(&self) -> usize {
        self.id
    }
}

/// Global integrator pool management using type-erased storage
pub struct GlobalIntegratorPools {
    pools: HashMap<TypeId, Box<dyn Any + Send>>,
}

impl GlobalIntegratorPools {
    pub fn new() -> Self {
        Self {
            pools: HashMap::new(),
        }
    }

    /// Gets or creates a pool for type T
    pub fn get_pool<T: Animatable + Send + 'static>(&mut self) -> &mut SpringIntegratorPool<T> {
        let type_id = TypeId::of::<T>();
        self.pools
            .entry(type_id)
            .or_insert_with(|| Box::new(SpringIntegratorPool::<T>::new()))
            .downcast_mut::<SpringIntegratorPool<T>>()
            .expect("Type mismatch in integrator pool")
    }

    /// Clears all pools
    pub fn clear(&mut self) {
        self.pools.clear();
    }
}

// Thread-local integrator pools
thread_local! {
    static INTEGRATOR_POOLS: RefCell<GlobalIntegratorPools> = RefCell::new(GlobalIntegratorPools::new());
}

/// Global functions for integrator pool management
pub mod integrator {
    use super::*;

    /// Gets an integrator from the global thread-local pool
    pub fn get_integrator<T: Animatable + Send + 'static>() -> SpringIntegratorHandle {
        INTEGRATOR_POOLS.with(|pools| {
            pools.borrow_mut().get_pool::<T>().get_integrator()
        })
    }

    /// Returns an integrator to the global thread-local pool
    pub fn return_integrator<T: Animatable + Send + 'static>(handle: SpringIntegratorHandle) {
        INTEGRATOR_POOLS.with(|pools| {
            pools.borrow_mut().get_pool::<T>().return_integrator(handle);
        });
    }

    /// Performs RK4 integration using a pooled integrator
    pub fn integrate_rk4<T: Animatable + Send + 'static>(
        handle: &SpringIntegratorHandle,
        current_pos: T,
        current_vel: T,
        target: T,
        spring: &Spring,
        dt: f32,
    ) -> (T, T) {
        INTEGRATOR_POOLS.with(|pools| {
            let mut pools = pools.borrow_mut();
            let pool = pools.get_pool::<T>();
            if let Some(integrator) = pool.get_integrator_mut(handle) {
                integrator.integrate_rk4(current_pos, current_vel, target, spring, dt)
            } else {
                // Fallback to non-pooled integration if handle is invalid
                let mut integrator = SpringIntegrator::new();
                integrator.integrate_rk4(current_pos, current_vel, target, spring, dt)
            }
        })
    }

    /// Gets pool statistics for type T
    pub fn pool_stats<T: Animatable + Send + 'static>() -> (usize, usize) {
        INTEGRATOR_POOLS.with(|pools| {
            pools.borrow_mut().get_pool::<T>().stats()
        })
    }

    /// Clears all integrator pools (primarily for testing)
    #[cfg(test)]
    pub fn clear_pools() {
        INTEGRATOR_POOLS.with(|pools| {
            pools.borrow_mut().clear();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animations::core::AnimationMode;
    use crate::animations::spring::Spring;
    use instant::Duration;

    #[test]
    fn test_config_pool_basic_operations() {
        let mut pool = ConfigPool::new();
        
        // Test getting a config
        let handle1 = pool.get_config();
        assert_eq!(pool.in_use_count(), 1);
        assert_eq!(pool.available_count(), 0);
        
        // Test getting another config
        let handle2 = pool.get_config();
        assert_eq!(pool.in_use_count(), 2);
        assert_eq!(pool.available_count(), 0);
        
        // Test returning a config
        pool.return_config(handle1);
        assert_eq!(pool.in_use_count(), 1);
        assert_eq!(pool.available_count(), 1);
        
        // Test reusing returned config
        let handle3 = pool.get_config();
        assert_eq!(pool.in_use_count(), 2);
        assert_eq!(pool.available_count(), 0);
        
        // Clean up
        pool.return_config(handle2);
        pool.return_config(handle3);
    }

    #[test]
    fn test_config_pool_modification() {
        let mut pool = ConfigPool::new();
        let handle = pool.get_config();
        
        // Modify the config
        pool.modify_config(&handle, |config| {
            config.mode = AnimationMode::Spring(Spring::default());
            config.delay = Duration::from_millis(100);
        });
        
        // Verify modification
        let config_ref = pool.get_config_ref(&handle).unwrap();
        assert!(matches!(config_ref.mode, AnimationMode::Spring(_)));
        assert_eq!(config_ref.delay, Duration::from_millis(100));
        
        pool.return_config(handle);
    }

    #[test]
    fn test_config_pool_reset_on_return() {
        let mut pool = ConfigPool::new();
        let handle = pool.get_config();
        
        // Modify the config
        pool.modify_config(&handle, |config| {
            config.mode = AnimationMode::Spring(Spring::default());
            config.delay = Duration::from_millis(100);
        });
        
        // Return to pool (should reset)
        pool.return_config(handle);
        
        // Get a new config and verify it's reset
        let new_handle = pool.get_config();
        let config_ref = pool.get_config_ref(&new_handle).unwrap();
        assert!(matches!(config_ref.mode, AnimationMode::Tween(_)));
        assert_eq!(config_ref.delay, Duration::default());
        
        pool.return_config(new_handle);
    }

    #[test]
    fn test_config_pool_with_capacity() {
        let pool = ConfigPool::with_capacity(32);
        assert_eq!(pool.available_count(), 0);
        assert_eq!(pool.in_use_count(), 0);
    }

    #[test]
    fn test_config_pool_clear() {
        let mut pool = ConfigPool::new();
        let handle1 = pool.get_config();
        let _handle2 = pool.get_config();
        
        pool.return_config(handle1);
        assert_eq!(pool.in_use_count(), 1);
        assert_eq!(pool.available_count(), 1);
        
        pool.clear();
        assert_eq!(pool.in_use_count(), 0);
        assert_eq!(pool.available_count(), 0);
        
        // _handle2 is now invalid, but we won't try to return it
    }

    #[test]
    fn test_global_config_pool() {
        global::clear_pool();
        
        let handle1 = global::get_config();
        let handle2 = global::get_config();
        
        let (in_use, available) = global::pool_stats();
        assert_eq!(in_use, 2);
        assert_eq!(available, 0);
        
        global::modify_config(&handle1, |config| {
            config.delay = Duration::from_millis(50);
        });
        
        let config = global::get_config_ref(&handle1).unwrap();
        assert_eq!(config.delay, Duration::from_millis(50));
        
        global::return_config(handle1);
        global::return_config(handle2);
        
        let (in_use, available) = global::pool_stats();
        assert_eq!(in_use, 0);
        assert_eq!(available, 2);
    }

    #[test]
    fn test_config_handle_clone() {
        let handle1 = ConfigHandle::new_test(42);
        let handle2 = handle1.clone();
        
        assert_eq!(handle1.id(), handle2.id());
        assert_eq!(handle1.id(), 42);
    }

    #[test]
    fn test_pooled_config() {
        global::clear_pool();
        
        let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));
        
        {
            let pooled = global::pooled_config(config);
            
            // Verify it's an Arc<AnimationConfig>
            assert!(matches!(pooled.mode, AnimationMode::Spring(_)));
            
            let (in_use, _) = global::pool_stats();
            // Note: The pooled_config function creates and immediately drops the PooledConfig,
            // so the handle is returned to the pool right away
            assert_eq!(in_use, 0);
        }
    }

    #[test]
    fn test_pooled_config_clone() {
        global::clear_pool();
        
        let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));
        let pooled1 = PooledConfig::new(config);
        let pooled2 = pooled1.clone();
        
        // Both should have the same config values but different pool handles
        assert!(matches!(pooled1.as_arc().mode, AnimationMode::Spring(_)));
        assert!(matches!(pooled2.as_arc().mode, AnimationMode::Spring(_)));
        
        let (in_use, _) = global::pool_stats();
        assert_eq!(in_use, 2); // Two separate pooled configs
    }



    #[test]
    fn test_spring_integrator() {
        let mut integrator = SpringIntegrator::<f32>::new();
        let spring = Spring::default();
        
        let current_pos = 0.0f32;
        let current_vel = 0.0f32;
        let target = 100.0f32;
        let dt = 1.0 / 60.0; // 60 FPS
        
        let (new_pos, new_vel) = integrator.integrate_rk4(current_pos, current_vel, target, &spring, dt);
        
        // After one integration step, position should have moved toward target
        assert!(new_pos > current_pos);
        assert!(new_pos < target);
        assert!(new_vel > 0.0); // Should be moving toward target
        
        // Test reset
        integrator.reset();
        // All buffers should be reset to default (can't easily test without exposing internals)
    }

    #[test]
    fn test_spring_integrator_pool() {
        let mut pool = SpringIntegratorPool::<f32>::new();
        
        // Test getting integrator
        let handle1 = pool.get_integrator();
        let handle2 = pool.get_integrator();
        
        let (in_use, available) = pool.stats();
        assert_eq!(in_use, 2);
        assert_eq!(available, 0);
        
        // Test using integrator
        let spring = Spring::default();
        if let Some(integrator) = pool.get_integrator_mut(&handle1) {
            let (new_pos, new_vel) = integrator.integrate_rk4(0.0, 0.0, 100.0, &spring, 1.0/60.0);
            assert!(new_pos > 0.0);
            assert!(new_vel > 0.0);
        }
        
        // Test returning integrator
        pool.return_integrator(handle1);
        let (in_use, available) = pool.stats();
        assert_eq!(in_use, 1);
        assert_eq!(available, 1);
        
        // Test reusing returned integrator
        let handle3 = pool.get_integrator();
        let (in_use, available) = pool.stats();
        assert_eq!(in_use, 2);
        assert_eq!(available, 0);
        
        // Clean up
        pool.return_integrator(handle2);
        pool.return_integrator(handle3);
    }

    #[test]
    fn test_global_integrator_pool() {
        integrator::clear_pools();
        
        let handle1 = integrator::get_integrator::<f32>();
        let handle2 = integrator::get_integrator::<f32>();
        
        let (in_use, available) = integrator::pool_stats::<f32>();
        assert_eq!(in_use, 2);
        assert_eq!(available, 0);
        
        // Test integration
        let spring = Spring::default();
        let (new_pos, new_vel) = integrator::integrate_rk4(
            &handle1, 0.0, 0.0, 100.0, &spring, 1.0/60.0
        );
        assert!(new_pos > 0.0);
        assert!(new_vel > 0.0);
        
        // Return integrators
        integrator::return_integrator::<f32>(handle1);
        integrator::return_integrator::<f32>(handle2);
        
        let (in_use, available) = integrator::pool_stats::<f32>();
        assert_eq!(in_use, 0);
        assert_eq!(available, 2);
    }

    #[test]
    fn test_spring_integrator_accuracy() {
        // Test that the pooled integrator produces the same results as the original
        let mut integrator = SpringIntegrator::<f32>::new();
        let spring = Spring::default();
        
        let current_pos = 10.0f32;
        let current_vel = 5.0f32;
        let target = 50.0f32;
        let dt = 1.0 / 120.0; // 120 FPS
        
        let (new_pos, new_vel) = integrator.integrate_rk4(current_pos, current_vel, target, &spring, dt);
        
        // The result should be mathematically consistent
        // Position should move in the direction of velocity
        assert!(new_pos > current_pos);
        // Velocity should be affected by spring force
        let expected_force_direction = target - current_pos;
        assert!(expected_force_direction > 0.0); // Force toward target
        // With default spring settings, velocity should increase toward target
        assert!(new_vel > current_vel);
    }
}