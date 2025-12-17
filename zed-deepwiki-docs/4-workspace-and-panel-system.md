<!-- Source: https://deepwiki.com/zed-industries/zed/4-workspace-and-panel-system -->

# 4 Workspace And Panel System

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

# Workspace and Panel System

Relevant source files

  * [crates/editor/src/items.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/items.rs>)
  * [crates/language/src/proto.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/proto.rs>)
  * [crates/project/src/search.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/search.rs>)
  * [crates/proto/proto/buffer.proto](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/proto/proto/buffer.proto>)
  * [crates/search/src/buffer_search.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/search/src/buffer_search.rs>)
  * [crates/search/src/project_search.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/search/src/project_search.rs>)
  * [crates/vim/src/normal/search.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/search.rs>)
  * [crates/workspace/src/item.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/item.rs>)
  * [crates/workspace/src/pane.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/pane.rs>)
  * [crates/workspace/src/searchable.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/searchable.rs>)
  * [crates/workspace/src/workspace.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace.rs>)


This document describes Zed's workspace and panel architecture, which organizes all UI content within application windows. The workspace provides a flexible container for editors, tools, and panels, supporting features like split panes, dockable panels, and state persistence.

For information about individual item types (editors, search results, etc.), see their specific sections. For window management and GPUI integration, see [Window and Event System](</zed-industries/zed/2.3-window-and-platform-abstraction>).

## Purpose and Scope

The workspace system provides the organizational structure for all content displayed in a Zed window. It manages:

  * **Workspace container** : The top-level window content manager
  * **Pane system** : Splittable containers for multiple items
  * **Item trait hierarchy** : Uniform interface for different content types
  * **Dock system** : Collapsible side panels for tools and auxiliary views
  * **State persistence** : Serialization and restoration of workspace layout


## Workspace Architecture

The `Workspace` struct [workspace.rs1116-1500](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L1116-L1500>) is the primary container for a Zed window's content. It coordinates between the central editing area (stored in `center_pane_group`) and surrounding tool panels in three docks (left, bottom, right).

**Workspace Structure Diagram**


**Key Components:**

Component| Purpose| Definition  
---|---|---  
`Workspace`| Root container managing window content| [workspace.rs1116-1190](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L1116-L1190>)  
`PaneGroup`| Recursive splittable container using `Member` enum| [pane_group.rs31-49](<https://github.com/zed-industries/zed/blob/4109c9dd/pane_group.rs#L31-L49>)  
`Pane`| Tab container with ordered items| [pane.rs343-405](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L343-L405>)  
`Dock`| Collapsible side panel with single pane| [dock.rs27-48](<https://github.com/zed-industries/zed/blob/4109c9dd/dock.rs#L27-L48>)  
`Toolbar`| Contextual toolbar for active pane item| [toolbar.rs31-39](<https://github.com/zed-industries/zed/blob/4109c9dd/toolbar.rs#L31-L39>)  
`StatusBar`| Bottom status bar with left/right items| [status_bar.rs17-23](<https://github.com/zed-industries/zed/blob/4109c9dd/status_bar.rs#L17-L23>)  
  
Sources: [workspace.rs1116-1190](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L1116-L1190>) [pane_group.rs31-49](<https://github.com/zed-industries/zed/blob/4109c9dd/pane_group.rs#L31-L49>) [pane.rs343-405](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L343-L405>) [dock.rs27-48](<https://github.com/zed-industries/zed/blob/4109c9dd/dock.rs#L27-L48>)

## Pane System

The `Pane` struct [pane.rs343-405](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L343-L405>) manages multiple items with a tabbed interface. Each pane tracks its item order, active item, navigation history, and associated toolbar.

**Pane Structure and Key Fields**


**Pane Features:**

Feature| Implementation| Key Methods  
---|---|---  
Item storage| `items` vector with `active_item_index`| `add_item()` [pane.rs888-1076](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L888-L1076>)  
Tab rendering| `render_tab_bar()` method| [pane.rs2163-2519](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L2163-L2519>)  
Navigation history| `NavHistory` with backward/forward stacks| `navigate_backward()` [pane.rs838-847](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L838-L847>)  
Preview tabs| `preview_item_id` field, controlled by `PreviewTabsSettings`| `set_preview_item()` [pane.rs1348-1367](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L1348-L1367>)  
Pinned tabs| `pinned_tab_count` tracking| `toggle_pin()` [pane.rs3088-3126](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L3088-L3126>)  
Split operations| Creates new pane with split direction| `split()` [pane.rs1668-1748](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L1668-L1748>)  
Focus management| `last_focus_handle_by_item` map| `focus_in()` [pane.rs600-644](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L600-L644>)  
Diagnostics| `diagnostics` map updated from project events| `update_diagnostics()` [pane.rs686-710](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L686-L710>)  
  
Sources: [pane.rs343-405](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L343-L405>) [pane.rs471-554](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L471-L554>) [pane.rs600-644](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L600-L644>) [pane.rs686-710](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L686-L710>)

### Pane Operations

**Opening an Item in a Pane**


**Splitting a Pane**


Sources: [workspace.rs2334-2425](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L2334-L2425>) [pane.rs888-1076](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L888-L1076>) [pane.rs1668-1748](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L1668-L1748>) [pane_group.rs239-354](<https://github.com/zed-industries/zed/blob/4109c9dd/pane_group.rs#L239-L354>)

## Item Trait System

The `Item` trait [item.rs165-348](<https://github.com/zed-industries/zed/blob/4109c9dd/item.rs#L165-L348>) provides a uniform interface for all content types displayable in panes. The trait is typically paired with extension traits for specific capabilities.

**Item Trait Hierarchy**


**Core Item Trait Methods:**

Method| Purpose| Required| Default  
---|---|---|---  
`tab_content_text()`| Text displayed in tab| Yes| -  
`tab_content()`| Rendered tab element| No| `Label` from `tab_content_text()`  
`tab_icon()`| Optional icon for tab| No| `None`  
`tab_tooltip_text()`| Tooltip on hover| No| `None`  
`is_dirty()`| Has unsaved changes| No| `false`  
`can_save()`| Supports saving| No| `false`  
`save()`| Save content| If `can_save()`| Unimplemented  
`save_as()`| Save with new path| No| Unimplemented  
`reload()`| Reload from disk| No| Unimplemented  
`navigate()`| Navigate to location| No| `false`  
`as_searchable()`| Get searchable handle| No| `None`  
`breadcrumbs()`| Breadcrumb trail| No| `None`  
`breadcrumb_location()`| Where to show breadcrumbs| No| `Hidden`  
  
Sources: [item.rs165-348](<https://github.com/zed-industries/zed/blob/4109c9dd/item.rs#L165-L348>) [items.rs576-714](<https://github.com/zed-industries/zed/blob/4109c9dd/items.rs#L576-L714>)

### SerializableItem Trait

The `SerializableItem` trait [item.rs350-379](<https://github.com/zed-industries/zed/blob/4109c9dd/item.rs#L350-L379>) enables workspace state persistence. Items are stored in the `WORKSPACE_DB` SQLite database with type-specific serialization.

**Serialization Flow**


**Serialization Methods:**

Method| Purpose| When Called  
---|---|---  
`serialized_item_kind()`| Returns unique type string (e.g., "Editor")| During registration  
`serialize()`| Saves item state to DB with `ItemId`| On workspace save or throttled edits  
`deserialize()`| Restores item from DB by `ItemId`| On workspace restore  
`cleanup()`| Removes unloaded items from DB| After workspace restoration  
`should_serialize()`| Filter events that trigger serialization| On every item event  
  
Sources: [item.rs350-379](<https://github.com/zed-industries/zed/blob/4109c9dd/item.rs#L350-L379>) [workspace.rs834-892](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L834-L892>) [persistence/mod.rs16-89](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence/mod.rs#L16-L89>)

### FollowableItem Trait

The `FollowableItem` trait [items.rs64-263](<https://github.com/zed-industries/zed/blob/4109c9dd/items.rs#L64-L263>) enables real-time collaboration by synchronizing view state between collaborators. The `Editor` is the primary implementation.

**Collaboration Protocol**


**FollowableItem Methods:**

Method| Purpose| Editor Implementation  
---|---|---  
`remote_id()`| Get unique view ID for network sync| Returns `self.remote_id` field  
`to_state_proto()`| Serialize full view state| Serializes excerpts, selections, scroll [items.rs203-247](<https://github.com/zed-industries/zed/blob/4109c9dd/items.rs#L203-L247>)  
`from_state_proto()`| Deserialize view from proto| Opens buffers, creates Editor [items.rs69-177](<https://github.com/zed-industries/zed/blob/4109c9dd/items.rs#L69-L177>)  
`add_event_to_update_proto()`| Add incremental update| Adds selections/scroll changes [items.rs264-330](<https://github.com/zed-industries/zed/blob/4109c9dd/items.rs#L264-L330>)  
`apply_update_proto()`| Apply incremental update| Updates selections/scroll position [items.rs332-344](<https://github.com/zed-industries/zed/blob/4109c9dd/items.rs#L332-L344>)  
`set_leader_id()`| Start/stop following a peer| Updates `self.leader_id` field [items.rs179-201](<https://github.com/zed-industries/zed/blob/4109c9dd/items.rs#L179-L201>)  
  
Sources: [items.rs64-382](<https://github.com/zed-industries/zed/blob/4109c9dd/items.rs#L64-L382>)

### SearchableItem Trait

The `SearchableItem` trait [searchable.rs58-170](<https://github.com/zed-industries/zed/blob/4109c9dd/searchable.rs#L58-L170>) provides a uniform search interface for different item types. It's used by `BufferSearchBar` and project-wide search.

**Search Interface**


**SearchableItem Methods:**

Method| Purpose| Editor Implementation  
---|---|---  
`supported_options()`| Which search options are available| Returns case/word/regex/replacement/selection support  
`find_matches()`| Async search for query| Searches buffer text, returns `Vec<Range<Anchor>>`  
`clear_matches()`| Remove search highlights| Clears background highlights  
`update_matches()`| Show matches with active index| Updates highlights with active match color  
`activate_match()`| Navigate to specific match| Scrolls to match, updates selection  
`replace()`| Replace single match| Replaces text at match range  
`replace_all()`| Replace all matches| Replaces all matches in order  
`query_suggestion()`| Get default query text| Returns current selection text  
  
Sources: [searchable.rs58-170](<https://github.com/zed-industries/zed/blob/4109c9dd/searchable.rs#L58-L170>) [buffer_search.rs103-127](<https://github.com/zed-industries/zed/blob/4109c9dd/buffer_search.rs#L103-L127>) [project_search.rs247-250](<https://github.com/zed-industries/zed/blob/4109c9dd/project_search.rs#L247-L250>)

## Dock System

Docks are collapsible side panels that each contain a single pane. The workspace has three docks at fixed positions: left, bottom, and right.

**Dock Structure**


**Dock Management Actions:**

Action| Description| Default Keybinding| Implementation  
---|---|---|---  
`ToggleLeftDock`| Show/hide left dock| Cmd+B (macOS)| [workspace.rs195-287](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L195-L287>)  
`ToggleBottomDock`| Show/hide bottom dock| Cmd+J (macOS)| [workspace.rs195-287](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L195-L287>)  
`ToggleRightDock`| Show/hide right dock| Cmd+R (macOS)| [workspace.rs195-287](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L195-L287>)  
`CloseAllDocks`| Hide all three docks| -| [workspace.rs195-287](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L195-L287>)  
`CloseActiveDock`| Hide the focused dock| -| [workspace.rs195-287](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L195-L287>)  
`IncreaseActiveDockSize`| Grow focused dock by pixels| -| [workspace.rs393-411](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L393-L411>)  
`DecreaseActiveDockSize`| Shrink focused dock by pixels| -| [workspace.rs403-411](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L403-L411>)  
  
The `Dock` struct manages its visibility (`is_open`), size, and maintains the pane containing panels. Panels within a dock are displayed with tabs, similar to items in center panes.

Sources: [workspace.rs195-287](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L195-L287>) [workspace.rs393-431](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L393-L431>) [dock.rs27-48](<https://github.com/zed-industries/zed/blob/4109c9dd/dock.rs#L27-L48>)

### Panel Trait

Panels are items that typically live in docks. The `Panel` trait extends `Item` with dock-specific behavior:


Sources: [dock.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/dock.rs>) [workspace.rs20-23](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L20-L23>)

## Workspace Operations

The `Workspace` coordinates high-level operations including file opening, pane management, and action dispatch.

**Opening a File Flow**


Sources: [workspace.rs2334-2425](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L2334-L2425>) [workspace.rs2601-2676](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L2601-L2676>) [workspace.rs718-734](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L718-L734>) [pane.rs888-1076](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L888-L1076>)

### Opening Paths

The workspace provides multiple entry points for opening files:


**Path Opening Flow:**

  1. **Resolve path** : Convert user input to `ProjectPath`
  2. **Find or create item** : Use `ProjectItemRegistry` to build appropriate item
  3. **Choose pane** : Determine which pane should contain the item
  4. **Add to pane** : Insert item and optionally activate it
  5. **Handle navigation** : Update history and focus


Sources: [workspace.rs534-610](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L534-L610>) [workspace.rs2100-2300](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L2100-L2300>) [workspace.rs612-750](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L612-L750>)

### ProjectItemRegistry

The `ProjectItemRegistry` [workspace.rs634-754](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L634-L754>) is a global registry that maps project item types to workspace item builders. It enables opening files and creating items from project entities.

**Registry Architecture**


**How Registration Works:**

  1. Extensions call `register_project_item::<T>()` where `T: ProjectItem`
  2. The registry stores two closures: 
     * `BuildProjectItemFn`: Creates workspace item from existing project item
     * `BuildProjectItemForPathFn`: Tries to open path and create workspace item
  3. When opening a path, workspace tries each `BuildProjectItemForPathFn` in reverse order
  4. When converting a project item to workspace item, uses the `BuildProjectItemFn` by `TypeId`


Sources: [workspace.rs634-761](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L634-L761>)

## Item Lifecycle

Understanding the item lifecycle helps with implementing custom items and debugging workspace issues.


**Key Lifecycle Hooks:**

Hook| When Called| Purpose  
---|---|---  
`added_to_pane()`| Item added to workspace| Initialize workspace integration  
`deactivated()`| Item loses active status| Cleanup temporary state  
`workspace_deactivated()`| Workspace loses focus| Pause background work  
`on_removed()`| Item removed from pane| Final cleanup  
`discarded()`| Item discarded from project| Release project resources  
  
Sources: [item.rs165-348](<https://github.com/zed-industries/zed/blob/4109c9dd/item.rs#L165-L348>) [pane.rs1200-1500](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L1200-L1500>)

### Serialization and Persistence

Items implementing `SerializableItem` are automatically persisted:


Sources: [workspace.rs3500-3800](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L3500-L3800>) [persistence/mod.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence/mod.rs>) [items.rs850-950](<https://github.com/zed-industries/zed/blob/4109c9dd/items.rs#L850-L950>)

## Toolbar System

Each pane has a `Toolbar` [toolbar.rs31-39](<https://github.com/zed-industries/zed/blob/4109c9dd/toolbar.rs#L31-L39>) that displays contextual tools. Items implement `ToolbarItemView` to appear in the toolbar.

**Toolbar Architecture**


**ToolbarItemLocation Placement:**

Location| Position| Typical Use  
---|---|---  
`PrimaryLeft`| Far left| Breadcrumbs showing file path/symbol hierarchy  
`PrimaryRight`| Far right| Search bars (buffer search, project search)  
`Secondary`| Between primary areas| Additional contextual tools  
`Hidden`| Not displayed| When item doesn't need toolbar presence  
  
When the active item changes, the toolbar calls `set_active_pane_item()` on each toolbar item to determine its visibility and position.

Sources: [toolbar.rs31-102](<https://github.com/zed-industries/zed/blob/4109c9dd/toolbar.rs#L31-L102>) [buffer_search.rs462-496](<https://github.com/zed-industries/zed/blob/4109c9dd/buffer_search.rs#L462-L496>) [project_search.rs247-250](<https://github.com/zed-industries/zed/blob/4109c9dd/project_search.rs#L247-L250>)

## Focus Management

The workspace maintains complex focus state to ensure proper keyboard navigation and action dispatch.


**Focus Behavior:**

  1. When a pane gains focus, it attempts to restore the last focused element within the active item
  2. If no last focus exists, it focuses the item's primary focus handle
  3. Focus changes trigger action context updates for keybinding resolution
  4. Modal layers can intercept focus to implement dialogs and overlays


Sources: [pane.rs590-630](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L590-L630>) [workspace.rs1500-1700](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L1500-L1700>)

## Action Dispatch

Actions propagate through the focus hierarchy, allowing items and panes to handle workspace-level commands.


**Action Registration:**

Workspace actions are registered in `init()` and on workspace creation:


Sources: [workspace.rs566-610](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L566-L610>) [pane.rs199-251](<https://github.com/zed-industries/zed/blob/4109c9dd/pane.rs#L199-L251>) [keybinding.rs (referenced)](<https://github.com/zed-industries/zed/blob/4109c9dd/keybinding.rs \(referenced\)>)

## Workspace State Persistence

Workspace state is serialized to the `WORKSPACE_DB` SQLite database for session restoration. The database uses the `WorkspaceDb` [persistence/mod.rs16-89](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence/mod.rs#L16-L89>) interface.

**Persistence Data Model**


**Serialization Triggers:**

Trigger| Throttle| Implementation  
---|---|---  
Workspace close| Immediate| [workspace.rs3523-3594](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L3523-L3594>)  
Item changes| 200ms debounce| [workspace.rs143](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L143-L143>) `DelayedDebouncedEditAction`  
Window bounds change| Immediate| [workspace.rs3596-3640](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L3596-L3640>)  
Dock state change| Immediate| [workspace.rs3642-3741](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L3642-L3741>)  
  
**Restoration Flow:**

  1. On startup, workspace queries `WORKSPACE_DB` for saved state by location/SSH project
  2. Deserializes `SerializedWorkspace` structure
  3. Recreates `PaneGroup` hierarchy with splits and flexes
  4. Calls `SerializableItem::deserialize()` for each saved item
  5. Restores active item indices, preview tabs, and dock visibility
  6. After restoration completes, calls `SerializableItem::cleanup()` to remove orphaned items


Sources: [workspace.rs3523-3741](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L3523-L3741>) [persistence/mod.rs16-89](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence/mod.rs#L16-L89>) [persistence/model.rs15-87](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence/model.rs#L15-L87>)

## Integration Points

The workspace integrates with other major systems:

System| Integration Point| Purpose  
---|---|---  
Project| `workspace.project()`| Access to files, LSP, Git  
Client| `workspace.client()`| Collaboration features  
Settings| `WorkspaceSettings::get_global()`| User preferences  
Theme| `cx.theme()`| Visual styling  
Notifications| `workspace.show_notification()`| User feedback  
Modals| `workspace.toggle_modal()`| Dialog display  
  
Sources: [workspace.rs1120-1360](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L1120-L1360>) [workspace.rs566-610](<https://github.com/zed-industries/zed/blob/4109c9dd/workspace.rs#L566-L610>)

Dismiss

Refresh this wiki

This wiki was recently refreshed. Please wait 7 days to refresh again.

### On this page

  * [Workspace and Panel System](<#workspace-and-panel-system>)
  * [Purpose and Scope](<#purpose-and-scope>)
  * [Workspace Architecture](<#workspace-architecture>)
  * [Pane System](<#pane-system>)
  * [Pane Operations](<#pane-operations>)
  * [Item Trait System](<#item-trait-system>)
  * [SerializableItem Trait](<#serializableitem-trait>)
  * [FollowableItem Trait](<#followableitem-trait>)
  * [SearchableItem Trait](<#searchableitem-trait>)
  * [Dock System](<#dock-system>)
  * [Panel Trait](<#panel-trait>)
  * [Workspace Operations](<#workspace-operations>)
  * [Opening Paths](<#opening-paths>)
  * [ProjectItemRegistry](<#projectitemregistry>)
  * [Item Lifecycle](<#item-lifecycle>)
  * [Serialization and Persistence](<#serialization-and-persistence>)
  * [Toolbar System](<#toolbar-system>)
  * [Focus Management](<#focus-management>)
  * [Action Dispatch](<#action-dispatch>)
  * [Workspace State Persistence](<#workspace-state-persistence>)
  * [Integration Points](<#integration-points>)
