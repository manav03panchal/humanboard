# Humanboard Architecture

Humanboard is a Miro-style infinite canvas moodboard application built with the GPUI framework. This document describes the architecture for developers working on the codebase.

## High-Level Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                           GPUI Framework                             │
│  (GPU-accelerated UI with reactive state management)                │
└─────────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         Humanboard (Entity)                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │   State     │  │   Render    │  │   Input     │  │   Actions   │ │
│  │  (app/)     │  │  (render/)  │  │ (input.rs)  │  │(actions.rs) │ │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
                                   │
          ┌────────────────────────┼────────────────────────┐
          ▼                        ▼                        ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────────┐
│     Board       │    │  Preview Panel  │    │      WebViews       │
│   (board.rs)    │    │   (preview.rs)  │    │ (pdf/youtube/audio) │
└─────────────────┘    └─────────────────┘    └─────────────────────┘
```

## Module Organization

### Core Application (`src/app/`)

The app module is split into focused submodules:

| Module | Purpose |
|--------|---------|
| `state.rs` | `Humanboard` struct definition with all application state |
| `lifecycle.rs` | Initialization (`new()`) and cleanup logic |
| `board_management.rs` | Board CRUD operations (create, open, delete, trash) |
| `settings_methods.rs` | Settings modal and configuration |
| `command_palette_methods.rs` | Command palette logic |
| `preview_core.rs` | Preview panel core functionality |
| `preview_tabs.rs` | Tab management (open, close, switch, drag) |
| `preview_panes.rs` | Split pane operations |
| `preview_webviews.rs` | YouTube, Audio, Video webview management |
| `preview_search.rs` | In-preview search functionality |
| `textbox.rs` | TextBox editing on canvas |
| `types.rs` | Type definitions (AppView, PreviewTab, etc.) |

### Rendering (`src/render/`)

| Module | Purpose |
|--------|---------|
| `mod.rs` | Main `Render` impl for Humanboard, view routing |
| `canvas.rs` | Canvas item rendering (images, PDFs, shapes, arrows) |
| `dock.rs` | Tool dock sidebar rendering |
| `overlays.rs` | Modals, shortcuts overlay, toasts |
| `preview.rs` | Preview panel rendering |

### Other Key Modules

| Module | Purpose |
|--------|---------|
| `board.rs` | Board state, items, undo/redo history, debounced saving |
| `board_index.rs` | Board index/metadata, storage locations |
| `focus.rs` | Focus management system |
| `input.rs` | Mouse/scroll event handling |
| `actions.rs` | Action definitions and handlers |
| `types.rs` | Core types (CanvasItem, ItemContent, etc.) |
| `settings.rs` | Settings loading/saving |
| `notifications.rs` | Toast notification system |

## State Management

### Application State (`Humanboard`)

The `Humanboard` struct is the central state container, organized into logical groups:

```rust
pub struct Humanboard {
    // === View State ===
    pub view: AppView,           // Landing, Board, or Onboarding
    pub board_index: BoardIndex, // All boards metadata

    // === Board State (when view == Board) ===
    pub board: Option<Board>,    // Current board data
    pub selected_items: HashSet<u64>,
    pub selected_tool: ToolType,

    // === Focus Management ===
    pub focus: FocusManager,     // Priority-based focus system

    // === Preview Panel ===
    pub preview: Option<PreviewPanel>,

    // === WebViews ===
    pub youtube_webviews: HashMap<u64, YouTubeWebView>,
    pub audio_webviews: HashMap<u64, AudioWebView>,
    pub video_webviews: HashMap<u64, VideoWebView>,

    // === UI State ===
    pub show_settings: bool,
    pub command_palette: Option<Entity<InputState>>,
    pub toast_manager: ToastManager,
    // ...
}
```

### Board State (`Board`)

Each board manages its own state independently:

```rust
pub struct Board {
    pub id: String,
    pub canvas_offset: Point<Pixels>,
    pub zoom: f32,
    pub items: Vec<CanvasItem>,

    // Undo/Redo
    history: VecDeque<BoardState>,
    history_index: usize,

    // Debounced saving
    dirty: bool,
    last_change: Instant,
}
```

### State Flow

```
User Action → Action Handler → Mutate State → cx.notify() → Re-render
     │
     └─→ (For board changes) → mark_dirty() → Debounced Save
```

## Focus System

The focus system follows Zed's patterns with priority-based contexts:

### Focus Hierarchy (highest to lowest)

```
┌─────────────────────────────────────┐
│  1. Modal (settings, confirmations) │  ← Captures all input
├─────────────────────────────────────┤
│  2. CommandPalette                  │  ← Captures keyboard
├─────────────────────────────────────┤
│  3. TextboxEditing                  │  ← Editing text on canvas
├─────────────────────────────────────┤
│  4. CodeEditor                      │  ← Code editing mode
├─────────────────────────────────────┤
│  5. Preview                         │  ← Markdown editing
├─────────────────────────────────────┤
│  6. Landing                         │  ← Board name input
├─────────────────────────────────────┤
│  7. Canvas (default)                │  ← Keyboard shortcuts
└─────────────────────────────────────┘
```

### Key Principles

1. **No Focus Stealing**: Lower-priority contexts cannot steal focus from higher ones
2. **Explicit Transitions**: `focus.focus(context, window)` for gaining focus
3. **Clean Release**: `focus.release(context, window)` returns to previous context
4. **Key Contexts**: Each context has a key_context string for keybindings

### Usage Example

```rust
// Gain focus
self.focus.focus(FocusContext::CommandPalette, window);

// Check if input is active (to suppress single-key shortcuts)
if self.focus.is_input_active() { return; }

// Release focus back to canvas
self.focus.release(FocusContext::CommandPalette, window);
```

## Event Flow

### Mouse Events

```
MouseDownEvent
     │
     ├─→ Check splitter drag
     ├─→ Check preview panel area
     ├─→ Check drawing tool active
     ├─→ Check item under cursor
     │        │
     │        ├─→ Resize corner? → Start resize
     │        ├─→ Item click? → Select/multi-select
     │        └─→ Empty space? → Start marquee/pan
     │
     └─→ cx.notify()

MouseMoveEvent
     │
     ├─→ Splitter dragging? → Update split size
     ├─→ Item dragging? → Update item position
     ├─→ Resizing? → Update item size
     ├─→ Drawing? → Update preview
     └─→ Marquee? → Update selection

MouseUpEvent
     │
     ├─→ Finalize drag/resize
     ├─→ Create drawn item (shape/arrow/textbox)
     └─→ Complete marquee selection
```

### Action Flow

```
Keyboard Shortcut / Command Palette
              │
              ▼
┌─────────────────────────────┐
│   Action Dispatch (GPUI)    │
│   Routes to key_context     │
└─────────────────────────────┘
              │
              ▼
┌─────────────────────────────┐
│    Action Handler           │
│    (in actions.rs)          │
└─────────────────────────────┘
              │
              ▼
┌─────────────────────────────┐
│   State Mutation            │
│   + cx.notify()             │
└─────────────────────────────┘
              │
              ▼
┌─────────────────────────────┐
│   Render (automatic)        │
└─────────────────────────────┘
```

## Key Types

### Canvas Items

```rust
pub struct CanvasItem {
    pub id: u64,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub content: ItemContent,
}

pub enum ItemContent {
    Image(PathBuf),
    Pdf(PathBuf),
    Markdown(PathBuf),
    Code(PathBuf, String),  // path, language
    YouTube(String),         // video_id
    Audio(PathBuf),
    Video(PathBuf),
    Link(String),
    TextBox { text: String, font_size: f32, color: String },
    Shape { shape_type: ShapeType, fill: String, border: String },
    Arrow { start: (f32, f32), end: (f32, f32), head: ArrowHead },
}
```

### Preview Tabs

```rust
pub enum PreviewTab {
    Pdf { path: PathBuf, webview: Option<PdfWebView>, ... },
    Markdown { path: PathBuf, content: String, ... },
    Code { path: PathBuf, content: String, language: String, ... },
    Image { path: PathBuf },
}
```

### App Views

```rust
pub enum AppView {
    Onboarding,           // First-run welcome
    Landing,              // Board selection
    Board(String),        // Active board (by ID)
}
```

## WebView Integration

WebViews are used for content that benefits from native rendering:

| Type | Purpose | Implementation |
|------|---------|----------------|
| `PdfWebView` | PDF viewing | Uses platform's native PDF renderer via WKWebView |
| `YouTubeWebView` | YouTube embeds | Local HTTP server + iframe embed |
| `AudioWebView` | Audio playback | HTML5 audio element |
| `VideoWebView` | Video playback | HTML5 video element |

### WebView Lifecycle

```
Board loaded → ensure_*_webviews() → Create WebViews for items
                      │
                      ▼
              Update visibility based on viewport
                      │
                      ▼
Board closed / Item deleted → Remove WebViews
```

## Settings System

Settings use a layered approach with hot-reloading:

```
~/.config/humanboard/settings.toml  ←── SettingsWatcher monitors
              │
              ▼
       Settings::load()
              │
              ▼
     Humanboard.settings
              │
              └─→ Theme, font, etc.
```

## Error Handling

Errors are surfaced to users via the toast notification system:

```rust
// In methods that can fail
let errors = board.handle_file_drop(pos, paths);
for error in errors {
    self.toast_manager.push(Toast::error(error));
}
```

Toast types: `success`, `error`, `warning`, `info`

## Performance Considerations

1. **Debounced Saving**: Board saves are debounced (500ms) to avoid excessive I/O
2. **WebView Visibility**: WebViews are hidden when scrolled out of view
3. **Hit Testing**: Cached hit test results for mouse interactions
4. **Background Tasks**: Long operations use `BackgroundExecutor`

## Testing

```bash
cargo test                    # Run all tests
cargo test board_tests        # Run specific test module
```

Key test modules:
- `board_tests` - Board state and operations
- `validation_tests` - Item validation
- `notifications_tests` - Toast system

## Adding New Features

### Adding a New Action

1. Add action to `actions!` macro in `actions.rs`
2. Implement handler in `impl Humanboard`
3. Add keybinding in `main.rs`

### Adding a New Item Type

1. Add variant to `ItemContent` enum in `types.rs`
2. Update `default_size()` and `display_name()` methods
3. Add rendering in `render/canvas.rs`
4. Handle in `input.rs` for interactions

### Adding a New Focus Context

1. Add variant to `FocusContext` in `focus.rs`
2. Add `KEY_*` constant
3. Update `priority()` and `all()` methods
4. Add FocusHandle to `FocusManager`
