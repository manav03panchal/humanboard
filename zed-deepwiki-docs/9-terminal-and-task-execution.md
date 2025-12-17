<!-- Source: https://deepwiki.com/zed-industries/zed/9-terminal-and-task-execution -->

# 9 Terminal And Task Execution

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

# Terminal and Task Execution

Relevant source files

  * [crates/project/src/terminals.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/project/src/terminals.rs>)
  * [crates/repl/src/outputs/plain.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/repl/src/outputs/plain.rs>)
  * [crates/task/src/task.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/task/src/task.rs>)
  * [crates/terminal/Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/terminal/Cargo.toml>)
  * [crates/terminal/src/terminal.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/terminal/src/terminal.rs>)
  * [crates/terminal/src/terminal_hyperlinks.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/terminal/src/terminal_hyperlinks.rs>)
  * [crates/terminal_view/src/persistence.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/terminal_view/src/persistence.rs>)
  * [crates/terminal_view/src/terminal_element.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/terminal_view/src/terminal_element.rs>)
  * [crates/terminal_view/src/terminal_panel.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/terminal_view/src/terminal_panel.rs>)
  * [crates/terminal_view/src/terminal_view.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/terminal_view/src/terminal_view.rs>)


This document covers Zed's integrated terminal system and task execution infrastructure. It explains how terminals are created, managed, and how they integrate with the task system to run build commands, tests, and other development workflows.

For information about the Git integration that may also spawn terminals, see [Git Integration](</zed-industries/zed/4.1-workspace-organization>). For information about Vim mode which has special terminal keybinding considerations, see [Vim Mode](</zed-industries/zed/4.3-pane-management>).

## Purpose and Scope

Zed's terminal system provides an embedded terminal emulator within the editor, allowing users to run shell commands, execute tasks, and interact with command-line tools without leaving the development environment. The system is built on the Alacritty terminal emulator backend and provides deep integration with Zed's workspace, project management, and task systems.

The terminal system handles:

  * Creating and managing terminal instances with PTY (pseudo-terminal) communication
  * Rendering terminal content with ANSI color support and text styling
  * Task execution with configurable reveal, hide, and concurrency strategies
  * Terminal session persistence across workspace restarts
  * Multi-pane terminal layouts with splits and tabs


## Terminal Architecture

The terminal system is structured in layers, from the low-level PTY communication to the high-level UI components.

### Core Terminal Entity


**Diagram: Terminal Core Architecture**

The `Terminal` entity [terminal/terminal.rs820-849](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L820-L849>) is the central component that manages a terminal session. Key fields include:

  * **term** : `Arc<FairMutex<Term<ZedListener>>>` [terminal/terminal.rs823](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L823-L823>) \- The Alacritty terminal backend with thread-safe access
  * **terminal_type** : `TerminalType` [terminal/terminal.rs821](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L821-L821>) \- Either `Pty` variant with `Notifier` and `PtyProcessInfo` [terminal/terminal.rs813-816](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L813-L816>) or `DisplayOnly` [terminal/terminal.rs817](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L817-L817>) for rendering without shell
  * **task** : `Option<TaskState>` [terminal/terminal.rs838](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L838-L838>) \- Tracks task execution state [terminal/terminal.rs862-867](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L862-L867>)
  * **event_loop_task** : `Task<Result<()>>` [terminal/terminal.rs848](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L848-L848>) \- Processes events from Alacritty's event loop [terminal/terminal.rs666-724](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L666-L724>)
  * **template** : `CopyTemplate` [terminal/terminal.rs845](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L845-L845>) \- Stores configuration for cloning terminals [terminal/terminal.rs851-860](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L851-L860>)


Sources: [terminal/terminal.rs820-849](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L820-L849>) [terminal/terminal.rs813-817](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L813-L817>) [terminal/terminal.rs862-867](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L862-L867>) [terminal/terminal.rs666-724](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L666-L724>)

### Terminal Creation Flow


**Diagram: Terminal Creation and Event Loop**

Terminal creation follows the builder pattern:

  1. **TerminalBuilder::new()** [terminal/terminal.rs406-661](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L406-L661>) initializes the terminal:

     * **Environment Setup** [terminal/terminal.rs424-441](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L424-L441>): Removes `SHLVL`, sets `ZED_TERM=true`, `TERM=xterm-256color`, `COLORTERM=truecolor`, `TERM_PROGRAM=zed`
     * **Shell Resolution** [terminal/terminal.rs465-483](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L465-L483>): Handles `Shell::System`, `Shell::Program`, `Shell::WithArguments` variants
     * **Config Creation** [terminal/terminal.rs532-536](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L532-L536>): Sets scrolling history (10k default, 100k for tasks) and cursor style
  2. **PTY Setup** [terminal/terminal.rs539-550](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L539-L550>) creates the pseudo-terminal using `tty::new()` with shell and working directory

  3. **Alacritty Components** [terminal/terminal.rs556-579](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L556-L579>):

     * Creates `Term<ZedListener>` with config and bounds
     * Creates `EventLoop` connecting term to PTY
     * Spawns I/O thread via `event_loop.spawn()` [terminal/terminal.rs582](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L582-L582>)
  4. **Activation Scripts** [terminal/terminal.rs630-648](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L630-L648>): If `activation_script` is provided (e.g., for Python venv), writes commands to PTY followed by clear screen command

  5. **Event Subscription** [terminal/terminal.rs664-725](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L664-L725>) starts async task that:

     * Receives events from `events_rx` unbounded channel
     * Batches events with 4ms timer [terminal/terminal.rs679-682](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L679-L682>) or up to 100 events [terminal/terminal.rs696](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L696-L696>)
     * Calls `process_event()` [terminal/terminal.rs670-718](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L670-L718>) for each event


Sources: [terminal/terminal.rs406-661](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L406-L661>) [terminal/terminal.rs424-441](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L424-L441>) [terminal/terminal.rs539-550](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L539-L550>) [terminal/terminal.rs664-725](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L664-L725>)

## Terminal Types and Variants

Zed supports two terminal variants:

Variant| Purpose| PTY Connection| Use Cases  
---|---|---|---  
`TerminalType::Pty`| Full interactive terminal| Yes, with `Notifier` and `PtyProcessInfo`| User shells, task execution, interactive commands  
`TerminalType::DisplayOnly`| Display-only terminal| No PTY, pure rendering| REPL output, test results, log viewing  
  
The `DisplayOnly` variant is created with `TerminalBuilder::new_display_only()` [terminal/terminal.rs331-403](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L331-L403>) and is used by components like the REPL system [repl/outputs/plain.rs49-135](<https://github.com/zed-industries/zed/blob/4109c9dd/repl/outputs/plain.rs#L49-L135>) to render terminal output without shell interaction.

Sources: [terminal/terminal.rs806-812](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L806-L812>) [terminal/terminal.rs331-403](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L331-L403>) [repl/outputs/plain.rs49-135](<https://github.com/zed-industries/zed/blob/4109c9dd/repl/outputs/plain.rs#L49-L135>)

## PTY Communication

### Event Flow from PTY to UI


**Diagram: PTY Event Processing Pipeline**

The PTY communication system [terminal/terminal.rs889-968](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L889-L968>) processes various event types:

  * **Title Events** [terminal/terminal.rs891-912](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L891-L912>): Update terminal breadcrumbs from shell escape sequences
  * **Clipboard Events** [terminal/terminal.rs913-925](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L913-L925>): Handle clipboard read/write requests from terminal programs
  * **Wakeup Events** [terminal/terminal.rs942-950](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L942-L950>): Trigger UI refresh and process info updates
  * **Exit Events** [terminal/terminal.rs938-966](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L938-L966>): Handle process termination and cleanup


The `ZedListener` [terminal/terminal.rs168-176](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L168-L176>) serves as the bridge, forwarding Alacritty events through an unbounded channel for main thread processing.

Sources: [terminal/terminal.rs889-968](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L889-L968>) [terminal/terminal.rs168-176](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L168-L176>)

### Writing to PTY


**Diagram: Input Flow to Shell**

Writing to the PTY happens through the `Notifier` wrapper [terminal/terminal.rs808](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L808-L808>) which sends messages to the event loop's channel. The `Terminal::write_to_pty()` method [terminal/terminal.rs1175-1181](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L1175-L1181>) handles:

  * User input from keyboard events
  * Paste operations
  * Clipboard content
  * Terminal commands


Sources: [terminal/terminal.rs1175-1181](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L1175-L1181>) [terminal/terminal.rs808](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L808-L808>)

## Task Execution Integration

### Task Lifecycle in Terminals


**Diagram: Task Execution Flow Through Terminals**

Task execution integrates deeply with the terminal system through the `TaskState` structure [terminal/terminal.rs856-872](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L856-L872>):
    
    
    pub struct TaskState {
        pub status: TaskStatus,
        pub completion_rx: Receiver<Option<ExitStatus>>,
        pub spawned_task: SpawnInTerminal,
    }
    

Sources: [terminal/terminal.rs856-872](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L856-L872>)

### Task Spawning Strategy

The `TerminalPanel::spawn_task()` method [terminal_panel.rs521-610](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L521-L610>) implements sophisticated task spawning logic:

  1. **Concurrent Run Check** [terminal_panel.rs562-580](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L562-L580>):

     * If `allow_concurrent_runs && use_new_terminal`: spawn immediately in new terminal
     * Otherwise: search for existing terminals with matching task label
  2. **Terminal Reuse** [terminal_panel.rs566-581](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L566-L581>):

     * `terminals_for_task()` finds all terminals running the same task
     * If not `allow_concurrent_runs`: defer until existing tasks complete
     * If reusing terminal: replace content via `replace_terminal()`
  3. **Task Queueing** [terminal_panel.rs584-607](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L584-L607>):

     * Deferred tasks stored in `deferred_tasks` HashMap keyed by TaskId
     * Uses `wait_for_terminals_tasks()` to await completion
     * Automatically spawns queued task when previous instances finish


Sources: [terminal_panel.rs521-610](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L521-L610>) [terminal_panel.rs566-581](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L566-L581>)

### Environment and Shell Resolution


**Diagram: Task Environment Resolution**

The `Project::create_terminal_task()` method [project/terminals.rs46-276](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L46-L276>) performs environment setup:

  1. **Environment Resolution** [project/terminals.rs98](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L98-L98>): Calls `resolve_directory_environment()` [project/terminals.rs539-561](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L539-L561>) which uses `ProjectEnvironment` to get shell environment for the working directory

  2. **Shell Selection** [project/terminals.rs86-94](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L86-L94>):

     * For remote: Uses `RemoteClient.shell()` or falls back to system shell
     * For local: Uses `TerminalSettings.shell.program()`
     * Creates `ShellKind` to determine POSIX vs CMD vs PowerShell [project/terminals.rs94](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L94-L94>)
  3. **Toolchain Detection** [project/terminals.rs100-138](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L100-L138>):

     * If `detect_venv` is enabled, queries `active_toolchain()` for Python [project/terminals.rs114](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L114-L114>)
     * Calls `ToolchainLister::activation_script()` to generate venv activation commands [project/terminals.rs131-133](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L131-L133>)
     * Returns `Vec<String>` of shell commands to activate toolchain
  4. **Command Construction** [project/terminals.rs140-250](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L140-L250>):

     * **ShellBuilder** [project/terminals.rs554-556](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L554-L556>): Creates appropriate command syntax for the shell type
     * **Command Formatting** : `command_label()` for display, `build_no_quote()` for execution [project/terminals.rs555-556](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L555-L556>)
     * **Script Chaining** [project/terminals.rs163-210](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L163-L210>): Combines activation script with task command using shell-specific separators (`;` for POSIX, `&&` for CMD)
     * **Remote vs Local** : For remote, uses `create_remote_shell()` [project/terminals.rs176-181](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L176-L181>); for local, creates `Shell::WithArguments` [project/terminals.rs212-216](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L212-L216>)
  5. **Remote Shell Creation** [project/terminals.rs564-586](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L564-L586>):

     * Uses `RemoteClient.build_command()` to wrap shell in remote execution
     * Sets `TERM=xterm-256color` for proper terminal emulation [project/terminals.rs571-572](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L571-L572>)
     * Returns `(Shell, HashMap<String, String>)` tuple


Sources: [project/terminals.rs46-276](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L46-L276>) [project/terminals.rs539-561](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L539-L561>) [project/terminals.rs564-586](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L564-L586>)

## Terminal View and Rendering

### Terminal View Component


**Diagram: TerminalView Architecture**

The `TerminalView` [terminal_view/terminal_view.rs119-142](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_view/terminal_view.rs#L119-L142>) is the primary UI component that wraps a `Terminal` entity. It handles:

  * **Focus Management** : Integrates with GPUI's focus system [terminal_view/terminal_view.rs189-193](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_view/terminal_view.rs#L189-L193>)
  * **Cursor Blinking** : Uses `BlinkManager` for cursor visibility [terminal_view/terminal_view.rs128-661](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_view/terminal_view.rs#L128-L661>)
  * **Content Modes** : Supports both standalone scrollable and embedded inline modes [terminal_view/terminal_view.rs144-176](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_view/terminal_view.rs#L144-L176>)
  * **Input Handling** : Processes keyboard, mouse, and scroll events
  * **IME Support** : Handles marked text for Input Method Editors [terminal_view/terminal_view.rs327-362](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_view/terminal_view.rs#L327-L362>)


Sources: [terminal_view/terminal_view.rs119-142](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_view/terminal_view.rs#L119-L142>) [terminal_view/terminal_view.rs189-193](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_view/terminal_view.rs#L189-L193>) [terminal_view/terminal_view.rs636-661](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_view/terminal_view.rs#L636-L661>)

### Terminal Rendering Pipeline

The rendering system uses `TerminalElement` [terminal_element.rs275-318](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_element.rs#L275-L318>) which implements a sophisticated batching system:


**Diagram: Terminal Rendering Optimization**

The `TerminalElement::layout_grid()` method [terminal_element.rs322-492](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_element.rs#L322-L492>) optimizes rendering by:

  1. **Text Run Batching** [terminal_element.rs343-462](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_element.rs#L343-L462>):

     * Groups adjacent cells with identical styling
     * Reduces GPU draw calls significantly
     * Handles wide characters and zero-width combining characters
  2. **Background Region Merging** [terminal_element.rs242-271](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_element.rs#L242-L271>):

     * Collects colored background cells
     * Merges adjacent regions horizontally and vertically
     * Minimizes rectangle count for efficient rendering
  3. **Performance Logging** [terminal_element.rs482-489](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_element.rs#L482-L489>): Tracks layout time and counts for optimization


The batched output is then painted in the paint phase [terminal_element.rs134-155](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_element.rs#L134-L155>) by iterating over the optimized structures.

Sources: [terminal_element.rs322-492](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_element.rs#L322-L492>) [terminal_element.rs242-271](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_element.rs#L242-L271>) [terminal_element.rs134-155](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_element.rs#L134-L155>)

## Terminal Panel Organization

### Panel Structure


**Diagram: Terminal Panel Layout**

The `TerminalPanel` [terminal_panel.rs77-90](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L77-L90>) manages multiple terminal panes:

  * **PaneGroup Center** [terminal_panel.rs79](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L79-L79>): Contains the root pane structure with splits
  * **Active Pane Tracking** [terminal_panel.rs78](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L78-L78>): Tracks which pane has focus
  * **Tab Bar Buttons** [terminal_panel.rs136-226](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L136-L226>): Renders custom tab bar controls for new terminals, splits, and zoom


Sources: [terminal_panel.rs77-90](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L77-L90>) [terminal_panel.rs136-226](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L136-L226>)

### Pane Event Handling

The panel responds to various pane events [terminal_panel.rs333-431](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L333-L431>):

Event| Handling  
---|---  
`ActivateItem`| Serialize panel state  
`RemovedItem`| Serialize panel state  
`Remove`| Close panel if last pane removed, refocus otherwise  
`ZoomIn/ZoomOut`| Propagate zoom state to all panes  
`AddItem`| Call `added_to_pane()` on item, serialize state  
`Split`| Create new pane, optionally clone active terminal  
`Focus`| Update active pane reference  
  
Sources: [terminal_panel.rs333-431](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L333-L431>)

### Terminal Addition Flow


**Diagram: Adding a New Terminal**

The `add_terminal_shell()` method [terminal_panel.rs772-848](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L772-L848>) orchestrates terminal creation:

  1. Resolves working directory from workspace context
  2. Requests terminal from Project
  3. Wraps Terminal in TerminalView
  4. Adds to active pane
  5. Optionally reveals and focuses based on `RevealStrategy`


For tasks, `add_terminal_task()` [terminal_panel.rs850-932](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L850-L932>) follows a similar pattern but passes `TaskState` configuration.

Sources: [terminal_panel.rs772-848](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L772-L848>) [terminal_panel.rs850-932](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L850-L932>)

## Terminal Persistence

### Serialization Structure


**Diagram: Terminal Persistence Schema**

Terminal persistence uses a two-level storage system:

  1. **Panel Structure** [persistence.rs324-357](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence.rs#L324-L357>): `SerializedTerminalPanel` stored in key-value store contains:

     * `SerializedItems`: Either flat list (legacy) or full pane group tree
     * `width`/`height`: Panel dimensions
     * `active_item_id`: Which terminal was active
  2. **Per-Terminal Data** [persistence.rs390-449](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence.rs#L390-L449>): `TerminalDb` domain stores individual terminal state:

     * `workspace_id`: Links to workspace
     * `item_id`: Unique terminal identifier
     * `working_directory`: CWD to restore


Sources: [persistence.rs324-357](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence.rs#L324-L357>) [persistence.rs390-449](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence.rs#L390-L449>)

### Serialization Process

The `serialize_pane_group()` function [persistence.rs27-88](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence.rs#L27-L88>) recursively walks the pane structure:
    
    
    1. For each pane:
       - Collect item IDs (excluding tasks)
       - Mark active item
       - Store pinned count
    
    2. For splits (PaneAxis):
       - Serialize axis direction (horizontal/vertical)
       - Recursively serialize child members
       - Store flex ratios
    

Task terminals are explicitly excluded [persistence.rs67](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence.rs#L67-L67>) since they're one-time executions.

Sources: [persistence.rs27-88](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence.rs#L27-L88>) [persistence.rs67](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence.rs#L67-L67>)

### Deserialization and Restoration


**Diagram: Terminal Panel Restoration**

The `TerminalPanel::load()` method [terminal_panel.rs236-331](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L236-L331>) restores terminals asynchronously:

  1. **Read Serialized State** [terminal_panel.rs242-274](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L242-L274>): Loads `SerializedTerminalPanel` from key-value store
  2. **Deserialize Pane Group** [persistence.rs169-287](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence.rs#L169-L287>): Recursively rebuilds pane structure
  3. **Create Terminals** [persistence.rs289-321](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence.rs#L289-L321>): For each item ID, deserializes `TerminalView` which creates new terminal with saved CWD
  4. **Cleanup** [terminal_panel.rs292-308](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L292-L308>): Removes stale terminal records from database
  5. **Focus Restoration** [terminal_panel.rs311-329](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L311-L329>): Focuses active pane if panel is open


If deserialization fails, a fresh panel is created [terminal_panel.rs275-281](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L275-L281>)

Sources: [terminal_panel.rs236-331](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_panel.rs#L236-L331>) [persistence.rs169-287](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence.rs#L169-L287>) [persistence.rs289-321](<https://github.com/zed-industries/zed/blob/4109c9dd/persistence.rs#L289-L321>)

## Hyperlink Detection and Navigation

The terminal system includes sophisticated hyperlink detection that recognizes URLs and file paths in terminal output.

### Hyperlink Types and Detection


**Diagram: Hyperlink Detection Pipeline**

The `find_from_grid_point()` function [terminal_hyperlinks.rs68-153](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L68-L153>) implements a three-tier detection strategy:

  1. **OSC 8 Hyperlinks** [terminal_hyperlinks.rs75-99](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L75-L99>): Checks if the cell has a hyperlink set via OSC 8 escape sequences (e.g., by programs like `ls --hyperlink=auto`)

  2. **URL Detection** [terminal_hyperlinks.rs101-114](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L101-L114>): Uses `URL_REGEX` [terminal_hyperlinks.rs19](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L19-L19>) to find URLs with protocols: `http://`, `https://`, `git://`, `ssh://`, `file://`, `mailto:`, etc.

  3. **Path Detection** [terminal_hyperlinks.rs115-125](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L115-L125>): Applies user-configured regex patterns to find file paths with optional line/column numbers


Sources: [terminal_hyperlinks.rs68-153](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L68-L153>) [terminal_hyperlinks.rs19](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L19-L19>)

### Path Hyperlink Regex System

Path detection uses configurable regex patterns from `terminal.path_hyperlink_regexes` settings:


**Diagram: Path Hyperlink Detection**

The `path_match()` function [terminal_hyperlinks.rs206-360](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L206-L360>) performs regex-based path detection with these features:

  * **Named Capture Groups** [terminal_hyperlinks.rs320-332](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L320-L332>): Supports `(?P<path>...)`, `(?P<line>...)`, `(?P<column>...)` for extracting file paths with positions
  * **Timeout Protection** [terminal_hyperlinks.rs221-225](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L221-L225>): Prevents excessive regex processing time
  * **Cell-Accurate Matching** [terminal_hyperlinks.rs231-268](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L231-L268>): Handles wide characters and tabs correctly to map byte offsets to grid coordinates
  * **Format Construction** [terminal_hyperlinks.rs301-310](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L301-L310>): Builds `path:line:column` strings for navigation


Sources: [terminal_hyperlinks.rs206-360](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L206-L360>) [terminal_hyperlinks.rs320-332](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L320-L332>)

### URL Sanitization

URLs detected by regex undergo sanitization to remove trailing punctuation that's typically not part of the URL:


The `sanitize_url_punctuation()` function [terminal_hyperlinks.rs155-204](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L155-L204>) implements parentheses balancing to avoid breaking Wikipedia-style URLs like `https://en.wikipedia.org/wiki/Example_(disambiguation)` [terminal_hyperlinks.rs163-186](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L163-L186>)

Sources: [terminal_hyperlinks.rs155-204](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L155-L204>) [terminal_hyperlinks.rs163-186](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L163-L186>)

### Hyperlink Interaction Flow


**Diagram: Hyperlink Interaction**

When the user hovers over terminal content:

  1. `TerminalElement` [terminal_element.rs275-318](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_element.rs#L275-L318>) converts mouse coordinates to grid points
  2. `Terminal::find_hyperlink()` [terminal.rs1434-1472](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal.rs#L1434-L1472>) calls `find_from_grid_point()`
  3. Results are stored as `HoveredWord` [terminal.rs773-777](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal.rs#L773-L777>)
  4. Tooltips display the full hyperlink text
  5. On Cmd+Click, `open_path_like_target()` [terminal_path_like_target.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_path_like_target.rs>) navigates to files or opens URLs


File paths with `file://` protocol are converted to regular paths [terminal_hyperlinks.rs130-145](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L130-L145>) to support line number extraction.

Sources: [terminal_element.rs275-318](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_element.rs#L275-L318>) [terminal.rs1434-1472](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal.rs#L1434-L1472>) [terminal.rs773-777](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal.rs#L773-L777>) [terminal_hyperlinks.rs130-145](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_hyperlinks.rs#L130-L145>)

## Key Actions and Commands

### Terminal Actions

The terminal system defines numerous actions [terminal/terminal.rs75-109](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L75-L109>):

Action| Description| Implementation  
---|---|---  
`Clear`| Clears terminal screen| Resets grid, moves cursor to top [terminal.rs1009-1041](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal.rs#L1009-L1041>)  
`Copy`| Copies selection to clipboard| Extracts selected text via `copy()` method  
`Paste`| Pastes from clipboard| Writes clipboard to PTY [terminal.rs923-932](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal.rs#L923-L932>)  
`ScrollLineUp/Down`| Scrolls by one line| Adjusts display offset  
`ScrollPageUp/Down`| Scrolls by page| Adjusts by viewport height  
`ScrollToTop/Bottom`| Jumps to top/bottom| Sets display offset to extremes  
`ToggleViMode`| Enables vi-mode navigation| Toggles Alacritty vi mode [terminal.rs1063-1076](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal.rs#L1063-L1076>)  
`SelectAll`| Selects all terminal text| Creates full selection range [terminal.rs1078-1081](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal.rs#L1078-L1081>)  
  
Sources: [terminal/terminal.rs75-109](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L75-L109>) [terminal.rs1009-1041](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal.rs#L1009-L1041>) [terminal.rs1063-1076](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal.rs#L1063-L1076>)

### Terminal View Actions

`TerminalView` adds UI-specific actions [terminal_view/terminal_view.rs84-90](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_view/terminal_view.rs#L84-L90>):

  * **SendText** : Sends raw text to terminal [terminal_view/terminal_view.rs696-701](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_view/terminal_view.rs#L696-L701>)
  * **SendKeystroke** : Sends parsed keystroke [terminal_view/terminal_view.rs703-715](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_view/terminal_view.rs#L703-L715>)
  * **RerunTask** : Reruns completed task [terminal_view/terminal_view.rs493-501](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_view/terminal_view.rs#L493-L501>)


Sources: [terminal_view/terminal_view.rs84-90](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_view/terminal_view.rs#L84-L90>) [terminal_view/terminal_view.rs696-715](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal_view/terminal_view.rs#L696-L715>)

## Remote Terminal Support

For remote projects (via SSH or collab), terminals use the remote client to spawn shells on the remote host. The `create_remote_shell()` function [project/terminals.rs564-586](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L564-L586>) builds commands that:

  1. Set `TERM=xterm-256color` for proper terminal emulation
  2. Forward environment variables through the remote client
  3. Execute shell commands on the remote host
  4. Support activation scripts in remote environments


The terminal marks itself as remote [terminal/terminal.rs604](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L604-L604>) which affects:

  * Working directory resolution
  * Shell spawning
  * Environment variable handling


Sources: [project/terminals.rs564-586](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L564-L586>) [terminal/terminal.rs604](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal.rs#L604-L604>)

## Configuration and Settings

Terminal behavior is controlled by `TerminalSettings` [terminal/terminal_settings](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal_settings>):

Setting| Default| Purpose  
---|---|---  
`shell.program`| System shell| Which shell to use  
`shell.args`| None| Additional shell arguments  
`env`| `{}`| Environment variable overrides  
`cursor_shape`| Block| Terminal cursor appearance  
`alternate_scroll`| On| Mouse scrolling in alt screen  
`max_scroll_history_lines`| 10,000| Scrollback buffer size (max 100,000)  
`blinking`| TerminalControlled| Cursor blink behavior  
`detect_venv`| On| Auto-activate Python virtual envs  
  
Task-specific settings:

  * Tasks use maximum scroll history (100,000 lines) [project/terminals.rs518-526](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L518-L526>)
  * Settings are resolved per-worktree [project/terminals.rs65-74](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L65-L74>)


Sources: [terminal/terminal_settings](<https://github.com/zed-industries/zed/blob/4109c9dd/terminal/terminal_settings>) [project/terminals.rs518-526](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L518-L526>) [project/terminals.rs65-74](<https://github.com/zed-industries/zed/blob/4109c9dd/project/terminals.rs#L65-L74>)

Dismiss

Refresh this wiki

This wiki was recently refreshed. Please wait 7 days to refresh again.

### On this page

  * [Terminal and Task Execution](<#terminal-and-task-execution>)
  * [Purpose and Scope](<#purpose-and-scope>)
  * [Terminal Architecture](<#terminal-architecture>)
  * [Core Terminal Entity](<#core-terminal-entity>)
  * [Terminal Creation Flow](<#terminal-creation-flow>)
  * [Terminal Types and Variants](<#terminal-types-and-variants>)
  * [PTY Communication](<#pty-communication>)
  * [Event Flow from PTY to UI](<#event-flow-from-pty-to-ui>)
  * [Writing to PTY](<#writing-to-pty>)
  * [Task Execution Integration](<#task-execution-integration>)
  * [Task Lifecycle in Terminals](<#task-lifecycle-in-terminals>)
  * [Task Spawning Strategy](<#task-spawning-strategy>)
  * [Environment and Shell Resolution](<#environment-and-shell-resolution>)
  * [Terminal View and Rendering](<#terminal-view-and-rendering>)
  * [Terminal View Component](<#terminal-view-component>)
  * [Terminal Rendering Pipeline](<#terminal-rendering-pipeline>)
  * [Terminal Panel Organization](<#terminal-panel-organization>)
  * [Panel Structure](<#panel-structure>)
  * [Pane Event Handling](<#pane-event-handling>)
  * [Terminal Addition Flow](<#terminal-addition-flow>)
  * [Terminal Persistence](<#terminal-persistence>)
  * [Serialization Structure](<#serialization-structure>)
  * [Serialization Process](<#serialization-process>)
  * [Deserialization and Restoration](<#deserialization-and-restoration>)
  * [Hyperlink Detection and Navigation](<#hyperlink-detection-and-navigation>)
  * [Hyperlink Types and Detection](<#hyperlink-types-and-detection>)
  * [Path Hyperlink Regex System](<#path-hyperlink-regex-system>)
  * [URL Sanitization](<#url-sanitization>)
  * [Hyperlink Interaction Flow](<#hyperlink-interaction-flow>)
  * [Key Actions and Commands](<#key-actions-and-commands>)
  * [Terminal Actions](<#terminal-actions>)
  * [Terminal View Actions](<#terminal-view-actions>)
  * [Remote Terminal Support](<#remote-terminal-support>)
  * [Configuration and Settings](<#configuration-and-settings>)
