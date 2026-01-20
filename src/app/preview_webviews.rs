//! Webview management - YouTube, Audio, Video webviews and visibility updates.
//!
//! ## Memory Optimization
//!
//! Webviews are only created for items within WEBVIEW_PRELOAD_DISTANCE of the viewport.
//! Webviews for items beyond WEBVIEW_UNLOAD_DISTANCE are destroyed after a delay
//! to prevent rapid create/destroy cycles during fast panning.

use super::{Humanboard, PreviewTab, SplitDirection};
use crate::audio_webview::AudioWebView;
use crate::constants::{HEADER_HEIGHT, WEBVIEW_PRELOAD_DISTANCE, WEBVIEW_UNLOAD_DELAY_MS, WEBVIEW_UNLOAD_DISTANCE};
use crate::video_webview::VideoWebView;
use crate::youtube_webview::YouTubeWebView;
use gpui::*;
use std::time::{Duration, Instant};
use tracing::error;

impl Humanboard {
    /// Calculate viewport bounds in canvas coordinates
    fn get_viewport_bounds(&self, window: &Window) -> Option<(f32, f32, f32, f32)> {
        let board = self.board.as_ref()?;
        let bounds = window.bounds();
        let window_width = f32::from(bounds.size.width);
        let window_height = f32::from(bounds.size.height);

        // Account for preview panel
        let (canvas_width, canvas_height) = if let Some(ref preview) = self.preview {
            match preview.split {
                SplitDirection::Vertical => ((1.0 - preview.size) * window_width, window_height),
                SplitDirection::Horizontal => (window_width, (1.0 - preview.size) * window_height),
            }
        } else {
            (window_width, window_height)
        };

        // Convert to canvas coordinates
        let offset_x = f32::from(board.canvas_offset.x);
        let offset_y = f32::from(board.canvas_offset.y);
        let zoom = board.zoom;

        let vp_left = -offset_x / zoom;
        let vp_top = -offset_y / zoom;
        let vp_right = (canvas_width - offset_x) / zoom;
        let vp_bottom = (canvas_height - offset_y) / zoom;

        Some((vp_left, vp_top, vp_right, vp_bottom))
    }

    /// Calculate distance from item to viewport edge (0 if inside viewport)
    fn item_distance_to_viewport(
        item_pos: (f32, f32),
        item_size: (f32, f32),
        viewport: (f32, f32, f32, f32),
    ) -> f32 {
        let (vp_left, vp_top, vp_right, vp_bottom) = viewport;
        let (ix, iy) = item_pos;
        let (iw, ih) = item_size;

        // Calculate distance to viewport edges
        let dist_left = vp_left - (ix + iw);
        let dist_right = ix - vp_right;
        let dist_top = vp_top - (iy + ih);
        let dist_bottom = iy - vp_bottom;

        // Return max of positive distances (0 if overlapping viewport)
        dist_left.max(0.0).max(dist_right.max(0.0)).max(dist_top.max(0.0)).max(dist_bottom.max(0.0))
    }

    /// Ensure YouTube webviews are created for items near viewport.
    /// Destroys webviews for items far from viewport (with delay to prevent thrashing).
    pub fn ensure_youtube_webviews(&mut self, window: &mut Window, cx: &mut App) -> Vec<String> {
        use crate::types::ItemContent;
        let mut errors = Vec::new();

        let Some(ref board) = self.board else {
            self.youtube_webviews.clear();
            self.webview_out_of_range_since.clear();
            return errors;
        };

        let viewport = self.get_viewport_bounds(window);

        // Collect YouTube items with their distance from viewport
        let youtube_items: Vec<(u64, String, (f32, f32), (f32, f32))> = board
            .items
            .iter()
            .filter_map(|item| {
                if let ItemContent::YouTube(video_id) = &item.content {
                    Some((item.id, video_id.clone(), item.position, item.size))
                } else {
                    None
                }
            })
            .collect();

        let now = Instant::now();
        let unload_delay = Duration::from_millis(WEBVIEW_UNLOAD_DELAY_MS);

        // Create WebViews for items within preload distance
        for (item_id, video_id, pos, size) in &youtube_items {
            let distance = viewport
                .map(|vp| Self::item_distance_to_viewport(*pos, *size, vp))
                .unwrap_or(0.0);

            if distance <= WEBVIEW_PRELOAD_DISTANCE {
                // Clear out-of-range tracking since item is now in range
                self.webview_out_of_range_since.remove(item_id);

                if !self.youtube_webviews.contains_key(item_id) {
                    match YouTubeWebView::new(video_id.clone(), window, cx) {
                        Ok(webview) => {
                            self.youtube_webviews.insert(*item_id, webview);
                        }
                        Err(e) => {
                            errors.push(format!("Failed to load YouTube video: {}", e));
                            error!("Failed to create YouTube WebView for video {}: {}", video_id, e);
                        }
                    }
                }
            }
        }

        // Remove WebViews for items far from viewport (with delay)
        let youtube_ids: std::collections::HashSet<u64> =
            youtube_items.iter().map(|(id, _, _, _)| *id).collect();

        let ids_to_remove: Vec<u64> = self
            .youtube_webviews
            .keys()
            .filter(|id| {
                // Always remove if item was deleted
                if !youtube_ids.contains(id) {
                    return true;
                }

                // Check distance for existing items
                if let Some(vp) = viewport {
                    if let Some((_, _, pos, size)) = youtube_items.iter().find(|(i, _, _, _)| i == *id) {
                        let distance = Self::item_distance_to_viewport(*pos, *size, vp);
                        if distance > WEBVIEW_UNLOAD_DISTANCE {
                            // Track when item went out of range
                            let out_since = self.webview_out_of_range_since
                                .entry(**id)
                                .or_insert(now);
                            // Only remove if out of range for long enough
                            return now.duration_since(*out_since) >= unload_delay;
                        }
                    }
                }
                false
            })
            .copied()
            .collect();

        for id in ids_to_remove {
            if let Some(webview) = self.youtube_webviews.remove(&id) {
                webview.hide(cx);
            }
            self.webview_out_of_range_since.remove(&id);
        }

        errors
    }

    /// Ensure Audio webviews are created for items near viewport.
    /// Destroys webviews for items far from viewport (with delay to prevent thrashing).
    pub fn ensure_audio_webviews(&mut self, window: &mut Window, cx: &mut App) -> Vec<String> {
        use crate::types::ItemContent;
        let mut errors = Vec::new();

        let Some(ref board) = self.board else {
            self.audio_webviews.clear();
            return errors;
        };

        let viewport = self.get_viewport_bounds(window);
        let now = Instant::now();
        let unload_delay = Duration::from_millis(WEBVIEW_UNLOAD_DELAY_MS);

        // Collect Audio items with position info
        let audio_items: Vec<(u64, std::path::PathBuf, (f32, f32), (f32, f32))> = board
            .items
            .iter()
            .filter_map(|item| {
                if let ItemContent::Audio(path) = &item.content {
                    Some((item.id, path.clone(), item.position, item.size))
                } else {
                    None
                }
            })
            .collect();

        // Create WebViews for items within preload distance
        for (item_id, path, pos, size) in &audio_items {
            let distance = viewport
                .map(|vp| Self::item_distance_to_viewport(*pos, *size, vp))
                .unwrap_or(0.0);

            if distance <= WEBVIEW_PRELOAD_DISTANCE {
                self.webview_out_of_range_since.remove(item_id);

                if !self.audio_webviews.contains_key(item_id) {
                    match AudioWebView::new(path.clone(), window, cx) {
                        Ok(webview) => {
                            self.audio_webviews.insert(*item_id, webview);
                        }
                        Err(e) => {
                            let filename = path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("audio file");
                            errors.push(format!("Failed to load '{}': {}", filename, e));
                            error!("Failed to create Audio WebView for {:?}: {}", path, e);
                        }
                    }
                }
            }
        }

        // Remove WebViews for items far from viewport (with delay)
        let audio_ids: std::collections::HashSet<u64> =
            audio_items.iter().map(|(id, _, _, _)| *id).collect();
        let ids_to_remove: Vec<u64> = self
            .audio_webviews
            .keys()
            .filter(|id| {
                if !audio_ids.contains(id) {
                    return true;
                }
                if let Some(vp) = viewport {
                    if let Some((_, _, pos, size)) = audio_items.iter().find(|(i, _, _, _)| i == *id) {
                        let distance = Self::item_distance_to_viewport(*pos, *size, vp);
                        if distance > WEBVIEW_UNLOAD_DISTANCE {
                            let out_since = self.webview_out_of_range_since.entry(**id).or_insert(now);
                            return now.duration_since(*out_since) >= unload_delay;
                        }
                    }
                }
                false
            })
            .copied()
            .collect();

        for id in ids_to_remove {
            if let Some(webview) = self.audio_webviews.remove(&id) {
                webview.hide(cx);
            }
            self.webview_out_of_range_since.remove(&id);
        }
        errors
    }

    /// Ensure Video webviews are created for items near viewport.
    /// Destroys webviews for items far from viewport (with delay to prevent thrashing).
    pub fn ensure_video_webviews(&mut self, window: &mut Window, cx: &mut App) -> Vec<String> {
        use crate::types::ItemContent;
        let mut errors = Vec::new();

        let Some(ref board) = self.board else {
            self.video_webviews.clear();
            return errors;
        };

        let viewport = self.get_viewport_bounds(window);
        let now = Instant::now();
        let unload_delay = Duration::from_millis(WEBVIEW_UNLOAD_DELAY_MS);

        // Collect Video items with position info
        let video_items: Vec<(u64, std::path::PathBuf, (f32, f32), (f32, f32))> = board
            .items
            .iter()
            .filter_map(|item| {
                if let ItemContent::Video(path) = &item.content {
                    Some((item.id, path.clone(), item.position, item.size))
                } else {
                    None
                }
            })
            .collect();

        // Create WebViews for items within preload distance
        for (item_id, path, pos, size) in &video_items {
            let distance = viewport
                .map(|vp| Self::item_distance_to_viewport(*pos, *size, vp))
                .unwrap_or(0.0);

            if distance <= WEBVIEW_PRELOAD_DISTANCE {
                self.webview_out_of_range_since.remove(item_id);

                if !self.video_webviews.contains_key(item_id) {
                    match VideoWebView::new(path.clone(), window, cx) {
                        Ok(webview) => {
                            self.video_webviews.insert(*item_id, webview);
                        }
                        Err(e) => {
                            let filename = path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("video file");
                            errors.push(format!("Failed to load '{}': {}", filename, e));
                            error!("Failed to create Video WebView for {:?}: {}", path, e);
                        }
                    }
                }
            }
        }

        // Remove WebViews for items far from viewport (with delay)
        let video_ids: std::collections::HashSet<u64> =
            video_items.iter().map(|(id, _, _, _)| *id).collect();
        let ids_to_remove: Vec<u64> = self
            .video_webviews
            .keys()
            .filter(|id| {
                if !video_ids.contains(id) {
                    return true;
                }
                if let Some(vp) = viewport {
                    if let Some((_, _, pos, size)) = video_items.iter().find(|(i, _, _, _)| i == *id) {
                        let distance = Self::item_distance_to_viewport(*pos, *size, vp);
                        if distance > WEBVIEW_UNLOAD_DISTANCE {
                            let out_since = self.webview_out_of_range_since.entry(**id).or_insert(now);
                            return now.duration_since(*out_since) >= unload_delay;
                        }
                    }
                }
                false
            })
            .copied()
            .collect();

        for id in ids_to_remove {
            if let Some(webview) = self.video_webviews.remove(&id) {
                webview.hide(cx);
            }
            self.webview_out_of_range_since.remove(&id);
        }
        errors
    }

    /// Update webview visibility based on canvas viewport
    /// Hides webviews that are scrolled out of view to prevent z-index issues
    pub fn update_webview_visibility(&mut self, window: &mut Window, cx: &mut App) {
        let Some(ref board) = self.board else { return };

        // Hide all webviews when settings modal or shortcuts overlay is open
        if self.show_settings || self.show_shortcuts {
            for (_, webview) in &self.youtube_webviews {
                webview.webview().update(cx, |wv, _| wv.hide());
            }
            for (_, webview) in &self.audio_webviews {
                webview.webview_entity.update(cx, |wv, _| wv.hide());
            }
            for (_, webview) in &self.video_webviews {
                webview.webview_entity.update(cx, |wv, _| wv.hide());
            }
            // Also hide PDF webviews in preview panel
            if let Some(ref preview) = self.preview {
                for tab in &preview.tabs {
                    if let PreviewTab::Pdf {
                        webview: Some(wv), ..
                    } = tab
                    {
                        wv.webview().update(cx, |view, _| view.hide());
                    }
                }
            }
            return;
        }

        let bounds = window.bounds();
        let window_width = f32::from(bounds.size.width);
        let window_height = f32::from(bounds.size.height);

        // Account for preview panel if open
        let (canvas_width, canvas_height) = if let Some(ref preview) = self.preview {
            match preview.split {
                SplitDirection::Vertical => ((1.0 - preview.size) * window_width, window_height),
                SplitDirection::Horizontal => (window_width, (1.0 - preview.size) * window_height),
            }
        } else {
            (window_width, window_height)
        };

        // Header offset
        let header_height = HEADER_HEIGHT;
        let canvas_top = header_height;

        let zoom = board.zoom;
        let offset_x = f32::from(board.canvas_offset.x);
        let offset_y = f32::from(board.canvas_offset.y);

        // Check each item with a webview
        for item in &board.items {
            let item_x = item.position.0 * zoom + offset_x;
            let item_y = item.position.1 * zoom + offset_y + header_height;
            let item_w = item.size.0 * zoom;
            let item_h = item.size.1 * zoom;

            // Check if item is visible and not overlapping UI chrome
            // Webviews don't clip, so hide if any edge goes outside canvas bounds
            let footer_height = 28.0;
            let canvas_bottom = canvas_height - footer_height;

            let overlaps_header = item_y < canvas_top;
            let overlaps_footer = item_y + item_h > canvas_bottom;
            let overlaps_left = item_x < 0.0;
            let overlaps_right = item_x + item_w > canvas_width;

            let is_visible =
                !overlaps_header && !overlaps_footer && !overlaps_left && !overlaps_right;

            // Update YouTube webview visibility
            if let Some(webview) = self.youtube_webviews.get(&item.id) {
                webview.webview().update(cx, |wv, _| {
                    if is_visible {
                        wv.show();
                    } else {
                        wv.hide();
                    }
                });
            }

            // Update Audio webview visibility
            if let Some(webview) = self.audio_webviews.get(&item.id) {
                webview.webview_entity.update(cx, |wv, _| {
                    if is_visible {
                        wv.show();
                    } else {
                        wv.hide();
                    }
                });
            }

            // Update Video webview visibility
            if let Some(webview) = self.video_webviews.get(&item.id) {
                webview.webview_entity.update(cx, |wv, _| {
                    if is_visible {
                        wv.show();
                    } else {
                        wv.hide();
                    }
                });
            }
        }

        // Show PDF webviews in preview panel (active tab only)
        if let Some(ref preview) = self.preview {
            for (idx, tab) in preview.tabs.iter().enumerate() {
                if let PreviewTab::Pdf {
                    webview: Some(wv), ..
                } = tab
                {
                    if idx == preview.active_tab {
                        wv.webview().update(cx, |view, _| view.show());
                    } else {
                        wv.webview().update(cx, |view, _| view.hide());
                    }
                }
            }
        }
    }
}
