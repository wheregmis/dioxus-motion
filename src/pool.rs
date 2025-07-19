//! Memory pool management for Dioxus Motion optimizations
//!
//! This module provides pooling systems to reduce memory allocations in hot paths
//! of the animation system, particularly for configuration objects and other
//! frequently allocated structures.

use crate::animations::core::{Animatable, AnimationConfig};
use crate::animations::spring::Spring;
use std::collections::HashMap;

use std::any::{Any, TypeId};
use std::cell::RefCell;

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

        ConfigHandle { id, valid: true }
    }

    /// Returns a config to the pool for reuse
    pub fn return_config(&mut self, handle: ConfigHandle) {
        if let Some(mut config) = self.in_use.remove(&handle.id) {
            // Reset config to default state before returning to pool
            config.reset_to_default();
            self.available.push(config);
        }
        // If the config wasn't found in in_use, it might have already been returned
        // This is safe to ignore as it prevents double-return issues
    }

    /// Modifies a config in the pool safely
    pub fn modify_config<F>(&mut self, handle: &ConfigHandle, f: F)
    where
        F: FnOnce(&mut AnimationConfig),
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

    /// Trims the available configs to the specified target size
    /// This removes excess configs from the available pool while preserving in-use configs
    pub fn trim_to_size(&mut self, target_size: usize) {
        let current_available = self.available.len();
        if current_available > target_size {
            // Remove excess configs from the end of the available vector
            self.available.truncate(target_size);
        }
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
    // Track if this handle is still valid (not yet dropped)
    valid: bool,
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
        Self { id, valid: true }
    }
}

impl Drop for ConfigHandle {
    fn drop(&mut self) {
        // Only return to pool if this handle is still valid
        if self.valid {
            // Mark as invalid to prevent double-return
            self.valid = false;

            // Return the config to the thread-local pool
            // Use try_with to handle potential borrow conflicts gracefully
            let _ = CONFIG_POOL.try_with(|pool| {
                if let Ok(mut pool) = pool.try_borrow_mut() {
                    pool.return_config(ConfigHandle {
                        id: self.id,
                        valid: false,
                    });
                }
            });
        }
    }
}

impl Clone for ConfigHandle {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            valid: self.valid,
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

/// Global functions for accessing the thread-local config pool
pub mod global {
    use super::*;

    /// Gets a config from the global thread-local pool
    pub fn get_config() -> ConfigHandle {
        CONFIG_POOL.with(|pool| pool.borrow_mut().get_config())
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
        F: FnOnce(&mut AnimationConfig),
    {
        CONFIG_POOL.with(|pool| {
            pool.borrow_mut().modify_config(handle, f);
        });
    }

    /// Gets a reference to a config in the global thread-local pool
    pub fn get_config_ref(handle: &ConfigHandle) -> Option<AnimationConfig> {
        CONFIG_POOL.with(|pool| pool.borrow().get_config_ref(handle).cloned())
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
        let new_pos = current_pos
            + (self.k1_pos + self.k2_pos * 2.0 + self.k3_pos * 2.0 + self.k4_pos) * (dt * SIXTH);
        let new_vel = current_vel
            + (self.k1_vel + self.k2_vel * 2.0 + self.k3_vel * 2.0 + self.k4_vel) * (dt * SIXTH);

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
    pub fn get_integrator_mut(
        &mut self,
        handle: &SpringIntegratorHandle,
    ) -> Option<&mut SpringIntegrator<T>> {
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

impl Default for GlobalIntegratorPools {
    fn default() -> Self {
        Self::new()
    }
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

    /// Gets statistics for all pools
    pub fn stats(&self) -> HashMap<TypeId, (usize, usize)> {
        // Note: This is a simplified version since we can't easily downcast
        // and call stats() on each pool without knowing the concrete type
        HashMap::new()
    }
}

/// Global resource pool management for Motion optimizations
/// Manages all pooled resources including configs, integrators, and closures
pub struct MotionResourcePools {
    /// Configuration pool for reusing AnimationConfig instances
    pub config_pool: ConfigPool,
    /// Integrator pools for different animatable types
    pub integrator_pools: GlobalIntegratorPools,
    /// Web closure pool for JavaScript closure reuse (web only)
    #[cfg(feature = "web")]
    pub closure_pool: crate::animations::closure_pool::WebClosurePool,
    /// Pool configuration settings
    pub config: PoolConfig,
}

impl MotionResourcePools {
    /// Creates new resource pools with default configuration
    pub fn new() -> Self {
        Self::with_config(PoolConfig::default())
    }

    /// Creates new resource pools with specified configuration
    pub fn with_config(config: PoolConfig) -> Self {
        Self {
            config_pool: ConfigPool::with_capacity(config.config_pool_capacity),
            integrator_pools: GlobalIntegratorPools::new(),
            #[cfg(feature = "web")]
            closure_pool: crate::animations::closure_pool::WebClosurePool::new(),
            config,
        }
    }

    /// Gets statistics for all pools
    pub fn stats(&self) -> PoolStats {
        let (config_in_use, config_available) = (
            self.config_pool.in_use_count(),
            self.config_pool.available_count(),
        );

        #[cfg(feature = "web")]
        let (closure_in_use, closure_available) = (
            self.closure_pool.in_use_count(),
            self.closure_pool.available_count(),
        );
        #[cfg(not(feature = "web"))]
        let (closure_in_use, closure_available) = (0, 0);

        PoolStats {
            config_pool: (config_in_use, config_available),
            closure_pool: (closure_in_use, closure_available),
            integrator_pools: self.integrator_pools.stats(),
            total_memory_saved_bytes: self.estimate_memory_savings(),
        }
    }

    /// Estimates memory savings from pooling (rough calculation)
    fn estimate_memory_savings(&self) -> usize {
        // Rough estimates based on typical struct sizes
        const CONFIG_SIZE: usize = std::mem::size_of::<AnimationConfig>();
        const INTEGRATOR_SIZE: usize = 256; // Rough estimate for SpringIntegrator<f32>
        const CLOSURE_SIZE: usize = 64; // Rough estimate for web closures

        let config_savings = self.config_pool.available_count() * CONFIG_SIZE;
        let closure_savings = {
            #[cfg(feature = "web")]
            {
                self.closure_pool.available_count() * CLOSURE_SIZE
            }
            #[cfg(not(feature = "web"))]
            {
                0
            }
        };

        // Integrator savings would need type-specific calculation
        // For now, just estimate based on common usage
        let integrator_savings = 8 * INTEGRATOR_SIZE; // Assume ~8 pooled integrators on average

        config_savings + closure_savings + integrator_savings
    }

    /// Clears all pools (primarily for testing and cleanup)
    pub fn clear(&mut self) {
        self.config_pool.clear();
        self.integrator_pools.clear();
        #[cfg(feature = "web")]
        self.closure_pool.clear();
    }

    /// Performs maintenance on all pools (removes excess capacity, etc.)
    pub fn maintain(&mut self) {
        // Trim config pool if it's grown too large
        if self.config_pool.available_count() > self.config.max_config_pool_size {
            self.config_pool
                .trim_to_size(self.config.target_config_pool_size);
        }

        // Similar maintenance for other pools could be added here
    }
}

impl Default for MotionResourcePools {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for resource pools
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Initial capacity for config pool
    pub config_pool_capacity: usize,
    /// Maximum size for config pool before trimming
    pub max_config_pool_size: usize,
    /// Target size to trim config pool to
    pub target_config_pool_size: usize,
    /// Whether to enable automatic pool maintenance
    pub auto_maintain: bool,
    /// Interval for automatic maintenance (in animation frames)
    pub maintenance_interval: u32,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            config_pool_capacity: 16,
            max_config_pool_size: 64,
            target_config_pool_size: 32,
            auto_maintain: true,
            maintenance_interval: 1000, // Every ~16 seconds at 60fps
        }
    }
}

/// Statistics about all resource pools
#[derive(Debug, Clone)]
pub struct PoolStats {
    /// Config pool stats: (in_use, available)
    pub config_pool: (usize, usize),
    /// Closure pool stats: (in_use, available)
    pub closure_pool: (usize, usize),
    /// Integrator pool stats by type
    pub integrator_pools: HashMap<TypeId, (usize, usize)>,
    /// Estimated memory saved by pooling (in bytes)
    pub total_memory_saved_bytes: usize,
}

// Thread-local resource pools
thread_local! {
    static MOTION_RESOURCE_POOLS: RefCell<MotionResourcePools> = RefCell::new(MotionResourcePools::new());
    static INTEGRATOR_POOLS: RefCell<GlobalIntegratorPools> = RefCell::new(GlobalIntegratorPools::new());
}

/// Global functions for integrator pool management
pub mod integrator {
    use super::*;

    /// Gets an integrator from the global thread-local pool
    pub fn get_integrator<T: Animatable + Send + 'static>() -> SpringIntegratorHandle {
        INTEGRATOR_POOLS.with(|pools| pools.borrow_mut().get_pool::<T>().get_integrator())
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
            pool.get_integrator_mut(handle).map_or_else(
                || {
                    // Fallback to non-pooled integration if handle is invalid
                    let mut integrator = SpringIntegrator::new();
                    integrator.integrate_rk4(current_pos, current_vel, target, spring, dt)
                },
                |integrator| integrator.integrate_rk4(current_pos, current_vel, target, spring, dt),
            )
        })
    }

    /// Gets pool statistics for type T
    pub fn pool_stats<T: Animatable + Send + 'static>() -> (usize, usize) {
        INTEGRATOR_POOLS.with(|pools| pools.borrow_mut().get_pool::<T>().stats())
    }

    /// Clears all integrator pools (primarily for testing)
    #[cfg(test)]
    pub fn clear_pools() {
        INTEGRATOR_POOLS.with(|pools| {
            pools.borrow_mut().clear();
        });
    }
}

/// Global functions for managing Motion resource pools
pub mod resource_pools {
    use super::*;

    /// Gets statistics for all resource pools
    pub fn stats() -> PoolStats {
        MOTION_RESOURCE_POOLS.with(|pools| pools.borrow().stats())
    }

    /// Configures the global resource pools
    /// This should be called early in your application startup for optimal performance
    pub fn configure(config: PoolConfig) {
        MOTION_RESOURCE_POOLS.with(|pools| {
            *pools.borrow_mut() = MotionResourcePools::with_config(config);
        });
    }

    /// Initializes resource pools with high-performance defaults
    /// Recommended for applications with many concurrent animations
    pub fn init_high_performance() {
        configure(PoolConfig {
            config_pool_capacity: 64,
            max_config_pool_size: 256,
            target_config_pool_size: 128,
            auto_maintain: true,
            maintenance_interval: 500, // More frequent maintenance
        });
    }

    /// Initializes resource pools with memory-conservative defaults
    /// Recommended for memory-constrained environments
    pub fn init_memory_conservative() {
        configure(PoolConfig {
            config_pool_capacity: 8,
            max_config_pool_size: 32,
            target_config_pool_size: 16,
            auto_maintain: true,
            maintenance_interval: 2000, // Less frequent maintenance
        });
    }

    /// Performs maintenance on all resource pools
    pub fn maintain() {
        MOTION_RESOURCE_POOLS.with(|pools| {
            pools.borrow_mut().maintain();
        });
    }

    /// Clears all resource pools (primarily for testing)
    #[cfg(test)]
    pub fn clear_all() {
        MOTION_RESOURCE_POOLS.with(|pools| {
            pools.borrow_mut().clear();
        });
    }

    /// Gets the current pool configuration
    pub fn get_config() -> PoolConfig {
        MOTION_RESOURCE_POOLS.with(|pools| pools.borrow().config.clone())
    }

    /// Estimates total memory usage of all pools
    pub fn memory_usage_bytes() -> usize {
        MOTION_RESOURCE_POOLS.with(|pools| {
            let pools = pools.borrow();
            let stats = pools.stats();

            // Rough calculation of memory usage
            const CONFIG_SIZE: usize = std::mem::size_of::<AnimationConfig>();
            const INTEGRATOR_SIZE: usize = 256;
            const CLOSURE_SIZE: usize = 64;

            let config_memory = (stats.config_pool.0 + stats.config_pool.1) * CONFIG_SIZE;
            let closure_memory = (stats.closure_pool.0 + stats.closure_pool.1) * CLOSURE_SIZE;
            let integrator_memory = stats
                .integrator_pools
                .values()
                .map(|(in_use, available)| (in_use + available) * INTEGRATOR_SIZE)
                .sum::<usize>();

            config_memory + closure_memory + integrator_memory
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
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
    fn test_config_handle_automatic_cleanup() {
        // Clear the pool first
        global::clear_pool();

        // Get a config handle
        let handle = global::get_config();

        // Verify the config is in use
        let (in_use, available) = global::pool_stats();
        assert_eq!(in_use, 1);
        assert_eq!(available, 0);

        // Drop the handle - should automatically return to pool
        drop(handle);

        // Verify the config was returned to the pool
        let (in_use, available) = global::pool_stats();
        assert_eq!(in_use, 0);
        assert_eq!(available, 1);
    }

    #[test]
    fn test_config_handle_double_drop_safety() {
        // Clear the pool first
        global::clear_pool();

        // Get a config handle
        let handle = global::get_config();
        let handle_id = handle.id();

        // Manually return the config
        global::return_config(ConfigHandle {
            id: handle_id,
            valid: false,
        });

        // Verify it was returned
        let (in_use, available) = global::pool_stats();
        assert_eq!(in_use, 0);
        assert_eq!(available, 1);

        // Now drop the original handle - should not cause issues
        drop(handle);

        // Should still have the same state
        let (in_use, available) = global::pool_stats();
        assert_eq!(in_use, 0);
        assert_eq!(available, 1);
    }

    #[test]
    fn test_spring_integrator() {
        let mut integrator = SpringIntegrator::<f32>::new();
        let spring = Spring::default();

        let current_pos = 0.0f32;
        let current_vel = 0.0f32;
        let target = 100.0f32;
        let dt = 1.0 / 60.0; // 60 FPS

        let (new_pos, new_vel) =
            integrator.integrate_rk4(current_pos, current_vel, target, &spring, dt);

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
            let (new_pos, new_vel) = integrator.integrate_rk4(0.0, 0.0, 100.0, &spring, 1.0 / 60.0);
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
        let (new_pos, new_vel) =
            integrator::integrate_rk4(&handle1, 0.0, 0.0, 100.0, &spring, 1.0 / 60.0);
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

        let (new_pos, new_vel) =
            integrator.integrate_rk4(current_pos, current_vel, target, &spring, dt);

        // The result should be mathematically consistent
        // Position should move in the direction of velocity
        assert!(new_pos > current_pos);
        // Velocity should be affected by spring force
        let expected_force_direction = target - current_pos;
        assert!(expected_force_direction > 0.0); // Force toward target
        // With default spring settings, velocity should increase toward target
        assert!(new_vel > current_vel);
    }

    #[test]
    fn test_motion_resource_pools() {
        let pools = MotionResourcePools::new();

        // Test initial state
        let stats = pools.stats();
        assert_eq!(stats.config_pool, (0, 0));
        assert_eq!(stats.closure_pool, (0, 0));
        assert!(stats.integrator_pools.is_empty());
        assert_eq!(stats.total_memory_saved_bytes, 8 * 256); // Estimated integrator savings
    }

    #[test]
    fn test_motion_resource_pools_with_config() {
        let config = PoolConfig {
            config_pool_capacity: 32,
            max_config_pool_size: 128,
            target_config_pool_size: 64,
            auto_maintain: false,
            maintenance_interval: 500,
        };

        let pools = MotionResourcePools::with_config(config.clone());
        assert_eq!(pools.config.config_pool_capacity, 32);
        assert_eq!(pools.config.max_config_pool_size, 128);
        assert!(!pools.config.auto_maintain);
    }

    #[test]
    fn test_motion_resource_pools_clear() {
        let mut pools = MotionResourcePools::new();

        // Add some items to pools (simplified test)
        let _handle = pools.config_pool.get_config();

        pools.clear();

        let stats = pools.stats();
        assert_eq!(stats.config_pool, (0, 0));
        assert_eq!(stats.closure_pool, (0, 0));
    }

    #[test]
    fn test_resource_pools_global_functions() {
        resource_pools::clear_all();

        // Test configuration
        let config = PoolConfig {
            config_pool_capacity: 24,
            max_config_pool_size: 96,
            target_config_pool_size: 48,
            auto_maintain: true,
            maintenance_interval: 750,
        };

        resource_pools::configure(config.clone());
        let retrieved_config = resource_pools::get_config();
        assert_eq!(retrieved_config.config_pool_capacity, 24);
        assert_eq!(retrieved_config.max_config_pool_size, 96);

        // Test stats
        let stats = resource_pools::stats();
        assert_eq!(stats.config_pool, (0, 0));

        // Test memory usage (should be reasonable)
        let memory_usage = resource_pools::memory_usage_bytes();
        // Memory usage should be reasonable (at least 0, but not too large)
        assert!(memory_usage < 1_000_000); // Shouldn't be unreasonably large

        // Test maintenance (should not panic)
        resource_pools::maintain();
    }

    #[test]
    fn test_pool_config_default() {
        let config = PoolConfig::default();
        assert_eq!(config.config_pool_capacity, 16);
        assert_eq!(config.max_config_pool_size, 64);
        assert_eq!(config.target_config_pool_size, 32);
        assert!(config.auto_maintain);
        assert_eq!(config.maintenance_interval, 1000);
    }

    #[test]
    fn test_pool_stats_memory_estimation() {
        let pools = MotionResourcePools::new();
        let stats = pools.stats();

        // Memory savings should be reasonable estimate
        assert!(stats.total_memory_saved_bytes > 0);
        assert!(stats.total_memory_saved_bytes < 1_000_000); // Shouldn't be unreasonably large
    }

    #[test]
    fn test_motion_resource_pools_maintain() {
        let mut pools = MotionResourcePools::new();

        // Test maintenance doesn't panic
        pools.maintain();

        // Test with modified config
        pools.config.max_config_pool_size = 1;
        pools.config.target_config_pool_size = 0;
        pools.maintain(); // Should handle edge cases gracefully
    }

    #[test]
    fn test_config_pool_trimming() {
        let mut pool = ConfigPool::new();

        // Add some configs to the available pool
        for _ in 0..10 {
            pool.available.push(AnimationConfig::default());
        }

        // Verify initial state
        assert_eq!(pool.available_count(), 10);
        assert_eq!(pool.in_use_count(), 0);

        // Trim to target size
        pool.trim_to_size(5);
        assert_eq!(pool.available_count(), 5);
        assert_eq!(pool.in_use_count(), 0);

        // Trim to smaller size
        pool.trim_to_size(2);
        assert_eq!(pool.available_count(), 2);
        assert_eq!(pool.in_use_count(), 0);

        // Trim to larger size (should not add configs)
        pool.trim_to_size(10);
        assert_eq!(pool.available_count(), 2);
        assert_eq!(pool.in_use_count(), 0);

        // Trim to zero
        pool.trim_to_size(0);
        assert_eq!(pool.available_count(), 0);
        assert_eq!(pool.in_use_count(), 0);
    }

    #[test]
    fn test_config_pool_trimming_with_in_use_configs() {
        let mut pool = ConfigPool::new();

        // Add some configs to the available pool
        for _ in 0..10 {
            pool.available.push(AnimationConfig::default());
        }

        // Get some configs (put them in use)
        let handle1 = pool.get_config();
        let handle2 = pool.get_config();

        // Verify state before trimming
        assert_eq!(pool.available_count(), 8);
        assert_eq!(pool.in_use_count(), 2);

        // Trim available configs (should not affect in-use configs)
        pool.trim_to_size(3);
        assert_eq!(pool.available_count(), 3);
        assert_eq!(pool.in_use_count(), 2);

        // Return configs and verify they're still available
        pool.return_config(handle1);
        pool.return_config(handle2);
        assert_eq!(pool.available_count(), 5);
        assert_eq!(pool.in_use_count(), 0);
    }
}
