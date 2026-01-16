//! Settings module - Layered settings system following Zed patterns
//!
//! This module implements a layered settings architecture where settings
//! from multiple sources are merged with clear precedence:
//!
//! 1. Default settings (lowest priority)
//! 2. User settings (~/.config/humanboard/settings.json)
//! 3. Project settings (.humanboard/settings.json) (highest priority)

use crate::error::SettingsError;
use gpui::*;
use gpui_component::theme::{Theme, ThemeMode, ThemeRegistry};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use tracing::{debug, error, info, warn};

// ============================================================================
// Settings Trait - Core abstraction for typed settings access
// ============================================================================

/// Trait for types that represent a settings category.
///
/// Implementors define how to extract their settings from the merged
/// settings content and provide type-safe access to configuration values.
pub trait Setting: Sized + Clone + 'static {
    /// Returns the unique key for this settings category in the JSON file.
    fn key() -> &'static str;

    /// Creates a new instance from the merged settings content.
    /// Falls back to defaults for any missing values.
    fn from_content(content: &SettingsContent) -> Self;

    /// Returns the default settings values.
    fn default_value() -> Self;
}

// ============================================================================
// Settings Content - The raw JSON-serializable settings structure
// ============================================================================

/// Raw settings content that can be parsed from JSON.
/// All fields are optional to support partial overrides.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsContent {
    /// UI theme name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,

    /// Font family for UI
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,

    /// Font size in pixels
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f32>,

    /// Canvas background color (hex)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canvas_background: Option<String>,

    /// Default grid size for snapping
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grid_size: Option<f32>,

    /// Whether to show grid lines
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_grid: Option<bool>,

    /// Whether to enable snap-to-grid
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snap_to_grid: Option<bool>,

    /// Auto-save interval in seconds (0 to disable)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_save_interval: Option<u64>,

    /// Maximum undo history depth
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_undo_history: Option<usize>,

    /// Zoom sensitivity multiplier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zoom_sensitivity: Option<f32>,

    /// Pan sensitivity multiplier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pan_sensitivity: Option<f32>,

    /// Whether onboarding has been completed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_completed: Option<bool>,

    /// Reduce motion preference: "system" (default), "on", or "off"
    /// When "system", follows OS prefers-reduced-motion setting
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduce_motion: Option<String>,

    /// Whether high contrast mode is enabled (accessibility)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high_contrast: Option<bool>,
}

impl SettingsContent {
    /// Merge another settings content into this one.
    /// Values from `other` override values in `self`.
    pub fn merge_from(&mut self, other: &SettingsContent) {
        if other.theme.is_some() {
            self.theme = other.theme.clone();
        }
        if other.font.is_some() {
            self.font = other.font.clone();
        }
        if other.font_size.is_some() {
            self.font_size = other.font_size;
        }
        if other.canvas_background.is_some() {
            self.canvas_background = other.canvas_background.clone();
        }
        if other.grid_size.is_some() {
            self.grid_size = other.grid_size;
        }
        if other.show_grid.is_some() {
            self.show_grid = other.show_grid;
        }
        if other.snap_to_grid.is_some() {
            self.snap_to_grid = other.snap_to_grid;
        }
        if other.auto_save_interval.is_some() {
            self.auto_save_interval = other.auto_save_interval;
        }
        if other.max_undo_history.is_some() {
            self.max_undo_history = other.max_undo_history;
        }
        if other.zoom_sensitivity.is_some() {
            self.zoom_sensitivity = other.zoom_sensitivity;
        }
        if other.pan_sensitivity.is_some() {
            self.pan_sensitivity = other.pan_sensitivity;
        }
        if other.onboarding_completed.is_some() {
            self.onboarding_completed = other.onboarding_completed;
        }
        if other.reduce_motion.is_some() {
            self.reduce_motion = other.reduce_motion.clone();
        }
        if other.high_contrast.is_some() {
            self.high_contrast = other.high_contrast;
        }
    }
}

// ============================================================================
// App Settings - The main settings type
// ============================================================================

/// Main application settings with resolved (non-optional) values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub font: String,
    pub font_size: f32,
    pub canvas_background: String,
    pub grid_size: f32,
    pub show_grid: bool,
    pub snap_to_grid: bool,
    pub auto_save_interval: u64,
    pub max_undo_history: usize,
    pub zoom_sensitivity: f32,
    pub pan_sensitivity: f32,
    /// Reduce motion preference: "system", "on", or "off"
    pub reduce_motion: String,
    pub high_contrast: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "Default Dark".to_string(),
            font: "Iosevka Nerd Font".to_string(),
            font_size: 14.0,
            canvas_background: "#1a1a1a".to_string(),
            grid_size: 20.0,
            show_grid: false,
            snap_to_grid: false,
            auto_save_interval: 30,
            max_undo_history: 100,
            zoom_sensitivity: 1.0,
            pan_sensitivity: 1.0,
            reduce_motion: "system".to_string(),
            high_contrast: false,
        }
    }
}

impl Setting for AppSettings {
    fn key() -> &'static str {
        "app"
    }

    fn from_content(content: &SettingsContent) -> Self {
        let defaults = Self::default();
        Self {
            theme: content.theme.clone().unwrap_or(defaults.theme),
            font: content.font.clone().unwrap_or(defaults.font),
            font_size: content.font_size.unwrap_or(defaults.font_size),
            canvas_background: content
                .canvas_background
                .clone()
                .unwrap_or(defaults.canvas_background),
            grid_size: content.grid_size.unwrap_or(defaults.grid_size),
            show_grid: content.show_grid.unwrap_or(defaults.show_grid),
            snap_to_grid: content.snap_to_grid.unwrap_or(defaults.snap_to_grid),
            auto_save_interval: content
                .auto_save_interval
                .unwrap_or(defaults.auto_save_interval),
            max_undo_history: content
                .max_undo_history
                .unwrap_or(defaults.max_undo_history),
            zoom_sensitivity: content
                .zoom_sensitivity
                .unwrap_or(defaults.zoom_sensitivity),
            pan_sensitivity: content
                .pan_sensitivity
                .unwrap_or(defaults.pan_sensitivity),
            reduce_motion: content
                .reduce_motion
                .clone()
                .unwrap_or(defaults.reduce_motion),
            high_contrast: content.high_contrast.unwrap_or(defaults.high_contrast),
        }
    }

    fn default_value() -> Self {
        Self::default()
    }
}

impl AppSettings {
    /// Curated list of fonts - Nerd Fonts + common system fonts
    pub fn available_fonts() -> Vec<&'static str> {
        vec![
            // Nerd Fonts (monospace, great for code)
            "JetBrainsMono Nerd Font",
            "FiraCode Nerd Font",
            "Hack Nerd Font",
            "Iosevka Nerd Font",
            "CaskaydiaCove Nerd Font",
            "VictorMono Nerd Font",
            "SourceCodePro Nerd Font",
            "RobotoMono Nerd Font",
            "UbuntuMono Nerd Font",
            "Meslo Nerd Font",
            // System fonts (always available)
            "SF Pro",
            "SF Mono",
            "Futura",
            "Helvetica Neue",
            "Menlo",
            "Monaco",
            "Courier New",
            // Generic fallbacks
            "system-ui",
            "sans-serif",
            "monospace",
        ]
    }

    /// Check if animations should be reduced based on settings and system preference.
    /// Returns true if animations should be minimized/disabled.
    pub fn should_reduce_motion(&self) -> bool {
        match self.reduce_motion.as_str() {
            "on" => true,
            "off" => false,
            _ => Self::system_prefers_reduced_motion(), // "system" or any other value
        }
    }

    /// Detect system prefers-reduced-motion setting.
    /// Returns true if the OS has reduced motion enabled.
    #[cfg(target_os = "macos")]
    pub fn system_prefers_reduced_motion() -> bool {
        use std::process::Command;
        // On macOS, check the accessibility preference
        let output = Command::new("defaults")
            .args(["read", "com.apple.universalaccess", "reduceMotion"])
            .output();

        match output {
            Ok(out) => {
                let result = String::from_utf8_lossy(&out.stdout);
                result.trim() == "1"
            }
            Err(_) => false,
        }
    }

    #[cfg(target_os = "windows")]
    pub fn system_prefers_reduced_motion() -> bool {
        // On Windows, this would check SPI_GETCLIENTAREAANIMATION
        // For now, return false as default
        false
    }

    #[cfg(target_os = "linux")]
    pub fn system_prefers_reduced_motion() -> bool {
        use std::process::Command;
        // On Linux/GNOME, check gtk-enable-animations setting
        let output = Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "enable-animations"])
            .output();

        match output {
            Ok(out) => {
                let result = String::from_utf8_lossy(&out.stdout);
                // If animations are disabled, reduce motion
                result.trim() == "false"
            }
            Err(_) => false,
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    pub fn system_prefers_reduced_motion() -> bool {
        false
    }
}

// ============================================================================
// Settings Store - Central manager for layered settings
// ============================================================================

/// Sources of settings in precedence order (lowest to highest).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SettingsSource {
    /// Built-in defaults
    Default,
    /// User settings from ~/.config/humanboard/settings.json
    User,
    /// Project settings from .humanboard/settings.json
    Project,
}

/// Central store for managing layered settings.
///
/// Settings are merged from multiple sources with clear precedence:
/// Default < User < Project
pub struct SettingsStore {
    /// Default settings (always present)
    default_content: SettingsContent,

    /// User settings from ~/.config/humanboard/settings.json
    user_content: Option<SettingsContent>,

    /// Project settings from .humanboard/settings.json
    project_content: Option<SettingsContent>,

    /// Current project directory (for locating project settings)
    project_dir: Option<PathBuf>,

    /// Merged settings content
    merged_content: SettingsContent,

    /// Resolved app settings
    app_settings: AppSettings,

    /// Parse errors from settings files
    file_errors: Vec<(SettingsSource, String)>,
}

impl SettingsStore {
    /// Create a new settings store with defaults.
    pub fn new() -> Self {
        let default_content = Self::default_content();
        let app_settings = AppSettings::from_content(&default_content);

        Self {
            default_content: default_content.clone(),
            user_content: None,
            project_content: None,
            project_dir: None,
            merged_content: default_content,
            app_settings,
            file_errors: Vec::new(),
        }
    }

    /// Returns the default settings content.
    fn default_content() -> SettingsContent {
        let defaults = AppSettings::default();
        SettingsContent {
            theme: Some(defaults.theme),
            font: Some(defaults.font),
            font_size: Some(defaults.font_size),
            canvas_background: Some(defaults.canvas_background),
            grid_size: Some(defaults.grid_size),
            show_grid: Some(defaults.show_grid),
            snap_to_grid: Some(defaults.snap_to_grid),
            auto_save_interval: Some(defaults.auto_save_interval),
            max_undo_history: Some(defaults.max_undo_history),
            zoom_sensitivity: Some(defaults.zoom_sensitivity),
            pan_sensitivity: Some(defaults.pan_sensitivity),
            onboarding_completed: Some(false),
            reduce_motion: Some(defaults.reduce_motion),
            high_contrast: Some(defaults.high_contrast),
        }
    }

    /// Get the user settings file path.
    pub fn user_settings_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home)
            .join(".config")
            .join("humanboard")
            .join("settings.json")
    }

    /// Get the project settings file path for the current project.
    pub fn project_settings_path(&self) -> Option<PathBuf> {
        self.project_dir
            .as_ref()
            .map(|dir| dir.join(".humanboard").join("settings.json"))
    }

    /// Load user settings from disk.
    pub fn load_user_settings(&mut self) -> Result<(), SettingsError> {
        let path = Self::user_settings_path();
        self.load_settings_file(&path, SettingsSource::User)
    }

    /// Load project settings from disk.
    pub fn load_project_settings(&mut self, project_dir: PathBuf) -> Result<(), SettingsError> {
        self.project_dir = Some(project_dir.clone());
        let path = project_dir.join(".humanboard").join("settings.json");
        self.load_settings_file(&path, SettingsSource::Project)
    }

    /// Load settings from a specific file.
    fn load_settings_file(
        &mut self,
        path: &PathBuf,
        source: SettingsSource,
    ) -> Result<(), SettingsError> {
        // Remove any previous errors for this source
        self.file_errors.retain(|(s, _)| *s != source);

        if !path.exists() {
            debug!("Settings file not found: {:?}", path);
            match source {
                SettingsSource::User => self.user_content = None,
                SettingsSource::Project => self.project_content = None,
                SettingsSource::Default => {}
            }
            self.recompute_merged();
            return Ok(());
        }

        match std::fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str::<SettingsContent>(&content) {
                Ok(parsed) => {
                    info!("Loaded settings from {:?}", path);
                    match source {
                        SettingsSource::User => self.user_content = Some(parsed),
                        SettingsSource::Project => self.project_content = Some(parsed),
                        SettingsSource::Default => {}
                    }
                    self.recompute_merged();
                    Ok(())
                }
                Err(e) => {
                    let error_msg = format!("Parse error in {:?}: {}", path, e);
                    warn!("{}", error_msg);
                    self.file_errors.push((source, error_msg.clone()));
                    Err(SettingsError::ParseFailed {
                        path: path.clone(),
                        message: e.to_string(),
                    })
                }
            },
            Err(e) => {
                let error_msg = format!("Failed to read {:?}: {}", path, e);
                warn!("{}", error_msg);
                self.file_errors.push((source, error_msg.clone()));
                Err(SettingsError::LoadFailed {
                    path: path.clone(),
                    source: e,
                })
            }
        }
    }

    /// Recompute merged settings from all sources.
    fn recompute_merged(&mut self) {
        // Start with defaults
        let mut merged = self.default_content.clone();

        // Layer user settings
        if let Some(ref user) = self.user_content {
            merged.merge_from(user);
        }

        // Layer project settings (highest priority)
        if let Some(ref project) = self.project_content {
            merged.merge_from(project);
        }

        self.merged_content = merged;
        self.app_settings = AppSettings::from_content(&self.merged_content);

        debug!("Settings recomputed: {:?}", self.app_settings);
    }

    /// Get the current app settings.
    pub fn app_settings(&self) -> &AppSettings {
        &self.app_settings
    }

    /// Get the merged settings content.
    pub fn merged_content(&self) -> &SettingsContent {
        &self.merged_content
    }

    /// Check if there are any parse errors.
    pub fn has_errors(&self) -> bool {
        !self.file_errors.is_empty()
    }

    /// Get all parse errors.
    pub fn errors(&self) -> &[(SettingsSource, String)] {
        &self.file_errors
    }

    /// Update a setting and save to the appropriate file.
    pub fn update<F>(&mut self, source: SettingsSource, updater: F) -> Result<(), SettingsError>
    where
        F: FnOnce(&mut SettingsContent),
    {
        let content = match source {
            SettingsSource::Default => {
                return Err(SettingsError::InvalidSource(
                    "Cannot modify default settings".to_string(),
                ))
            }
            SettingsSource::User => self.user_content.get_or_insert_with(SettingsContent::default),
            SettingsSource::Project => self
                .project_content
                .get_or_insert_with(SettingsContent::default),
        };

        updater(content);
        self.recompute_merged();
        self.save(source)
    }

    /// Save settings to the appropriate file.
    pub fn save(&self, source: SettingsSource) -> Result<(), SettingsError> {
        let (path, content) = match source {
            SettingsSource::Default => {
                return Err(SettingsError::InvalidSource(
                    "Cannot save default settings".to_string(),
                ))
            }
            SettingsSource::User => (Self::user_settings_path(), &self.user_content),
            SettingsSource::Project => {
                let path = self.project_settings_path().ok_or_else(|| {
                    SettingsError::InvalidSource("No project directory set".to_string())
                })?;
                (path, &self.project_content)
            }
        };

        let content = content.as_ref().ok_or_else(|| {
            SettingsError::InvalidSource("No settings to save for this source".to_string())
        })?;

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| SettingsError::SaveFailed {
                path: path.clone(),
                source: e,
            })?;
        }

        let json = serde_json::to_string_pretty(content).map_err(|e| {
            SettingsError::ParseFailed {
                path: path.clone(),
                message: e.to_string(),
            }
        })?;

        std::fs::write(&path, json).map_err(|e| SettingsError::SaveFailed {
            path: path.clone(),
            source: e,
        })?;

        info!("Saved settings to {:?}", path);
        Ok(())
    }
}

impl Default for SettingsStore {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Global Settings Store - GPUI integration
// ============================================================================

/// Global settings store accessible via GPUI context.
static SETTINGS_STORE: std::sync::OnceLock<std::sync::RwLock<SettingsStore>> =
    std::sync::OnceLock::new();

/// Get a reference to the global settings store.
pub fn global_settings() -> &'static std::sync::RwLock<SettingsStore> {
    SETTINGS_STORE.get_or_init(|| std::sync::RwLock::new(SettingsStore::new()))
}

/// Initialize the global settings store and load user settings.
pub fn init_settings() -> Result<(), SettingsError> {
    let store = global_settings();
    let mut guard = store
        .write()
        .map_err(|_| SettingsError::LockPoisoned("settings store write lock poisoned".into()))?;
    guard.load_user_settings()?;
    Ok(())
}

/// Get the current app settings (convenience function).
pub fn app_settings() -> AppSettings {
    let store = global_settings();
    // Use unwrap_or_else to recover from poisoned lock - settings reads should not fail
    let guard = store.read().unwrap_or_else(|poisoned| poisoned.into_inner());
    guard.app_settings().clone()
}

/// Update a setting value.
pub fn update_setting<F>(source: SettingsSource, updater: F) -> Result<(), SettingsError>
where
    F: FnOnce(&mut SettingsContent),
{
    let store = global_settings();
    let mut guard = store
        .write()
        .map_err(|_| SettingsError::LockPoisoned("settings store write lock poisoned".into()))?;
    guard.update(source, updater)
}

/// Check if onboarding has been completed.
pub fn is_onboarding_completed() -> bool {
    let store = global_settings();
    let guard = store.read().unwrap_or_else(|poisoned| poisoned.into_inner());
    guard
        .merged_content()
        .onboarding_completed
        .unwrap_or(false)
}

/// Mark onboarding as completed.
pub fn mark_onboarding_completed() -> Result<(), SettingsError> {
    update_setting(SettingsSource::User, |content| {
        content.onboarding_completed = Some(true);
    })
}

/// Check if high contrast mode is enabled.
pub fn is_high_contrast() -> bool {
    let store = global_settings();
    let guard = store.read().unwrap_or_else(|poisoned| poisoned.into_inner());
    guard.merged_content().high_contrast.unwrap_or(false)
}

/// Set high contrast mode.
pub fn set_high_contrast(enabled: bool) -> Result<(), SettingsError> {
    update_setting(SettingsSource::User, |content| {
        content.high_contrast = Some(enabled);
    })
}

// ============================================================================
// Legacy Settings struct - Backwards compatibility
// ============================================================================

/// Legacy Settings struct for backwards compatibility.
/// Use `AppSettings` and `SettingsStore` for new code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,
    #[serde(default = "default_font")]
    pub font: String,
}

fn default_font() -> String {
    "Iosevka Nerd Font".to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: "Default Dark".to_string(),
            font: default_font(),
        }
    }
}

impl Settings {
    /// Curated list of fonts - Nerd Fonts + common system fonts
    pub fn available_fonts() -> Vec<&'static str> {
        AppSettings::available_fonts()
    }

    /// Load settings from disk
    pub fn load() -> Self {
        let app = app_settings();
        Self {
            theme: app.theme,
            font: app.font,
        }
    }

    /// Save settings to disk
    pub fn save(&self) {
        let theme = self.theme.clone();
        let font = self.font.clone();

        if let Err(e) = update_setting(SettingsSource::User, |content| {
            content.theme = Some(theme);
            content.font = Some(font);
        }) {
            error!("Failed to save settings: {}", e);
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

            // Update the appropriate theme slot (light or dark) and apply
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

// ============================================================================
// Theme Initialization
// ============================================================================

/// Find the themes directory in various locations
fn find_themes_dir() -> Option<PathBuf> {
    // Try current directory first (for development)
    let cwd_path = std::env::current_dir().ok().map(|p| p.join("themes"));
    if let Some(ref p) = cwd_path {
        if p.exists() {
            return Some(p.clone());
        }
    }

    // Try relative to executable
    let exe_path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .map(|p| p.join("themes"));
    if let Some(ref p) = exe_path {
        if p.exists() {
            return Some(p.clone());
        }
    }

    // Try macOS bundle Resources folder
    let bundle_path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf())) // MacOS/
        .and_then(|p| p.parent().map(|p| p.to_path_buf())) // Contents/
        .map(|p| p.join("Resources").join("themes"));
    if let Some(ref p) = bundle_path {
        if p.exists() {
            return Some(p.clone());
        }
    }

    None
}

/// Initialize themes by watching the themes directory
pub fn init_themes(cx: &mut App) {
    // Initialize settings first
    if let Err(e) = init_settings() {
        warn!("Failed to initialize settings: {}", e);
    }

    // Try multiple locations for themes directory
    let themes_dir = find_themes_dir();

    if let Some(themes_dir) = themes_dir {
        let saved_theme = app_settings().theme;
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
            warn!("Theme directory watch failed");
        }
    }
}
