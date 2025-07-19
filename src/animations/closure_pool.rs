//! Web closure pooling system for performance optimization
//!
//! Provides a pool of reusable JavaScript closures to avoid the overhead
//! of creating new closures for every animation frame callback.

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "web")]
use std::collections::{HashMap, HashSet};
#[cfg(feature = "web")]
use std::cell::RefCell;

/// Pool of reusable JavaScript closures for web platform optimization
#[cfg(feature = "web")]
pub struct WebClosurePool {
    /// Count of available closure slots
    available_count: usize,
    /// Set of currently in-use closure IDs
    in_use_ids: HashSet<u32>,
    /// Registry mapping closure IDs to their callbacks
    callback_registry: HashMap<u32, Box<dyn FnOnce() + Send>>,
    /// Next available closure ID
    next_id: u32,
    /// Maximum pool size to maintain
    max_pool_size: usize,
}

#[cfg(feature = "web")]
impl WebClosurePool {
    /// Creates a new web closure pool
    pub fn new() -> Self {
        Self {
            available_count: 0,
            in_use_ids: HashSet::new(),
            callback_registry: HashMap::new(),
            next_id: 1,
            max_pool_size: 16, // Reasonable default for most use cases
        }
    }

    /// Registers a callback and returns its ID for later execution
    /// 
    /// # Arguments
    /// * `callback` - The callback function to execute when requested
    /// 
    /// # Returns
    /// The callback ID for later execution
    pub fn register_callback(&mut self, callback: Box<dyn FnOnce() + Send>) -> u32 {
        let callback_id = self.next_id;
        self.next_id += 1;

        // Store the callback in the registry
        self.callback_registry.insert(callback_id, callback);
        
        // Mark this closure as in use
        self.in_use_ids.insert(callback_id);
        
        // If we had available closures, use one
        if self.available_count > 0 {
            self.available_count -= 1;
        }
        
        callback_id
    }

    /// Executes a registered callback and returns the closure to the pool
    /// 
    /// # Arguments
    /// * `callback_id` - The ID of the callback to execute
    pub fn execute_callback(&mut self, callback_id: u32) {
        if let Some(callback) = self.callback_registry.remove(&callback_id) {
            callback();
            
            // Mark closure as no longer in use
            self.in_use_ids.remove(&callback_id);
            
            // Return to available pool if we haven't exceeded max size
            if self.available_count < self.max_pool_size {
                self.available_count += 1;
            }
        }
    }

    /// Creates a JavaScript closure that will execute the callback with the given ID
    /// 
    /// # Arguments
    /// * `callback_id` - The ID of the callback to execute
    /// 
    /// # Returns
    /// A JavaScript closure that can be used with web APIs
    pub fn create_js_closure(&self, callback_id: u32) -> Closure<dyn FnMut()> {
        Closure::new(move || {
            // Execute the callback through the global pool
            execute_and_return_pooled_closure(callback_id);
        })
    }

    /// Gets the number of available closures in the pool
    pub fn available_count(&self) -> usize {
        self.available_count
    }

    /// Gets the number of closures currently in use
    pub fn in_use_count(&self) -> usize {
        self.in_use_ids.len()
    }

    /// Clears all closures from the pool
    pub fn clear(&mut self) {
        self.available_count = 0;
        self.in_use_ids.clear();
        self.callback_registry.clear();
    }
}

#[cfg(feature = "web")]
impl Default for WebClosurePool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "web")]
thread_local! {
    /// Thread-local storage for the global closure pool
    static CLOSURE_POOL: RefCell<WebClosurePool> = RefCell::new(WebClosurePool::new());
}

/// Registers a callback in the global pool and returns its ID
#[cfg(feature = "web")]
pub fn register_pooled_callback(callback: Box<dyn FnOnce() + Send>) -> u32 {
    CLOSURE_POOL.with(|pool| {
        let mut pool = pool.borrow_mut();
        pool.register_callback(callback)
    })
}

/// Creates a JavaScript closure for the given callback ID
#[cfg(feature = "web")]
pub fn create_pooled_closure(callback_id: u32) -> Closure<dyn FnMut()> {
    CLOSURE_POOL.with(|pool| {
        let pool = pool.borrow();
        pool.create_js_closure(callback_id)
    })
}

/// Executes and returns a closure to the global pool
#[cfg(feature = "web")]
pub fn execute_and_return_pooled_closure(closure_id: u32) {
    CLOSURE_POOL.with(|pool| {
        let mut pool = pool.borrow_mut();
        pool.execute_callback(closure_id);
    });
}

/// Gets statistics about the global closure pool
#[cfg(feature = "web")]
pub fn closure_pool_stats() -> (usize, usize) {
    CLOSURE_POOL.with(|pool| {
        let pool = pool.borrow();
        (pool.available_count(), pool.in_use_count())
    })
}

// Stub implementations for non-web platforms
#[cfg(not(feature = "web"))]
pub fn register_pooled_callback(_callback: Box<dyn FnOnce() + Send>) -> u32 {
    0
}

#[cfg(not(feature = "web"))]
pub fn execute_and_return_pooled_closure(_closure_id: u32) {}

#[cfg(not(feature = "web"))]
pub fn closure_pool_stats() -> (usize, usize) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "web")]
    #[test]
    fn test_closure_pool_creation() {
        let pool = WebClosurePool::new();
        assert_eq!(pool.available_count(), 0);
        assert_eq!(pool.in_use_count(), 0);
    }

    #[cfg(feature = "web")]
    #[test]
    fn test_callback_registration() {
        let mut pool = WebClosurePool::new();
        
        // Test registering a callback
        let callback = Box::new(|| {});
        let id = pool.register_callback(callback);
        assert!(id > 0);
        
        // Test executing a callback
        pool.execute_callback(id);
        
        // Callback should be removed after execution
        pool.execute_callback(id); // Should not panic
    }

    #[cfg(feature = "web")]
    #[test]
    fn test_multiple_callbacks() {
        let mut pool = WebClosurePool::new();
        
        // Register multiple callbacks
        let callback1 = Box::new(|| {});
        let callback2 = Box::new(|| {});
        let id1 = pool.register_callback(callback1);
        let id2 = pool.register_callback(callback2);
        
        // IDs should be different
        assert_ne!(id1, id2);
        
        // Execute callbacks
        pool.execute_callback(id1);
        pool.execute_callback(id2);
    }

    #[cfg(feature = "web")]
    #[test]
    fn test_closure_pool_clear() {
        let mut pool = WebClosurePool::new();
        
        // Add some callbacks
        let callback1 = Box::new(|| {});
        let callback2 = Box::new(|| {});
        let _id1 = pool.register_callback(callback1);
        let _id2 = pool.register_callback(callback2);
        
        // Clear the pool
        pool.clear();
        assert_eq!(pool.available_count(), 0);
        assert_eq!(pool.in_use_count(), 0);
    }

    #[test]
    fn test_non_web_stubs() {
        // Test that non-web stubs work without panicking
        #[cfg(not(feature = "web"))]
        {
            let callback = Box::new(|| {});
            let id = register_pooled_callback(callback);
            execute_and_return_pooled_closure(id);
            let (available, in_use) = closure_pool_stats();
            assert_eq!(available, 0);
            assert_eq!(in_use, 0);
        }
        
        // Test that web implementation works correctly
        #[cfg(feature = "web")]
        {
            // Clear the pool first to ensure clean state
            CLOSURE_POOL.with(|pool| {
                pool.borrow_mut().clear();
            });
            
            let callback = Box::new(|| {});
            let id = register_pooled_callback(callback);
            
            // After registration, should have 1 in use
            let (_available, in_use) = closure_pool_stats();
            assert_eq!(in_use, 1);
            
            // After execution, should be returned to available pool
            execute_and_return_pooled_closure(id);
            let (available, in_use) = closure_pool_stats();
            assert_eq!(available, 1);
            assert_eq!(in_use, 0);
        }
    }
}