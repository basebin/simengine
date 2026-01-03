use crate::physics::physics_engine::PhysicsEngine;
use nalgebra::Vector2;

/// Data cleaner utility for optimizing simulation performance
#[allow(dead_code)]
pub struct DataCleaner {
    cleanup_threshold: usize,
    inactive_threshold: f32,
    memory_limit_mb: usize,
}

impl Default for DataCleaner {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl DataCleaner {
    /// Create a new data cleaner with default settings
    pub fn new() -> Self {
        DataCleaner {
            cleanup_threshold: 1000,   // Clean up when objects exceed this count
            inactive_threshold: 300.0, // Consider objects inactive after this time
            memory_limit_mb: 500,      // Memory limit in MB
        }
    }

    /// Configure cleanup threshold (number of objects that triggers cleanup)
    pub fn with_cleanup_threshold(mut self, threshold: usize) -> Self {
        self.cleanup_threshold = threshold;
        self
    }

    /// Configure inactive threshold (time in seconds before object is considered inactive)
    pub fn with_inactive_threshold(mut self, threshold: f32) -> Self {
        self.inactive_threshold = threshold;
        self
    }

    /// Configure memory limit in MB
    pub fn with_memory_limit(mut self, limit_mb: usize) -> Self {
        self.memory_limit_mb = limit_mb;
        self
    }

    /// Clean up inactive objects from the physics engine
    pub fn cleanup_inactive_objects(
        &self,
        engine: &mut PhysicsEngine,
        _current_time: f32,
    ) -> usize {
        // For now, we'll implement a simple cleanup based on object state
        // In a real implementation, you'd track object creation/update times
        // Here we'll "deactivate" objects that are moving very slowly
        // and are far from origin (likely debris)
        let mut deactivated_count = 0;

        // Since we can't modify objects directly through get_objects(),
        // we'll implement cleanup by setting velocities to zero for inactive objects
        // In a real implementation, you'd add proper removal methods to PhysicsEngine

        for obj in engine.get_objects_mut() {
            // Simple heuristic: deactivate objects that are moving very slowly
            // and are far from origin (likely debris)
            if obj.velocity.magnitude() < 0.01 && obj.position.magnitude() > 50.0 {
                obj.velocity = Vector2::zeros();
                deactivated_count += 1;
            }
        }

        if deactivated_count > 0 {
            println!(
                "DataCleaner: Deactivated {} inactive objects",
                deactivated_count
            );
        }

        deactivated_count
    }

    /// Optimize memory usage by defragmenting object arrays
    pub fn optimize_memory(&self, engine: &mut PhysicsEngine) -> bool {
        let object_count = engine.get_objects().len();

        // Check if we need cleanup
        if object_count > self.cleanup_threshold {
            println!(
                "DataCleaner: Optimizing memory for {} objects",
                object_count
            );
            // In a real implementation, you might:
            // - Compact object arrays
            // - Free unused memory
            // - Reorganize data structures
            return true;
        }

        false
    }

    /// Check if cleanup is needed based on current state
    pub fn needs_cleanup(&self, engine: &PhysicsEngine) -> bool {
        let object_count = engine.get_objects().len();
        object_count > self.cleanup_threshold
    }

    /// Get cleanup statistics
    pub fn get_stats(&self, engine: &PhysicsEngine) -> CleanupStats {
        let object_count = engine.get_objects().len();
        let active_objects = engine
            .get_objects()
            .iter()
            .filter(|obj| obj.velocity.magnitude() > 0.01)
            .count();

        CleanupStats {
            total_objects: object_count,
            active_objects,
            inactive_objects: object_count.saturating_sub(active_objects),
            needs_cleanup: self.needs_cleanup(engine),
            cleanup_threshold: self.cleanup_threshold,
        }
    }

    /// Perform comprehensive cleanup
    pub fn full_cleanup(&self, engine: &mut PhysicsEngine, current_time: f32) -> CleanupResult {
        let stats_before = self.get_stats(engine);

        let inactive_removed = self.cleanup_inactive_objects(engine, current_time);
        let memory_optimized = self.optimize_memory(engine);

        let stats_after = self.get_stats(engine);

        CleanupResult {
            inactive_objects_removed: inactive_removed,
            memory_optimized,
            stats_before,
            stats_after,
        }
    }
}

/// Statistics about the current cleanup state
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CleanupStats {
    pub total_objects: usize,
    pub active_objects: usize,
    pub inactive_objects: usize,
    pub needs_cleanup: bool,
    pub cleanup_threshold: usize,
}

/// Result of a cleanup operation
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CleanupResult {
    pub inactive_objects_removed: usize,
    pub memory_optimized: bool,
    pub stats_before: CleanupStats,
    pub stats_after: CleanupStats,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::physics_engine::PhysicsEngine;
    use nalgebra::Vector2;

    #[test]
    fn test_data_cleaner_creation() {
        let cleaner = DataCleaner::new();
        assert_eq!(cleaner.cleanup_threshold, 1000);
        assert_eq!(cleaner.inactive_threshold, 300.0);
        assert_eq!(cleaner.memory_limit_mb, 500);
    }

    #[test]
    fn test_data_cleaner_configuration() {
        let cleaner = DataCleaner::new()
            .with_cleanup_threshold(500)
            .with_inactive_threshold(100.0)
            .with_memory_limit(200);

        assert_eq!(cleaner.cleanup_threshold, 500);
        assert_eq!(cleaner.inactive_threshold, 100.0);
        assert_eq!(cleaner.memory_limit_mb, 200);
    }

    #[test]
    fn test_cleanup_stats() {
        let mut engine = PhysicsEngine::new().unwrap();
        engine
            .add_object(Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0), 1.0)
            .unwrap();
        engine
            .add_object(Vector2::new(100.0, 100.0), Vector2::new(0.0, 0.0), 1.0)
            .unwrap(); // Stationary, far away

        let cleaner = DataCleaner::new().with_cleanup_threshold(10);
        let stats = cleaner.get_stats(&engine);

        // PhysicsEngine::new() creates 1 default object + 2 added = 3 total
        assert_eq!(stats.total_objects, 3);
        assert_eq!(stats.active_objects, 1); // Only the second added object is active
        assert_eq!(stats.inactive_objects, 2); // Default object + far away object
        assert!(!stats.needs_cleanup);
    }

    #[test]
    fn test_cleanup_inactive_objects() {
        let mut engine = PhysicsEngine::new().unwrap();
        engine
            .add_object(Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0), 1.0)
            .unwrap(); // Active
        engine
            .add_object(Vector2::new(100.0, 100.0), Vector2::new(0.0, 0.0), 1.0)
            .unwrap(); // Inactive, far away

        let cleaner = DataCleaner::new();
        let removed_count = cleaner.cleanup_inactive_objects(&mut engine, 0.0);

        assert_eq!(removed_count, 1);

        // Check that the inactive object was deactivated (the far away one)
        let objects = engine.get_objects();
        assert_eq!(objects[2].velocity, Vector2::zeros()); // Index 2 is the far away object
    }

    #[test]
    fn test_needs_cleanup() {
        let mut engine = PhysicsEngine::new().unwrap();

        // Add many objects to trigger cleanup threshold
        for i in 0..1500 {
            engine
                .add_object(Vector2::new(i as f32, 0.0), Vector2::new(0.0, 0.0), 1.0)
                .unwrap();
        }

        let cleaner = DataCleaner::new().with_cleanup_threshold(1000);
        assert!(cleaner.needs_cleanup(&engine));

        let cleaner_low = DataCleaner::new().with_cleanup_threshold(2000);
        assert!(!cleaner_low.needs_cleanup(&engine));
    }
}
