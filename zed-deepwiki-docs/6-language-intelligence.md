<!-- Source: https://deepwiki.com/zed-industries/zed/6-language-intelligence -->

# 6 Language Intelligence

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

# Language Intelligence

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


This document describes Zed's language intelligence system, which provides code completions, diagnostics, go-to-definition, and other IDE features through the Language Server Protocol (LSP). The system is built around the `LspStore` and manages the lifecycle of multiple language servers per project.

For information about the text editing system that consumes LSP features, see [Editor Architecture](</zed-industries/zed/3-editor-architecture>). For information about project-level orchestration, see [Project Management](</zed-industries/zed/5-project-management>).

## Architecture Overview

Zed's language intelligence system is split into three main layers:

  1. **LspStore** \- Unified interface for interacting with language servers ([6.1](</zed-industries/zed/6.1-lsp-store-architecture>))
  2. **LocalLspStore** \- Manages language server processes on the host machine
  3. **RemoteLspStore** \- Proxies requests via RPC for remote projects


**Sources:** [crates/project/src/lsp_store.rs1-299](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L1-L299>) [crates/project/src/project.rs183-220](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L183-L220>)

The `LspStore` provides a consistent interface regardless of whether language servers run locally or remotely. This abstraction allows the same code to work seamlessly in both single-user and collaborative scenarios.

### Key Components

Component| Purpose| Key Methods  
---|---|---  
`LspStore`| Unified interface for LSP operations| `language_server_for_buffer()`, `request_lsp()`  
`LocalLspStore`| Manages language server processes| `start_language_server()`, `register_buffer_with_language_servers()`  
`RemoteLspStore`| Proxies to remote project| `handle_lsp_request()`  
`LanguageServer`| LSP client wrapper| `request()`, `notify()`, `on_notification()`  
`LspAdapter`| Language-specific configuration| `name()`, `fetch_server_binary()`, `initialization_options()`  
  
**Sources:** [crates/project/src/lsp_store.rs254-299](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L254-L299>) [crates/lsp/src/lsp.rs86-111](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/lsp/src/lsp.rs#L86-L111>)

## Language Server Lifecycle

### Starting a Language Server

Language servers are started lazily when needed for a buffer or language. The lifecycle involves several stages:


**Sources:** [crates/project/src/lsp_store.rs359-570](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L359-L570>) [crates/lsp/src/lsp.rs316-383](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/lsp/src/lsp.rs#L316-L383>)

### Language Server States

Language servers transition through several states during their lifecycle:


**Sources:** [crates/project/src/lsp_store.rs554-570](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L554-L570>)

The `LanguageServerState` enum tracks these states:

  * **Starting** : Contains a `Task<Option<Arc<LanguageServer>>>` that resolves when initialization completes
  * **Running** : Contains the `Arc<LanguageServer>` and associated metadata like the `LspAdapter` and pending requests


### Initialization Process

The initialization follows the LSP specification precisely:

  1. **Binary Resolution** : The `LspAdapter` locates or downloads the language server binary
  2. **Process Spawn** : The binary is launched with appropriate environment variables and working directory
  3. **Initialize Request** : Sent with client capabilities, workspace folders, and initialization options
  4. **Initialized Notification** : Acknowledges successful initialization
  5. **Configuration** : `didChangeConfiguration` sends workspace settings


**Sources:** [crates/project/src/lsp_store.rs431-553](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L431-L553>)

### Server Identification

Each language server is uniquely identified by a `LanguageServerSeed`:


This allows multiple instances of the same language server (e.g., TypeScript) to run with different configurations or toolchains across different worktrees or manifest locations.

**Sources:** [crates/project/src/lsp_store.rs225-231](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L225-L231>)

## LSP Features Implementation

### Code Completions

Completions are requested through the `GetCompletions` command:


**Sources:** [crates/project/src/lsp_command.rs218-223](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_command.rs#L218-L223>)

The completion flow involves:

  1. **Trigger Detection** : Editor detects trigger characters or manual invocation
  2. **Context Building** : Creates `CompletionContext` with trigger kind and character
  3. **LSP Request** : Sends `textDocument/completion` with file URI and UTF-16 position
  4. **Response Processing** : Converts LSP positions to Zed `Anchor`s for stability across edits
  5. **Filtering** : Client-side filtering based on query string
  6. **Resolution** : Lazy resolution of documentation on selection


The `CoreCompletion` structure maintains both the LSP data and Zed-specific anchors:

**Sources:** [crates/project/src/lsp_command.rs646-651](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_command.rs#L646-L651>) [crates/project/src/project.rs469-521](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L469-L521>)

### Diagnostics

Zed supports both push and pull diagnostics models:


**Sources:** [crates/project/src/lsp_store.rs673-716](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L673-L716>)

Diagnostics are stored with:

  * **Source Kind** : Distinguishes between push, pull, and disk-based sources
  * **Server ID** : Tracks which language server provided the diagnostic
  * **Version** : Result IDs for incremental updates in pull diagnostics
  * **Anchors** : Positions that remain valid as the buffer is edited


### Go-to-Definition and Related Features

Definition lookups use the `GetDefinitions` command pattern:


**Sources:** [crates/project/src/lsp_command.rs175-177](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_command.rs#L175-L177>)

Similar patterns are used for:

  * **Declarations** : `textDocument/declaration`
  * **Type Definitions** : `textDocument/typeDefinition`
  * **Implementations** : `textDocument/implementation`
  * **References** : `textDocument/references`


All convert between Zed's `Anchor` type and LSP's UTF-16 positions.

### Document Highlights

Document highlights show related symbols in the current file:

**Sources:** [crates/project/src/lsp_command.rs200-202](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_command.rs#L200-L202>)

### Hover Information

Hover requests provide documentation and type information:


Hover blocks can be:

  * **PlainText** : Simple documentation
  * **Markdown** : Formatted documentation
  * **Code** : Syntax-highlighted code snippets


**Sources:** [crates/project/src/project.rs809-820](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L809-L820>)

### Code Actions

Code actions provide refactorings and quick fixes:


Code actions may require resolution before execution if they contain data payloads that need to be fetched lazily.

**Sources:** [crates/project/src/project.rs653-677](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L653-L677>)

### Inlay Hints

Inlay hints display inline parameter names, type annotations, and other aids:


Inlay hints are cached per-buffer and invalidated when:

  * The buffer is edited in the hint's range
  * Settings change
  * A refresh request is received from the language server


**Sources:** [crates/project/src/project.rs430-438](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project.rs#L430-L438>) [crates/project/src/lsp_store.rs19-21](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L19-L21>)

## Multi-Language Server Coordination

### Multiple Servers per Buffer

A single buffer can have multiple language servers providing features:


**Sources:** [crates/project/src/lsp_store.rs34-37](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L34-L37>) [crates/project/src/lsp_store.rs288](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L288-L288>)

### Request Routing

When multiple servers provide the same capability (e.g., completions), requests are:

  1. **Broadcast** : Sent to all applicable servers concurrently
  2. **Merged** : Responses are combined (e.g., completion lists are concatenated)
  3. **Deduplicated** : Results with identical content are removed
  4. **Sorted** : Based on relevance scores and provider priority


**Sources:** [crates/project/src/lsp_store.rs262-273](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L262-L273>)

### Capability Checking

Before sending requests, Zed checks if the server supports the capability:


This prevents unnecessary requests and provides fallback behavior when features are unavailable.

**Sources:** [crates/project/src/lsp_command.rs110](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_command.rs#L110-L110>)

### Server-Specific Extensions

Some language servers provide custom capabilities beyond the LSP specification:

  * **rust-analyzer** : Expand macro recursively, join lines, structural search/replace
  * **clangd** : Switch between source/header, memory usage
  * **TypeScript** : Organize imports, source actions
  * **JSON** : Schema validation


These are handled through custom request/notification types.

**Sources:** [crates/project/src/lsp_store.rs12-17](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L12-L17>)

## Language Server Adapters

### Adapter Responsibilities

Each language server has an associated `LspAdapter` that provides:

  1. **Binary Management** : Locating, downloading, and installing the language server
  2. **Initialization** : Providing server-specific initialization options
  3. **Configuration** : Translating workspace settings to server config
  4. **Capabilities** : Declaring supported code action kinds and features
  5. **Diagnostic Processing** : Filtering or transforming diagnostics


**Sources:** [language/src/lsp.rs650-850](<https://github.com/zed-industries/zed/blob/4109c9dd/language/src/lsp.rs#L650-L850>)

### Adapter Interface


**Sources:** [language/src/lsp.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/language/src/lsp.rs>)

### Built-in Adapters

Zed includes built-in adapters for many popular language servers. Extensions can also provide adapters for additional languages.

### Binary Resolution

Language server binaries are resolved through a multi-stage process:


**Sources:** [crates/project/src/lsp_store.rs572-663](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L572-L663>)

### Configuration Updates

Configuration can be updated dynamically:

  1. **Settings Change** : User modifies `.zed/settings.json`
  2. **Workspace Configuration** : Adapter translates to server format
  3. **didChangeConfiguration** : Notification sent to language server
  4. **Server Restart** : Some settings require restart (detected by adapter)


**Sources:** [crates/project/src/lsp_store.rs717-778](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L717-L778>)

## Buffer Registration

### Registration Flow

When a buffer is opened, it must be registered with relevant language servers:


**Sources:** [crates/project/src/lsp_store.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs>)

### Buffer Snapshots

The `LocalLspStore` maintains snapshots of each buffer's state for each language server:


These snapshots track:

  * The buffer version sent to the server
  * Outstanding edits not yet acknowledged
  * The buffer's file path at the time of the snapshot


This allows Zed to handle rapid edits and server restarts gracefully.

**Sources:** [crates/project/src/lsp_store.rs286](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs#L286-L286>)

### Incremental Sync

After the initial `didOpen`, buffer changes are sent incrementally:

  1. **Edit Made** : User modifies buffer
  2. **Diff Calculation** : Compute minimal change set
  3. **didChange** : Send `TextDocumentContentChangeEvent[]`
  4. **Snapshot Update** : Record new version


Zed uses full document sync (`TextDocumentSyncKind::Full`) or incremental sync (`TextDocumentSyncKind::Incremental`) based on server capabilities.

**Sources:** [crates/project/src/lsp_store.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_store.rs>)

## Remote Language Servers

### Architecture

For remote projects (SSH, collab), language servers run on the server machine:


**Sources:** [crates/remote_server/src/headless_project.rs45-63](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/remote_server/src/headless_project.rs#L45-L63>)

### Request Proxying

Remote LSP requests are serialized to protobuf and sent over RPC:

  1. **Client Request** : `RemoteLspStore::request_lsp()`
  2. **Serialize** : Convert command to `proto::LspRequest`
  3. **RPC** : Send via `proto::PerformLspRequest`
  4. **Server Execute** : `HeadlessProject` executes on `LocalLspStore`
  5. **Serialize Response** : Convert to `proto::LspResponse`
  6. **RPC Response** : Send back to client
  7. **Deserialize** : Convert to native types


**Sources:** [crates/project/src/lsp_command.rs76-155](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/lsp_command.rs#L76-L155>)

### Protobuf Definitions

LSP requests have corresponding protobuf messages:


**Sources:** [crates/proto/proto/lsp.proto8-17](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/proto/proto/lsp.proto#L8-L17>)

This allows language intelligence to work seamlessly across network boundaries with minimal latency overhead.

Dismiss

Refresh this wiki

This wiki was recently refreshed. Please wait 7 days to refresh again.

### On this page

  * [Language Intelligence](<#language-intelligence>)
  * [Architecture Overview](<#architecture-overview>)
  * [Key Components](<#key-components>)
  * [Language Server Lifecycle](<#language-server-lifecycle>)
  * [Starting a Language Server](<#starting-a-language-server>)
  * [Language Server States](<#language-server-states>)
  * [Initialization Process](<#initialization-process>)
  * [Server Identification](<#server-identification>)
  * [LSP Features Implementation](<#lsp-features-implementation>)
  * [Code Completions](<#code-completions>)
  * [Diagnostics](<#diagnostics>)
  * [Go-to-Definition and Related Features](<#go-to-definition-and-related-features>)
  * [Document Highlights](<#document-highlights>)
  * [Hover Information](<#hover-information>)
  * [Code Actions](<#code-actions>)
  * [Inlay Hints](<#inlay-hints>)
  * [Multi-Language Server Coordination](<#multi-language-server-coordination>)
  * [Multiple Servers per Buffer](<#multiple-servers-per-buffer>)
  * [Request Routing](<#request-routing>)
  * [Capability Checking](<#capability-checking>)
  * [Server-Specific Extensions](<#server-specific-extensions>)
  * [Language Server Adapters](<#language-server-adapters>)
  * [Adapter Responsibilities](<#adapter-responsibilities>)
  * [Adapter Interface](<#adapter-interface>)
  * [Built-in Adapters](<#built-in-adapters>)
  * [Binary Resolution](<#binary-resolution>)
  * [Configuration Updates](<#configuration-updates>)
  * [Buffer Registration](<#buffer-registration>)
  * [Registration Flow](<#registration-flow>)
  * [Buffer Snapshots](<#buffer-snapshots>)
  * [Incremental Sync](<#incremental-sync>)
  * [Remote Language Servers](<#remote-language-servers>)
  * [Architecture](<#architecture>)
  * [Request Proxying](<#request-proxying>)
  * [Protobuf Definitions](<#protobuf-definitions>)
