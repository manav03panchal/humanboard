//! Spatial indexing module for efficient item lookup.
//!
//! Implements a grid-based spatial index for O(1) average-case item lookup
//! during mouse interactions. This dramatically improves performance when
//! the canvas contains many items.

use std::collections::HashMap;

/// Grid cell size in canvas units (not screen pixels)
const CELL_SIZE: f32 = 200.0;

/// A cell coordinate in the spatial grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellCoord {
    pub x: i32,
    pub y: i32,
}

impl CellCoord {
    /// Create a cell coordinate from canvas position
    #[inline]
    pub fn from_position(x: f32, y: f32) -> Self {
        Self {
            x: (x / CELL_SIZE).floor() as i32,
            y: (y / CELL_SIZE).floor() as i32,
        }
    }

    /// Get all cells that an axis-aligned bounding box overlaps
    pub fn cells_for_bounds(x: f32, y: f32, width: f32, height: f32) -> Vec<CellCoord> {
        let min_cell = Self::from_position(x, y);
        let max_cell = Self::from_position(x + width, y + height);

        let mut cells = Vec::with_capacity(
            ((max_cell.x - min_cell.x + 1) * (max_cell.y - min_cell.y + 1)) as usize,
        );

        for cx in min_cell.x..=max_cell.x {
            for cy in min_cell.y..=max_cell.y {
                cells.push(CellCoord { x: cx, y: cy });
            }
        }

        cells
    }
}

/// Entry in the spatial index containing item metadata for quick access
#[derive(Debug, Clone)]
pub struct SpatialEntry {
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    /// Index in the items vector for quick lookup
    pub vec_index: usize,
}

impl SpatialEntry {
    /// Check if a point (in canvas coordinates) is inside this entry's bounds
    #[inline]
    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }

    /// Get the cells this entry occupies
    pub fn cells(&self) -> Vec<CellCoord> {
        CellCoord::cells_for_bounds(self.x, self.y, self.width, self.height)
    }
}

/// Grid-based spatial index for fast item lookup
#[derive(Debug, Default)]
pub struct SpatialIndex {
    /// Map from cell coordinates to item IDs in that cell
    grid: HashMap<CellCoord, Vec<u64>>,
    /// Map from item ID to spatial entry (for quick bounds lookup)
    entries: HashMap<u64, SpatialEntry>,
    /// Track if index needs rebuilding
    dirty: bool,
}

impl SpatialIndex {
    /// Create a new empty spatial index
    pub fn new() -> Self {
        Self {
            grid: HashMap::new(),
            entries: HashMap::new(),
            dirty: false,
        }
    }

    /// Clear the entire index
    pub fn clear(&mut self) {
        self.grid.clear();
        self.entries.clear();
        self.dirty = false;
    }

    /// Insert or update an item in the index
    pub fn insert(&mut self, id: u64, x: f32, y: f32, width: f32, height: f32, vec_index: usize) {
        // Remove old entry if exists
        self.remove(id);

        let entry = SpatialEntry {
            id,
            x,
            y,
            width,
            height,
            vec_index,
        };

        // Insert into all overlapping cells
        for cell in entry.cells() {
            self.grid.entry(cell).or_default().push(id);
        }

        self.entries.insert(id, entry);
    }

    /// Remove an item from the index
    pub fn remove(&mut self, id: u64) {
        if let Some(entry) = self.entries.remove(&id) {
            // Remove from all cells
            for cell in entry.cells() {
                if let Some(cell_items) = self.grid.get_mut(&cell) {
                    cell_items.retain(|&item_id| item_id != id);
                    // Clean up empty cells
                    if cell_items.is_empty() {
                        self.grid.remove(&cell);
                    }
                }
            }
        }
    }

    /// Update an item's position/size
    #[inline]
    pub fn update(&mut self, id: u64, x: f32, y: f32, width: f32, height: f32, vec_index: usize) {
        // Check if bounds actually changed
        if let Some(entry) = self.entries.get(&id) {
            if (entry.x - x).abs() < 0.001
                && (entry.y - y).abs() < 0.001
                && (entry.width - width).abs() < 0.001
                && (entry.height - height).abs() < 0.001
            {
                // Only vec_index might have changed
                if entry.vec_index != vec_index {
                    if let Some(e) = self.entries.get_mut(&id) {
                        e.vec_index = vec_index;
                    }
                }
                return;
            }
        }

        // Bounds changed, need full update
        self.insert(id, x, y, width, height, vec_index);
    }

    /// Query items at a point (in canvas coordinates).
    /// Returns item IDs in reverse order (top items first for z-order).
    pub fn query_point(&self, x: f32, y: f32) -> Vec<u64> {
        let cell = CellCoord::from_position(x, y);

        let mut results = Vec::new();

        if let Some(cell_items) = self.grid.get(&cell) {
            for &id in cell_items {
                if let Some(entry) = self.entries.get(&id) {
                    if entry.contains_point(x, y) {
                        results.push(id);
                    }
                }
            }
        }

        // Sort by vec_index descending (top items first)
        results.sort_by(|a, b| {
            let idx_a = self.entries.get(a).map(|e| e.vec_index).unwrap_or(0);
            let idx_b = self.entries.get(b).map(|e| e.vec_index).unwrap_or(0);
            idx_b.cmp(&idx_a)
        });

        results
    }

    /// Query items in a rectangular region
    pub fn query_rect(&self, x: f32, y: f32, width: f32, height: f32) -> Vec<u64> {
        let cells = CellCoord::cells_for_bounds(x, y, width, height);
        let mut seen = std::collections::HashSet::new();
        let mut results = Vec::new();

        for cell in cells {
            if let Some(cell_items) = self.grid.get(&cell) {
                for &id in cell_items {
                    if seen.insert(id) {
                        if let Some(entry) = self.entries.get(&id) {
                            // Check for AABB intersection
                            if entry.x < x + width
                                && entry.x + entry.width > x
                                && entry.y < y + height
                                && entry.y + entry.height > y
                            {
                                results.push(id);
                            }
                        }
                    }
                }
            }
        }

        results
    }

    /// Get the entry for an item by ID
    pub fn get(&self, id: u64) -> Option<&SpatialEntry> {
        self.entries.get(&id)
    }

    /// Get the number of items in the index
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the index is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Mark the index as needing a rebuild
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Check if the index needs rebuilding
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Rebuild the index from scratch given items
    pub fn rebuild<'a, I>(&mut self, items: I)
    where
        I: Iterator<Item = (u64, f32, f32, f32, f32, usize)>,
    {
        self.clear();
        for (id, x, y, w, h, idx) in items {
            self.insert(id, x, y, w, h, idx);
        }
        self.dirty = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_coord_from_position() {
        let cell = CellCoord::from_position(0.0, 0.0);
        assert_eq!(cell, CellCoord { x: 0, y: 0 });

        let cell = CellCoord::from_position(199.0, 199.0);
        assert_eq!(cell, CellCoord { x: 0, y: 0 });

        let cell = CellCoord::from_position(200.0, 200.0);
        assert_eq!(cell, CellCoord { x: 1, y: 1 });

        let cell = CellCoord::from_position(-100.0, -100.0);
        assert_eq!(cell, CellCoord { x: -1, y: -1 });
    }

    #[test]
    fn test_cells_for_bounds() {
        // Small item in one cell
        let cells = CellCoord::cells_for_bounds(50.0, 50.0, 50.0, 50.0);
        assert_eq!(cells.len(), 1);

        // Item spanning 4 cells
        let cells = CellCoord::cells_for_bounds(150.0, 150.0, 100.0, 100.0);
        assert_eq!(cells.len(), 4);
    }

    #[test]
    fn test_spatial_entry_contains_point() {
        let entry = SpatialEntry {
            id: 1,
            x: 100.0,
            y: 100.0,
            width: 200.0,
            height: 150.0,
            vec_index: 0,
        };

        assert!(entry.contains_point(150.0, 150.0)); // Inside
        assert!(entry.contains_point(100.0, 100.0)); // Top-left corner
        assert!(entry.contains_point(300.0, 250.0)); // Bottom-right corner
        assert!(!entry.contains_point(50.0, 50.0)); // Outside
        assert!(!entry.contains_point(350.0, 150.0)); // Outside right
    }

    #[test]
    fn test_spatial_index_insert_and_query() {
        let mut index = SpatialIndex::new();

        index.insert(1, 100.0, 100.0, 50.0, 50.0, 0);
        index.insert(2, 200.0, 200.0, 50.0, 50.0, 1);

        // Query point in first item
        let results = index.query_point(125.0, 125.0);
        assert_eq!(results, vec![1]);

        // Query point in second item
        let results = index.query_point(225.0, 225.0);
        assert_eq!(results, vec![2]);

        // Query point in empty space
        let results = index.query_point(0.0, 0.0);
        assert!(results.is_empty());
    }

    #[test]
    fn test_spatial_index_overlapping_items() {
        let mut index = SpatialIndex::new();

        // Two overlapping items
        index.insert(1, 100.0, 100.0, 100.0, 100.0, 0);
        index.insert(2, 150.0, 150.0, 100.0, 100.0, 1);

        // Query point in overlap region - should return both, top first
        let results = index.query_point(175.0, 175.0);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], 2); // Higher vec_index = on top
        assert_eq!(results[1], 1);
    }

    #[test]
    fn test_spatial_index_remove() {
        let mut index = SpatialIndex::new();

        index.insert(1, 100.0, 100.0, 50.0, 50.0, 0);
        assert_eq!(index.len(), 1);

        index.remove(1);
        assert_eq!(index.len(), 0);

        let results = index.query_point(125.0, 125.0);
        assert!(results.is_empty());
    }

    #[test]
    fn test_spatial_index_update() {
        let mut index = SpatialIndex::new();

        index.insert(1, 100.0, 100.0, 50.0, 50.0, 0);

        // Query old position
        assert!(!index.query_point(125.0, 125.0).is_empty());

        // Update position
        index.update(1, 500.0, 500.0, 50.0, 50.0, 0);

        // Old position should be empty
        assert!(index.query_point(125.0, 125.0).is_empty());

        // New position should have item
        assert!(!index.query_point(525.0, 525.0).is_empty());
    }

    #[test]
    fn test_spatial_index_query_rect() {
        let mut index = SpatialIndex::new();

        index.insert(1, 100.0, 100.0, 50.0, 50.0, 0);
        index.insert(2, 200.0, 200.0, 50.0, 50.0, 1);
        index.insert(3, 1000.0, 1000.0, 50.0, 50.0, 2);

        // Query rect that includes first two items
        let results = index.query_rect(50.0, 50.0, 250.0, 250.0);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&1));
        assert!(results.contains(&2));
        assert!(!results.contains(&3));
    }

    #[test]
    fn test_spatial_index_rebuild() {
        let mut index = SpatialIndex::new();

        let items = vec![
            (1u64, 100.0f32, 100.0f32, 50.0f32, 50.0f32, 0usize),
            (2, 200.0, 200.0, 50.0, 50.0, 1),
            (3, 300.0, 300.0, 50.0, 50.0, 2),
        ];

        index.rebuild(items.into_iter());

        assert_eq!(index.len(), 3);
        assert!(!index.query_point(125.0, 125.0).is_empty());
        assert!(!index.query_point(225.0, 225.0).is_empty());
        assert!(!index.query_point(325.0, 325.0).is_empty());
    }
}
