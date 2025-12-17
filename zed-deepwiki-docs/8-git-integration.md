<!-- Source: https://deepwiki.com/zed-industries/zed/8-git-integration -->

# 8 Git Integration

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

# Git Integration

Relevant source files

  * [crates/collab/src/rpc.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/collab/src/rpc.rs>)
  * [crates/fs/src/fake_git_repo.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/fs/src/fake_git_repo.rs>)
  * [crates/git/src/git.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/git.rs>)
  * [crates/git/src/repository.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs>)
  * [crates/git_ui/Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/Cargo.toml>)
  * [crates/git_ui/src/branch_picker.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/branch_picker.rs>)
  * [crates/git_ui/src/commit_modal.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/commit_modal.rs>)
  * [crates/git_ui/src/git_panel.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs>)
  * [crates/git_ui/src/git_ui.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_ui.rs>)
  * [crates/git_ui/src/project_diff.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/project_diff.rs>)
  * [crates/project/src/git_store.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs>)
  * [crates/proto/proto/git.proto](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/proto/proto/git.proto>)
  * [crates/proto/proto/zed.proto](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/proto/proto/zed.proto>)
  * [crates/proto/src/proto.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/proto/src/proto.rs>)


Zed's git integration provides comprehensive version control features throughout the editor, from repository discovery and status tracking to staging, committing, and remote operations. The system is built on a local/remote architecture that works seamlessly in both standalone and collaborative environments.

This page documents the complete git integration stack, from the UI components users interact with (`GitPanel`, `ProjectDiff`) down through the state management layer (`GitStore`, `Repository`) to the underlying git operations (`GitRepository` trait, `RealGitRepository`).

**Key Subsystems:**

  * **Git Panel and UI** ([8.1](</zed-industries/zed/8.1-git-panel-and-ui>)) - User interface for viewing changes and performing git operations
  * **Git Store and State Management** ([8.2](</zed-industries/zed/8.2-git-store-and-state-management>)) - Central coordination and state synchronization
  * **Repository Operations** ([8.3](</zed-industries/zed/8.3-repository-operations>)) - Core git operations like commit, push, pull, and staging
  * **Diff System** ([8.4](</zed-industries/zed/8.4-diff-system>)) - Buffer-level diff computation and visualization


## System Architecture

The git integration is structured in four layers: UI, state management, repository operations, and backend implementation.

**High-Level Architecture Diagram**


**Sources:** [crates/git_ui/src/git_panel.rs560-601](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L560-L601>) [crates/project/src/git_store.rs89-102](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L89-L102>) [crates/project/src/git_store.rs278-293](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L278-L293>) [crates/git/src/repository.rs409-638](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L409-L638>) [crates/git/src/repository.rs657-683](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L657-L683>)

### Component Responsibilities

Component| Location| Primary Responsibilities  
---|---|---  
`GitPanel`| [crates/git_ui/src/git_panel.rs560](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L560-L560>)| Display file status, handle staging/unstaging, manage commits  
`ProjectDiff`| [crates/git_ui/src/project_diff.rs64](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/project_diff.rs#L64-L64>)| Show unified diff view across all changed files  
`GitStore`| [crates/project/src/git_store.rs89](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L89-L89>)| Manage all repositories, coordinate diff loading, sync state  
`Repository`| [crates/project/src/git_store.rs278](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L278-L278>)| Represent a single git repository, queue operations, track state  
`GitRepository` trait| [crates/git/src/repository.rs409](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L409-L409>)| Define interface for all git operations  
`RealGitRepository`| [crates/git/src/repository.rs657](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L657-L657>)| Implement git operations using libgit2 and git binary  
`BufferDiff`| [crates/buffer_diff/src/buffer_diff.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/buffer_diff/src/buffer_diff.rs>)| Track and compute diffs between buffer states  
  
**Sources:** [crates/git_ui/src/git_panel.rs560-601](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L560-L601>) [crates/git_ui/src/project_diff.rs64-75](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/project_diff.rs#L64-L75>) [crates/project/src/git_store.rs89-102](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L89-L102>) [crates/project/src/git_store.rs278-293](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L278-L293>) [crates/git/src/repository.rs409-638](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L409-L638>)

## Git Store and State Management

`GitStore` ([crates/project/src/git_store.rs89](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L89-L89>)) is the central coordinator for all git operations in a project. It maintains the authoritative state for all repositories, coordinates diff loading, and synchronizes changes across local and remote contexts.

**GitStore Internal Structure**


**Sources:** [crates/project/src/git_store.rs89-102](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L89-L102>) [crates/project/src/git_store.rs156-168](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L156-L168>)

### Local vs Remote Operation Modes

Mode| Purpose| State Fields| Operations  
---|---|---|---  
**Local**|  Direct filesystem access| `fs`, `project_environment`, `next_repository_id`, optional `downstream`| Creates repositories, runs git operations locally, optionally broadcasts to collaborators  
**Remote**|  Proxy to headless server| `upstream_client`, `upstream_project_id`, optional `downstream`| Forwards requests to host, receives state updates, optionally shares with collaborators  
  
**Sources:** [crates/project/src/git_store.rs156-168](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L156-L168>) [crates/project/src/git_store.rs404-441](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L404-L441>)

### Repository Discovery and Tracking

When a `WorktreeStore` detects a git repository (by finding a `.git` directory), it notifies `GitStore`, which creates or updates a `Repository` entity:


**Sources:** [crates/project/src/git_store.rs1038-1248](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L1038-L1248>) [crates/git/src/repository.rs666-683](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L666-L683>)

### Repository Entity

Each `Repository` entity ([crates/project/src/git_store.rs278](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L278-L278>)) represents a single git repository and coordinates all operations for that repository.

**Repository Structure**


**Sources:** [crates/project/src/git_store.rs278-293](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L278-L293>) [crates/project/src/git_store.rs256-268](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L256-L268>)

### RepositoryState: Local vs Remote

The `repository_state` field contains a `Shared<Task<Result<RepositoryState>>>` that resolves to either a local or remote backend:


**Sources:** [crates/project/src/git_store.rs303-361](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L303-L361>)

### Status Tracking with SumTree

Git status information is stored in a `SumTree<StatusEntry>` ([crates/project/src/git_store.rs188](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L188-L188>)), enabling efficient path-based queries and iteration:

**StatusEntry Structure**


**Sources:** [crates/project/src/git_store.rs188-243](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L188-L243>) [crates/git/src/status.rs1-150](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/status.rs#L1-L150>)

The `SumTree` structure allows efficient operations:

Operation| Method| Complexity  
---|---|---  
Find entry by path| `statuses_by_path.get(&PathKey(path), &())`| O(log n)  
Iterate entries in range| `statuses_by_path.cursor()`| O(log n + k)  
Count entries by status| `GitSummary` accumulation| O(1) after scan  
Update entry| `edit()` and `insert()`| O(log n)  
  
**Sources:** [crates/project/src/git_store.rs256-268](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L256-L268>) [crates/sum_tree/src/sum_tree.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/sum_tree/src/sum_tree.rs>)

## Diff System

The diff system tracks differences between the working copy, index (staging area), and HEAD commit for each open buffer.

### BufferGitState

`GitStore` maintains a `BufferGitState` ([crates/project/src/git_store.rs110](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L110-L110>)) for each buffer requiring diff tracking:

**BufferGitState Structure**


**Sources:** [crates/project/src/git_store.rs110-137](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L110-L137>)

### Diff Kinds

Two types of diffs are computed:

**DiffKind Enum**


**Sources:** [crates/project/src/git_store.rs150-154](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L150-L154>)

Diff Kind| Base Text| Current Text| Purpose  
---|---|---|---  
**Unstaged**|  Index (staging area)| Buffer content| Shows unstaged changes, used for staging hunks  
**Uncommitted**|  HEAD commit| Buffer content| Shows all uncommitted changes, used for commit preview  
  
### Diff Loading Process

When a diff is requested (e.g., when opening a file or after staging changes), `GitStore` loads the base text and creates a `BufferDiff`:

**open_unstaged_diff() Flow**


**Sources:** [crates/project/src/git_store.rs636-688](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L636-L688>)

### Synchronizing Diff Bases

When the index or HEAD changes (due to staging, committing, or checking out), `GitStore` must update diff base texts:

**update_buffer_diff_bases() Method**


The `DiffBasesChange` enum specifies what changed:


**Sources:** [crates/project/src/git_store.rs139-148](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L139-L148>) [crates/project/src/git_store.rs857-945](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L857-L945>)

### Hunk Staging Operation Count

To ensure diff base texts are up-to-date when staging individual hunks, `BufferGitState` tracks operation counts:


This prevents race conditions where git operations complete out of order.

**Sources:** [crates/project/src/git_store.rs121-134](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L121-L134>)

## Git Panel and UI

`GitPanel` ([crates/git_ui/src/git_panel.rs560](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L560-L560>)) is the primary user interface for viewing and manipulating git state. It implements the `Panel` trait and displays in the workspace dock.

**GitPanel Core Structure**


**Sources:** [crates/git_ui/src/git_panel.rs560-601](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L560-L601>)

### Entry List Organization

The panel displays entries in three sections, each with a header:

**GitListEntry Enum**


**Sources:** [crates/git_ui/src/git_panel.rs228-277](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L228-L277>) [crates/git_ui/src/git_panel.rs475-495](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L475-L495>)

The sections appear in priority order:

  1. **Conflicts** \- Files with merge conflicts (unmerged status)
  2. **Tracked** \- Modified or deleted files that exist in HEAD
  3. **Untracked** \- New files not yet in the repository


**Sources:** [crates/git_ui/src/git_panel.rs228-258](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L228-L258>)

### View Modes: Flat vs Tree

`GitPanel` supports two view modes controlled by `GitPanelSettings::tree_view`:

**GitPanelViewMode Enum**


**Sources:** [crates/git_ui/src/git_panel.rs279-316](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L279-L316>)

In tree view, the `TreeViewState::build_tree_entries()` method constructs a hierarchical structure:

  * Groups files by directory path
  * Compacts sequential single-child directories (e.g., `src/editor/display` becomes `src/editor/display`)
  * Tracks expansion state per directory
  * Maps visible indices to actual entry indices for rendering


**Sources:** [crates/git_ui/src/git_panel.rs318-443](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L318-L443>)

### Staging and Unstaging Operations

The panel provides multiple interaction patterns for staging operations:

**Staging Operation Flow**


**Sources:** [crates/git_ui/src/git_panel.rs1088-1193](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L1088-L1193>) [crates/project/src/git_store.rs1883-1948](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L1883-L1948>)

Staging Action| User Input| Method| Behavior  
---|---|---|---  
**Toggle single**|  Click or `ToggleStaged` action| `toggle_stage_entry()`| Stage if unstaged, unstage if staged  
**Range staging**| `StageRange` action| `stage_range()`| Stage all entries between anchor and cursor  
**Bulk staging**|  Navigate with selection| `BulkStaging` state| Automatically stage entries as user moves through list  
**Stage all**| `StageAll` action| `stage_all()`| Stage all unstaged files  
**Unstage all**| `UnstageAll` action| `unstage_all()`| Unstage all staged files  
  
**Sources:** [crates/git_ui/src/git_panel.rs1088-1289](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L1088-L1289>)

### Bulk Staging

When `bulk_staging` is active, `GitPanel` automatically stages entries as the user navigates through the list. This is useful for quickly reviewing and staging many files:


The anchor tracks where bulk staging started, allowing the panel to stage all entries between the anchor and current selection.

**Sources:** [crates/git_ui/src/git_panel.rs603-607](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L603-L607>)

### Commit Editor and Modal

The commit message editor exists in two forms:

**commit_message_editor() Function**


This function creates an `Editor` configured for commit messages:

  * Uses `EditorMode::AutoHeight` with 6 lines (panel) or 18 lines (modal)
  * Disables autoclosing, gutters, wrap guides, and indent guides
  * Sets collaboration hub for co-author suggestions
  * Uses modal editing (respects vim mode)


**Sources:** [crates/git_ui/src/git_panel.rs611-640](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L611-L640>)

The `CommitModal` ([crates/git_ui/src/commit_modal.rs61](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/commit_modal.rs#L61-L61>)) provides an expanded editing experience:

  * Larger editor (18 lines vs 6)
  * Displays branch selector and commit options
  * Shows amend/signoff toggles
  * Can generate commit messages using AI


**Sources:** [crates/git_ui/src/commit_modal.rs61-227](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/commit_modal.rs#L61-L227>)

## ProjectDiff: Unified Diff View

`ProjectDiff` ([crates/git_ui/src/project_diff.rs64](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/project_diff.rs#L64-L64>)) displays diffs for all changed files in a single multi-buffer editor. It implements the `Item` trait for workspace integration.

**ProjectDiff Structure**


**Sources:** [crates/git_ui/src/project_diff.rs64-75](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/project_diff.rs#L64-L75>)

### BranchDiff: Diff Base Selection

The `branch_diff::BranchDiff` entity determines what to compare against:

**DiffBase Enum**


**Sources:** [crates/project/src/git_store/branch_diff.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store/branch_diff.rs>)

  * **DiffBase::Head** : Shows unstaged and uncommitted changes (git status view)
  * **DiffBase::Merge** : Shows all commits on current branch not in base branch (branch diff view)


**Sources:** [crates/git_ui/src/project_diff.rs88-222](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/project_diff.rs#L88-L222>)

### Multi-Buffer Organization

`ProjectDiff` uses a `MultiBuffer` to show excerpts from multiple files:

**Multi-Buffer Setup Process**


**Sources:** [crates/git_ui/src/project_diff.rs499-609](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/project_diff.rs#L499-L609>)

Files are sorted using status prefixes:


The `sort_prefix()` function maps file status to these prefixes, ensuring conflicts appear first, then tracked changes, then new files.

**Sources:** [crates/git_ui/src/project_diff.rs84-86](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/project_diff.rs#L84-L86>) [crates/git_ui/src/project_diff.rs724-748](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/project_diff.rs#L724-L748>)

## Repository Operations

Git operations are executed through the `Repository` entity, which serializes operations via a job queue to prevent conflicts.

### Job Queue Architecture

**Job Execution Flow**


**Sources:** [crates/project/src/git_store.rs1441-1688](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L1441-L1688>)

Each `Repository` spawns a worker task that processes jobs sequentially:


The `key` field enables deduplication - only one job of each type can be queued at once.

**Sources:** [crates/project/src/git_store.rs390-401](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L390-L401>)

### GitRepository Trait

The `GitRepository` trait ([crates/git/src/repository.rs409](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L409-L409>)) defines the interface for all git operations. `RealGitRepository` ([crates/git/src/repository.rs657](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L657-L657>)) implements this using libgit2 for index operations and git binary for complex operations.

**GitRepository Method Categories**


**Sources:** [crates/git/src/repository.rs409-638](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L409-L638>)

### RealGitRepository Implementation

`RealGitRepository` uses different backends for different operations:

Operation Category| Backend| Rationale  
---|---|---  
Index reading| libgit2 `Repository::index()`| Fast, in-process  
Tree reading| libgit2 `Repository::find_tree()`| Fast, in-process  
Status| git binary `git status --porcelain=2`| More reliable, handles submodules  
Staging| git binary `git update-index` or `git add`| Handles permissions correctly  
Commits| git binary `git commit`| Runs hooks, handles GPG signing  
Remote operations| git binary `git push/pull/fetch`| Handles authentication, credentials  
History| git binary `git log`| Fast, handles complex queries  
Blame| git binary `git blame`| Full feature set  
  
**Sources:** [crates/git/src/repository.rs657-1500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L657-L1500>)

### Staging Operations in Detail

Staging and unstaging modify the git index (staging area):

**stage_paths() Implementation**


**Sources:** [crates/project/src/git_store.rs1883-1911](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L1883-L1911>) [crates/git/src/repository.rs1227-1270](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L1227-L1270>)

The `stage_paths()` method ([crates/git/src/repository.rs514-518](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L514-L518>)):

  1. Runs `git update-index --add --remove` for specified paths
  2. Uses the repository's environment variables (PATH, GIT_DIR, etc.)
  3. Reloads the libgit2 index after modification
  4. Returns any errors encountered


**unstage_paths() Implementation**

Unstaging uses `git reset` to copy content from HEAD to the index:


This runs `git reset HEAD -- <paths>`, restoring the index to match HEAD for those paths.

**Sources:** [crates/git/src/repository.rs522-526](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L522-L526>) [crates/git/src/repository.rs1272-1298](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L1272-L1298>)

### Commit Operation

The commit operation creates a new commit from staged changes:

**commit() Flow**


**Sources:** [crates/project/src/git_store.rs1983-2056](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L1983-L2056>) [crates/git/src/repository.rs534-541](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L534-L541>) [crates/git/src/repository.rs1463-1500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L1463-L1500>)

The `CommitOptions` struct controls commit behavior:

  * `amend`: Modify the previous commit instead of creating a new one
  * `signoff`: Add `Signed-off-by:` trailer to commit message


**Sources:** [crates/git/src/repository.rs150-153](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L150-L153>)

### Remote Operations

Push, pull, and fetch operations interact with remote repositories and require authentication.

**push() Implementation**


**Sources:** [crates/git_ui/src/git_panel.rs1395-1536](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/git_panel.rs#L1395-L1536>) [crates/git/src/repository.rs567-577](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L567-L577>) [crates/git/src/repository.rs1502-1614](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L1502-L1614>)

**PushOptions Enum**

  * `SetUpstream`: Sets the upstream tracking branch with `--set-upstream`
  * `Force`: Force pushes with `--force`


**Sources:** [crates/git/src/repository.rs645-649](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L645-L649>)

**pull() and fetch() Operations**

Similar to push, these operations use the askpass system for authentication:


The `rebase` parameter controls whether to use `git pull --rebase` or regular merge.

**Sources:** [crates/git/src/repository.rs579-599](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L579-L599>) [crates/git/src/repository.rs1616-1691](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L1616-L1691>)

### AskPass Authentication System

Git remote operations use the `GIT_ASKPASS` mechanism for authentication. Zed provides a custom askpass implementation:

**AskPass Architecture**


**Sources:** [crates/askpass/src/askpass.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/askpass/src/askpass.rs>) [crates/git_ui/src/askpass_modal.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/askpass_modal.rs>)

The `Repository` maintains an `askpass_delegates` map ([crates/project/src/git_store.rs290](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L290-L290>)) that tracks active askpass sessions by ID. When git requests credentials, the askpass script sends an RPC message, and Zed displays a modal to collect user input.

**Sources:** [crates/project/src/git_store.rs290](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L290-L290>) [crates/project/src/git_store.rs2637-2705](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L2637-L2705>)

## Local vs Remote Architecture

The git system follows Zed's dual-mode pattern for local and remote projects.


**Sources:** [crates/project/src/git_store.rs156-168](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L156-L168>) [crates/project/src/git_store.rs404-441](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L404-L441>)

### RPC Protocol

Remote git operations are serialized as protocol buffer messages:

Message| Direction| Purpose  
---|---|---  
`UpdateRepository`| Host → Guest| Sync repository state changes  
`RemoveRepository`| Host → Guest| Notify repository removal  
`UpdateDiffBases`| Host → Guest| Sync diff base text  
`Stage` / `Unstage`| Guest → Host| Request staging operations  
`Commit`| Guest → Host| Request commit  
`Push` / `Pull` / `Fetch`| Guest → Host| Request remote operations  
`OpenUnstagedDiff`| Guest → Host| Request unstaged diff  
`OpenUncommittedDiff`| Guest → Host| Request uncommitted diff  
`SetIndexText`| Guest → Host| Update staging area  
  
**Sources:** [crates/proto/proto/git.proto](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/proto/proto/git.proto>) [crates/project/src/git_store.rs468-510](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L468-L510>)

### Downstream Synchronization

When a local GitStore is shared (in collaboration or as a remote server), it sends repository updates to downstream clients:


**Sources:** [crates/project/src/git_store.rs525-604](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L525-L604>) [crates/collab/src/rpc.rs328-334](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/collab/src/rpc.rs#L328-L334>)

### Conflict Detection and Resolution

When merge conflicts occur, `GitStore` detects them via `FileStatus::Unmerged` in the status tree and creates a `ConflictSet` entity.

**Conflict Handling Flow**


**Sources:** [crates/project/src/git_store.rs947-1036](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store.rs#L947-L1036>) [crates/git_ui/src/conflict_view.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/conflict_view.rs>)

The `ConflictSet` ([crates/project/src/git_store/conflict_set.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store/conflict_set.rs>)) parses conflict markers in the buffer:
    
    
    <<<<<<< HEAD
    content from HEAD
    =======
    content from merging branch
    >>>>>>> branch-name
    

The `ConflictAddon` ([crates/git_ui/src/conflict_view.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/conflict_view.rs>)) provides UI for resolution:

  * Visual indicators for conflict regions
  * Actions to accept ours, theirs, or both
  * Quick navigation between conflicts


**Sources:** [crates/project/src/git_store/conflict_set.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/git_store/conflict_set.rs>) [crates/git_ui/src/conflict_view.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/conflict_view.rs>)

### Stash Operations

Git stash allows temporarily storing uncommitted changes. The `RepositorySnapshot` includes a `stash_entries` field:

**GitStash Structure**


**Sources:** [crates/git/src/stash.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/stash.rs>)

**Stash Operations in GitRepository**


**Sources:** [crates/git/src/repository.rs454](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L454-L454>) [crates/git/src/repository.rs543-565](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L543-L565>)

  * `stash_paths()`: Runs `git stash push -- <paths>`
  * `stash_pop()`: Runs `git stash pop [stash@{n}]`
  * `stash_apply()`: Runs `git stash apply [stash@{n}]`
  * `stash_drop()`: Runs `git stash drop [stash@{n}]`


**Sources:** [crates/git/src/repository.rs1300-1391](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L1300-L1391>)

## Branch Management

Branch operations are exposed through the Repository interface:


**Sources:** [crates/git/src/repository.rs37-72](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L37-L72>) [crates/git/src/repository.rs447-454](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git/src/repository.rs#L447-L454>) [crates/git_ui/src/branch_picker.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/git_ui/src/branch_picker.rs>)

Dismiss

Refresh this wiki

This wiki was recently refreshed. Please wait 7 days to refresh again.

### On this page

  * [Git Integration](<#git-integration>)
  * [System Architecture](<#system-architecture>)
  * [Component Responsibilities](<#component-responsibilities>)
  * [Git Store and State Management](<#git-store-and-state-management>)
  * [Local vs Remote Operation Modes](<#local-vs-remote-operation-modes>)
  * [Repository Discovery and Tracking](<#repository-discovery-and-tracking>)
  * [Repository Entity](<#repository-entity>)
  * [RepositoryState: Local vs Remote](<#repositorystate-local-vs-remote>)
  * [Status Tracking with SumTree](<#status-tracking-with-sumtree>)
  * [Diff System](<#diff-system>)
  * [BufferGitState](<#buffergitstate>)
  * [Diff Kinds](<#diff-kinds>)
  * [Diff Loading Process](<#diff-loading-process>)
  * [Synchronizing Diff Bases](<#synchronizing-diff-bases>)
  * [Hunk Staging Operation Count](<#hunk-staging-operation-count>)
  * [Git Panel and UI](<#git-panel-and-ui>)
  * [Entry List Organization](<#entry-list-organization>)
  * [View Modes: Flat vs Tree](<#view-modes-flat-vs-tree>)
  * [Staging and Unstaging Operations](<#staging-and-unstaging-operations>)
  * [Bulk Staging](<#bulk-staging>)
  * [Commit Editor and Modal](<#commit-editor-and-modal>)
  * [ProjectDiff: Unified Diff View](<#projectdiff-unified-diff-view>)
  * [BranchDiff: Diff Base Selection](<#branchdiff-diff-base-selection>)
  * [Multi-Buffer Organization](<#multi-buffer-organization>)
  * [Repository Operations](<#repository-operations>)
  * [Job Queue Architecture](<#job-queue-architecture>)
  * [GitRepository Trait](<#gitrepository-trait>)
  * [RealGitRepository Implementation](<#realgitrepository-implementation>)
  * [Staging Operations in Detail](<#staging-operations-in-detail>)
  * [Commit Operation](<#commit-operation>)
  * [Remote Operations](<#remote-operations>)
  * [AskPass Authentication System](<#askpass-authentication-system>)
  * [Local vs Remote Architecture](<#local-vs-remote-architecture>)
  * [RPC Protocol](<#rpc-protocol>)
  * [Downstream Synchronization](<#downstream-synchronization>)
  * [Conflict Detection and Resolution](<#conflict-detection-and-resolution>)
  * [Stash Operations](<#stash-operations>)
  * [Branch Management](<#branch-management>)
