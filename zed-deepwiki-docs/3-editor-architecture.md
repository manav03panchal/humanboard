<!-- Source: https://deepwiki.com/zed-industries/zed/3-editor-architecture -->

# 3 Editor Architecture

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

# Editor Architecture

Relevant source files

  * [crates/buffer_diff/src/buffer_diff.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/buffer_diff/src/buffer_diff.rs>)
  * [crates/editor/src/actions.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/actions.rs>)
  * [crates/editor/src/code_completion_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/code_completion_tests.rs>)
  * [crates/editor/src/code_context_menus.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/code_context_menus.rs>)
  * [crates/editor/src/editor.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs>)
  * [crates/editor/src/editor_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor_tests.rs>)
  * [crates/editor/src/element.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/element.rs>)
  * [crates/language/src/buffer.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/buffer.rs>)
  * [crates/language/src/buffer_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/buffer_tests.rs>)
  * [crates/language/src/syntax_map.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/syntax_map.rs>)
  * [crates/multi_buffer/src/anchor.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/anchor.rs>)
  * [crates/multi_buffer/src/multi_buffer.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer.rs>)
  * [crates/multi_buffer/src/multi_buffer_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer_tests.rs>)


**Purpose** : This document describes the architecture of Zed's text editor component, covering its layered design from text storage through display transformations to UI rendering. This includes the Editor entity itself, the buffer system, display pipeline, and key subsystems like selections and editing operations.

**Scope** : This page focuses on the core editor component and its internal architecture. For information about workspace-level editor management (panes, tabs), see [Workspace and Panel System](</zed-industries/zed/4-workspace-and-panel-system>). For language server integration and code intelligence features, see [Language Intelligence](</zed-industries/zed/6-language-intelligence>). For collaborative editing and CRDT synchronization, see [CRDT and Synchronization](</zed-industries/zed/12.4-crdt-and-synchronization>).

## Overview

The editor architecture consists of four primary layers that transform text data from storage to screen:


Sources: [crates/editor/src/editor.rs1-226](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L1-L226>) [Diagram 3 from system architecture](<https://github.com/zed-industries/zed/blob/4109c9dd/Diagram 3 from system architecture>)

## Core Components

### Editor Entity

The `Editor` is the main entity that coordinates all editor functionality. It serves as the entry point for user interactions and orchestrates the various subsystems.


Sources: [crates/editor/src/editor.rs1027-1227](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L1027-L1227>)

**Key fields** :

Field| Type| Purpose  
---|---|---  
`buffer`| `Entity<MultiBuffer>`| The text being edited  
`display_map`| `Entity<DisplayMap>`| Transforms buffer text for display  
`selections`| `SelectionsCollection`| Current cursor positions and selections  
`scroll_manager`| `ScrollManager`| Manages viewport and scrolling  
`mode`| `EditorMode`| SingleLine, AutoHeight, or Full  
`project`| `Option<Entity<Project>>`| Optional project context for LSP features  
`completion_tasks`| `Vec<(CompletionId, Task<()>)>`| Active completion requests  
  
Sources: [crates/editor/src/editor.rs1027-1124](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L1027-L1124>)

### EditorElement - Rendering Pipeline

`EditorElement` is responsible for painting the editor UI. It implements the GPUI element trait and handles the complete rendering pipeline.


Sources: [crates/editor/src/element.rs196-220](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/element.rs#L196-L220>) [crates/editor/src/element.rs213-627](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/element.rs#L213-L627>)

The rendering process has two main phases:

  1. **Prepaint** (layout): Computes what's visible and generates line layouts
  2. **Paint** : Renders all visual elements in layers


Sources: [crates/editor/src/element.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/element.rs#L1-L100>)

## Text Storage Architecture

### Buffer and TextBuffer

The `Buffer` provides syntax-aware text storage built on top of `TextBuffer`, which implements CRDT-based text editing.


Sources: [crates/language/src/buffer.rs95-134](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/buffer.rs#L95-L134>) [crates/editor/src/editor.rs8-11](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L8-L11>)

**Buffer responsibilities** :

  * Text storage with CRDT support for collaboration
  * Syntax tree management via `SyntaxMap`
  * Diagnostic tracking from language servers
  * File association and save state
  * Transaction management for undo/redo


Sources: [crates/language/src/buffer.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/buffer.rs#L1-L100>)

### MultiBuffer - Composing Multiple Buffers

`MultiBuffer` allows displaying one or more buffers (or portions of buffers) in a single editor. This is used for search results, project-wide find, and other multi-file views.


Sources: [crates/multi_buffer/src/multi_buffer.rs73-98](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer.rs#L73-L98>) [crates/multi_buffer/src/multi_buffer.rs676-693](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer.rs#L676-L693>)

**Key concepts** :

  * **Excerpt** : A slice of a buffer with a specific range
  * **ExcerptId** : Unique identifier for each excerpt
  * **Singleton mode** : Optimized path for single-buffer editors
  * **Multi-buffer mode** : Used for search results, diagnostics panels, etc.


**MultiBufferSnapshot** provides an immutable view of the multi-buffer state for rendering and queries:

Sources: [crates/multi_buffer/src/multi_buffer.rs546-564](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer.rs#L546-L564>)

## Display Transformation Pipeline

### DisplayMap - Logical to Display Coordinates

The `DisplayMap` transforms buffer coordinates to display coordinates, handling folds, soft wraps, inlay hints, and other visual transformations.


Sources: [crates/editor/src/editor.rs8-10](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L8-L10>) [crates/editor/src/display_map.rs1-20](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/display_map.rs#L1-L20>)

**DisplayPoint vs Point** :

  * `Point`: Row and column in the buffer's raw text
  * `DisplayPoint`: Row and column as displayed (after folds, wraps, etc.)
  * `DisplayRow`: A single row in the display coordinate space


The display map maintains bidirectional mappings between these coordinate systems.

Sources: [crates/editor/src/display_map.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/display_map.rs>)

### Coordinate Systems

There are multiple coordinate systems in the editor:

Coordinate Type| Description| Use Case  
---|---|---  
`BufferOffset`| Byte offset in buffer| Internal text operations  
`Point`| (row, column) in buffer| Logical text positions  
`DisplayPoint`| (row, column) in display| Cursor positioning, rendering  
`Anchor`| Stable position across edits| Persistent references  
`MultiBufferOffset`| Offset in multi-buffer| Cross-excerpt operations  
  
Sources: [crates/multi_buffer/src/multi_buffer.rs192-257](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer.rs#L192-L257>) [crates/multi_buffer/src/anchor.rs12-16](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/anchor.rs#L12-L16>)

## Selections and Editing

### SelectionsCollection

`SelectionsCollection` manages all active selections (cursors) in the editor, supporting multi-cursor editing.


Sources: [crates/editor/src/selections_collection.rs1-50](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/selections_collection.rs#L1-L50>)

**Selection properties** :

  * Stored as `Anchor` pairs for stability across edits
  * `reversed` indicates if cursor is at start or end
  * `goal` column for vertical movement
  * Automatically merged if overlapping
  * Sorted by position


Sources: [crates/editor/src/selections_collection.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/selections_collection.rs>)

### Editing Operations

Editing operations flow through multiple layers:


Sources: [crates/editor/src/editor.rs4500-4700](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L4500-L4700>) [crates/multi_buffer/src/multi_buffer.rs1500-1800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer.rs#L1500-L1800>)

**Transaction management** :

  * Edits grouped into transactions for undo/redo
  * Transactions can span multiple buffers
  * History maintained at buffer level
  * Editor coordinates multi-buffer transactions


Sources: [crates/language/src/buffer.rs2500-2800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/buffer.rs#L2500-L2800>)

## Code Intelligence Integration

### Completions Menu

The completions menu provides code completion suggestions from multiple sources.


Sources: [crates/editor/src/code_context_menus.rs224-252](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/code_context_menus.rs#L224-L252>) [crates/editor/src/editor.rs9000-9300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L9000-L9300>)

**CompletionsMenu structure** :

Sources: [crates/editor/src/code_context_menus.rs224-252](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/code_context_menus.rs#L224-L252>)

**Filtering and sorting** :

  * Fuzzy matching on `filter_text`
  * Sort by: fuzzy score, sort_text, kind, exact match
  * Snippet ordering controlled by settings


Sources: [crates/editor/src/code_completion_tests.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/code_completion_tests.rs#L1-L200>)

### Code Actions Menu

Provides quick fixes, refactorings, and runnable tasks at cursor position.


Sources: [crates/editor/src/code_context_menus.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/code_context_menus.rs#L1-L100>) [crates/editor/src/editor.rs10500-10800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L10500-L10800>)

## Display Features

### Folding

Code folding allows collapsing regions of code:


Sources: [crates/editor/src/editor.rs11000-11500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L11000-L11500>)

**Fold types** :

  * Manual folds: User-initiated
  * Syntax-based folds: Based on tree-sitter nodes
  * Multi-level folding: Nested folds
  * Fold indicators in gutter


Sources: [crates/editor/src/actions.rs446-467](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/actions.rs#L446-L467>)

### Inlay Hints

Inlay hints display inline type information, parameter names, etc.


Sources: [crates/editor/src/inlays/inlay_hints.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/inlays/inlay_hints.rs#L1-L100>)

### Soft Wrapping

Soft wrapping breaks long lines for display without modifying the buffer:

**Wrap modes** :

Mode| Description  
---|---  
`None`| No wrapping (horizontal scroll)  
`EditorWidth`| Wrap at editor width  
`Column(n)`| Wrap at column n  
`Bounded(n)`| Wrap at min(editor_width, n)  
  
Sources: [crates/editor/src/editor.rs526-540](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L526-L540>)

## Diff Integration

The editor integrates git diff information to show changes:


Sources: [crates/buffer_diff/src/buffer_diff.rs23-28](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/buffer_diff/src/buffer_diff.rs#L23-L28>) [crates/editor/src/git/mod.rs1-50](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/git/mod.rs#L1-L50>)

**Diff hunk structure** :

Sources: [crates/buffer_diff/src/buffer_diff.rs75-88](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/buffer_diff/src/buffer_diff.rs#L75-L88>)

## Editor Modes

Editors can operate in different modes:


Sources: [crates/editor/src/editor.rs481-523](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L481-L523>)

**Mode-specific behavior** :

  * **SingleLine** : Used for search boxes, input fields
  * **AutoHeight** : Used for inline editors, comments
  * **Full** : Primary code editing
  * **Minimap** : Small overview of another editor


Sources: [crates/editor/src/editor.rs500-523](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L500-L523>)

## Event Flow

Events flow through the editor subsystems:


Sources: [crates/editor/src/editor.rs6000-6300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L6000-L6300>) [crates/multi_buffer/src/multi_buffer.rs107-140](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer.rs#L107-L140>)

## Key Data Structures

### EditorSnapshot

`EditorSnapshot` provides an immutable view of the editor state for queries and rendering:


Sources: [crates/editor/src/editor.rs300-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L300-L400>)

### Anchor System

Anchors provide stable references to positions across edits:


Sources: [crates/multi_buffer/src/anchor.rs12-81](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/anchor.rs#L12-L81>)

**Anchor usage** :

  * Selections stored as `Selection<Anchor>`
  * Diagnostic ranges as `Range<Anchor>`
  * Completion/code action positions
  * Any persistent position reference


Sources: [crates/multi_buffer/src/anchor.rs1-258](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/anchor.rs#L1-L258>)

## Summary

The Editor architecture achieves separation of concerns through clear layering:

  1. **Text Storage** : CRDT-based buffers with syntax and diagnostics
  2. **Buffer Composition** : Multi-buffer with excerpts for flexible views
  3. **Display Transformation** : DisplayMap pipeline for visual transformations
  4. **UI Rendering** : EditorElement for efficient painting
  5. **Interaction** : Editor entity coordinating all subsystems


This design enables:

  * Real-time collaboration via CRDT
  * Flexible multi-file views
  * Rich code intelligence integration
  * Efficient rendering with partial updates
  * Extensible display transformations


Sources: [crates/editor/src/editor.rs1-226](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L1-L226>) [crates/multi_buffer/src/multi_buffer.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/multi_buffer/src/multi_buffer.rs#L1-L100>) [crates/language/src/buffer.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/buffer.rs#L1-L100>) [crates/editor/src/element.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/element.rs#L1-L100>)

Dismiss

Refresh this wiki

This wiki was recently refreshed. Please wait 7 days to refresh again.

### On this page

  * [Editor Architecture](<#editor-architecture>)
  * [Overview](<#overview>)
  * [Core Components](<#core-components>)
  * [Editor Entity](<#editor-entity>)
  * [EditorElement - Rendering Pipeline](<#editorelement---rendering-pipeline>)
  * [Text Storage Architecture](<#text-storage-architecture>)
  * [Buffer and TextBuffer](<#buffer-and-textbuffer>)
  * [MultiBuffer - Composing Multiple Buffers](<#multibuffer---composing-multiple-buffers>)
  * [Display Transformation Pipeline](<#display-transformation-pipeline>)
  * [DisplayMap - Logical to Display Coordinates](<#displaymap---logical-to-display-coordinates>)
  * [Coordinate Systems](<#coordinate-systems>)
  * [Selections and Editing](<#selections-and-editing>)
  * [SelectionsCollection](<#selectionscollection>)
  * [Editing Operations](<#editing-operations>)
  * [Code Intelligence Integration](<#code-intelligence-integration>)
  * [Completions Menu](<#completions-menu>)
  * [Code Actions Menu](<#code-actions-menu>)
  * [Display Features](<#display-features>)
  * [Folding](<#folding>)
  * [Inlay Hints](<#inlay-hints>)
  * [Soft Wrapping](<#soft-wrapping>)
  * [Diff Integration](<#diff-integration>)
  * [Editor Modes](<#editor-modes>)
  * [Event Flow](<#event-flow>)
  * [Key Data Structures](<#key-data-structures>)
  * [EditorSnapshot](<#editorsnapshot>)
  * [Anchor System](<#anchor-system>)
  * [Summary](<#summary>)
