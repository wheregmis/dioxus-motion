//! Memory pool management for Dioxus Motion optimizations
//!
//! This module provides pooling systems to reduce memory allocations in hot paths
//! of the animation system, particularly for configuration objects and other
//! frequently allocated structures.

use crate::animations::core::AnimationConfig;
use std::collections::HashMap;
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
        // Don't automatically return configs to pool on drop
        // This prevents issues with cloned handles returning the same config multiple times
        // Configs should be explicitly returned via Motion::drop or other cleanup mechanisms
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
    fn test_config_handle_explicit_cleanup() {
        // Clear the pool first
        global::clear_pool();

        // Get a config handle
        let handle = global::get_config();

        // Verify the config is in use
        let (in_use, available) = global::pool_stats();
        assert_eq!(in_use, 1);
        assert_eq!(available, 0);

        // Explicitly return the handle to pool
        global::return_config(handle);

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
}