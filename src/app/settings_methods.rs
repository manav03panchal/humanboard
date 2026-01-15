//! Settings-related methods - theme, font, dropdowns, shortcuts

use super::{Humanboard, SettingsTab};
use crate::focus::FocusContext;
use gpui::*;

impl Humanboard {
    pub fn toggle_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.show_settings = !self.show_settings;
        if self.show_settings {
            // Set focus context to Modal
            self.focus.focus(FocusContext::Modal, window);

            // Initialize theme index to current theme
            let themes = crate::settings::Settings::available_themes(cx);
            self.settings_theme_index = themes
                .iter()
                .position(|t| t == &self.settings.theme)
                .unwrap_or(0);

            // Initialize font index to current font
            let fonts = crate::settings::Settings::available_fonts();
            self.settings_font_index = fonts
                .iter()
                .position(|f| *f == self.settings.font)
                .unwrap_or(0);
        } else {
            // Force focus back to canvas when closing settings
            self.focus.force_canvas_focus(window);
        }
        cx.notify();
    }

    pub fn select_next_theme(&mut self, cx: &mut Context<Self>) {
        let themes = crate::settings::Settings::available_themes(cx);
        if !themes.is_empty() {
            self.settings_theme_index = (self.settings_theme_index + 1) % themes.len();
            self.settings_theme_scroll
                .scroll_to_item(self.settings_theme_index);
            // Apply theme immediately
            self.set_theme(themes[self.settings_theme_index].clone(), cx);
        }
    }

    pub fn select_prev_theme(&mut self, cx: &mut Context<Self>) {
        let themes = crate::settings::Settings::available_themes(cx);
        if !themes.is_empty() {
            self.settings_theme_index = if self.settings_theme_index == 0 {
                themes.len() - 1
            } else {
                self.settings_theme_index - 1
            };
            self.settings_theme_scroll
                .scroll_to_item(self.settings_theme_index);
            // Apply theme immediately
            self.set_theme(themes[self.settings_theme_index].clone(), cx);
        }
    }

    /// Show a toast notification
    pub fn show_toast(&mut self, toast: crate::notifications::Toast) {
        self.toast_manager.push(toast);
    }

    pub fn set_theme(&mut self, theme_name: String, cx: &mut Context<Self>) {
        self.settings.theme = theme_name.clone();
        self.settings.save();

        // Apply theme using the App context
        let theme_name = gpui::SharedString::from(theme_name);
        let config = gpui_component::theme::ThemeRegistry::global(cx)
            .themes()
            .get(&theme_name)
            .cloned();

        if let Some(config) = config {
            let mode = config.mode;
            if mode.is_dark() {
                gpui_component::theme::Theme::global_mut(cx).dark_theme = config.clone();
            } else {
                gpui_component::theme::Theme::global_mut(cx).light_theme = config.clone();
            }
            gpui_component::theme::Theme::global_mut(cx).mode = mode;
            gpui_component::theme::Theme::global_mut(cx).apply_config(&config);
        }

        cx.notify();
    }

    pub fn set_font(&mut self, font_name: String, cx: &mut Context<Self>) {
        self.settings.font = font_name;
        self.settings.save();
        cx.notify();
    }

    pub fn select_next_font(&mut self, cx: &mut Context<Self>) {
        let fonts = crate::settings::Settings::available_fonts();
        if !fonts.is_empty() {
            self.settings_font_index = (self.settings_font_index + 1) % fonts.len();
            self.settings_font_scroll
                .scroll_to_item(self.settings_font_index);
            self.set_font(fonts[self.settings_font_index].to_string(), cx);
        }
    }

    pub fn select_prev_font(&mut self, cx: &mut Context<Self>) {
        let fonts = crate::settings::Settings::available_fonts();
        if !fonts.is_empty() {
            self.settings_font_index = if self.settings_font_index == 0 {
                fonts.len() - 1
            } else {
                self.settings_font_index - 1
            };
            self.settings_font_scroll
                .scroll_to_item(self.settings_font_index);
            self.set_font(fonts[self.settings_font_index].to_string(), cx);
        }
    }

    pub fn toggle_shortcuts(&mut self, cx: &mut Context<Self>) {
        self.show_shortcuts = !self.show_shortcuts;
        cx.notify();
    }

    pub fn set_settings_tab(&mut self, tab: SettingsTab, cx: &mut Context<Self>) {
        self.settings_tab = tab;
        cx.notify();
    }

    /// Close all settings dropdowns
    fn close_all_dropdowns(&mut self, cx: &mut Context<Self>) {
        if cx
            .try_global::<crate::render::overlays::ThemeDropdownOpen>()
            .is_some()
        {
            cx.remove_global::<crate::render::overlays::ThemeDropdownOpen>();
        }
        if cx
            .try_global::<crate::render::overlays::FontDropdownOpen>()
            .is_some()
        {
            cx.remove_global::<crate::render::overlays::FontDropdownOpen>();
        }
    }

    pub fn toggle_theme_dropdown(&mut self, cx: &mut Context<Self>) {
        let was_open = cx
            .try_global::<crate::render::overlays::ThemeDropdownOpen>()
            .is_some();
        // Always close all dropdowns first
        self.close_all_dropdowns(cx);
        // Only open theme dropdown if it wasn't already open
        if !was_open {
            cx.set_global(crate::render::overlays::ThemeDropdownOpen);
        }
        cx.notify();
    }

    pub fn close_theme_dropdown(&mut self, cx: &mut Context<Self>) {
        if cx
            .try_global::<crate::render::overlays::ThemeDropdownOpen>()
            .is_some()
        {
            cx.remove_global::<crate::render::overlays::ThemeDropdownOpen>();
        }
        cx.notify();
    }

    pub fn toggle_font_dropdown(&mut self, cx: &mut Context<Self>) {
        let was_open = cx
            .try_global::<crate::render::overlays::FontDropdownOpen>()
            .is_some();
        // Always close all dropdowns first
        self.close_all_dropdowns(cx);
        // Only open font dropdown if it wasn't already open
        if !was_open {
            cx.set_global(crate::render::overlays::FontDropdownOpen);
        }
        cx.notify();
    }

    pub fn close_font_dropdown(&mut self, cx: &mut Context<Self>) {
        if cx
            .try_global::<crate::render::overlays::FontDropdownOpen>()
            .is_some()
        {
            cx.remove_global::<crate::render::overlays::FontDropdownOpen>();
        }
        cx.notify();
    }
}
