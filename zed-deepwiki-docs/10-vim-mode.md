<!-- Source: https://deepwiki.com/zed-industries/zed/10-vim-mode -->

# 10 Vim Mode

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

# Vim Mode

Relevant source files

  * [assets/keymaps/vim.json](<https://github.com/zed-industries/zed/blob/4109c9dd/assets/keymaps/vim.json>)
  * [crates/editor/src/selections_collection.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/selections_collection.rs>)
  * [crates/vim/Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/Cargo.toml>)
  * [crates/vim/src/command.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/command.rs>)
  * [crates/vim/src/helix.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/helix.rs>)
  * [crates/vim/src/motion.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/motion.rs>)
  * [crates/vim/src/normal.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal.rs>)
  * [crates/vim/src/normal/change.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/change.rs>)
  * [crates/vim/src/normal/delete.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/delete.rs>)
  * [crates/vim/src/normal/mark.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/mark.rs>)
  * [crates/vim/src/normal/paste.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/paste.rs>)
  * [crates/vim/src/normal/yank.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/yank.rs>)
  * [crates/vim/src/object.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/object.rs>)
  * [crates/vim/src/replace.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/replace.rs>)
  * [crates/vim/src/state.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/state.rs>)
  * [crates/vim/src/surrounds.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/surrounds.rs>)
  * [crates/vim/src/test/vim_test_context.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/test/vim_test_context.rs>)
  * [crates/vim/src/vim.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs>)
  * [crates/vim/src/visual.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/visual.rs>)


Vim Mode provides modal editing capabilities to Zed's Editor component. It implements Vim's operator-motion-text object model, supporting multiple modes (Normal, Insert, Visual, Replace), text objects, motions, operators, registers, marks, and ex-commands. Additionally, it includes support for Helix-style keybindings as an alternative modal editing system.

For information about the underlying Editor component that Vim Mode extends, see [Editor Architecture](</zed-industries/zed/2.5-keybinding-and-action-system>).

## Architecture and Integration

Vim Mode is implemented as an Editor addon that wraps and extends the behavior of Zed's `Editor` entity. Rather than implementing a separate text editing engine, it intercepts user input and translates Vim semantics into Editor operations.

### Integration with Editor


Sources: [crates/vim/src/vim.rs1-1300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L1-L1300>) [crates/editor/src/editor.rs870-892](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L870-L892>)

The `Vim` entity is created per-editor and stored as an addon using the `Addon` trait defined in [crates/editor/src/editor.rs871-892](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L871-L892>) This trait allows Vim to:

  * Extend the editor's key context with mode-specific bindings
  * Override buffer status displays
  * Render custom UI elements (mode indicator)


Sources: [crates/vim/src/vim.rs196-262](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L196-L262>) [crates/editor/src/editor.rs323-370](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L323-L370>)

### State Management

Vim Mode maintains state at two levels:

  1. **Per-Editor State** (`Vim` entity): Tracks the current mode, pending operators, count, and editor-specific configuration
  2. **Global State** (`VimGlobals`): Stores registers, marks, search history, and settings shared across all editors


Sources: [crates/vim/src/vim.rs93-164](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L93-L164>) [crates/vim/src/state.rs51-123](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/state.rs#L51-L123>)

## Mode System

Vim Mode implements six primary editing modes, each with distinct behavior and available commands. Mode transitions occur through specific actions and are managed by the `Mode` enum.

### Mode Enumeration and Transitions

Mode| Description| Entry Actions| Typical Exit  
---|---|---|---  
`Normal`| Default command mode| `<Esc>`, completing insert/visual action| Enter insert/visual mode  
`Insert`| Text insertion| `i`, `a`, `o`, `O`, `c`, `s`| `<Esc>` to Normal  
`Visual`| Character-wise selection| `v`| `<Esc>` to Normal or operator  
`VisualLine`| Line-wise selection| `V`| `<Esc>` to Normal or operator  
`VisualBlock`| Block-wise selection| `Ctrl-v`| `<Esc>` to Normal or operator  
`Replace`| Overwrite text| `R`, `r`| `<Esc>` to Normal  
  

Sources: [crates/vim/src/state.rs124-135](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/state.rs#L124-L135>) [crates/vim/src/vim.rs630-731](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L630-L731>)

### Mode-Specific Key Contexts

Vim Mode uses GPUI's `KeyContext` system to enable different keybindings per mode. The active mode is exposed through key context predicates:


Sources: [crates/vim/src/vim.rs748-828](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L748-L828>) [assets/keymaps/vim.json1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/assets/keymaps/vim.json#L1-L800>)

The `extend_key_context` method in [crates/vim/src/vim.rs748-828](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L748-L828>) adds context variables like:

  * `vim_mode`: The current mode name
  * `VimControl`: Present when Vim mode is enabled
  * `VimOperator`: Present when an operator is pending (e.g., after pressing `d`)
  * `VimObject`: Present when waiting for a text object (e.g., after pressing `di`)
  * `VimWaiting`: Present when waiting for additional keys (e.g., after pressing `f`)


## Operator-Motion Model

Vim's operator-motion model allows commands to be composed from an operator (action) and a motion (range). This compositional approach is central to Vim's efficiency.

### Execution Flow


Sources: [crates/vim/src/vim.rs863-1075](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L863-L1075>) [crates/vim/src/normal.rs83-244](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal.rs#L83-L244>) [crates/vim/src/object.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/object.rs#L1-L800>)

### Operator Types

The `Operator` enum defines the actions that can be composed with motions:


Sources: [crates/vim/src/state.rs137-152](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/state.rs#L137-L152>) [crates/vim/src/normal/change.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/change.rs#L1-L100>) [crates/vim/src/normal/delete.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/delete.rs#L1-L100>) [crates/vim/src/normal/yank.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/yank.rs#L1-L100>)

### Count Handling

Vim supports numeric counts to repeat operations. Counts can appear before the operator (`3dw`) or after (`d3w`), or both (`2d3w` = delete 6 words).


Sources: [crates/vim/src/vim.rs832-862](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L832-L862>) [crates/vim/src/vim.rs160-164](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L160-L164>)

The count is accumulated digit-by-digit via the `Number` action [crates/vim/src/vim.rs64-66](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L64-L66>) and applied in [crates/vim/src/vim.rs879-920](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L879-L920>) by repeating the motion or operation.

## Motions and Text Objects

### Motion Types

Motions define how to move the cursor or select ranges. They are categorized by their selection behavior:


Sources: [crates/vim/src/motion.rs24-28](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/motion.rs#L24-L28>) [crates/vim/src/motion.rs44-175](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/motion.rs#L44-L175>)

Key motion implementations:

Motion Family| Examples| Implementation  
---|---|---  
Character| `h`, `j`, `k`, `l`| [crates/vim/src/motion.rs240-380](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/motion.rs#L240-L380>)  
Word| `w`, `e`, `b`, `ge`, `W`, `E`, `B`| [crates/vim/src/motion.rs381-570](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/motion.rs#L381-L570>)  
Line| `0`, `^`, `$`, `gg`, `G`| [crates/vim/src/motion.rs571-740](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/motion.rs#L571-L740>)  
Paragraph| `{`, `}`| [crates/vim/src/motion.rs741-820](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/motion.rs#L741-L820>)  
Search| `f{char}`, `t{char}`, `/`, `?`, `*`, `#`| [crates/vim/src/motion.rs821-1050](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/motion.rs#L821-L1050>)  
Matching| `%`, `]]`, `][`, `[[`, `[]`| [crates/vim/src/motion.rs1051-1200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/motion.rs#L1051-L1200>)  
  
### Text Objects

Text objects define semantic ranges in the text (words, sentences, paragraphs, brackets, etc.) with "inner" and "around" variants:


Sources: [crates/vim/src/object.rs21-60](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/object.rs#L21-L60>) [crates/vim/src/object.rs180-440](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/object.rs#L180-L440>)

The text object system integrates with tree-sitter for syntax-aware selections. For example, `dif` (delete inner function) uses the syntax tree to find the function body:


Sources: [crates/vim/src/object.rs450-680](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/object.rs#L450-L680>)

## Registers and Clipboard

Vim Mode implements a register system compatible with traditional Vim, allowing users to store and retrieve text in named registers.

### Register System


Sources: [crates/vim/src/state.rs220-288](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/state.rs#L220-L288>) [crates/vim/src/normal/yank.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/yank.rs#L1-L200>)

The register system is integrated with Zed's clipboard in [crates/vim/src/normal/yank.rs50-150](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/yank.rs#L50-L150>) and [crates/vim/src/normal/paste.rs1-350](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/paste.rs#L1-L350>) When `use_system_clipboard` is enabled, the unnamed register syncs with the system clipboard.

### Yank and Paste Operations


Sources: [crates/vim/src/normal/yank.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/yank.rs#L1-L200>) [crates/vim/src/normal/paste.rs1-350](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/paste.rs#L1-L350>)

## Visual Modes

Visual modes allow selecting text before applying operations. Zed's Vim Mode supports three visual modes with distinct selection behaviors.

### Visual Mode Implementation


Sources: [crates/vim/src/visual.rs1-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/visual.rs#L1-L600>) [crates/vim/src/state.rs289-318](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/state.rs#L289-L318>)

Visual mode stores a `RecordedSelection` that captures the initial anchor point. As the user moves the cursor, the selection is updated to span from the anchor to the current position. The `head_selection` flag tracks which end of the selection is being moved [crates/vim/src/state.rs306-318](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/state.rs#L306-L318>)

### Visual Block Mode

Visual block mode is particularly complex as it creates multiple cursors for column-based editing:


Sources: [crates/vim/src/visual.rs400-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/visual.rs#L400-L600>) [crates/editor/src/editor.rs3800-4200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/editor/src/editor.rs#L3800-L4200>)

## Command-Line Mode

Vim Mode implements ex-commands (`:` commands) with support for ranges, arguments, and command-specific parsing.

### Command Structure


Sources: [crates/vim/src/command.rs1-1200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/command.rs#L1-L1200>) [crates/vim/src/command.rs150-450](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/command.rs#L150-L450>)

### Range Syntax

Vim commands support range specifications that determine which lines the command operates on:

Range Syntax| Description| Example  
---|---|---  
`{number}`| Single line| `:5d` (delete line 5)  
`{start},{end}`| Line range| `:1,10d` (delete lines 1-10)  
`%`| Entire file| `:%s/old/new/g`  
`.`| Current line| `:.d`  
`$`| Last line| `:1,$s/x/y/g`  
`'<,'>`| Visual selection| `:'<,'>s/a/b/g`  
`+{n}`, `-{n}`| Relative offset| `:+3d` (delete 3 lines down)  
  

Sources: [crates/vim/src/command.rs1100-1200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/command.rs#L1100-L1200>) [crates/vim/src/command.rs850-950](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/command.rs#L850-L950>)

### Search and Replace

The `:s` (substitute) command is one of the most complex, supporting patterns, replacements, and flags:


Sources: [crates/vim/src/normal/search.rs1-600](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/search.rs#L1-L600>) [crates/vim/src/command.rs700-900](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/command.rs#L700-L900>)

## Marks and Jumps

Marks allow saving and returning to specific positions in buffers. Vim Mode implements both buffer-local and global marks.

### Mark Types


Sources: [crates/vim/src/state.rs319-380](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/state.rs#L319-L380>) [crates/vim/src/normal/mark.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/mark.rs#L1-L300>)

Marks are set with `m{char}` and jumped to with `'{char}` (line) or ``{char}` (exact position). The implementation stores marks as `Anchor` positions that track through buffer edits [crates/vim/src/state.rs320-340](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/state.rs#L320-L340>)

### Jump List

The jump list tracks navigation history across buffers:


Sources: [crates/vim/src/state.rs52-70](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/state.rs#L52-L70>) [crates/vim/src/normal/mark.rs150-250](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/mark.rs#L150-L250>)

## Recording and Macros

Vim Mode supports recording sequences of actions and replaying them with macros.

### Recording System


Sources: [crates/vim/src/state.rs419-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/state.rs#L419-L500>) [crates/vim/src/normal/repeat.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/repeat.rs#L1-L300>)

Recording is initiated with `q{register}` and stopped with `q` again. During recording, all actions and text insertions are captured in [crates/vim/src/vim.rs1270-1320](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L1270-L1320>) and stored in the global state. Playback occurs via the `Replayer` struct that re-dispatches the recorded actions [crates/vim/src/normal/repeat.rs150-280](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/normal/repeat.rs#L150-L280>)

## Helix Mode Support

In addition to Vim emulation, Zed includes experimental support for Helix-style modal editing, which uses a selection-first model.

### Helix vs Vim Model


Sources: [crates/vim/src/helix.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/helix.rs#L1-L800>) [crates/vim/src/helix/select.rs1-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/helix/select.rs#L1-L400>)

Helix mode is enabled via settings and changes the fundamental editing model:

Aspect| Vim| Helix  
---|---|---  
Selection visibility| Only in visual mode| Always visible  
Motion behavior| Moves cursor| Extends selection  
Operator execution| After motion| On existing selection  
Multiple cursors| Limited (visual block)| Primary feature  
  
Key Helix-specific actions implemented in [crates/vim/src/helix.rs29-59](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/helix.rs#L29-L59>):

  * `HelixYank`: Yank current selection
  * `HelixInsert`: Insert at start of selection
  * `HelixAppend`: Insert at end of selection
  * `HelixSelectLine`: Select entire lines
  * `HelixSelectRegex`: Select all regex matches in selection
  * `HelixDuplicateBelow`/`Above`: Duplicate selections


Sources: [crates/vim/src/helix.rs29-59](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/helix.rs#L29-L59>) [crates/vim/src/helix/select.rs1-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/helix/select.rs#L1-L400>)

## Settings and Configuration

Vim Mode behavior is controlled through Zed's settings system, supporting both Vim-compatible options and Zed-specific configuration.

### Settings Structure


Sources: [crates/vim/src/vim.rs45-91](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L45-L91>) [crates/vim/src/command.rs85-93](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/command.rs#L85-L93>)

Example settings configuration:


The `:set` command [crates/vim/src/command.rs300-450](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/command.rs#L300-L450>) provides Vim-style runtime configuration, updating both Vim's internal state and propagating changes to Editor settings as appropriate.

## Integration Points Summary

This table summarizes how Vim Mode integrates with other Zed systems:

Zed System| Integration Point| Vim Component| Purpose  
---|---|---|---  
Editor| `Addon` trait| `VimAddon`| Extend editor behavior per-instance  
Keymap| Key context| `extend_key_context()`| Mode-dependent key bindings  
Action System| Action registration| `Vim::action()`| Register Vim-specific actions  
Settings| Settings schema| `VimSettings`| User configuration  
Workspace| Command palette| `command_interceptor()`| Ex-command execution  
Clipboard| System integration| Register system| Unified clipboard access  
Language| Syntax tree| Text objects| Semantic selections  
Project| File operations| `:e`, `:w`, `:bd`| Buffer management  
  
Sources: [crates/vim/src/vim.rs196-262](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L196-L262>) [crates/vim/src/vim.rs748-828](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/vim.rs#L748-L828>) [crates/vim/src/command.rs1-1200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/vim/src/command.rs#L1-L1200>)

Dismiss

Refresh this wiki

This wiki was recently refreshed. Please wait 7 days to refresh again.

### On this page

  * [Vim Mode](<#vim-mode>)
  * [Architecture and Integration](<#architecture-and-integration>)
  * [Integration with Editor](<#integration-with-editor>)
  * [State Management](<#state-management>)
  * [Mode System](<#mode-system>)
  * [Mode Enumeration and Transitions](<#mode-enumeration-and-transitions>)
  * [Mode-Specific Key Contexts](<#mode-specific-key-contexts>)
  * [Operator-Motion Model](<#operator-motion-model>)
  * [Execution Flow](<#execution-flow>)
  * [Operator Types](<#operator-types>)
  * [Count Handling](<#count-handling>)
  * [Motions and Text Objects](<#motions-and-text-objects>)
  * [Motion Types](<#motion-types>)
  * [Text Objects](<#text-objects>)
  * [Registers and Clipboard](<#registers-and-clipboard>)
  * [Register System](<#register-system>)
  * [Yank and Paste Operations](<#yank-and-paste-operations>)
  * [Visual Modes](<#visual-modes>)
  * [Visual Mode Implementation](<#visual-mode-implementation>)
  * [Visual Block Mode](<#visual-block-mode>)
  * [Command-Line Mode](<#command-line-mode>)
  * [Command Structure](<#command-structure>)
  * [Range Syntax](<#range-syntax>)
  * [Search and Replace](<#search-and-replace>)
  * [Marks and Jumps](<#marks-and-jumps>)
  * [Mark Types](<#mark-types>)
  * [Jump List](<#jump-list>)
  * [Recording and Macros](<#recording-and-macros>)
  * [Recording System](<#recording-system>)
  * [Helix Mode Support](<#helix-mode-support>)
  * [Helix vs Vim Model](<#helix-vs-vim-model>)
  * [Settings and Configuration](<#settings-and-configuration>)
  * [Settings Structure](<#settings-structure>)
  * [Integration Points Summary](<#integration-points-summary>)
