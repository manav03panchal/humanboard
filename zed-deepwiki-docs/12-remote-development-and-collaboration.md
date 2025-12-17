<!-- Source: https://deepwiki.com/zed-industries/zed/12-remote-development-and-collaboration -->

# 12 Remote Development And Collaboration

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

# Remote Development and Collaboration

Relevant source files

  * [crates/collab/src/tests/editor_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/collab/src/tests/editor_tests.rs>)
  * [crates/collab/src/tests/integration_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/collab/src/tests/integration_tests.rs>)
  * [crates/collab/src/tests/remote_editing_collaboration_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/collab/src/tests/remote_editing_collaboration_tests.rs>)
  * [crates/copilot/Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/copilot/Cargo.toml>)
  * [crates/copilot/src/copilot.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/copilot/src/copilot.rs>)
  * [crates/copilot/src/sign_in.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/copilot/src/sign_in.rs>)
  * [crates/editor/src/hover_links.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/hover_links.rs>)
  * [crates/editor/src/items.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/items.rs>)
  * [crates/fs/src/fs.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/fs/src/fs.rs>)
  * [crates/git/src/status.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/status.rs>)
  * [crates/language/src/proto.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/proto.rs>)
  * [crates/lsp/src/lsp.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/lsp/src/lsp.rs>)
  * [crates/prettier/src/prettier.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/prettier/src/prettier.rs>)
  * [crates/prettier/src/prettier_server.js](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/prettier/src/prettier_server.js>)
  * [crates/project/src/buffer_store.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs>)
  * [crates/project/src/lsp_command.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_command.rs>)
  * [crates/project/src/lsp_store.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs>)
  * [crates/project/src/prettier_store.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/prettier_store.rs>)
  * [crates/project/src/project.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs>)
  * [crates/project/src/project_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project_tests.rs>)
  * [crates/project/src/search.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/search.rs>)
  * [crates/project/src/worktree_store.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/worktree_store.rs>)
  * [crates/proto/proto/buffer.proto](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/proto/proto/buffer.proto>)
  * [crates/proto/proto/lsp.proto](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/proto/proto/lsp.proto>)
  * [crates/remote_server/src/headless_project.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/remote_server/src/headless_project.rs>)
  * [crates/remote_server/src/remote_editing_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/remote_server/src/remote_editing_tests.rs>)
  * [crates/search/src/buffer_search.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/search/src/buffer_search.rs>)
  * [crates/search/src/project_search.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/search/src/project_search.rs>)
  * [crates/vim/src/normal/search.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/search.rs>)
  * [crates/workspace/src/item.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/item.rs>)
  * [crates/workspace/src/pane.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/pane.rs>)
  * [crates/workspace/src/searchable.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/searchable.rs>)
  * [crates/workspace/src/workspace.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace.rs>)
  * [crates/worktree/src/worktree.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs>)
  * [crates/worktree/src/worktree_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree_tests.rs>)


This document describes Zed's architecture for remote development and real-time collaboration. Remote development enables working on codebases hosted on remote machines (e.g., via SSH), while collaboration features enable multiple users to edit the same project simultaneously with shared cursors and following capabilities.

For information about the Project orchestration layer that coordinates these features, see [Project Management](</zed-industries/zed/5-project-management>). For details on the CRDT-based text storage that enables conflict-free merging, see [CRDT and Synchronization](</zed-industries/zed/12.4-crdt-and-synchronization>).

## Architectural Pattern: Local/Remote Duality

Zed's remote architecture follows a consistent pattern throughout the codebase: major subsystems are split into **Local** and **Remote** variants, unified under a common interface. This pattern enables the same code paths to work seamlessly whether the project is local or accessed remotely.


**Sources:** [crates/project/src/lsp_store.rs1-15](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L1-L15>) [crates/worktree/src/worktree.rs87-90](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs#L87-L90>) [crates/project/src/buffer_store.rs30-68](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs#L30-L68>) [crates/remote_server/src/headless_project.rs45-61](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/remote_server/src/headless_project.rs#L45-L61>)

### Project Client State

The `Project` entity tracks its operational mode through the `ProjectClientState` enum, which determines how operations are routed:

State| Description| Use Case  
---|---|---  
`Local`| Single-player mode| Local development  
`Shared { remote_id }`| Multi-player mode, local project| Host sharing a project  
`Remote { remote_id, replica_id, capability }`| Multi-player mode, remote project| Guest accessing shared project or SSH remote  
  

**Sources:** [crates/project/src/project.rs269-283](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L269-L283>) [crates/workspace/src/workspace.rs82-85](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace.rs#L82-L85>)

## HeadlessProject: Server-Side Coordination

`HeadlessProject` runs on the remote server (e.g., SSH host or project host in collaboration) and manages the actual resources. It instantiates the "Local" variants of all stores since, from the server's perspective, it has direct access to resources.

### HeadlessProject Structure


The `HeadlessProject` is initialized with:

  * **RPC session** (`AnyProtoClient`): Bidirectional communication channel to clients
  * **File system** : Direct access to local file system on the server
  * **Language registry** : For syntax highlighting and language server management
  * **All stores in Local mode** : Since the server has direct resource access


**Sources:** [crates/remote_server/src/headless_project.rs45-61](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/remote_server/src/headless_project.rs#L45-L61>) [crates/remote_server/src/headless_project.rs107-201](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/remote_server/src/headless_project.rs#L107-L201>)

### Server RPC Handlers

`HeadlessProject` registers handlers for client requests:

Handler| Purpose| Returns  
---|---|---  
`handle_add_worktree`| Add directory to project| Worktree metadata  
`handle_open_buffer_by_path`| Open file for editing| Buffer content + state  
`handle_save_buffer`| Save buffer changes| Success confirmation  
`handle_update_buffer`| Apply operations to buffer| Acknowledgement  
`handle_start_language_server`| Launch language server| Server capabilities  
`handle_search_project`| Execute search query| Search results stream  
`handle_get_code_actions`| Request code actions| Available actions  
`handle_apply_code_action`| Execute code action| Applied changes  
  
**Sources:** [crates/remote_server/src/headless_project.rs202-430](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/remote_server/src/headless_project.rs#L202-L430>)

## Remote Project Architecture (Client-Side)

On the client side, when working with a remote project, the `Project` uses Remote variants of stores that proxy all operations through RPC to the server.

### Remote Store Pattern

Each Remote store maintains:

  * **Upstream client** (`AnyProtoClient`): Connection to server
  * **Project ID** : Identifies the remote project
  * **Cached state** : Local copy of remote state for reads
  * **Background sync** : Task to apply updates from server


**Sources:** [crates/worktree/src/worktree.rs147-159](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs#L147-L159>) [crates/project/src/lsp_store.rs1-15](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L1-L15>) [crates/project/src/buffer_store.rs54-62](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs#L54-L62>)

## RemoteWorktree: File System Proxy

`RemoteWorktree` provides file tree operations by forwarding to the server.

### Key Components


### Update Flow


Operations like `create_file`, `rename_file`, `delete_entry` are sent as RPC requests:

  1. Client calls method on `RemoteWorktree`
  2. Request serialized and sent via `AnyProtoClient`
  3. Server `HeadlessProject` receives and processes on `LocalWorktree`
  4. Server sends response
  5. Client applies optimistic update or waits for confirmation


**Sources:** [crates/worktree/src/worktree.rs147-159](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs#L147-L159>) [crates/worktree/src/worktree.rs521-576](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree.rs#L521-L576>)

## RemoteLspStore: Language Server Proxy

`RemoteLspStore` provides language intelligence features by forwarding requests to language servers running on the remote machine.

### Architecture


### Key Operations

Operation| RPC Message| Description  
---|---|---  
`get_completions`| `proto::GetCompletions`| Request code completions  
`apply_code_action`| `proto::ApplyCodeAction`| Execute code action  
`get_hover`| `proto::GetHover`| Request hover info  
`get_definition`| `proto::GetDefinition`| Jump to definition  
`prepare_rename`| `proto::PrepareRename`| Validate rename  
`perform_rename`| `proto::PerformRename`| Execute rename  
`get_document_highlights`| `proto::GetDocumentHighlights`| Find symbol references  
`format_buffers`| `proto::FormatBuffers`| Format code  
  
The `RemoteLspStore` does not spawn or manage language servers itself - it simply forwards all requests to the server, which has `LocalLspStore` doing the actual work.

**Sources:** [crates/project/src/lsp_store.rs1-15](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L1-L15>) [crates/project/src/lsp_store.rs1300-1500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L1300-L1500>)

## RemoteBufferStore: Text Buffer Proxy

`RemoteBufferStore` manages open text buffers in remote projects, ensuring text state stays synchronized through CRDT operations.

### Buffer Lifecycle


### Operation Synchronization

Text edits are synchronized as `Operation` objects:

  * **Insert** : Add text at position
  * **Delete** : Remove text range
  * **UpdateSelections** : Update cursor positions
  * **Undo** : Revert operations


Each buffer maintains a `ReplicaId` to track operation origin and enable CRDT conflict resolution.

**Sources:** [crates/project/src/buffer_store.rs54-62](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs#L54-L62>) [crates/project/src/buffer_store.rs103-154](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/buffer_store.rs#L103-L154>) [crates/language/src/proto.rs1-28](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/proto.rs#L1-L28>)

## RPC Communication Protocol

Zed's remote protocol uses protobuf messages over a bidirectional RPC channel.

### Protocol Stack


### Core Message Types

Category| Messages| Purpose  
---|---|---  
**Project**| `OpenProject`, `CloseProject`, `UpdateProject`| Project lifecycle  
**Worktree**| `AddWorktree`, `UpdateWorktree`, `UpdateEntries`| File system sync  
**Buffer**| `OpenBuffer`, `UpdateBuffer`, `SaveBuffer`, `CloseBuffer`| Text editing  
**LSP**| `StartLanguageServer`, `UpdateLanguageServer`, `GetCompletions`, `ApplyCodeAction`| Language intelligence  
**Collaboration**| `UpdateActiveView`, `UpdateFollowers`, `JoinProject`| Multi-user features  
**Search**| `SearchProject`, `SearchProjectResponse`| Cross-file search  
  
**Sources:** [crates/rpc/proto/lsp.proto1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/rpc/proto/lsp.proto#L1-L100>) [crates/project/src/project.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L1-L100>)

### Request/Response Pattern


**Sources:** [crates/rpc/src/rpc.rs1-50](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/rpc/src/rpc.rs#L1-L50>) [crates/remote_server/src/headless_project.rs300-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/remote_server/src/headless_project.rs#L300-L400>)

## Collaboration Features

Zed supports real-time collaboration where multiple users can work on the same project simultaneously.

### Following System

Users can "follow" collaborators to see their active view and cursor position:


### Shared Cursors Implementation

Each buffer tracks selections for all collaborators:


When a user updates their selection:

  1. Local buffer updates its selection
  2. Selection serialized to `proto::UpdateSelections`
  3. Sent to server via `proto::UpdateBuffer`
  4. Server broadcasts to all other clients
  5. Remote clients display cursor at that position


**Sources:** [crates/workspace/src/workspace.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace.rs#L1-L100>) [crates/editor/src/items.rs179-201](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/items.rs#L179-L201>)

### Collaborative Editing Flow


**Sources:** [crates/collab/src/tests/editor_tests.rs48-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/collab/src/tests/editor_tests.rs#L48-L100>) [crates/editor/src/items.rs264-296](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/items.rs#L264-L296>)

## CRDT and Synchronization

Zed uses Conflict-Free Replicated Data Types (CRDTs) for text buffers to enable concurrent editing without conflicts.

### CRDT Text Buffer Architecture


### Operation Format

Each operation carries:

  * **Replica ID** : Identifies which client made the edit
  * **Lamport timestamp** : Logical clock for ordering
  * **Position** : Anchor-based position (stable across edits)
  * **Content** : Text to insert or range to delete


### Conflict Resolution

When operations from different replicas arrive:

  1. **Operation application** : Each operation has a unique ID based on replica + timestamp
  2. **Deterministic ordering** : Operations are applied in a consistent order across all replicas
  3. **Anchor-based positions** : Positions are anchors that track their location as text changes
  4. **Convergence** : All replicas converge to the same final state


This design ensures:

  * **No locking** : Users never block each other
  * **Eventual consistency** : All clients reach the same state
  * **Causality preservation** : Dependent edits apply in correct order


**Sources:** [crates/language/src/buffer.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/buffer.rs#L1-L100>) [crates/text/src/text.rs1-50](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/text/src/text.rs#L1-L50>) [crates/language/src/proto.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/proto.rs#L1-L100>)

### Version Vectors

Each buffer maintains a version vector tracking the latest operation from each replica:


This enables:

  * **Detecting which operations are new** : Compare incoming version with local version
  * **Handling disconnections** : Replay missed operations when reconnecting
  * **Garbage collection** : Identify which operations all replicas have seen


**Sources:** [crates/clock/src/clock.rs1-50](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/clock/src/clock.rs#L1-L50>) [crates/language/src/proto.rs50-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/proto.rs#L50-L100>)

## Remote LSP Integration

Language server communication must work across the network boundary while maintaining responsiveness.

### LSP Request Flow


### Diagnostic Synchronization

Language servers push diagnostics (errors, warnings) to the server, which then broadcasts to all clients:


### LSP Server State Synchronization

When a buffer is opened:

  1. Client sends `proto::OpenBufferByPath`
  2. Server opens buffer locally
  3. Server sends `textDocument/didOpen` to language server
  4. Language server indexes the file
  5. Diagnostics flow back to all clients


When buffer changes:

  1. Client sends `proto::UpdateBuffer` with operations
  2. Server applies operations to buffer
  3. Server sends `textDocument/didChange` to language server
  4. Language server re-analyzes
  5. New diagnostics broadcast to clients


**Sources:** [crates/project/src/lsp_store.rs665-716](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L665-L716>) [crates/project/src/lsp_command.rs1-50](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_command.rs#L1-L50>) [crates/remote_server/src/headless_project.rs400-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/remote_server/src/headless_project.rs#L400-L500>)

### Caching and Performance

To reduce latency, `RemoteLspStore` caches:

  * **Diagnostics** : Stored locally, updated via push notifications
  * **Server capabilities** : Fetched once during initialization
  * **Symbol indexes** : Some queries cached client-side


Language server requests have timeouts (`LSP_REQUEST_TIMEOUT` = 2 minutes) to handle unresponsive servers.

**Sources:** [crates/lsp/src/lsp.rs48-49](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/lsp/src/lsp.rs#L48-L49>) [crates/project/src/lsp_store.rs141-144](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L141-L144>)

Dismiss

Refresh this wiki

This wiki was recently refreshed. Please wait 7 days to refresh again.

### On this page

  * [Remote Development and Collaboration](<#remote-development-and-collaboration>)
  * [Architectural Pattern: Local/Remote Duality](<#architectural-pattern-localremote-duality>)
  * [Project Client State](<#project-client-state>)
  * [HeadlessProject: Server-Side Coordination](<#headlessproject-server-side-coordination>)
  * [HeadlessProject Structure](<#headlessproject-structure>)
  * [Server RPC Handlers](<#server-rpc-handlers>)
  * [Remote Project Architecture (Client-Side)](<#remote-project-architecture-client-side>)
  * [Remote Store Pattern](<#remote-store-pattern>)
  * [RemoteWorktree: File System Proxy](<#remoteworktree-file-system-proxy>)
  * [Key Components](<#key-components>)
  * [Update Flow](<#update-flow>)
  * [RemoteLspStore: Language Server Proxy](<#remotelspstore-language-server-proxy>)
  * [Architecture](<#architecture>)
  * [Key Operations](<#key-operations>)
  * [RemoteBufferStore: Text Buffer Proxy](<#remotebufferstore-text-buffer-proxy>)
  * [Buffer Lifecycle](<#buffer-lifecycle>)
  * [Operation Synchronization](<#operation-synchronization>)
  * [RPC Communication Protocol](<#rpc-communication-protocol>)
  * [Protocol Stack](<#protocol-stack>)
  * [Core Message Types](<#core-message-types>)
  * [Request/Response Pattern](<#requestresponse-pattern>)
  * [Collaboration Features](<#collaboration-features>)
  * [Following System](<#following-system>)
  * [Shared Cursors Implementation](<#shared-cursors-implementation>)
  * [Collaborative Editing Flow](<#collaborative-editing-flow>)
  * [CRDT and Synchronization](<#crdt-and-synchronization>)
  * [CRDT Text Buffer Architecture](<#crdt-text-buffer-architecture>)
  * [Operation Format](<#operation-format>)
  * [Conflict Resolution](<#conflict-resolution>)
  * [Version Vectors](<#version-vectors>)
  * [Remote LSP Integration](<#remote-lsp-integration>)
  * [LSP Request Flow](<#lsp-request-flow>)
  * [Diagnostic Synchronization](<#diagnostic-synchronization>)
  * [LSP Server State Synchronization](<#lsp-server-state-synchronization>)
  * [Caching and Performance](<#caching-and-performance>)
