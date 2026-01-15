//! Selection Module
//!
//! This module manages item selection on the canvas, including single selection,
//! multi-selection, and marquee (rubber-band) selection.
//!
//! ## Features
//!
//! - **Single Selection**: Click to select one item
//! - **Multi-Selection**: Shift+click or Cmd+click to add/remove items
//! - **Marquee Selection**: Click and drag to select multiple items
//! - **Select All**: Select all items on the canvas

use gpui::*;
use std::collections::HashSet;

/// State for marquee (rubber-band) selection.
#[derive(Clone, Debug)]
pub struct MarqueeState {
    /// Starting point of the marquee in screen coordinates
    pub start: Point<Pixels>,
    /// Current point of the marquee in screen coordinates
    pub current: Point<Pixels>,
}

impl MarqueeState {
    /// Create a new marquee starting at the given point.
    pub fn new(start: Point<Pixels>) -> Self {
        Self {
            start,
            current: start,
        }
    }

    /// Update the current position of the marquee.
    pub fn update(&mut self, current: Point<Pixels>) {
        self.current = current;
    }

    /// Get the bounding rectangle of the marquee.
    pub fn bounds(&self) -> Bounds<Pixels> {
        let min_x = self.start.x.min(self.current.x);
        let min_y = self.start.y.min(self.current.y);
        let max_x = self.start.x.max(self.current.x);
        let max_y = self.start.y.max(self.current.y);

        Bounds {
            origin: point(min_x, min_y),
            size: size(max_x - min_x, max_y - min_y),
        }
    }

    /// Check if a point is within the marquee bounds.
    pub fn contains(&self, point: Point<Pixels>) -> bool {
        self.bounds().contains(&point)
    }

    /// Check if an item rectangle intersects with the marquee.
    pub fn intersects(&self, item_bounds: Bounds<Pixels>) -> bool {
        let marquee = self.bounds();

        // Check for intersection
        marquee.origin.x < item_bounds.origin.x + item_bounds.size.width
            && marquee.origin.x + marquee.size.width > item_bounds.origin.x
            && marquee.origin.y < item_bounds.origin.y + item_bounds.size.height
            && marquee.origin.y + marquee.size.height > item_bounds.origin.y
    }
}

/// Manages selection state for canvas items.
pub struct SelectionManager {
    /// Set of currently selected item IDs
    selected: HashSet<u64>,
    /// Active marquee selection (if any)
    marquee: Option<MarqueeState>,
    /// Items that were selected before marquee started (for additive selection)
    pre_marquee_selection: HashSet<u64>,
}

impl SelectionManager {
    /// Create a new selection manager.
    pub fn new() -> Self {
        Self {
            selected: HashSet::new(),
            marquee: None,
            pre_marquee_selection: HashSet::new(),
        }
    }

    /// Get the set of selected item IDs.
    pub fn selected(&self) -> &HashSet<u64> {
        &self.selected
    }

    /// Check if any items are selected.
    pub fn has_selection(&self) -> bool {
        !self.selected.is_empty()
    }

    /// Check if a specific item is selected.
    pub fn is_selected(&self, id: u64) -> bool {
        self.selected.contains(&id)
    }

    /// Get the number of selected items.
    pub fn count(&self) -> usize {
        self.selected.len()
    }

    /// Select a single item, clearing any existing selection.
    pub fn select(&mut self, id: u64) {
        self.selected.clear();
        self.selected.insert(id);
    }

    /// Select multiple items, clearing any existing selection.
    pub fn select_many(&mut self, ids: impl IntoIterator<Item = u64>) {
        self.selected.clear();
        self.selected.extend(ids);
    }

    /// Toggle selection of an item (for Cmd+click behavior).
    pub fn toggle(&mut self, id: u64) {
        if self.selected.contains(&id) {
            self.selected.remove(&id);
        } else {
            self.selected.insert(id);
        }
    }

    /// Add an item to the selection (for Shift+click behavior).
    pub fn add(&mut self, id: u64) {
        self.selected.insert(id);
    }

    /// Remove an item from the selection.
    pub fn remove(&mut self, id: u64) {
        self.selected.remove(&id);
    }

    /// Clear all selection.
    pub fn clear(&mut self) {
        self.selected.clear();
    }

    /// Select all items from the given iterator.
    pub fn select_all(&mut self, ids: impl IntoIterator<Item = u64>) {
        self.selected.clear();
        self.selected.extend(ids);
    }

    // ==================== Marquee Selection ====================

    /// Start a marquee selection at the given point.
    pub fn start_marquee(&mut self, start: Point<Pixels>, additive: bool) {
        self.marquee = Some(MarqueeState::new(start));

        if additive {
            // Keep existing selection for additive mode
            self.pre_marquee_selection = self.selected.clone();
        } else {
            // Clear selection for non-additive mode
            self.pre_marquee_selection.clear();
            self.selected.clear();
        }
    }

    /// Update the marquee selection.
    pub fn update_marquee(&mut self, current: Point<Pixels>) {
        if let Some(ref mut marquee) = self.marquee {
            marquee.update(current);
        }
    }

    /// Get the current marquee state.
    pub fn marquee(&self) -> Option<&MarqueeState> {
        self.marquee.as_ref()
    }

    /// Check if a marquee selection is active.
    pub fn is_marquee_active(&self) -> bool {
        self.marquee.is_some()
    }

    /// Complete the marquee selection with the given items that intersect.
    /// Returns the newly selected items.
    pub fn finish_marquee(&mut self, intersecting_ids: impl IntoIterator<Item = u64>) -> Vec<u64> {
        let new_selection: Vec<u64> = intersecting_ids.into_iter().collect();

        // Combine with pre-marquee selection
        self.selected = self.pre_marquee_selection.clone();
        self.selected.extend(new_selection.iter().copied());

        self.marquee = None;
        self.pre_marquee_selection.clear();

        new_selection
    }

    /// Cancel the marquee selection.
    pub fn cancel_marquee(&mut self) {
        // Restore pre-marquee selection
        self.selected = self.pre_marquee_selection.clone();
        self.marquee = None;
        self.pre_marquee_selection.clear();
    }

    /// Update selection during marquee drag (for live preview).
    pub fn update_marquee_selection(&mut self, intersecting_ids: impl IntoIterator<Item = u64>) {
        if self.marquee.is_some() {
            self.selected = self.pre_marquee_selection.clone();
            self.selected.extend(intersecting_ids);
        }
    }
}

impl Default for SelectionManager {
    fn default() -> Self {
        Self::new()
    }
}
