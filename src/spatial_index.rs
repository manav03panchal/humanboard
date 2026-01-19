//! Spatial Index Module
//!
//! Provides R-tree based spatial indexing for efficient hit testing on the canvas.
//! This reduces hit testing from O(n) to O(log n) for point queries.

use rstar::{RTree, RTreeObject, AABB};
use std::collections::HashMap;

/// A spatial entry representing a canvas item's bounding box.
#[derive(Debug, Clone, Copy)]
pub struct SpatialEntry {
    pub item_id: u64,
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl SpatialEntry {
    pub fn new(item_id: u64, position: (f32, f32), size: (f32, f32)) -> Self {
        Self {
            item_id,
            min_x: position.0,
            min_y: position.1,
            max_x: position.0 + size.0,
            max_y: position.1 + size.1,
        }
    }

    #[inline]
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }
}

impl RTreeObject for SpatialEntry {
    type Envelope = AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners([self.min_x, self.min_y], [self.max_x, self.max_y])
    }
}

impl PartialEq for SpatialEntry {
    fn eq(&self, other: &Self) -> bool {
        self.item_id == other.item_id
    }
}

/// Spatial index for canvas items using an R-tree.
/// Provides O(log n) point queries and range queries for hit testing.
pub struct SpatialIndex {
    tree: RTree<SpatialEntry>,
    entries: HashMap<u64, SpatialEntry>,
}

impl SpatialIndex {
    pub fn new() -> Self {
        Self {
            tree: RTree::new(),
            entries: HashMap::new(),
        }
    }

    /// Build a spatial index from an iterator of items.
    pub fn from_items<I>(items: I) -> Self
    where
        I: Iterator<Item = (u64, (f32, f32), (f32, f32))>,
    {
        let entries: Vec<SpatialEntry> = items
            .map(|(id, pos, size)| SpatialEntry::new(id, pos, size))
            .collect();

        let entries_map: HashMap<u64, SpatialEntry> =
            entries.iter().map(|e| (e.item_id, *e)).collect();

        Self {
            tree: RTree::bulk_load(entries),
            entries: entries_map,
        }
    }

    pub fn insert(&mut self, item_id: u64, position: (f32, f32), size: (f32, f32)) {
        if let Some(old_entry) = self.entries.remove(&item_id) {
            self.tree.remove(&old_entry);
        }

        let entry = SpatialEntry::new(item_id, position, size);
        self.tree.insert(entry);
        self.entries.insert(item_id, entry);
    }

    pub fn remove(&mut self, item_id: u64) -> bool {
        if let Some(entry) = self.entries.remove(&item_id) {
            self.tree.remove(&entry);
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, item_id: u64, position: (f32, f32), size: (f32, f32)) {
        self.insert(item_id, position, size);
    }

    /// Query all items that contain the given point (in canvas coordinates).
    pub fn query_point(&self, x: f32, y: f32) -> Vec<u64> {
        let point_envelope = AABB::from_point([x, y]);

        self.tree
            .locate_in_envelope_intersecting(&point_envelope)
            .filter(|entry| entry.contains_point(x, y))
            .map(|entry| entry.item_id)
            .collect()
    }

    /// Query all items that intersect a rectangular region.
    pub fn query_rect(&self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Vec<u64> {
        let envelope = AABB::from_corners([min_x, min_y], [max_x, max_y]);

        self.tree
            .locate_in_envelope_intersecting(&envelope)
            .map(|entry| entry.item_id)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn rebuild<I>(&mut self, items: I)
    where
        I: Iterator<Item = (u64, (f32, f32), (f32, f32))>,
    {
        let entries: Vec<SpatialEntry> = items
            .map(|(id, pos, size)| SpatialEntry::new(id, pos, size))
            .collect();

        self.entries = entries.iter().map(|e| (e.item_id, *e)).collect();
        self.tree = RTree::bulk_load(entries);
    }

    pub fn clear(&mut self) {
        self.tree = RTree::new();
        self.entries.clear();
    }
}

impl Default for SpatialIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_query() {
        let mut index = SpatialIndex::new();
        index.insert(1, (0.0, 0.0), (100.0, 100.0));
        index.insert(2, (50.0, 50.0), (100.0, 100.0));
        index.insert(3, (200.0, 200.0), (50.0, 50.0));

        let results = index.query_point(25.0, 25.0);
        assert_eq!(results.len(), 1);
        assert!(results.contains(&1));

        let results = index.query_point(75.0, 75.0);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_remove() {
        let mut index = SpatialIndex::new();
        index.insert(1, (0.0, 0.0), (100.0, 100.0));
        assert_eq!(index.len(), 1);

        index.remove(1);
        assert_eq!(index.len(), 0);
        assert!(index.query_point(50.0, 50.0).is_empty());
    }

    #[test]
    fn test_query_rect() {
        let mut index = SpatialIndex::new();
        index.insert(1, (0.0, 0.0), (100.0, 100.0));
        index.insert(2, (150.0, 150.0), (100.0, 100.0));

        let results = index.query_rect(25.0, 25.0, 75.0, 75.0);
        assert_eq!(results.len(), 1);
        assert!(results.contains(&1));
    }
}
