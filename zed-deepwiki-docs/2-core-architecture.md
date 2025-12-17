<!-- Source: https://deepwiki.com/zed-industries/zed/2-core-architecture -->

# 2 Core Architecture

Loading...

Index your code with Devin

[DeepWiki](</>)

[DeepWiki](</>)

[zed-industries/zed ](<https://github.com/zed-industries/zed> "Open repository")

Index your code with

Devin

Edit WikiShare

Loading...

Last indexed: 16 December 2025 ([4109c9](<https://github.com/zed-industries/zed/commits/4109c9dd>))

  * [Overview](</zed-industries/zed/1-overview>)
  * [Core Architecture](</zed-industries/zed/2-core-architecture>)
  * [Application Initialization and Lifecycle](</zed-industries/zed/2.1-application-initialization-and-lifecycle>)
  * [GPUI Framework](</zed-industries/zed/2.2-gpui-framework>)
  * [Window and Platform Abstraction](</zed-industries/zed/2.3-window-and-platform-abstraction>)
  * [Event Flow and Input Handling](</zed-industries/zed/2.4-event-flow-and-input-handling>)
  * [Keybinding and Action System](</zed-industries/zed/2.5-keybinding-and-action-system>)
  * [Focus Management and Hit Testing](</zed-industries/zed/2.6-focus-management-and-hit-testing>)
  * [Editor Architecture](</zed-industries/zed/3-editor-architecture>)
  * [Editor Component and UI](</zed-industries/zed/3.1-editor-component-and-ui>)
  * [Buffer System and Text Storage](</zed-industries/zed/3.2-buffer-system-and-text-storage>)
  * [Display Pipeline and Rendering](</zed-industries/zed/3.3-display-pipeline-and-rendering>)
  * [Selections and Editing Operations](</zed-industries/zed/3.4-selections-and-editing-operations>)
  * [Code Intelligence Integration](</zed-industries/zed/3.5-code-intelligence-integration>)
  * [Diff Integration](</zed-industries/zed/3.6-diff-integration>)
  * [Workspace and Panel System](</zed-industries/zed/4-workspace-and-panel-system>)
  * [Workspace Organization](</zed-industries/zed/4.1-workspace-organization>)
  * [Item System and Lifecycle](</zed-industries/zed/4.2-item-system-and-lifecycle>)
  * [Pane Management](</zed-industries/zed/4.3-pane-management>)
  * [Search System](</zed-industries/zed/4.4-search-system>)
  * [Project Management](</zed-industries/zed/5-project-management>)
  * [Project Orchestration](</zed-industries/zed/5.1-project-orchestration>)
  * [Worktree and File System](</zed-industries/zed/5.2-worktree-and-file-system>)
  * [Buffer Store](</zed-industries/zed/5.3-buffer-store>)
  * [Language Intelligence](</zed-industries/zed/6-language-intelligence>)
  * [LSP Store Architecture](</zed-industries/zed/6.1-lsp-store-architecture>)
  * [Language Server Lifecycle](</zed-industries/zed/6.2-language-server-lifecycle>)
  * [Completions and Diagnostics](</zed-industries/zed/6.3-completions-and-diagnostics>)
  * [Multi-Language Server Coordination](</zed-industries/zed/6.4-multi-language-server-coordination>)
  * [Settings and Configuration](</zed-industries/zed/7-settings-and-configuration>)
  * [Settings Store and Layering](</zed-industries/zed/7.1-settings-store-and-layering>)
  * [Settings UI](</zed-industries/zed/7.2-settings-ui>)
  * [Settings Migration](</zed-industries/zed/7.3-settings-migration>)
  * [Keymap System](</zed-industries/zed/7.4-keymap-system>)
  * [Git Integration](</zed-industries/zed/8-git-integration>)
  * [Git Panel and UI](</zed-industries/zed/8.1-git-panel-and-ui>)
  * [Git Store and State Management](</zed-industries/zed/8.2-git-store-and-state-management>)
  * [Repository Operations](</zed-industries/zed/8.3-repository-operations>)
  * [Diff System](</zed-industries/zed/8.4-diff-system>)
  * [Terminal and Task Execution](</zed-industries/zed/9-terminal-and-task-execution>)
  * [Terminal Core](</zed-industries/zed/9.1-terminal-core>)
  * [Terminal View and Rendering](</zed-industries/zed/9.2-terminal-view-and-rendering>)
  * [Task System](</zed-industries/zed/9.3-task-system>)
  * [Vim Mode](</zed-industries/zed/10-vim-mode>)
  * [Mode State Machine](</zed-industries/zed/10.1-mode-state-machine>)
  * [Operators, Motions, and Objects](</zed-industries/zed/10.2-operators-motions-and-objects>)
  * [Visual Mode](</zed-industries/zed/10.3-visual-mode>)
  * [Helix Mode Integration](</zed-industries/zed/10.4-helix-mode-integration>)
  * [AI Agent System](</zed-industries/zed/11-ai-agent-system>)
  * [Agent Communication Protocol (ACP)](</zed-industries/zed/11.1-agent-communication-protocol-\(acp\)>)
  * [Agent UI and Thread Management](</zed-industries/zed/11.2-agent-ui-and-thread-management>)
  * [Agent Connection and Implementations](</zed-industries/zed/11.3-agent-connection-and-implementations>)
  * [Tool System](</zed-industries/zed/11.4-tool-system>)
  * [Mention System and Context](</zed-industries/zed/11.5-mention-system-and-context>)
  * [Legacy Agent Thread System](</zed-industries/zed/11.6-legacy-agent-thread-system>)
  * [Remote Development and Collaboration](</zed-industries/zed/12-remote-development-and-collaboration>)
  * [Local vs Remote Architecture](</zed-industries/zed/12.1-local-vs-remote-architecture>)
  * [Remote Project Architecture](</zed-industries/zed/12.2-remote-project-architecture>)
  * [Collaboration Features](</zed-industries/zed/12.3-collaboration-features>)
  * [CRDT and Synchronization](</zed-industries/zed/12.4-crdt-and-synchronization>)


Menu

# Core Architecture

Relevant source files

  * [Cargo.lock](<https://github.com/zed-industries/zed/blob/4109c9dd/Cargo.lock>)
  * [Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/Cargo.toml>)
  * [crates/gpui/Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/Cargo.toml>)
  * [crates/gpui/build.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/build.rs>)
  * [crates/gpui/examples/mouse_pressure.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/examples/mouse_pressure.rs>)
  * [crates/gpui/src/app.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app.rs>)
  * [crates/gpui/src/geometry.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/geometry.rs>)
  * [crates/gpui/src/interactive.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/interactive.rs>)
  * [crates/gpui/src/key_dispatch.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/key_dispatch.rs>)
  * [crates/gpui/src/keymap.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/keymap.rs>)
  * [crates/gpui/src/platform.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform.rs>)
  * [crates/gpui/src/platform/linux/headless/client.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/linux/headless/client.rs>)
  * [crates/gpui/src/platform/linux/platform.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/linux/platform.rs>)
  * [crates/gpui/src/platform/linux/wayland/client.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/linux/wayland/client.rs>)
  * [crates/gpui/src/platform/linux/wayland/window.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/linux/wayland/window.rs>)
  * [crates/gpui/src/platform/linux/x11/client.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/linux/x11/client.rs>)
  * [crates/gpui/src/platform/linux/x11/window.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/linux/x11/window.rs>)
  * [crates/gpui/src/platform/mac/events.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/mac/events.rs>)
  * [crates/gpui/src/platform/mac/platform.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/mac/platform.rs>)
  * [crates/gpui/src/platform/mac/window.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/mac/window.rs>)
  * [crates/gpui/src/platform/test/platform.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/test/platform.rs>)
  * [crates/gpui/src/platform/test/window.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/test/window.rs>)
  * [crates/gpui/src/platform/windows/events.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/windows/events.rs>)
  * [crates/gpui/src/platform/windows/platform.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/windows/platform.rs>)
  * [crates/gpui/src/platform/windows/util.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/windows/util.rs>)
  * [crates/gpui/src/platform/windows/window.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/windows/window.rs>)
  * [crates/gpui/src/taffy.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/taffy.rs>)
  * [crates/gpui/src/window.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/window.rs>)
  * [crates/ui/src/components/keybinding.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/ui/src/components/keybinding.rs>)
  * [crates/zed/Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/Cargo.toml>)
  * [crates/zed/src/main.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/main.rs>)
  * [crates/zed/src/zed.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/zed.rs>)


## Purpose and Scope

This document provides an overview of Zed's foundational architectural systems. It covers the major layers and components that form the basis of Zed's functionality, from application initialization through the GPUI UI framework, window management, and core editing capabilities.

For detailed information about specific subsystems:

  * Application startup sequence: see [Application Initialization and Lifecycle](</zed-industries/zed/2.1-application-initialization-and-lifecycle>)
  * GPUI framework internals: see [GPUI Framework](</zed-industries/zed/2.2-gpui-framework>)
  * Window event handling: see [Window and Event System](</zed-industries/zed/2.3-window-and-platform-abstraction>)
  * Text editing implementation: see [Editor Architecture](</zed-industries/zed/2.4-event-flow-and-input-handling>)
  * UI organization: see [Workspace and Panel System](</zed-industries/zed/2.5-keybinding-and-action-system>)
  * Project-level services: see [Project Management and File System](</zed-industries/zed/2.6-focus-management-and-hit-testing>)
  * Configuration system: see [Settings and Configuration System](<#2.7>)


* * *

## Architectural Overview

Zed is built as a layered architecture where each layer provides abstractions for the layer above it. The system follows a clear separation of concerns:

  1. **Platform Layer** \- OS-specific abstractions for windowing, input, rendering
  2. **GPUI Layer** \- GPU-accelerated UI framework with reactive state management
  3. **Application Layer** \- Global application state and lifecycle management
  4. **Workspace Layer** \- UI organization into panes, panels, and items
  5. **Editor Layer** \- Text editing with sophisticated buffer and display management
  6. **Project Layer** \- Language intelligence, version control, and file management


### High-Level System Layers


**Sources:** [crates/gpui/src/platform.rs1-150](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform.rs#L1-L150>) [crates/gpui/src/app.rs123-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app.rs#L123-L800>) [crates/gpui/src/window.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/window.rs#L1-L200>) [crates/zed/src/main.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/main.rs#L1-L100>) [crates/workspace/src/workspace.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace.rs#L1-L200>)

* * *

## Application Initialization Flow

The application bootstrap process follows a well-defined sequence from platform initialization through window creation to workspace setup.

### Application Startup Sequence


**Sources:** [crates/zed/src/main.rs168-420](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/main.rs#L168-L420>) [crates/gpui/src/app.rs129-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app.rs#L129-L200>) [crates/zed/src/zed.rs143-479](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/zed.rs#L143-L479>) [crates/zed/src/main.rs406-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/main.rs#L406-L600>)

### Key Initialization Steps

The initialization process involves several critical stages:

Stage| Component| Responsibility| File Reference  
---|---|---|---  
1\. Bootstrap| `main()`| Parse arguments, setup paths| [crates/zed/src/main.rs168-242](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/main.rs#L168-L242>)  
2\. Platform| `Application::new()`| Create OS-specific platform| [crates/gpui/src/platform.rs92-133](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform.rs#L92-L133>)  
3\. GPUI Setup| `App`| Initialize UI context| [crates/gpui/src/app.rs600-700](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app.rs#L600-L700>)  
4\. Settings| `SettingsStore`| Load configuration| [crates/zed/src/main.rs412-416](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/main.rs#L412-L416>)  
5\. Services| `AppState`| Initialize global services| [crates/zed/src/main.rs476-550](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/main.rs#L476-L550>)  
6\. Workspace| `initialize_workspace()`| Setup UI container| [crates/zed/src/zed.rs346-478](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/zed.rs#L346-L478>)  
  
**Sources:** [crates/zed/src/main.rs168-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/main.rs#L168-L600>) [crates/zed/src/zed.rs143-274](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/zed.rs#L143-L274>)

* * *

## Platform Abstraction Layer

Zed uses a trait-based platform abstraction to support macOS, Windows, and Linux (X11/Wayland). The `Platform` trait defines the interface that all platform implementations must provide.

### Platform Trait and Implementations


**Sources:** [crates/gpui/src/platform.rs125-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform.rs#L125-L400>) [crates/gpui/src/platform/mac/platform.rs27-120](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/mac/platform.rs#L27-L120>) [crates/gpui/src/platform/windows/platform.rs32-172](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/windows/platform.rs#L32-L172>) [crates/gpui/src/platform/linux/x11/client.rs62-150](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/linux/x11/client.rs#L62-L150>) [crates/gpui/src/platform/linux/wayland/client.rs29-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/linux/wayland/client.rs#L29-L100>)

### Platform Window Creation

Each platform provides its own `PlatformWindow` implementation:

  * **macOS** : `MacWindow` wraps `NSWindow` and uses Metal for rendering
  * **Windows** : `WindowsWindow` wraps `HWND` and uses DirectX 11
  * **Linux X11** : `X11Window` manages X11 window and uses Vulkan via Blade
  * **Linux Wayland** : `WaylandWindow` uses Wayland protocols and Vulkan


The window creation flow delegates to platform-specific code:


**Sources:** [crates/gpui/src/platform.rs150-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform.rs#L150-L200>) [crates/gpui/src/platform/mac/window.rs55-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/mac/window.rs#L55-L200>) [crates/gpui/src/platform/windows/window.rs65-237](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/windows/window.rs#L65-L237>) [crates/gpui/src/platform/linux/x11/window.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/linux/x11/window.rs#L1-L100>)

* * *

## GPUI Reactive State Management

GPUI uses an entity-component system for reactive state management. The core abstraction is the `Entity` trait, which represents any stateful component in the UI.

### Entity and Context System


**Sources:** [crates/gpui/src/app.rs123-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app.rs#L123-L800>) [crates/gpui/src/app/entity_map.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app/entity_map.rs#L1-L300>) [crates/gpui/src/app/context.rs1-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app/context.rs#L1-L500>)

### Entity Creation and Updates

Entities are created via `Context::new()` and stored in the `EntityMap`:


The `EntityMap` maintains weak references to entities and automatically cleans up deallocated entities. Entities are identified by `EntityId` which is a stable identifier throughout the entity's lifetime.

**Sources:** [crates/gpui/src/app/entity_map.rs50-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app/entity_map.rs#L50-L200>) [crates/gpui/src/app/context.rs400-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app/context.rs#L400-L600>)

* * *

## Window and Event Dispatch

The `Window` struct manages the rendering surface, event dispatch, and focus management for a single window.

### Window Event Flow


**Sources:** [crates/gpui/src/window.rs1800-2500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/window.rs#L1800-L2500>) [crates/gpui/src/key_dispatch.rs1-350](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/key_dispatch.rs#L1-L350>) [crates/gpui/src/platform/windows/events.rs34-116](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/windows/events.rs#L34-L116>) [crates/gpui/src/platform/mac/events.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/mac/events.rs#L1-L300>)

### Key Dispatch and Action System

Zed's action system maps keystrokes to actions through a two-phase dispatch:

  1. **Keymap Matching** : Keystrokes are matched against registered keybindings in context
  2. **Action Dispatch** : Actions are dispatched through the element tree


The `KeyContext` determines which keybindings are active:


**Sources:** [crates/gpui/src/key_dispatch.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/key_dispatch.rs#L1-L200>) [crates/gpui/src/keymap.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/keymap.rs#L1-L300>) [crates/gpui/src/window.rs2000-2500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/window.rs#L2000-L2500>)

### Focus Management

The focus system tracks which element should receive keyboard input:

  * `FocusHandle`: Strong reference to a focusable element
  * `FocusId`: Stable identifier for focus tracking
  * `DispatchTree`: Maintains focus path from root to focused element


Focus changes trigger `WindowFocusEvent` notifications to all observers.

**Sources:** [crates/gpui/src/window.rs186-409](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/window.rs#L186-L409>) [crates/gpui/src/window.rs2700-3000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/window.rs#L2700-L3000>)

* * *

## Window Rendering Pipeline

GPUI uses a multi-phase rendering approach that separates layout calculation from painting.

### Render Phases


**Sources:** [crates/gpui/src/window.rs1500-2200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/window.rs#L1500-L2200>) [crates/gpui/src/taffy.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/taffy.rs#L1-L300>) [crates/gpui/src/window.rs108-184](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/window.rs#L108-L184>)

### Element Tree and Rendering

Elements implement the `IntoElement` and `Element` traits to participate in rendering:

  * **Prepaint** : Build the element tree and compute element bounds
  * **Layout** : Use Taffy to compute CSS flexbox/grid layouts
  * **Paint** : Generate `Scene` objects with GPU primitives (quads, paths, text, images)


The rendering pipeline is optimized to minimize redraws:

  * Dirty tracking: Only re-render elements that have changed
  * Invalidation regions: Track which screen areas need updating
  * Frame coalescing: Multiple invalidations result in a single frame


**Sources:** [crates/gpui/src/window.rs1500-2000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/window.rs#L1500-L2000>) [crates/gpui/src/element.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/element.rs#L1-L300>)

* * *

## Editor Architecture Overview

The Editor is the core text editing component. It manages:

  * Text content via `MultiBuffer`
  * Visual display via `DisplayMap`
  * Selections and cursors
  * Syntax highlighting and language features
  * Edit prediction and AI completions


### Editor Component Structure


**Sources:** [crates/editor/src/editor.rs1-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L1-L600>) [crates/multi_buffer/src/multi_buffer.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer.rs#L1-L300>) [crates/editor/src/display_map.rs1-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/display_map.rs#L1-L400>) [crates/text/src/anchor.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/text/src/anchor.rs#L1-L200>) [crates/rope/src/rope.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/rope/src/rope.rs#L1-L300>)

* * *

## Workspace Organization

The `Workspace` is the top-level container that organizes the editor UI into panes, panels, and a status bar.

### Workspace Component Hierarchy


**Sources:** [crates/workspace/src/workspace.rs1-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace.rs#L1-L600>) [crates/workspace/src/pane.rs1-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/pane.rs#L1-L400>) [crates/workspace/src/dock.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/dock.rs#L1-L300>) [crates/workspace/src/status_bar.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/status_bar.rs#L1-L200>)

### Pane and Item System

Panes contain `Item` trait objects, which can be editors, terminals, image viewers, etc. Panes support:

  * Split horizontal/vertical
  * Tab management
  * Focus tracking
  * Serialization for session restore


**Sources:** [crates/workspace/src/pane.rs200-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/pane.rs#L200-L500>) [crates/workspace/src/item.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/item.rs#L1-L200>)

* * *

## Project Services Coordination

The `Project` entity coordinates all project-level services including language servers, buffers, worktrees, and version control.

### Project Service Architecture


**Sources:** [crates/project/src/project.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L1-L800>) [crates/project/src/lsp_store.rs1-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L1-L400>) [crates/project/src/buffer_store.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs#L1-L300>) [crates/worktree/src/worktree.rs1-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs#L1-L500>) [crates/git/src/repository.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L1-L200>)

* * *

## Settings System

Zed uses a hierarchical settings system where configuration files merge from defaults through user and project-specific overrides.

### Settings Merge Hierarchy


Settings are accessed via the `Settings::get_global(cx)` method, which returns typed settings structures. Changes to settings files trigger automatic reloads and notifications to observers.

**Sources:** [crates/settings/src/settings.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings.rs#L1-L300>) [crates/zed/src/zed.rs414-416](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/zed.rs#L414-L416>)

* * *

## Code Entity Reference Table

System Concept| Primary Code Entities| File Location  
---|---|---  
Application| `Application`, `App`, `AppCell`, `AppState`| [crates/gpui/src/gpui.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/gpui.rs>) [crates/gpui/src/app.rs63-121](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app.rs#L63-L121>) [crates/zed/src/main.rs406-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/main.rs#L406-L600>)  
Platform| `Platform` trait, `MacPlatform`, `WindowsPlatform`, `X11Client`, `WaylandClient`| [crates/gpui/src/platform.rs125-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform.rs#L125-L400>) [crates/gpui/src/platform/mac/platform.rs27-120](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/mac/platform.rs#L27-L120>) [crates/gpui/src/platform/windows/platform.rs32-172](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/windows/platform.rs#L32-L172>)  
Window| `Window`, `WindowInvalidator`, `PlatformWindow` trait, `MacWindow`, `WindowsWindow`, `X11Window`, `WaylandWindow`| [crates/gpui/src/window.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/window.rs#L1-L300>) [crates/gpui/src/platform/mac/window.rs55-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/mac/window.rs#L55-L200>) [crates/gpui/src/platform/windows/window.rs31-155](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform/windows/window.rs#L31-L155>)  
Entities| `Entity<T>` wrapper, `EntityMap`, `EntityId`, `Context<'a, T>`| [crates/gpui/src/app/entity_map.rs1-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app/entity_map.rs#L1-L400>) [crates/gpui/src/app/context.rs1-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app/context.rs#L1-L600>)  
Events| `DispatchTree`, `DispatchPhase`, `Action` trait, `KeyBinding`| [crates/gpui/src/window.rs70-99](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/window.rs#L70-L99>) [crates/gpui/src/key_dispatch.rs1-350](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/key_dispatch.rs#L1-L350>) [crates/gpui/src/keymap.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/keymap.rs#L1-L300>)  
Editor| `Editor` entity, `MultiBuffer`, `DisplayMap`, `EditorElement`| [crates/editor/src/editor.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L1-L800>) [crates/multi_buffer/src/multi_buffer.rs1-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer.rs#L1-L500>) [crates/editor/src/display_map.rs1-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/display_map.rs#L1-L400>)  
Workspace| `Workspace` entity, `Pane` entity, `Panel` trait, `Item` trait, `Dock`| [crates/workspace/src/workspace.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace.rs#L1-L800>) [crates/workspace/src/pane.rs1-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/pane.rs#L1-L600>) [crates/workspace/src/panel.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/panel.rs#L1-L200>)  
Project| `Project` entity, `LspStore`, `BufferStore`, `WorktreeStore`, `Worktree`| [crates/project/src/project.rs1-1000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L1-L1000>) [crates/project/src/lsp_store.rs1-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L1-L600>) [crates/worktree/src/worktree.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs#L1-L800>)  
Settings| `SettingsStore`, `Settings` trait, `KeymapFile`, `SettingsLocation`| [crates/settings/src/settings.rs1-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings.rs#L1-L600>) [crates/settings/src/keymap_file.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/keymap_file.rs#L1-L300>)  
  
**Sources:** [crates/gpui/src/](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/>) [crates/zed/src/](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/>) [crates/editor/src/](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/>) [crates/workspace/src/](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/>) [crates/project/src/](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/>) [crates/settings/src/](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/>)

* * *

## Summary

Zed's core architecture is built on several key principles:

  1. **Layered abstraction** : Clean separation between platform, UI framework, and application logic
  2. **Reactive state management** : Entity-based system with automatic invalidation and re-rendering
  3. **Trait-based polymorphism** : Platform abstraction, window management, and UI components use traits for flexibility
  4. **Multi-phase rendering** : Separate layout from painting for performance
  5. **Service coordination** : Project entity coordinates language servers, buffers, and version control
  6. **Hierarchical configuration** : Settings merge from multiple sources with type-safe access


The architecture enables:

  * Cross-platform support (macOS, Windows, Linux)
  * GPU-accelerated rendering
  * Responsive UI with efficient updates
  * Extensible through traits and actions
  * Collaborative editing through RPC
  * AI integration at multiple layers


Dismiss

Refresh this wiki

This wiki was recently refreshed. Please wait 7 days to refresh again.

### On this page

  * [Core Architecture](<#core-architecture>)
  * [Purpose and Scope](<#purpose-and-scope>)
  * [Architectural Overview](<#architectural-overview>)
  * [High-Level System Layers](<#high-level-system-layers>)
  * [Application Initialization Flow](<#application-initialization-flow>)
  * [Application Startup Sequence](<#application-startup-sequence>)
  * [Key Initialization Steps](<#key-initialization-steps>)
  * [Platform Abstraction Layer](<#platform-abstraction-layer>)
  * [Platform Trait and Implementations](<#platform-trait-and-implementations>)
  * [Platform Window Creation](<#platform-window-creation>)
  * [GPUI Reactive State Management](<#gpui-reactive-state-management>)
  * [Entity and Context System](<#entity-and-context-system>)
  * [Entity Creation and Updates](<#entity-creation-and-updates>)
  * [Window and Event Dispatch](<#window-and-event-dispatch>)
  * [Window Event Flow](<#window-event-flow>)
  * [Key Dispatch and Action System](<#key-dispatch-and-action-system>)
  * [Focus Management](<#focus-management>)
  * [Window Rendering Pipeline](<#window-rendering-pipeline>)
  * [Render Phases](<#render-phases>)
  * [Element Tree and Rendering](<#element-tree-and-rendering>)
  * [Editor Architecture Overview](<#editor-architecture-overview>)
  * [Editor Component Structure](<#editor-component-structure>)
  * [Workspace Organization](<#workspace-organization>)
  * [Workspace Component Hierarchy](<#workspace-component-hierarchy>)
  * [Pane and Item System](<#pane-and-item-system>)
  * [Project Services Coordination](<#project-services-coordination>)
  * [Project Service Architecture](<#project-service-architecture>)
  * [Settings System](<#settings-system>)
  * [Settings Merge Hierarchy](<#settings-merge-hierarchy>)
  * [Code Entity Reference Table](<#code-entity-reference-table>)
  * [Summary](<#summary>)
