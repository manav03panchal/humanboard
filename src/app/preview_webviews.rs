//! Webview management - YouTube, Audio, Video webviews and visibility updates.

use super::{Humanboard, PreviewTab, SplitDirection};
use crate::audio_webview::AudioWebView;
use crate::video_webview::VideoWebView;
use crate::youtube_webview::YouTubeWebView;
use gpui::*;
use tracing::error;

impl Humanboard {
    /// Ensure YouTube webviews are created for YouTube items.
    /// Returns a list of error messages for any webviews that failed to create.
    pub fn ensure_youtube_webviews(&mut self, window: &mut Window, cx: &mut App) -> Vec<String> {
        use crate::types::ItemContent;
        let mut errors = Vec::new();

        let Some(ref board) = self.board else {
            self.youtube_webviews.clear();
            return errors;
        };

        // Collect YouTube item IDs and video IDs
        let youtube_items: Vec<(u64, String)> = board
            .items
            .iter()
            .filter_map(|item| {
                if let ItemContent::YouTube(video_id) = &item.content {
                    Some((item.id, video_id.clone()))
                } else {
                    None
                }
            })
            .collect();

        // Create WebViews for new YouTube items
        for (item_id, video_id) in &youtube_items {
            if !self.youtube_webviews.contains_key(item_id) {
                match YouTubeWebView::new(video_id.clone(), window, cx) {
                    Ok(webview) => {
                        self.youtube_webviews.insert(*item_id, webview);
                    }
                    Err(e) => {
                        errors.push(format!("Failed to load YouTube video: {}", e));
                        error!(
                            "Failed to create YouTube WebView for video {}: {}",
                            video_id, e
                        );
                    }
                }
            }
        }

        // Remove WebViews for deleted items (hide before dropping to prevent orphaned UI)
        let youtube_ids: std::collections::HashSet<u64> =
            youtube_items.iter().map(|(id, _)| *id).collect();
        let ids_to_remove: Vec<u64> = self
            .youtube_webviews
            .keys()
            .filter(|id| !youtube_ids.contains(id))
            .copied()
            .collect();
        for id in ids_to_remove {
            if let Some(webview) = self.youtube_webviews.remove(&id) {
                webview.hide(cx);
            }
        }
        errors
    }

    /// Ensure Audio webviews are created for Audio items.
    /// Returns a list of error messages for any webviews that failed to create.
    pub fn ensure_audio_webviews(&mut self, window: &mut Window, cx: &mut App) -> Vec<String> {
        use crate::types::ItemContent;
        let mut errors = Vec::new();

        let Some(ref board) = self.board else {
            self.audio_webviews.clear();
            return errors;
        };

        // Collect Audio item IDs and paths
        let audio_items: Vec<(u64, std::path::PathBuf)> = board
            .items
            .iter()
            .filter_map(|item| {
                if let ItemContent::Audio(path) = &item.content {
                    Some((item.id, path.clone()))
                } else {
                    None
                }
            })
            .collect();

        // Create WebViews for new Audio items
        for (item_id, path) in &audio_items {
            if !self.audio_webviews.contains_key(item_id) {
                tracing::info!("[AUDIO] Creating new webview for item_id={}, existing_ids={:?}",
                    item_id, self.audio_webviews.keys().collect::<Vec<_>>());
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

        // Remove WebViews for deleted items (hide before dropping to prevent orphaned UI)
        let audio_ids: std::collections::HashSet<u64> =
            audio_items.iter().map(|(id, _)| *id).collect();
        let ids_to_remove: Vec<u64> = self
            .audio_webviews
            .keys()
            .filter(|id| !audio_ids.contains(id))
            .copied()
            .collect();
        for id in ids_to_remove {
            tracing::info!("[AUDIO] Removing webview for item_id={}", id);
            if let Some(webview) = self.audio_webviews.remove(&id) {
                webview.hide(cx);
            }
        }
        errors
    }

    /// Ensure Video webviews are created for Video items.
    /// Returns a list of error messages for any webviews that failed to create.
    pub fn ensure_video_webviews(&mut self, window: &mut Window, cx: &mut App) -> Vec<String> {
        use crate::types::ItemContent;
        let mut errors = Vec::new();

        let Some(ref board) = self.board else {
            self.video_webviews.clear();
            return errors;
        };

        // Collect Video item IDs and paths
        let video_items: Vec<(u64, std::path::PathBuf)> = board
            .items
            .iter()
            .filter_map(|item| {
                if let ItemContent::Video(path) = &item.content {
                    Some((item.id, path.clone()))
                } else {
                    None
                }
            })
            .collect();

        // Create WebViews for new Video items
        for (item_id, path) in &video_items {
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

        // Remove WebViews for deleted items (hide before dropping to prevent orphaned UI)
        let video_ids: std::collections::HashSet<u64> =
            video_items.iter().map(|(id, _)| *id).collect();
        let ids_to_remove: Vec<u64> = self
            .video_webviews
            .keys()
            .filter(|id| !video_ids.contains(id))
            .copied()
            .collect();
        for id in ids_to_remove {
            if let Some(webview) = self.video_webviews.remove(&id) {
                webview.hide(cx);
            }
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
        let header_height = 40.0;
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
