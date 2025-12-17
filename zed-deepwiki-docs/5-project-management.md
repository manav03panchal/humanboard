<!-- Source: https://deepwiki.com/zed-industries/zed/5-project-management -->

# 5 Project Management

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

# Project Management

Relevant source files

  * [crates/collab/src/tests/editor_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/collab/src/tests/editor_tests.rs>)
  * [crates/collab/src/tests/integration_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/collab/src/tests/integration_tests.rs>)
  * [crates/collab/src/tests/remote_editing_collaboration_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/collab/src/tests/remote_editing_collaboration_tests.rs>)
  * [crates/copilot/Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/copilot/Cargo.toml>)
  * [crates/copilot/src/copilot.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/copilot/src/copilot.rs>)
  * [crates/copilot/src/sign_in.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/copilot/src/sign_in.rs>)
  * [crates/editor/src/hover_links.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/hover_links.rs>)
  * [crates/fs/src/fs.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/fs/src/fs.rs>)
  * [crates/git/src/status.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/status.rs>)
  * [crates/lsp/src/lsp.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/lsp/src/lsp.rs>)
  * [crates/prettier/src/prettier.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/prettier/src/prettier.rs>)
  * [crates/prettier/src/prettier_server.js](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/prettier/src/prettier_server.js>)
  * [crates/project/src/buffer_store.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs>)
  * [crates/project/src/lsp_command.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_command.rs>)
  * [crates/project/src/lsp_store.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs>)
  * [crates/project/src/prettier_store.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/prettier_store.rs>)
  * [crates/project/src/project.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs>)
  * [crates/project/src/project_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project_tests.rs>)
  * [crates/project/src/worktree_store.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/worktree_store.rs>)
  * [crates/proto/proto/lsp.proto](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/proto/proto/lsp.proto>)
  * [crates/remote_server/src/headless_project.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/remote_server/src/headless_project.rs>)
  * [crates/remote_server/src/remote_editing_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/remote_server/src/remote_editing_tests.rs>)
  * [crates/worktree/src/worktree.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs>)
  * [crates/worktree/src/worktree_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree_tests.rs>)


The Project Management system serves as the central orchestrator for all file-related operations in Zed. It coordinates multiple specialized stores to provide a unified interface for file system access, buffer management, language server integration, version control, and task execution. The `Project` entity acts as the single point of coordination, ensuring consistency across all subsystems while supporting both local and remote (SSH/collaborative) development scenarios.

For information about specific subsystems, see:

  * Language server integration: [Language Intelligence](</zed-industries/zed/6-language-intelligence>)
  * Git operations: [Git Integration](</zed-industries/zed/8-git-integration>)
  * Task execution: [Terminal and Task Execution](</zed-industries/zed/9-terminal-and-task-execution>)


## Architecture Overview

The Project Management system is built around a central `Project` entity that delegates specialized concerns to individual stores. Each store manages its own domain while communicating through events and shared entities.

### Core Components and Store Organization


Sources: [crates/project/src/project.rs183-220](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L183-L220>) [crates/project/src/worktree_store.rs54-65](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/worktree_store.rs#L54-L65>) [crates/project/src/buffer_store.rs31-41](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs#L31-L41>) [crates/project/src/lsp_store.rs254-299](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L254-L299>)

### Project Entity Structure

The `Project` struct coordinates all project-level functionality:

Field| Type| Purpose  
---|---|---  
`worktree_store`| `Entity<WorktreeStore>`| Manages file system trees  
`buffer_store`| `Entity<BufferStore>`| Tracks open buffers  
`lsp_store`| `Entity<LspStore>`| Coordinates language servers  
`git_store`| `Entity<GitStore>`| Handles version control state  
`task_store`| `Entity<TaskStore>`| Manages task definitions and execution  
`dap_store`| `Entity<DapStore>`| Debugging adapter protocol integration  
`languages`| `Arc<LanguageRegistry>`| Language definitions and syntax  
`fs`| `Arc<dyn Fs>`| File system abstraction  
`client_state`| `ProjectClientState`| Local/Shared/Remote mode  
  
Sources: [crates/project/src/project.rs183-220](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L183-L220>)

## Project Client States

The Project supports three operational modes based on collaboration context:


Sources: [crates/project/src/project.rs271-283](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L271-L283>)

The `ProjectClientState` enum determines how operations are executed:


Sources: [crates/project/src/project.rs271-283](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L271-L283>)

## Worktree Store

The `WorktreeStore` manages one or more worktrees, each representing a directory tree being edited. It handles both local worktrees (direct file system access) and remote worktrees (proxied through RPC).

### Worktree Lifecycle


Sources: [crates/worktree/src/worktree.rs357-471](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs#L357-L471>) [crates/project/src/worktree_store.rs54-65](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/worktree_store.rs#L54-L65>)

### Local vs Remote Worktrees

The `Worktree` enum abstracts over local and remote implementations:

Type| File Access| Updates| Use Case  
---|---|---|---  
`LocalWorktree`| Direct FS operations| Background scanner with fs-watcher| Host machine, SSH server  
`RemoteWorktree`| RPC to host| Receives proto::UpdateWorktree| Collaborative guests  
  
**LocalWorktree structure:**

  * `snapshot: LocalSnapshot` \- Current state with git repositories and ignores
  * `scan_requests_tx: channel::Sender` \- Request specific path scans
  * `fs: Arc<dyn Fs>` \- File system interface
  * `fs_case_sensitive: bool` \- Platform detection for case sensitivity
  * Background scanner tasks monitoring file changes


**RemoteWorktree structure:**

  * `snapshot: Snapshot` \- Current state without local metadata
  * `client: AnyProtoClient` \- RPC connection to host
  * `project_id: u64` \- Remote project identifier
  * Updates applied in background task from RPC stream


Sources: [crates/worktree/src/worktree.rs87-90](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs#L87-L90>) [crates/worktree/src/worktree.rs121-135](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs#L121-L135>) [crates/worktree/src/worktree.rs147-159](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs#L147-L159>)

### Worktree Snapshot System

Every worktree maintains an immutable `Snapshot` representing its state at a point in time:


The snapshot provides efficient lookups by both path and entry ID using sum trees. The `scan_id` tracks which background scans have completed, allowing clients to wait for specific states.

Sources: [crates/worktree/src/worktree.rs162-184](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs#L162-L184>)

## Buffer Store

The `BufferStore` manages the lifecycle of all open text buffers in the project. It coordinates with worktrees to map file paths to buffers and handles synchronization for remote projects.

### Buffer Store Architecture


Sources: [crates/project/src/buffer_store.rs31-41](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs#L31-L41>) [crates/project/src/buffer_store.rs49-68](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs#L49-L68>)

### Opening Buffers

The buffer opening flow differs between local and remote projects:

**Local Buffer Opening:**

  1. Check `path_to_buffer_id` for existing buffer
  2. Check `loading_buffers` for in-progress loads
  3. Read file from `Worktree::load_file()`
  4. Create `Buffer` with file contents
  5. Register buffer in LSP store for language servers
  6. Emit `BufferOpened` event


**Remote Buffer Opening:**

  1. Send `proto::OpenBufferByPath` RPC request
  2. Wait for `proto::CreateBufferForPeer` response chunks
  3. Reconstruct buffer from operations
  4. Resolve waiting listeners with completed buffer


Sources: [crates/project/src/buffer_store.rs291-318](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs#L291-L318>)

### Buffer Synchronization

For remote projects, buffers synchronize operations through the RPC protocol:


Sources: [crates/project/src/buffer_store.rs104-154](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs#L104-L154>)

## Project Path Resolution

The `ProjectPath` struct uniquely identifies files within a project by combining worktree ID and relative path:


This abstraction allows the same relative path to exist in multiple worktrees (e.g., monorepo with multiple roots) while maintaining unique identities.

**Key operations:**

  * `find_worktree()` \- Locate worktree containing a path
  * `absolutize()` \- Convert ProjectPath to absolute PathBuf
  * `create_entry()` \- Create new file or directory
  * `rename()` \- Move/rename files with LSP notification
  * `delete()` \- Remove files with confirmation


Sources: [crates/project/src/project.rs361-399](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L361-L399>)

## Store Coordination Patterns

The Project orchestrates stores through event subscription and method delegation:

### Event Flow Pattern


Each store emits domain-specific events that Project translates into unified `Event` enum variants for UI consumption.

**Examples:**

  * `WorktreeStoreEvent::WorktreeAdded` → `Event::WorktreeAdded`
  * `BufferStoreEvent::BufferOpened` → Subscribe to buffer events
  * `LspStoreEvent::LanguageServerAdded` → `Event::LanguageServerAdded`
  * `GitStoreEvent::RepositoryUpdated` → Trigger diff recalculation


Sources: [crates/project/src/project.rs286-352](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L286-L352>)

### Remote Project Architecture

Remote projects use RPC to proxy operations to a headless server:


The headless server runs a complete Project instance with direct file system and language server access, responding to RPC requests from remote clients.

Sources: [crates/remote_server/src/headless_project.rs45-63](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/remote_server/src/headless_project.rs#L45-L63>) [crates/project/src/worktree_store.rs43-52](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/worktree_store.rs#L43-L52>)

## Key Operations

### Opening a Local Project

The typical flow for opening a local directory:

  1. **Create Project:** `Project::local()` initializes all stores
  2. **Add Worktree:** `find_or_create_worktree()` creates worktree for directory
  3. **Background Scan:** Worktree scans file system, respecting `.gitignore`
  4. **Detect Repositories:** Git repositories discovered during scan
  5. **Emit Events:** UI notified of new worktree via events


### Opening a Buffer for Editing

When a user opens a file:

  1. **Resolve Path:** Convert UI path to `ProjectPath`
  2. **Check Cache:** Look for existing buffer in `BufferStore`
  3. **Load File:** Read file contents through worktree
  4. **Create Buffer:** Instantiate `Buffer` with text and metadata
  5. **LSP Registration:** Notify relevant language servers via `LspStore`
  6. **Git Diff:** Compute diff against HEAD via `GitStore`


### Saving a Buffer

Buffer saves coordinate multiple systems:

  1. **Validate:** Check for external modifications since load
  2. **Format:** Apply formatters (LSP, Prettier) if configured
  3. **Write:** Save to file system through worktree
  4. **Update Mtime:** Record new modification time
  5. **Notify:** Send LSP `didSave` notifications
  6. **Reload Git:** Refresh git status for file


Sources: [crates/project/src/buffer_store.rs361-469](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs#L361-L469>)

## Project Settings and Configuration

Project-level settings are stored in `.zed/settings.json` within each worktree. These settings override user and global defaults for:

  * Language server configurations (`lsp` field)
  * Formatter selection (`formatter` field)
  * Tab sizes and indentation
  * Git integration options
  * Task definitions (via `tasks.json`)


The `SettingsObserver` entity monitors settings files and propagates changes to relevant stores.

Sources: [crates/project/src/project_settings.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project_settings.rs#L1-L100>)

## Search and Diagnostics

### Project-Wide Search

The `Search` trait provides project-wide text search:


Search is implemented by both `Project` (local search) and delegated implementations (remote search via RPC).

### Diagnostic Management

Diagnostics flow from language servers through `LspStore` to the project:

  1. LSP sends `publishDiagnostics` notification
  2. `LspStore` processes and stores by file path
  3. Project aggregates diagnostics across servers
  4. UI queries diagnostics for display


The `DiagnosticSummary` aggregates counts by severity for status displays.

Sources: [crates/project/src/lsp_store.rs665-716](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L665-L716>) [crates/project/src/project.rs1-50](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L1-L50>)

Dismiss

Refresh this wiki

This wiki was recently refreshed. Please wait 7 days to refresh again.

### On this page

  * [Project Management](<#project-management>)
  * [Architecture Overview](<#architecture-overview>)
  * [Core Components and Store Organization](<#core-components-and-store-organization>)
  * [Project Entity Structure](<#project-entity-structure>)
  * [Project Client States](<#project-client-states>)
  * [Worktree Store](<#worktree-store>)
  * [Worktree Lifecycle](<#worktree-lifecycle>)
  * [Local vs Remote Worktrees](<#local-vs-remote-worktrees>)
  * [Worktree Snapshot System](<#worktree-snapshot-system>)
  * [Buffer Store](<#buffer-store>)
  * [Buffer Store Architecture](<#buffer-store-architecture>)
  * [Opening Buffers](<#opening-buffers>)
  * [Buffer Synchronization](<#buffer-synchronization>)
  * [Project Path Resolution](<#project-path-resolution>)
  * [Store Coordination Patterns](<#store-coordination-patterns>)
  * [Event Flow Pattern](<#event-flow-pattern>)
  * [Remote Project Architecture](<#remote-project-architecture>)
  * [Key Operations](<#key-operations>)
  * [Opening a Local Project](<#opening-a-local-project>)
  * [Opening a Buffer for Editing](<#opening-a-buffer-for-editing>)
  * [Saving a Buffer](<#saving-a-buffer>)
  * [Project Settings and Configuration](<#project-settings-and-configuration>)
  * [Search and Diagnostics](<#search-and-diagnostics>)
  * [Project-Wide Search](<#project-wide-search>)
  * [Diagnostic Management](<#diagnostic-management>)
