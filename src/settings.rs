//! Settings module - handles user preferences including theme selection

use gpui::*;
use gpui_component::theme::{Theme, ThemeMode, ThemeRegistry};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: "Default Dark".to_string(),
        }
    }
}

impl Settings {
    /// Get the settings file path
    fn settings_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home)
            .join(".config")
            .join("humanboard")
            .join("settings.json")
    }

    /// Load settings from disk
    pub fn load() -> Self {
        let path = Self::settings_path();
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(settings) = serde_json::from_str(&content) {
                    return settings;
                }
            }
        }
        Self::default()
    }

    /// Save settings to disk
    pub fn save(&self) {
        let path = Self::settings_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(&path, content);
        }
    }

    /// Apply the current theme
    pub fn apply_theme(&self, cx: &mut App) {
        let theme_name = SharedString::from(self.theme.clone());

        // Get config clone from registry first
        let config = ThemeRegistry::global(cx).themes().get(&theme_name).cloned();

        if let Some(config) = config {
            // Set theme mode based on the config
            let mode = config.mode;

            // Update the appropriate theme slot (light or dark)
            if mode.is_dark() {
                Theme::global_mut(cx).dark_theme = config.clone();
            } else {
                Theme::global_mut(cx).light_theme = config.clone();
            }

            // Apply the config and set mode
            Theme::global_mut(cx).mode = mode;
            Theme::global_mut(cx).apply_config(&config);
            cx.refresh_windows();
        } else {
            // Fallback: check if it's a light/dark theme name
            if self.theme.to_lowercase().contains("light") {
                Theme::change(ThemeMode::Light, None, cx);
            } else {
                Theme::change(ThemeMode::Dark, None, cx);
            }
            cx.refresh_windows();
        }
    }

    /// Get available themes from the registry
    pub fn available_themes(cx: &App) -> Vec<String> {
        let registry = ThemeRegistry::global(cx);
        let themes: Vec<String> = registry
            .sorted_themes()
            .iter()
            .map(|t| t.name.to_string())
            .collect();

        themes
    }
}

/// Initialize themes by watching the themes directory
pub fn init_themes(cx: &mut App) {
    // Get themes directory relative to executable or cwd
    let themes_dir = std::env::current_dir()
        .ok()
        .map(|p| p.join("themes"))
        .unwrap_or_else(|| PathBuf::from("./themes"));

    if themes_dir.exists() {
        let saved_theme = Settings::load().theme;
        let saved_theme_clone = saved_theme.clone();

        if let Err(_err) = ThemeRegistry::watch_dir(themes_dir, cx, move |cx| {
            // Apply saved theme after themes are loaded
            let theme_name = SharedString::from(saved_theme_clone.clone());
            if let Some(config) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
                let mode = config.mode;
                if mode.is_dark() {
                    Theme::global_mut(cx).dark_theme = config.clone();
                } else {
                    Theme::global_mut(cx).light_theme = config.clone();
                }
                Theme::global_mut(cx).mode = mode;
                Theme::global_mut(cx).apply_config(&config);
                cx.refresh_windows();
            }
        }) {
            // Theme watch failed silently
        }
    }
}
