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
            Theme::global_mut(cx).apply_config(&config);
        } else {
            // Fallback to dark mode
            Theme::change(ThemeMode::Dark, None, cx);
        }
    }

    /// Get available themes from the registry
    pub fn available_themes(cx: &App) -> Vec<String> {
        let registry = ThemeRegistry::global(cx);
        let mut themes: Vec<String> = registry
            .sorted_themes()
            .iter()
            .map(|t| t.name.to_string())
            .collect();

        // If no custom themes, add default options
        if themes.is_empty() {
            themes.push("Default Light".to_string());
            themes.push("Default Dark".to_string());
        }

        themes
    }
}
