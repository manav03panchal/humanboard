<!-- Source: https://deepwiki.com/zed-industries/zed/7-settings-and-configuration -->

# 7 Settings And Configuration

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

# Settings and Configuration System

Relevant source files

  * [assets/settings/default.json](<https://github.com/zed-industries/zed/blob/4109c9dd/assets/settings/default.json>)
  * [crates/editor/src/editor_settings.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor_settings.rs>)
  * [crates/language/src/language_settings.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/language_settings.rs>)
  * [crates/migrator/Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/migrator/Cargo.toml>)
  * [crates/migrator/src/migrations.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/migrator/src/migrations.rs>)
  * [crates/migrator/src/migrator.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/migrator/src/migrator.rs>)
  * [crates/project/src/project_settings.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project_settings.rs>)
  * [crates/settings/Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/Cargo.toml>)
  * [crates/settings/src/keymap_file.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/keymap_file.rs>)
  * [crates/settings/src/settings.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings.rs>)
  * [crates/settings/src/settings_content.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content.rs>)
  * [crates/settings/src/settings_content/editor.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/editor.rs>)
  * [crates/settings/src/settings_content/language.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/language.rs>)
  * [crates/settings/src/settings_content/project.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/project.rs>)
  * [crates/settings/src/settings_content/terminal.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/terminal.rs>)
  * [crates/settings/src/settings_content/workspace.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/workspace.rs>)
  * [crates/settings/src/settings_store.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_store.rs>)
  * [crates/settings/src/vscode_import.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/vscode_import.rs>)
  * [crates/settings_ui/Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/Cargo.toml>)
  * [crates/settings_ui/src/page_data.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/page_data.rs>)
  * [crates/settings_ui/src/settings_ui.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/settings_ui.rs>)
  * [crates/terminal/src/terminal_settings.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/terminal/src/terminal_settings.rs>)
  * [crates/worktree/src/worktree_settings.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree_settings.rs>)
  * [crates/zed/src/zed/migrate.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/zed/src/zed/migrate.rs>)
  * [docs/src/configuring-zed.md](<https://github.com/zed-industries/zed/blob/4109c9dd/docs/src/configuring-zed.md>)
  * [docs/src/visual-customization.md](<https://github.com/zed-industries/zed/blob/4109c9dd/docs/src/visual-customization.md>)


## Purpose and Scope

This document describes Zed's hierarchical settings and configuration system, which manages application preferences, editor behavior, language-specific settings, and project configurations. The system supports multiple configuration sources that merge hierarchically, type-safe settings consumption, dynamic settings observation, and a visual settings editor.

For information about keybindings specifically, see the keybinding system documentation (not covered here). For theme configuration details beyond basic theme selection, see the theming system documentation.

* * *

## Architecture Overview

Zed's settings system is built around a **hierarchical merge** strategy where configuration from multiple sources combines to produce final effective settings. The system consists of four main layers:

  1. **Settings Sources** \- Multiple JSON files that provide configuration at different levels
  2. **Settings Processing** \- Migration, parsing, and hierarchical merging
  3. **Settings Core** \- A global `SettingsStore` that manages merged state and observation
  4. **Settings Consumption** \- Type-safe settings objects that components use


**Settings Sources and Processing Pipeline**


**Precedence Order (lowest to highest):**

  1. `assets/settings/default.json` \- Built-in defaults
  2. `~/.config/zed/settings.json` \- User global settings
  3. `.zed/settings.json` \- Project local settings
  4. Release channel overrides (`nightly`, `preview`, `stable`, `dev`)
  5. OS overrides (`macos`, `linux`, `windows`)
  6. `.editorconfig` properties (file-specific)


Sources: [crates/settings/src/settings_content.rs166-219](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content.rs#L166-L219>) [docs/src/configuring-zed.md46-67](<https://github.com/zed-industries/zed/blob/4109c9dd/docs/src/configuring-zed.md#L46-L67>) [crates/migrator/src/migrator.rs151-233](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/migrator/src/migrator.rs#L151-L233>)

* * *

## Core Components

### SettingsStore - Global Singleton

The `SettingsStore` is a global singleton that manages all settings in the application. It is the central registry for all configuration state.

**Core Responsibilities:**

  * Load settings from JSON files
  * Migrate outdated settings using `Migrator`
  * Merge settings hierarchically via `MergeFrom` trait
  * Cache EditorConfig properties per worktree
  * Notify observers when settings change
  * Provide type-safe access via `Settings::get()`


**SettingsStore Internal Structure**


**Key Methods:**

Method| Purpose| Returns  
---|---|---  
`Settings::get_global(cx)`| Get merged settings for current context| `&T where T: Settings`  
`get_value_from_file(file, pick_fn)`| Extract specific value from a settings file| `(SettingsFile, Option<&T>)`  
`get_content_for_file(file)`| Get raw content for a specific file| `Option<&SettingsContent>`  
`raw_default_settings()`| Access unmodified defaults| `&SettingsContent`  
`editorconfig_properties(worktree_id, path)`| Get EditorConfig for file| `Option<&Properties>`  
  
Sources: [crates/settings_ui/src/settings_ui.rs175-179](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/settings_ui.rs#L175-L179>) [crates/settings_ui/src/settings_ui.rs194-223](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/settings_ui.rs#L194-L223>) [crates/settings_ui/src/settings_ui.rs150-161](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/settings_ui.rs#L150-L161>)

### SettingsContent - Unified Schema

`SettingsContent` is the unified schema that represents all possible settings in Zed. It contains nested structures for each settings domain (editor, workspace, project, terminal, etc.).


Sources: [crates/settings/src/settings_content.rs34-163](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content.rs#L34-L163>) [crates/settings/src/settings_content/editor.rs14-288](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/editor.rs#L14-L288>) [crates/settings/src/settings_content/workspace.rs14-268](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/workspace.rs#L14-L268>)

### Settings Trait and Type Conversion

Components don't use `SettingsContent` directly. Instead, each domain defines a typed settings struct that implements the `Settings` trait:


**Example: EditorSettings::from_settings()**

The conversion from `SettingsContent` to typed settings extracts and validates fields:
    
    
    EditorSettings::from_settings(content: &SettingsContent) -> Self {
        let editor = content.editor.clone();
        let scrollbar = editor.scrollbar.unwrap();  // Unwrap safe due to defaults
        Self {
            cursor_blink: editor.cursor_blink.unwrap(),
            scrollbar: Scrollbar {
                show: scrollbar.show.map(Into::into).unwrap(),
                git_diff: scrollbar.git_diff.unwrap(),
                ...
            },
            ...
        }
    }
    

Sources: [crates/editor/src/editor_settings.rs189-290](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor_settings.rs#L189-L290>) [crates/workspace/src/workspace_settings.rs63-112](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace_settings.rs#L63-L112>) [crates/language/src/language_settings.rs22-33](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/language_settings.rs#L22-L33>)

### MergeFrom Trait - Hierarchical Merging

The `MergeFrom` trait enables hierarchical settings merging. It is automatically derived for most settings types:


**Merge Rules:**

  * `None` values are replaced by `Some` values from higher precedence sources
  * `Some` values in lower precedence are kept if higher precedence has `None`
  * Maps merge key-by-key
  * Arrays are typically **replaced** entirely (not merged element-wise)


Sources: [crates/settings/src/settings_content/language.rs30-56](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/language.rs#L30-L56>)

* * *

## Settings Migration System

Zed includes a migration system that automatically updates settings and keymaps when their schema changes between versions. This ensures users don't experience breaking changes when upgrading.

**Migration Architecture**


**How Migrations Work:**

  1. **Sequential Application** : Migrations are applied in chronological order (e.g., `m_2025_01_02` → `m_2025_01_29` → ...)
  2. **Non-Destructive** : If no migration is needed, original text is returned unchanged
  3. **Two Migration Types** : 
     * **TreeSitter Migrations** : Pattern-based transformations using tree-sitter queries
     * **JSON Migrations** : Direct manipulation of parsed JSON structure


**Example Migration:**

When `formatters_on_save` was renamed to `format_on_save`:


**Migration Files:**

Each migration is in `crates/migrator/src/migrations/m_YYYY_MM_DD/`:

  * `settings.rs` \- Settings migrations
  * `keymap.rs` \- Keymap migrations


Sources: [crates/migrator/src/migrator.rs1-16](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/migrator/src/migrator.rs#L1-L16>) [crates/migrator/src/migrator.rs69-118](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/migrator/src/migrator.rs#L69-L118>) [crates/migrator/src/migrator.rs151-233](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/migrator/src/migrator.rs#L151-L233>) [crates/migrator/src/migrations.rs1-162](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/migrator/src/migrations.rs#L1-L162>)

* * *

## Settings Sources in Detail

### default.json

The built-in defaults file contains comprehensive default values for all settings.

Location| `assets/settings/default.json`  
---|---  
Precedence| Lowest (base defaults)  
Can Modify| No (built into binary)  
Scope| Global  
  
Sources: [assets/settings/default.json1-267](<https://github.com/zed-industries/zed/blob/4109c9dd/assets/settings/default.json#L1-L267>)

### User Settings File

The user's global settings file. Opened via `zed::OpenSettingsFile` action.

Location| `~/.config/zed/settings.json` (Linux)  
`~/Library/Application Support/Zed/settings.json` (macOS)  
---|---  
Precedence| Higher than defaults  
Can Modify| Yes (user editable)  
Scope| All workspaces for this user  
Schema Support| `"$schema": "zed://schemas/settings"`  
  
Sources: [docs/src/configuring-zed.md14-27](<https://github.com/zed-industries/zed/blob/4109c9dd/docs/src/configuring-zed.md#L14-L27>)

### Project Settings File

Project-local settings stored in the project directory. Created via `zed::OpenProjectSettings` action.

Location| `.zed/settings.json`  
---|---  
Precedence| Higher than user settings  
Can Modify| Yes (project-specific)  
Scope| Single project only  
Restrictions| Some settings cannot be set here (e.g., `theme`, `vim_mode`)  
  
Project settings use `LocalSettingsKind` to determine which settings are allowed. Not all settings can be overridden at project level - settings that affect application-wide behavior (like `theme` or `vim_mode`) are restricted to user settings only.

Sources: [docs/src/configuring-zed.md35-43](<https://github.com/zed-industries/zed/blob/4109c9dd/docs/src/configuring-zed.md#L35-L43>) [crates/project/src/project_settings.rs22-34](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project_settings.rs#L22-L34>)

### Release Channel Overrides

Settings can be scoped to specific release channels:


Sources: [docs/src/configuring-zed.md46-67](<https://github.com/zed-industries/zed/blob/4109c9dd/docs/src/configuring-zed.md#L46-L67>) [crates/settings/src/settings_content.rs194-202](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content.rs#L194-L202>)

### OS Overrides

Settings can also be scoped to specific operating systems:


Sources: [crates/settings/src/settings_content.rs204-211](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content.rs#L204-L211>)

### EditorConfig Integration

Zed integrates with `.editorconfig` files for per-file editor configuration. EditorConfig settings have the **highest precedence** and override all other sources for matching files.

**Supported EditorConfig Properties:**

  * `indent_style` → `hard_tabs`
  * `indent_size` / `tab_width` → `tab_size`
  * `max_line_length` → `preferred_line_length`
  * `trim_trailing_whitespace` → `remove_trailing_whitespace_on_save`
  * `insert_final_newline` → `ensure_final_newline_on_save`


Sources: [crates/language/src/language_settings.rs466-498](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/language_settings.rs#L466-L498>)

* * *

## Settings Data Flow

**Settings Loading and Consumption Flow**


Sources: [crates/editor/src/editor_settings.rs189-290](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor_settings.rs#L189-L290>) [crates/settings_ui/src/settings_ui.rs175-179](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/settings_ui.rs#L175-L179>)

* * *

## Language-Specific Settings

Language settings support both **global defaults** and **per-language overrides**. This is one of the most sophisticated parts of the settings system.

### AllLanguageSettings Structure


Sources: [crates/language/src/language_settings.rs47-55](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/language_settings.rs#L47-L55>) [crates/settings/src/settings_content/language.rs13-28](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/language.rs#L13-L28>)

### Language Settings Merge Strategy

When resolving settings for a specific language:

  1. Start with global `defaults`
  2. If language-specific settings exist, merge them on top
  3. Apply EditorConfig properties if present


    
    
    effective_language_settings = 
        defaults 
        + languages[language_name] 
        + editorconfig_properties
    

**Example JSON:**


Sources: [crates/language/src/language_settings.rs424-447](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/language_settings.rs#L424-L447>) [crates/settings/src/settings_content/language.rs30-56](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/language.rs#L30-L56>)

### Language Settings Fields

Key fields in `LanguageSettings`:

Field| Type| Description  
---|---|---  
`tab_size`| `NonZeroU32`| Column width of tabs  
`hard_tabs`| `bool`| Use tab characters vs spaces  
`soft_wrap`| `SoftWrap`| How to wrap long lines  
`preferred_line_length`| `u32`| Soft wrap column  
`format_on_save`| `FormatOnSave`| Auto-format behavior  
`formatter`| `FormatterList`| Which formatter(s) to use  
`enable_language_server`| `bool`| Enable LSP  
`language_servers`| `Vec<String>`| Which LSP servers to use  
`inlay_hints`| `InlayHintSettings`| Inlay hint configuration  
`completions`| `CompletionSettings`| Completion behavior  
  
Sources: [crates/language/src/language_settings.rs64-158](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/language_settings.rs#L64-L158>)

* * *

## Settings UI System

Zed includes a visual settings editor that dynamically generates forms based on settings metadata.

### SettingsWindow Architecture


Sources: [crates/settings_ui/src/settings_ui.rs231-332](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/settings_ui.rs#L231-L332>) [crates/settings_ui/src/settings_ui.rs410-519](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/settings_ui.rs#L410-L519>)

### SettingField - Core Abstraction

Each configurable setting is represented by a `SettingField<T>`:


**Example: Cursor Blink Setting**


The `json_path` field enables deep linking to specific settings via URLs like `zed://settings/editor.cursor_blink`.

**Trait Bounds:**

`SettingField<T>` is wrapped in a trait object `Box<dyn AnySettingField>` to enable heterogeneous collections. The `AnySettingField` trait provides:

  * `file_set_in()` \- Determine which file defines this setting
  * `reset_to_default_fn()` \- Generate reset callback for UI
  * `json_path()` \- Get deep link path


Sources: [crates/settings_ui/src/settings_ui.rs89-111](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/settings_ui.rs#L89-L111>) [crates/settings_ui/src/settings_ui.rs146-229](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/settings_ui.rs#L146-L229>) [crates/settings_ui/src/page_data.rs28-62](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/page_data.rs#L28-L62>)

### Dynamic Field Rendering

The settings UI uses a type-based dispatch system to render different field types. Each Rust type is registered with a rendering function.

**Type-to-Renderer Dispatch System**


**Renderer Registration:**


The `add_basic_renderer` method creates a wrapper that:

  1. Downcasts `Box<dyn AnySettingField>` to `SettingField<T>`
  2. Calls the type-specific render function
  3. Wraps result in standard `render_settings_item()` layout


Sources: [crates/settings_ui/src/settings_ui.rs410-524](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/settings_ui.rs#L410-L524>) [crates/settings_ui/src/settings_ui.rs255-286](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/settings_ui.rs#L255-L286>) [crates/settings_ui/src/settings_ui.rs288-331](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/settings_ui.rs#L288-L331>)

### Settings Page Data

Settings are organized into pages defined in `page_data.rs`:


Sources: [crates/settings_ui/src/page_data.rs22-268](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/page_data.rs#L22-L268>)

* * *

## Settings Consumption Patterns

Components access settings using `Settings::get_global(cx)`:

### Static Access


### Location-Aware Access

For settings that vary by file location (like language settings):


Sources: [crates/language/src/language_settings.rs22-33](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/language_settings.rs#L22-L33>) [crates/editor/src/editor_settings.rs177-181](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor_settings.rs#L177-L181>)

### Settings Observation

Components can observe settings changes and react:


Sources: Referenced in settings system but implementation details in SettingsStore

* * *

## Project vs User Settings

Not all settings can be overridden at the project level. The system distinguishes between:

### User-Only Settings

These affect the application environment or user preferences:

Setting Category| Examples| Reason  
---|---|---  
**UI Preferences**| `theme`, `vim_mode`, `base_keymap`| Personal preference  
**Window Behavior**| `window_decorations`, `confirm_quit`| OS integration  
**Telemetry**| `telemetry.diagnostics`, `telemetry.metrics`| Privacy  
**Update Settings**| `auto_update`| Installation management  
  
### Project-Overridable Settings

These affect code editing and project behavior:

Setting Category| Examples| Scope  
---|---|---  
**Editor Behavior**| `tab_size`, `hard_tabs`, `soft_wrap`| Per-file/language  
**Language Settings**| `formatter`, `language_servers`, `format_on_save`| Language-specific  
**LSP Configuration**| `lsp[server].initialization_options`| Project infrastructure  
**Terminal**| `shell`, `working_directory`, `env`| Project environment  
**Git**|  Git-related settings| Project-specific VCS  
  
Sources: [docs/src/configuring-zed.md35-43](<https://github.com/zed-industries/zed/blob/4109c9dd/docs/src/configuring-zed.md#L35-L43>) [crates/project/src/project_settings.rs36-75](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project_settings.rs#L36-L75>)

* * *

## Settings File Format

Settings files use **JSON with comments** (JSONC):


### Schema Validation

Settings files have JSON schema support:


This enables IDE autocompletion and validation in editors that support JSON Schema.

Sources: [assets/settings/default.json1-5](<https://github.com/zed-industries/zed/blob/4109c9dd/assets/settings/default.json#L1-L5>) [docs/src/configuring-zed.md44](<https://github.com/zed-industries/zed/blob/4109c9dd/docs/src/configuring-zed.md#L44-L44>)

* * *

## Settings File Watching

The settings system watches settings files for changes and automatically reloads:


Sources: Referenced in project_settings.rs file watching logic

* * *

## VS Code Settings Import

Zed can import settings from VS Code or Cursor:


**Example Mappings:**

VS Code Setting| Zed Setting  
---|---  
`editor.cursorBlinking`| `cursor_blink`  
`editor.cursorStyle`| `cursor_shape`  
`editor.fontSize`| `buffer_font_size`  
`editor.fontFamily`| `buffer_font_family`  
`editor.tabSize`| `tab_size`  
`editor.wordWrap`| `soft_wrap`  
`editor.minimap.enabled`| `minimap.show`  
  
Sources: [crates/settings/src/vscode_import.rs29-219](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/vscode_import.rs#L29-L219>) [crates/settings/src/vscode_import.rs230-269](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/vscode_import.rs#L230-L269>)

* * *

## Key Implementation Files

File Path| Purpose| Key Symbols  
---|---|---  
[assets/settings/default.json](<https://github.com/zed-industries/zed/blob/4109c9dd/assets/settings/default.json>)| Built-in default settings| JSON schema document  
[crates/settings/src/settings_content.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content.rs>)| `SettingsContent` unified schema| `SettingsContent`, `UserSettingsContent`  
[crates/settings/src/settings_content/editor.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/editor.rs>)| Editor settings schema| `EditorSettingsContent`  
[crates/settings/src/settings_content/language.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/language.rs>)| Language settings schema| `AllLanguageSettingsContent`, `LanguageSettingsContent`  
[crates/settings/src/settings_content/workspace.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/workspace.rs>)| Workspace settings schema| `WorkspaceSettingsContent`  
[crates/settings/src/settings_content/project.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/project.rs>)| Project settings schema| `ProjectSettingsContent`, `LspSettings`  
[crates/settings/src/settings_content/terminal.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/settings_content/terminal.rs>)| Terminal settings schema| `ProjectTerminalSettingsContent`  
[crates/editor/src/editor_settings.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor_settings.rs>)| Typed `EditorSettings`| `EditorSettings::from_settings()`  
[crates/language/src/language_settings.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/language/src/language_settings.rs>)| Typed `LanguageSettings`| `AllLanguageSettings`, `language_settings()`  
[crates/project/src/project_settings.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/project_settings.rs>)| Typed `ProjectSettings`| `ProjectSettings`, `DiagnosticsSettings`, `GitSettings`  
[crates/workspace/src/workspace_settings.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/workspace/src/workspace_settings.rs>)| Typed `WorkspaceSettings`| `WorkspaceSettings::from_settings()`  
[crates/terminal/src/terminal_settings.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/terminal/src/terminal_settings.rs>)| Typed `TerminalSettings`| `TerminalSettings::from_settings()`  
[crates/worktree/src/worktree_settings.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/worktree/src/worktree_settings.rs>)| Typed `WorktreeSettings`| `WorktreeSettings`, `PathMatcher`  
[crates/settings_ui/src/settings_ui.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/settings_ui.rs>)| Visual settings editor| `SettingsWindow`, `SettingFieldRenderer`, `init_renderers()`  
[crates/settings_ui/src/page_data.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/page_data.rs>)| Settings page definitions| `settings_data()`, `SettingsPage`, `SettingItem`  
[crates/settings_ui/src/components.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings_ui/src/components.rs>)| UI components| `EnumVariantDropdown`, `SettingsInputField`  
[crates/settings/src/vscode_import.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/settings/src/vscode_import.rs>)| VS Code settings import| `VsCodeSettings`, `settings_content()`  
[crates/migrator/src/migrator.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/migrator/src/migrator.rs>)| Settings migration| `migrate_settings()`, `migrate_keymap()`  
[crates/migrator/src/migrations.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/migrator/src/migrations.rs>)| Migration modules| `m_2025_01_02`, `m_2025_10_01`, etc.  
  
* * *

## Summary

The settings system provides a flexible, type-safe configuration framework with:

  * **Hierarchical merging** from multiple sources (default → user → project → overrides)
  * **Type-safe consumption** via the `Settings` trait and typed settings structs
  * **Language-specific settings** with global defaults and per-language overrides
  * **Visual settings editor** with dynamic form generation
  * **Settings observation** for reactive UI updates
  * **EditorConfig integration** for per-file configuration
  * **VS Code import** for easy migration


The system balances flexibility (JSON-based configuration) with safety (typed settings objects) and provides both programmatic and visual interfaces for settings management.

Dismiss

Refresh this wiki

This wiki was recently refreshed. Please wait 7 days to refresh again.

### On this page

  * [Settings and Configuration System](<#settings-and-configuration-system>)
  * [Purpose and Scope](<#purpose-and-scope>)
  * [Architecture Overview](<#architecture-overview>)
  * [Core Components](<#core-components>)
  * [SettingsStore - Global Singleton](<#settingsstore---global-singleton>)
  * [SettingsContent - Unified Schema](<#settingscontent---unified-schema>)
  * [Settings Trait and Type Conversion](<#settings-trait-and-type-conversion>)
  * [MergeFrom Trait - Hierarchical Merging](<#mergefrom-trait---hierarchical-merging>)
  * [Settings Migration System](<#settings-migration-system>)
  * [Settings Sources in Detail](<#settings-sources-in-detail>)
  * [default.json](<#defaultjson>)
  * [User Settings File](<#user-settings-file>)
  * [Project Settings File](<#project-settings-file>)
  * [Release Channel Overrides](<#release-channel-overrides>)
  * [OS Overrides](<#os-overrides>)
  * [EditorConfig Integration](<#editorconfig-integration>)
  * [Settings Data Flow](<#settings-data-flow>)
  * [Language-Specific Settings](<#language-specific-settings>)
  * [AllLanguageSettings Structure](<#alllanguagesettings-structure>)
  * [Language Settings Merge Strategy](<#language-settings-merge-strategy>)
  * [Language Settings Fields](<#language-settings-fields>)
  * [Settings UI System](<#settings-ui-system>)
  * [SettingsWindow Architecture](<#settingswindow-architecture>)
  * [SettingField - Core Abstraction](<#settingfield---core-abstraction>)
  * [Dynamic Field Rendering](<#dynamic-field-rendering>)
  * [Settings Page Data](<#settings-page-data>)
  * [Settings Consumption Patterns](<#settings-consumption-patterns>)
  * [Static Access](<#static-access>)
  * [Location-Aware Access](<#location-aware-access>)
  * [Settings Observation](<#settings-observation>)
  * [Project vs User Settings](<#project-vs-user-settings>)
  * [User-Only Settings](<#user-only-settings>)
  * [Project-Overridable Settings](<#project-overridable-settings>)
  * [Settings File Format](<#settings-file-format>)
  * [Schema Validation](<#schema-validation>)
  * [Settings File Watching](<#settings-file-watching>)
  * [VS Code Settings Import](<#vs-code-settings-import>)
  * [Key Implementation Files](<#key-implementation-files>)
  * [Summary](<#summary>)
