<!-- Source: https://deepwiki.com/zed-industries/zed/1-overview -->

# 1 Overview

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

# Overview

Relevant source files

  * [Cargo.lock](<https://github.com/zed-industries/zed/blob/4109c9dd/Cargo.lock>)
  * [Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/Cargo.toml>)
  * [crates/gpui/Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/Cargo.toml>)
  * [crates/gpui/build.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/build.rs>)
  * [crates/gpui/src/geometry.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/geometry.rs>)
  * [crates/gpui/src/taffy.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/taffy.rs>)
  * [crates/zed/Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/Cargo.toml>)
  * [crates/zed/src/main.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/main.rs>)
  * [crates/zed/src/zed.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/zed.rs>)


## What is Zed?

Zed is a high-performance, collaborative code editor built in Rust. The codebase is structured as a monorepo with approximately 200 workspace crates, organized into distinct architectural layers. The application uses a custom GPU-accelerated UI framework (GPUI) for rendering, an entity-based reactive state management system, and supports both local and remote development workflows.

Key characteristics:

  * **GPU-accelerated rendering** : All UI rendering goes through GPUI's Metal/DirectX/Vulkan backends
  * **Entity-based architecture** : State management uses `Entity<T>` with reactive updates via `cx.notify()` and `cx.emit()`
  * **LSP-first language support** : Language intelligence through Language Server Protocol integration
  * **Collaborative by design** : Built-in support for real-time collaboration and SSH remote development
  * **Agent integration** : Native support for AI agents via Agent Client Protocol (ACP)


The application entry point is [crates/zed/src/main.rs1-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/main.rs#L1-L600>) which initializes `AppState` containing core services (`Client`, `LanguageRegistry`, `NodeRuntime`, etc.) and opens the initial `Workspace` window.

* * *

## Architectural Overview

Zed follows a layered architecture where each layer provides abstractions for the layers above it. The system is organized into approximately 200 workspace crates, with clear separation between platform abstractions, UI framework, workspace management, editor functionality, and project services.

### Architectural Layers


**Diagram: Core Architectural Layers**

The architecture follows strict layering:

**Application Layer** : `zed::main()` initializes `workspace::AppState` containing shared services (`Arc<Client>`, `Arc<LanguageRegistry>`, `Arc<UserStore>`, etc.). This state is passed to all windows.

**UI Framework** : `gpui::App` manages the event loop and global state. `gpui::Window` provides per-window context. The `Platform` trait abstracts OS-specific windowing (Wayland, X11, Cocoa, Win32). `Element` trait implementors render via GPU.

**Workspace & Editors**: `workspace::Workspace` is the root container per window. `workspace::Pane` contains items via the `Item` trait. `editor::Editor` uses `multi_buffer::MultiBuffer` to aggregate `language::Buffer` entities containing CRDT-based text storage.

**Project Management** : `project::Project` coordinates specialized stores: `lsp_store::LspStore` for language servers, `worktree_store::WorktreeStore` for file trees, `buffer_store::BufferStore` for open files, `git_store::GitStore` for repository state.

**AI Agent System** : `agent_ui::AgentPanel` manages UI for `acp_thread::AcpThread` conversations. `agent_servers::AgentConnection` communicates with external agent processes via stdio.

**Configuration & Input**: `settings::SettingsStore` manages hierarchical settings. `settings::KeymapFile` maps keystrokes to `gpui::Action` types which dispatch through the focus tree.

Sources: [crates/zed/src/main.rs1-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/main.rs#L1-L600>) [crates/gpui/src/gpui.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/gpui.rs#L1-L100>) [crates/workspace/src/workspace.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace.rs#L1-L300>) [crates/editor/src/editor.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L1-L300>) [crates/project/src/project.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L1-L200>)

* * *

## Primary Subsystems

### 1\. Application and Window Management

Entry point `zed::main()` at [crates/zed/src/main.rs168-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/main.rs#L168-L500>) performs initialization:

  1. **Parse command-line arguments** via `Args::parse()`
  2. **Initialize paths** : `paths::set_custom_data_dir()`, create config directories
  3. **Start logging** : `zlog::init()` and `ztracing::init()`
  4. **Create`Application`**: `Application::new().with_assets(Assets)`
  5. **Initialize background tasks** : System ID, installation ID, session tracking
  6. **Watch config files** : `watch_config_file()` for settings and keymaps
  7. **Start HTTP client** : `ReqwestClient::proxy_and_user_agent()`
  8. **Build`AppState`**: Construct with `Client`, `UserStore`, `LanguageRegistry`, `NodeRuntime`, etc.
  9. **Initialize services** : Extension host, language servers, git providers
  10. **Open windows** : Create initial `Workspace` via `workspace::open_new()`


**Key Types:**

  * `gpui::App` \- Global application context, event loop controller
  * `workspace::AppState` \- Contains `Arc<Client>`, `Arc<UserStore>`, `Arc<LanguageRegistry>`, `Arc<Fs>`, etc.
  * `gpui::WindowHandle<V>` \- Strong typed handle to window with root view `V`
  * `gpui::Window` \- Per-window rendering and event context
  * `settings::SettingsStore` \- Global hierarchical configuration


The `AppState` is set as a global via `AppState::set_global()` and accessed via `AppState::global(cx)` throughout the codebase.

Sources: [crates/zed/src/main.rs168-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/main.rs#L168-L500>) [crates/workspace/src/workspace.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace.rs#L1-L200>) [crates/gpui/src/app.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app.rs#L1-L200>)

### 2\. GPUI Framework

GPUI (`crates/gpui`) provides GPU-accelerated UI rendering with a reactive entity system. Architecture:

**Platform Abstraction** : `Platform` trait at [crates/gpui/src/platform.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform.rs#L1-L200>) with implementations:

  * `LinuxPlatform` (Wayland via `wayland-client`, X11 via `x11rb`)
  * `MacPlatform` (Cocoa via `objc`)
  * `WindowsPlatform` (Win32 via `windows` crate)


**Reactive Entities** : State management using reference-counted entities:

  * `Entity<T>` \- Strong reference to entity state
  * `WeakEntity<T>` \- Weak reference, upgradeable via `.upgrade()`
  * `Context<T>` \- Mutable access scope within `entity.update(cx, |entity, window, cx| ...)`


Entities notify observers via `cx.notify()` and emit typed events via `cx.emit(event)`. Components observe with `cx.observe(&entity, ...)` and subscribe with `cx.subscribe(&entity, ...)`.

**Element System** : UI rendering via `Element` trait at [crates/gpui/src/element.rs1-150](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/element.rs#L1-L150>) Two-phase rendering:

  1. `prepaint(bounds, cx)` \- Layout computation, returns `()` or prepaint state
  2. `paint(bounds, prepaint_state, cx)` \- Emit GPU commands via `cx.paint_quad()`, `cx.paint_text()`, etc.


Text rendering uses `ShapedLine` from `cosmic-text` crate. Quads, shadows, and sprites emit `Scene` commands that translate to Metal/DirectX/Vulkan.

**Action Dispatch** : Commands typed as `Action` trait implementors (via `#[derive(Action)]`). Keystroke → `KeymapFile` lookup → `Action` construction → focus tree dispatch → handler execution.

Sources: [crates/gpui/src/platform.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform.rs#L1-L200>) [crates/gpui/src/app.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app.rs#L1-L300>) [crates/gpui/src/element.rs1-150](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/element.rs#L1-L150>) [crates/editor/src/element.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/element.rs#L1-L200>)

### 3\. Workspace Organization

`workspace::Workspace` (at [crates/workspace/src/workspace.rs1-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace.rs#L1-L500>)) is the root container per window:


**Diagram: Workspace Structure**

**PaneGroup** : Recursive tree at [crates/workspace/src/pane_group.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/pane_group.rs#L1-L100>) Each node is `Member::Pane(Entity<Pane>)` or `Member::Axis { axis: Axis, members: Vec<Member> }` for splits.

**Pane** : Holds items via `workspace::Item` trait at [crates/workspace/src/item.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/item.rs#L1-L200>) Items include:

  * `editor::Editor` \- Text editor
  * `terminal_view::TerminalView` \- Integrated terminal
  * `image_viewer::ImageView` \- Image display
  * `search::ProjectSearchView` \- Search results
  * `collab_ui::channel_view::ChannelView` \- Chat view


**Docks** : Three docks (`LeftDock`, `RightDock`, `BottomDock`) at [crates/workspace/src/dock.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/dock.rs#L1-L300>) contain `Panel` trait implementors:

  * `project_panel::ProjectPanel` \- File tree
  * `outline_panel::OutlinePanel` \- Symbol outline
  * `git_ui::git_panel::GitPanel` \- Git staging UI
  * `agent_ui::AgentPanel` \- AI agent conversations
  * `terminal_view::terminal_panel::TerminalPanel` \- Terminal list


**StatusBar** : Bottom bar at [crates/workspace/src/status_bar.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/status_bar.rs#L1-L200>) with `StatusItemView` components (diagnostics, activity indicator, language selector, etc.).

**Toolbar** : Per-pane toolbar at [crates/workspace/src/toolbar.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/toolbar.rs#L1-L100>) with `ToolbarItemView` components (breadcrumbs, search bar, etc.).

Sources: [crates/workspace/src/workspace.rs1-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace.rs#L1-L500>) [crates/workspace/src/pane_group.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/pane_group.rs#L1-L100>) [crates/workspace/src/pane.rs1-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/pane.rs#L1-L400>) [crates/workspace/src/dock.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/dock.rs#L1-L300>) [crates/workspace/src/item.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/item.rs#L1-L200>)

### 4\. Editor and Text Model

`editor::Editor` (at [crates/editor/src/editor.rs1-3000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L1-L3000>)) provides text editing functionality:


**Diagram: Editor Text Pipeline**

**MultiBuffer** : At [crates/multi_buffer/src/multi_buffer.rs1-1500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer.rs#L1-L1500>) aggregates `Excerpt`s from one or more `Buffer` entities. Supports multiple files in one view (e.g., search results, diagnostics).

**Buffer** : At [crates/language/src/buffer.rs1-3500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/buffer.rs#L1-L3500>) stores text as `text::Rope` (CRDT) at [crates/text/src/text.rs1-2000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/text/src/text.rs#L1-L2000>) Integrates `tree_sitter::Tree` for syntax via `language::Grammar`.

**DisplayMap** : At [crates/editor/src/display_map.rs1-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/display_map.rs#L1-L400>) transforms buffer coordinates → display coordinates via pipeline:

  1. `FoldMap` \- Fold ranges (imports, functions, etc.)
  2. `InlayMap` \- Inlay hints from LSP, ghost text from edit predictions
  3. `WrapMap` \- Soft line wrapping
  4. `TabMap` \- Tab expansion
  5. `BlockMap` \- Block decorations (e.g., diagnostics, diffs)


**SelectionsCollection** : At [crates/editor/src/selections_collection.rs1-1200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/selections_collection.rs#L1-L1200>) manages multi-cursor state. Each selection has `head` and `tail` in buffer coordinates. Supports rectangular selections.

**EditorElement** : At [crates/editor/src/element.rs1-3000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/element.rs#L1-L3000>) renders editor via `Element::prepaint()` / `Element::paint()`. Computes visible line range, shapes text with `cosmic-text`, emits GPU quads.

Sources: [crates/editor/src/editor.rs1-3000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L1-L3000>) [crates/multi_buffer/src/multi_buffer.rs1-1500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer.rs#L1-L1500>) [crates/language/src/buffer.rs1-3500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/buffer.rs#L1-L3500>) [crates/text/src/text.rs1-2000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/text/src/text.rs#L1-L2000>) [crates/editor/src/display_map.rs1-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/display_map.rs#L1-L400>) [crates/editor/src/element.rs1-3000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/element.rs#L1-L3000>)

### 5\. Project Services

`project::Project` (at [crates/project/src/project.rs1-7000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L1-L7000>)) orchestrates project-level services via specialized stores:


**Diagram: Project Service Coordination**

**LspStore** : At [crates/project/src/lsp_store.rs1-3000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L1-L3000>) manages language server lifecycle. Variants:

  * `LocalLspStore` \- Spawns `lsp::LanguageServer` processes locally via stdio
  * `RemoteLspStore` \- Proxies LSP requests over RPC to remote machine


Sends LSP requests: `textDocument/completion`, `textDocument/definition`, `textDocument/hover`, `textDocument/codeAction`. Receives `textDocument/publishDiagnostics` notifications. Uses `lsp::LanguageServerBinary` from extensions or built-in definitions.

**WorktreeStore** : At [crates/project/src/worktree_store.rs1-1000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/worktree_store.rs#L1-L1000>) manages `Entity<Worktree>` per opened directory. `worktree::Worktree` at [crates/worktree/src/worktree.rs1-3000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs#L1-L3000>) maintains `Entry` tree and watches via `fs::Watcher` (inotify/FSEvents/ReadDirectoryChangesW).

**BufferStore** : At [crates/project/src/buffer_store.rs1-1500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs#L1-L1500>) tracks open `Entity<Buffer>` instances by `BufferId`. Coordinates with `WorktreeStore` for file saves, `LspStore` for diagnostics.

**GitStore** : At [crates/project/src/git_store.rs1-1500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L1-L1500>) uses `git::Repository` at [crates/git/src/repository.rs1-1000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L1-L1000>) to query `git status`, compute blame, extract diff hunks. Integrates with `buffer_diff::BufferDiff` for inline diff display.

**DapStore** : At [crates/dap/src/dap_store.rs1-1000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/dap/src/dap_store.rs#L1-L1000>) manages Debug Adapter Protocol sessions. Spawns `dap::DebugAdapterClient`, handles breakpoints, stack traces, variable inspection.

**TaskStore** : At [crates/task/src/task_store.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/task/src/task_store.rs#L1-L800>) loads task templates from `.zed/tasks.json`. Resolves variables (`$ZED_COLUMN`, `$ZED_FILE`), detects toolchains (npm, cargo, etc.).

Sources: [crates/project/src/project.rs1-7000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L1-L7000>) [crates/project/src/lsp_store.rs1-3000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L1-L3000>) [crates/project/src/buffer_store.rs1-1500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs#L1-L1500>) [crates/project/src/worktree_store.rs1-1000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/worktree_store.rs#L1-L1000>) [crates/project/src/git_store.rs1-1500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L1-L1500>) [crates/worktree/src/worktree.rs1-3000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs#L1-L3000>) [crates/git/src/repository.rs1-1000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L1-L1000>)

### 6\. AI Integration

Zed provides two AI integration modes:

**Edit Prediction (Inline Completions)** : At [crates/edit_prediction/src/edit_prediction.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/edit_prediction/src/edit_prediction.rs#L1-L800>) providers implement `EditPredictionProvider` trait:

  * `copilot::Copilot` \- GitHub Copilot via LSP
  * `supermaven::Supermaven` \- Supermaven via native client
  * `zeta::Zeta` \- Zed's first-party model
  * `codestral::Codestral` \- Mistral's code model


The `edit_prediction_registry::EditPredictionRegistry` at [crates/zed/src/zed.rs1-1200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/zed.rs#L1-L1200>) manages lifecycle. `Editor` receives predictions and displays as ghost text via `InlayMap` in the display pipeline.

**Conversational Agents** : Two systems with migration path:


**Diagram: Agent System Architecture**

**AcpThread** : At [crates/acp_thread/src/acp_thread.rs1-2000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/acp_thread.rs#L1-L2000>) implements Agent Client Protocol. Manages:

  * Message history (user messages, agent responses, tool calls)
  * Tool authorization workflow (user must approve dangerous operations)
  * Streaming responses from agent


**AgentConnection** : At [crates/agent_servers/src/agent_connection.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/agent_connection.rs#L1-L800>) spawns external agent binaries (e.g., `claude-code-acp`) and communicates via stdio using Agent Client Protocol messages.

**Built-in Tools** : At [crates/acp_tools/src/](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_tools/src/>) Tools include:

  * `ReadFileTool` \- Read file contents
  * `WriteFileTool` / `EditFileTool` \- Modify files
  * `SearchFileTool` \- Project-wide search
  * `TerminalTool` \- Execute shell commands
  * `DiagnosticsTool` \- Query LSP diagnostics


**Legacy AssistantThread** : At [crates/assistant_text_thread/src/thread.rs1-2000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/assistant_text_thread/src/thread.rs#L1-L2000>) uses prompt templates and `language_model::LanguageModelRegistry` to call Anthropic/OpenAI/Google APIs directly. Being phased out in favor of ACP.

Sources: [crates/edit_prediction/src/edit_prediction.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/edit_prediction/src/edit_prediction.rs#L1-L800>) [crates/agent_ui/src/agent_panel.rs1-1500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/agent_panel.rs#L1-L1500>) [crates/acp_thread/src/acp_thread.rs1-2000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/acp_thread.rs#L1-L2000>) [crates/agent_servers/src/agent_connection.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/agent_connection.rs#L1-L800>) [crates/acp_tools/src/lib.rs1-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_tools/src/lib.rs#L1-L500>) [crates/assistant_text_thread/src/thread.rs1-2000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/assistant_text_thread/src/thread.rs#L1-L2000>)

* * *

## Event Flow: User Input to Screen Update


**Diagram: Complete Input-to-Display Event Flow**

Event processing pipeline:

  1. **Platform Input** : Native events (NSEvent on macOS, XKeyEvent on X11, WM_KEYDOWN on Windows) normalized to `PlatformInput` at [crates/gpui/src/platform.rs1-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform.rs#L1-L500>)

  2. **Key Dispatch** : `DispatchTree` at [crates/gpui/src/window.rs2000-2500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/window.rs#L2000-L2500>) walks focus tree. `KeymapFile` at [crates/settings/src/keymap_file.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/keymap_file.rs#L1-L800>) matches keystroke sequences to `Action` types based on `KeyContext`

  3. **Action Handling** : `Action` dispatched to focused element. `Editor::handle_input()` at [crates/editor/src/editor.rs2000-2500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L2000-L2500>) processes typing

  4. **Text Mutation** : `MultiBuffer::edit()` at [crates/multi_buffer/src/multi_buffer.rs500-700](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer.rs#L500-L700>) applies to underlying `text::Rope` CRDT at [crates/text/src/text.rs1-2000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/text/src/text.rs#L1-L2000>) Emits `BufferEvent::Edited`

  5. **Invalidation** : `Editor` calls `cx.notify()` which marks window dirty and schedules next frame

  6. **Layout** : `EditorElement::prepaint()` at [crates/editor/src/element.rs500-1000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/element.rs#L500-L1000>) queries `DisplayMap::chunks()` to get visible text ranges with syntax highlighting

  7. **Rendering** : `EditorElement::paint()` at [crates/editor/src/element.rs1500-2500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/element.rs#L1500-L2500>) emits GPU commands via `cx.paint_text()`, `cx.paint_quad()` building a `Scene`

  8. **Presentation** : Platform-specific present (Metal, DirectX, or Vulkan) at [crates/gpui/src/platform.rs1-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform.rs#L1-L500>)


Sources: [crates/gpui/src/platform.rs1-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/platform.rs#L1-L500>) [crates/gpui/src/window.rs2000-2500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/window.rs#L2000-L2500>) [crates/settings/src/keymap_file.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/keymap_file.rs#L1-L800>) [crates/editor/src/editor.rs2000-2500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L2000-L2500>) [crates/multi_buffer/src/multi_buffer.rs500-700](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer.rs#L500-L700>) [crates/text/src/text.rs1-2000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/text/src/text.rs#L1-L2000>) [crates/editor/src/element.rs500-2500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/element.rs#L500-L2500>)

* * *

## Crate Organization

The workspace contains ~200 crates in [Cargo.toml1-230](<https://github.com/zed-industries/zed/blob/4109c9dd/Cargo.toml#L1-L230>) Major categories:

Category| Crates| Purpose  
---|---|---  
**Core Infrastructure**| `gpui`, `collections`, `util`, `text`, `rope`, `sum_tree`| UI framework, data structures  
**Editor & Text**| `editor`, `multi_buffer`, `language`, `languages`| Text editing, syntax parsing  
**Workspace & UI**| `workspace`, `ui`, `theme`, `picker`, `search`| Window management, UI components  
**Project Services**| `project`, `lsp`, `worktree`, `fs`, `git`| Project-level coordination  
**AI & Agents**| `agent`, `agent_ui`, `acp_thread`, `acp_tools`, `edit_prediction`, `zeta`| AI integrations  
**Collaboration**| `client`, `rpc`, `collab`, `remote`| RPC protocol, collaboration server  
**Development Tools**| `diagnostics`, `debugger_ui`, `dap`, `terminal`, `terminal_view`, `tasks_ui`| LSP diagnostics, debugger, terminal  
  
Key patterns:

  * Crates expose entities via `pub struct X` and `pub type Handle = Entity<X>`
  * Event emission via `EventEmitter` trait (`impl EventEmitter<XEvent> for X`)
  * Settings via `Settings::get_global(cx)` from `SettingsStore`


Sources: [Cargo.toml1-230](<https://github.com/zed-industries/zed/blob/4109c9dd/Cargo.toml#L1-L230>) [Cargo.lock1-1000](<https://github.com/zed-industries/zed/blob/4109c9dd/Cargo.lock#L1-L1000>)

* * *

## Entity and State Management

GPUI's entity system provides reactive state management with automatic memory management:


**Diagram: Entity Lifecycle and Reactivity**

**Creating Entities** :


Entities are reference-counted. Dropped when last `Entity<T>` reference released.

**Updating State** :


`update()` provides `Context<Editor>` scope for mutations.

**Observation** :


Observers fire on `cx.notify()` calls within observed entity.

**Event Subscription** :


Subscribers receive typed events from `cx.emit(event)`.

**Weak References** :


Store `WeakEntity<T>` in async tasks to avoid cycles.

**Key Types** : At [crates/gpui/src/app.rs1-1000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app.rs#L1-L1000>)

  * `Entity<T>` \- Strong reference, implements `Clone`
  * `WeakEntity<T>` \- Weak reference, `upgrade()` returns `Option<Entity<T>>`
  * `Context<T>` \- Scoped mutable access with `&mut Window`, `&mut App`


Sources: [crates/gpui/src/app.rs1-1000](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/app.rs#L1-L1000>) [crates/editor/src/editor.rs100-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L100-L200>) [crates/workspace/src/workspace.rs1-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace.rs#L1-L500>)

* * *

## Settings and Configuration

Zed uses a layered settings system with live reloading:


**Diagram: Settings Hierarchy and Propagation**

**Settings Precedence** (lowest to highest):

  1. `assets/settings/default.json` \- Embedded defaults
  2. `~/.config/zed/settings.json` \- User global settings
  3. Profile settings (if active profile)
  4. Release channel overrides
  5. OS-specific overrides
  6. `.zed/settings.json` \- Project-local settings
  7. `.editorconfig` \- Standard editor config


**SettingsStore** : At [crates/settings/src/settings_store.rs1-1500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_store.rs#L1-L1500>) manages hierarchy. Files watched via `watch_config_file()` at [crates/settings/src/settings_file.rs1-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_file.rs#L1-L500>) which uses `fs::Watcher`. Changes trigger reload and notify observers.

**Typed Settings** : At [crates/settings/src/settings.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings.rs#L1-L800>) types implement `Settings` trait:

  * `EditorSettings` at [crates/editor/src/editor_settings.rs1-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor_settings.rs#L1-L500>) \- Font, line numbers, tab size, scrollbar, etc.
  * `WorkspaceSettings` at [crates/workspace/src/workspace_settings.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace_settings.rs#L1-L300>) \- Dock positions, active pane direction
  * `LanguageSettings` at [crates/language/src/language_settings.rs1-700](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/language_settings.rs#L1-L700>) \- Per-language formatters, LSP config, tab size
  * `ThemeSettings` at [crates/theme/src/settings.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/theme/src/settings.rs#L1-L200>) \- Active theme, syntax colors
  * `ProjectSettings` at [crates/project/src/project_settings.rs1-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project_settings.rs#L1-L400>) \- LSP settings, git settings, tasks


**Access Patterns** :


**KeymapFile** : Separate system at [crates/settings/src/keymap_file.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/keymap_file.rs#L1-L800>) for key bindings. Loaded from `~/.config/zed/keymap.json`. Merged with base keymap (`assets/keymaps/default-*.json` per OS and `assets/keymaps/vim.json` if enabled).

Sources: [crates/settings/src/settings_store.rs1-1500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_store.rs#L1-L1500>) [crates/settings/src/settings.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings.rs#L1-L800>) [crates/settings/src/keymap_file.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/keymap_file.rs#L1-L800>) [crates/editor/src/editor_settings.rs1-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor_settings.rs#L1-L500>) [assets/settings/default.json1-1000](<https://github.com/zed-industries/zed/blob/4109c9dd/assets/settings/default.json#L1-L1000>)

* * *

## Collaboration Architecture

Zed supports two collaboration modes through a dual-mode architecture pattern:

**Remote Development (SSH):**

  * `remote_server::HeadlessProject` runs on SSH host
  * `client::Client` RPC connection synchronizes state
  * LSP servers (`lsp::LanguageServer`) run on remote machine
  * Components use `RemoteLspStore`, `RemoteProject`, etc.


**Peer-to-Peer Collaboration:**

  * `collab::Server` routes messages between clients
  * `text::Rope` CRDT operations for conflict-free merging
  * `workspace::Following` system for view following
  * `proto::UpdateFollowers`, `proto::UpdateProject` messages


**Dual-Mode Pattern:** Each major component has Local/Remote variants:

  * `project::Project` → `LocalProject` | `RemoteProject`
  * `lsp_store::LspStore` → `LocalLspStore` | `RemoteLspStore`
  * `buffer_store::BufferStore` → local | remote modes


Sources: [crates/remote/Cargo.toml1-50](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/remote/Cargo.toml#L1-L50>) [crates/collab/Cargo.toml1-50](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/collab/Cargo.toml#L1-L50>) [crates/project/src/project.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L1-L200>) [crates/project/src/lsp_store.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L1-L200>)

* * *

## Action System

Actions provide type-safe command dispatch:

  1. **Definition** : Types implementing `gpui::Action` trait via `#[derive(Action)]`


  2. **Registration** : `cx.on_action(|action: &Insert, cx| { ... })` registers handlers

  3. **Key Bindings** : `assets/keymaps/*.json` map keystrokes to actions:


  4. **Dispatch** : Actions bubble up focus chain until handler found

  5. **Context** : `KeyContext` determines which bindings are active (e.g., `"Editor && mode == full"`)


Action flow: Keystroke → KeymapFile lookup → Action construction → Focus chain dispatch → Handler execution

Sources: [crates/editor/src/actions.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/actions.rs#L1-L100>) [assets/keymaps/default-macos.json1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/assets/keymaps/default-macos.json#L1-L200>) [crates/gpui/Cargo.toml1-92](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/Cargo.toml#L1-L92>)

* * *

## Testing Patterns

Zed uses `#[gpui::test]` attribute for async tests with GPUI context:

Pattern| Usage| Example  
---|---|---  
**Unit tests**| `#[gpui::test]` in module files| `editor_tests.rs`, `buffer_tests.rs`  
**Integration tests**| `EditorTestContext` for editor setup| [crates/editor/src/editor_tests.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor_tests.rs#L1-L100>)  
**LSP tests**| `EditorLspTestContext` for language servers| [crates/editor/src/test/editor_lsp_test_context.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/test/editor_lsp_test_context.rs#L1-L100>)  
**Visual tests**| `VisualTestContext` for UI rendering| [crates/gpui/src/test.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/gpui/src/test.rs#L1-L100>)  
  
Test utilities:

  * `TestAppContext`: GPUI app context for tests
  * `build_editor()`: Creates `Entity<Editor>` with test buffer
  * `FakeLspAdapter`: Mock language server for testing
  * `FakeFs`: In-memory file system


Sources: [crates/editor/src/editor_tests.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor_tests.rs#L1-L200>) [crates/editor/src/test/editor_test_context.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/test/editor_test_context.rs#L1-L100>) [crates/project/src/project_tests.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project_tests.rs#L1-L200>)

* * *

## Summary

Zed's architecture follows clear layering principles:

  * **GPUI** provides platform abstraction and reactive UI
  * **Workspace** organizes editor windows into panes and panels
  * **Editor** handles text editing with sophisticated display transformations
  * **Project** coordinates language servers, git, and file system
  * **Remote/Collab** enables distributed development


This separation enables parallel development, testability, and clear ownership boundaries. Each major subsystem is encapsulated in its own crate with well-defined interfaces.

Sources: [crates/zed/src/zed.rs1-150](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/zed.rs#L1-L150>) [crates/workspace/src/workspace.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace.rs#L1-L200>) [crates/editor/src/editor.rs1-250](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L1-L250>) [crates/project/src/project.rs1-150](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L1-L150>)

Dismiss

Refresh this wiki

This wiki was recently refreshed. Please wait 7 days to refresh again.

### On this page

  * [Overview](<#overview>)
  * [What is Zed?](<#what-is-zed>)
  * [Architectural Overview](<#architectural-overview>)
  * [Architectural Layers](<#architectural-layers>)
  * [Primary Subsystems](<#primary-subsystems>)
  * [1\. Application and Window Management](<#1-application-and-window-management>)
  * [2\. GPUI Framework](<#2-gpui-framework>)
  * [3\. Workspace Organization](<#3-workspace-organization>)
  * [4\. Editor and Text Model](<#4-editor-and-text-model>)
  * [5\. Project Services](<#5-project-services>)
  * [6\. AI Integration](<#6-ai-integration>)
  * [Event Flow: User Input to Screen Update](<#event-flow-user-input-to-screen-update>)
  * [Crate Organization](<#crate-organization>)
  * [Entity and State Management](<#entity-and-state-management>)
  * [Settings and Configuration](<#settings-and-configuration>)
  * [Collaboration Architecture](<#collaboration-architecture>)
  * [Action System](<#action-system>)
  * [Testing Patterns](<#testing-patterns>)
  * [Summary](<#summary>)
