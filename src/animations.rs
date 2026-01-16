//! Animation utilities for smooth UI transitions.
//!
//! Provides reusable animation primitives for modals, selections, and other UI elements.

use std::time::{Duration, Instant};

/// Default duration for modal fade animations
pub const MODAL_FADE_DURATION: Duration = Duration::from_millis(150);

/// Default duration for quick transitions (hover, selection)
pub const QUICK_TRANSITION: Duration = Duration::from_millis(100);

/// Animation state for fade in/out effects
#[derive(Clone)]
pub struct FadeAnimation {
    /// When the animation started
    pub start_time: Instant,
    /// Duration of the animation
    pub duration: Duration,
    /// Direction: true = fading in, false = fading out
    pub fading_in: bool,
    /// Whether the animation has completed
    pub completed: bool,
}

impl FadeAnimation {
    /// Create a new fade-in animation
    pub fn fade_in(duration: Duration) -> Self {
        Self {
            start_time: Instant::now(),
            duration,
            fading_in: true,
            completed: false,
        }
    }

    /// Create a new fade-out animation
    pub fn fade_out(duration: Duration) -> Self {
        Self {
            start_time: Instant::now(),
            duration,
            fading_in: false,
            completed: false,
        }
    }

    /// Get the current opacity value (0.0 to 1.0)
    pub fn opacity(&self) -> f32 {
        if self.completed {
            return if self.fading_in { 1.0 } else { 0.0 };
        }

        let elapsed = self.start_time.elapsed().as_secs_f32();
        let duration = self.duration.as_secs_f32();
        let progress = (elapsed / duration).min(1.0);

        // Use ease-out cubic for smooth deceleration
        let eased = ease_out_cubic(progress);

        if self.fading_in {
            eased
        } else {
            1.0 - eased
        }
    }

    /// Check if the animation is complete
    pub fn is_complete(&self) -> bool {
        self.completed || self.start_time.elapsed() >= self.duration
    }

    /// Mark the animation as complete
    pub fn mark_complete(&mut self) {
        self.completed = true;
    }

    /// Update the animation state, returns true if still animating
    pub fn update(&mut self) -> bool {
        if self.is_complete() {
            self.completed = true;
            false
        } else {
            true
        }
    }
}

/// Animation state for a pulsing/glowing effect
#[derive(Clone)]
pub struct PulseAnimation {
    /// When the animation started
    pub start_time: Instant,
    /// Duration of one pulse cycle
    pub cycle_duration: Duration,
    /// Number of cycles (-1 for infinite)
    pub cycles: i32,
    /// Minimum intensity (0.0 to 1.0)
    pub min_intensity: f32,
    /// Maximum intensity (0.0 to 1.0)
    pub max_intensity: f32,
}

impl PulseAnimation {
    /// Create a new pulse animation
    pub fn new(cycle_duration: Duration, cycles: i32) -> Self {
        Self {
            start_time: Instant::now(),
            cycle_duration,
            cycles,
            min_intensity: 0.3,
            max_intensity: 1.0,
        }
    }

    /// Create a single pulse (one cycle)
    pub fn single() -> Self {
        Self::new(Duration::from_millis(300), 1)
    }

    /// Get the current intensity (0.0 to 1.0)
    pub fn intensity(&self) -> f32 {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let cycle = self.cycle_duration.as_secs_f32();

        // Use sine wave for smooth pulsing
        let phase = (elapsed / cycle) * std::f32::consts::PI * 2.0;
        let wave = (phase.sin() + 1.0) / 2.0; // Normalize to 0-1

        self.min_intensity + wave * (self.max_intensity - self.min_intensity)
    }

    /// Check if the animation is complete
    pub fn is_complete(&self) -> bool {
        if self.cycles < 0 {
            return false; // Infinite
        }
        let elapsed = self.start_time.elapsed();
        let total_duration = self.cycle_duration * self.cycles as u32;
        elapsed >= total_duration
    }
}

/// Ease-out cubic function for smooth deceleration
/// t should be 0.0 to 1.0
pub fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

/// Ease-in-out cubic function for smooth acceleration and deceleration
pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// Linear interpolation between two values
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}

/// State for tracking modal animations
#[derive(Default)]
pub struct ModalAnimationState {
    /// Settings modal animation
    pub settings: Option<FadeAnimation>,
    /// Create board modal animation
    pub create_board: Option<FadeAnimation>,
    /// Command palette animation
    pub command_palette: Option<FadeAnimation>,
    /// Shortcuts overlay animation
    pub shortcuts: Option<FadeAnimation>,
}

impl ModalAnimationState {
    /// Start a fade-in animation for settings modal
    pub fn open_settings(&mut self) {
        self.settings = Some(FadeAnimation::fade_in(MODAL_FADE_DURATION));
    }

    /// Start a fade-out animation for settings modal
    pub fn close_settings(&mut self) {
        if let Some(ref mut anim) = self.settings {
            if anim.fading_in {
                *anim = FadeAnimation::fade_out(MODAL_FADE_DURATION);
            }
        } else {
            self.settings = Some(FadeAnimation::fade_out(MODAL_FADE_DURATION));
        }
    }

    /// Start a fade-in animation for create board modal
    pub fn open_create_board(&mut self) {
        self.create_board = Some(FadeAnimation::fade_in(MODAL_FADE_DURATION));
    }

    /// Start a fade-out animation for create board modal
    pub fn close_create_board(&mut self) {
        if let Some(ref mut anim) = self.create_board {
            if anim.fading_in {
                *anim = FadeAnimation::fade_out(MODAL_FADE_DURATION);
            }
        } else {
            self.create_board = Some(FadeAnimation::fade_out(MODAL_FADE_DURATION));
        }
    }

    /// Start a fade-in animation for command palette
    pub fn open_command_palette(&mut self) {
        self.command_palette = Some(FadeAnimation::fade_in(MODAL_FADE_DURATION));
    }

    /// Start a fade-out animation for command palette
    pub fn close_command_palette(&mut self) {
        if let Some(ref mut anim) = self.command_palette {
            if anim.fading_in {
                *anim = FadeAnimation::fade_out(MODAL_FADE_DURATION);
            }
        } else {
            self.command_palette = Some(FadeAnimation::fade_out(MODAL_FADE_DURATION));
        }
    }

    /// Get settings modal opacity
    pub fn settings_opacity(&self) -> f32 {
        self.settings.as_ref().map(|a| a.opacity()).unwrap_or(1.0)
    }

    /// Get create board modal opacity
    pub fn create_board_opacity(&self) -> f32 {
        self.create_board
            .as_ref()
            .map(|a| a.opacity())
            .unwrap_or(1.0)
    }

    /// Get command palette opacity
    pub fn command_palette_opacity(&self) -> f32 {
        self.command_palette
            .as_ref()
            .map(|a| a.opacity())
            .unwrap_or(1.0)
    }

    /// Update all animations, returns true if any are still active
    pub fn update(&mut self) -> bool {
        let mut any_active = false;

        if let Some(ref mut anim) = self.settings {
            if anim.update() {
                any_active = true;
            }
        }
        if let Some(ref mut anim) = self.create_board {
            if anim.update() {
                any_active = true;
            }
        }
        if let Some(ref mut anim) = self.command_palette {
            if anim.update() {
                any_active = true;
            }
        }
        if let Some(ref mut anim) = self.shortcuts {
            if anim.update() {
                any_active = true;
            }
        }

        any_active
    }

    /// Check if settings close animation is complete (for cleanup)
    pub fn settings_close_complete(&self) -> bool {
        self.settings
            .as_ref()
            .map(|a| !a.fading_in && a.is_complete())
            .unwrap_or(false)
    }

    /// Check if create board close animation is complete
    pub fn create_board_close_complete(&self) -> bool {
        self.create_board
            .as_ref()
            .map(|a| !a.fading_in && a.is_complete())
            .unwrap_or(false)
    }

    /// Check if command palette close animation is complete
    pub fn command_palette_close_complete(&self) -> bool {
        self.command_palette
            .as_ref()
            .map(|a| !a.fading_in && a.is_complete())
            .unwrap_or(false)
    }
}
