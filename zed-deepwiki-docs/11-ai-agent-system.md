<!-- Source: https://deepwiki.com/zed-industries/zed/11-ai-agent-system -->

# 11 Ai Agent System

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

# AI Agent System

Relevant source files

  * [crates/acp_thread/Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/Cargo.toml>)
  * [crates/acp_thread/src/acp_thread.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/acp_thread.rs>)
  * [crates/acp_thread/src/connection.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/connection.rs>)
  * [crates/acp_thread/src/mention.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/mention.rs>)
  * [crates/agent_servers/Cargo.toml](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/Cargo.toml>)
  * [crates/agent_servers/src/acp.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/acp.rs>)
  * [crates/agent_servers/src/agent_servers.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/agent_servers.rs>)
  * [crates/agent_servers/src/claude.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/claude.rs>)
  * [crates/agent_servers/src/custom.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/custom.rs>)
  * [crates/agent_servers/src/e2e_tests.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/e2e_tests.rs>)
  * [crates/agent_servers/src/gemini.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/gemini.rs>)
  * [crates/agent_ui/src/acp/entry_view_state.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/acp/entry_view_state.rs>)
  * [crates/agent_ui/src/acp/message_editor.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/acp/message_editor.rs>)
  * [crates/agent_ui/src/acp/thread_view.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/acp/thread_view.rs>)
  * [crates/agent_ui/src/agent_panel.rs](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/agent_panel.rs>)


The AI Agent System provides AI-powered assistance within Zed through external agent servers that communicate via the Agent Communication Protocol (ACP). This system enables users to interact with various AI agents (Claude Code, Gemini CLI, Codex, custom agents) to perform code-related tasks, answer questions, and execute tools with project context.

For information about the text-based assistant threads (non-ACP), see [Legacy Agent Thread System](</zed-industries/zed/11.6-legacy-agent-thread-system>).

## Architecture Overview

The AI Agent System is organized into several layers:

**Agent Communication Protocol (ACP) Layer** : Defines the stdio-based protocol for communicating with external agent servers using JSON-RPC-style messages.

**Agent Connection Layer** : Abstracts different agent implementations behind a common `AgentConnection` trait, allowing multiple agent servers to work interchangeably.

**Thread Management Layer** : The `AcpThread` entity manages conversation state, message history, and coordinates tool execution.

**UI Layer** : `AgentPanel` and `AcpThreadView` provide the user interface for interacting with agents, displaying messages, tool calls, and managing context.

**Tool System** : Enables agents to perform actions like file operations, terminal execution, and code edits through a permission-based workflow.

**Mention System** : Allows users to attach rich context (files, symbols, threads, URLs) to messages using `@` mentions.

### High-Level Architecture


**Sources:** [crates/agent_ui/src/agent_panel.rs1-800](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/agent_panel.rs#L1-L800>) [crates/agent_ui/src/acp/thread_view.rs1-400](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/acp/thread_view.rs#L1-L400>) [crates/acp_thread/src/acp_thread.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/acp_thread.rs#L1-L200>) [crates/acp_thread/src/connection.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/connection.rs#L1-L100>) [crates/agent_servers/src/acp.rs1-250](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/acp.rs#L1-L250>)

## Agent Communication Protocol (ACP)

### Protocol Overview

The Agent Communication Protocol is a stdio-based protocol where Zed spawns an external agent server process and communicates via JSON messages over stdin/stdout. The protocol uses a request-response pattern with streaming updates for long-running operations.

**Key Protocol Concepts:**

  * **Sessions** : Each conversation thread corresponds to a session with a unique `SessionId`
  * **Prompts** : User messages sent to the agent with `PromptRequest`
  * **Session Updates** : Streaming responses from the agent (assistant messages, tool calls, etc.)
  * **Stop Reasons** : Indicates why the agent stopped generating (EndTurn, Cancelled, etc.)


### Connection Lifecycle


**Sources:** [crates/agent_servers/src/acp.rs56-235](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/acp.rs#L56-L235>)

### AcpConnection Implementation

`AcpConnection` implements the `AgentConnection` trait and manages communication with external agent servers via stdio:

**Key Components:**

  * `connection: Rc<acp::ClientSideConnection>` \- Protocol client from `agent-client-protocol` crate
  * `sessions: Rc<RefCell<HashMap<SessionId, AcpSession>>>` \- Active sessions by ID
  * `child: smol::process::Child` \- The spawned agent server process
  * `io_task: Task<Result<(), acp::Error>>` \- Background task handling IO
  * `agent_capabilities: acp::AgentCapabilities` \- Server capabilities from initialization


**Session State:**


**Sources:** [crates/agent_servers/src/acp.rs32-48](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/acp.rs#L32-L48>) [crates/agent_servers/src/acp.rs81-225](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/acp.rs#L81-L225>)

### Protocol Message Types

Message Type| Direction| Purpose  
---|---|---  
`InitializeRequest/Response`| Zed → Agent, Agent → Zed| Handshake and capability negotiation  
`NewSessionRequest/Response`| Zed → Agent, Agent → Zed| Create a new conversation session  
`PromptRequest/Response`| Zed → Agent, Agent → Zed| Send user message and receive response  
`SessionUpdate`| Agent → Zed| Streaming updates (messages, tool calls, thinking)  
`SetSessionModelRequest`| Zed → Agent| Change model for a session  
`SetSessionModeRequest`| Zed → Agent| Change mode for a session  
`CancelSessionRequest`| Zed → Agent| Cancel ongoing generation  
`AuthenticateRequest/Response`| Zed → Agent, Agent → Zed| Perform authentication  
  
**Sources:** [crates/agent_servers/src/acp.rs176-198](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/acp.rs#L176-L198>)

## Agent UI and Thread Management

### AcpThread Entity

`AcpThread` is the central entity managing conversation state. It maintains message history, coordinates tool execution, handles authorization workflows, and emits events for UI updates.

**Core State:**


**Entry Types:**

  * `AgentThreadEntry::UserMessage` \- User messages with content blocks and mentions
  * `AgentThreadEntry::AssistantMessage` \- Agent responses (text, thinking)
  * `AgentThreadEntry::ToolCall` \- Tool invocations with authorization state


**Sources:** [crates/acp_thread/src/acp_thread.rs40-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/acp_thread.rs#L40-L200>)

### Thread State Machine


**Sources:** [crates/acp_thread/src/acp_thread.rs600-900](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/acp_thread.rs#L600-L900>)

### AcpThreadView

`AcpThreadView` is the UI component that renders the conversation. It manages:

  * Displaying message history with scrolling
  * Message editor for user input
  * Tool call cards with expand/collapse
  * Loading states and error handling
  * Model/mode selectors


**View State Management:**


**Sources:** [crates/agent_ui/src/acp/thread_view.rs264-446](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/acp/thread_view.rs#L264-L446>) [crates/agent_ui/src/acp/entry_view_state.rs23-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/acp/entry_view_state.rs#L23-L200>)

### Message Editor

`MessageEditor` provides the input interface with support for:

  * Multi-line editing with auto-height
  * `@` mention completions (files, symbols, threads, URLs)
  * Slash command completions (agent-specific commands)
  * Image pasting
  * Clipboard integration with file context


**Mention Completion Flow:**


**Sources:** [crates/agent_ui/src/acp/message_editor.rs39-204](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/acp/message_editor.rs#L39-L204>) [crates/agent_ui/src/acp/message_editor.rs355-456](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/acp/message_editor.rs#L355-L456>)

### AgentPanel

`AgentPanel` is the dock panel that hosts agent threads. It manages:

  * Multiple thread types (ACP threads, text threads)
  * Thread history navigation
  * Agent selection (Native, Claude Code, Gemini, Codex, Custom)
  * Configuration UI


**Active View Types:**


**Sources:** [crates/agent_ui/src/agent_panel.rs211-289](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/agent_panel.rs#L211-L289>) [crates/agent_ui/src/agent_panel.rs418-700](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/agent_panel.rs#L418-L700>)

## Agent Connection and Implementations

### AgentConnection Trait

The `AgentConnection` trait abstracts different agent implementations:

**Core Methods:**

  * `new_thread()` \- Create a new conversation thread (returns `Entity<AcpThread>`)
  * `prompt()` \- Send a user message and receive streaming response
  * `cancel()` \- Cancel ongoing generation
  * `auth_methods()` \- List authentication methods
  * `authenticate()` \- Perform authentication


**Optional Capabilities:**

  * `model_selector()` \- Support for model selection (returns `Rc<dyn AgentModelSelector>`)
  * `session_modes()` \- Support for session modes (returns `Rc<dyn AgentSessionModes>`)
  * `truncate()` \- Support for editing history (returns `Rc<dyn AgentSessionTruncate>`)
  * `set_title()` \- Support for setting thread title
  * `telemetry()` \- Support for telemetry data


**Sources:** [crates/acp_thread/src/connection.rs22-96](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/connection.rs#L22-L96>)

### Agent Server Implementations

#### ClaudeCode

Claude Code is Anthropic's ACP-compatible agent server.

**Configuration:**

  * Default mode setting: `agent_servers.claude.default_mode`
  * Default model setting: `agent_servers.claude.default_model`


**Connection:**

  * Looks up agent in `AgentServerStore` by name `CLAUDE_CODE_NAME`
  * Spawns external `claude-code` process
  * Passes proxy environment variables


**Sources:** [crates/agent_servers/src/claude.rs1-122](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/claude.rs#L1-L122>)

#### Gemini

Gemini CLI is Google's command-line agent interface.

**Configuration:**

  * Loads `GEMINI_API_KEY` from `GoogleLanguageModelProvider`
  * Sets `SURFACE=zed` environment variable
  * Passes proxy settings


**Sources:** [crates/agent_servers/src/gemini.rs1-80](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/gemini.rs#L1-L80>)

#### Codex

OpenAI's Codex agent server.

**Configuration:**

  * Default mode setting: `agent_servers.codex.default_mode`
  * Default model setting: `agent_servers.codex.default_model`


**Sources:** [crates/agent_servers/src/codex.rs1-122](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/codex.rs#L1-L122>)

#### Custom Agents

Users can define custom ACP-compatible agents through settings or extensions.

**Configuration:**


**Sources:** [crates/agent_servers/src/custom.rs12-152](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/custom.rs#L12-L152>)

### Agent Server Comparison

Feature| ClaudeCode| Gemini| Codex| Custom  
---|---|---|---|---  
Icon| AiClaude| AiGemini| AiOpenAi| Terminal  
Model Selection| ✓| ✓| ✓| Varies  
Session Modes| ✓| ✓| ✓| Varies  
Tool Use| ✓| ✓| ✓| Varies  
Authentication| OAuth| API Key| API Key| Varies  
  
**Sources:** [crates/agent_servers/src/claude.rs24-74](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/claude.rs#L24-L74>) [crates/agent_servers/src/gemini.rs14-79](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/gemini.rs#L14-L79>) [crates/agent_servers/src/codex.rs14-74](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/codex.rs#L14-L74>) [crates/agent_servers/src/custom.rs23-102](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/custom.rs#L23-L102>)

## Tool System

Agents can invoke tools to perform actions. Zed implements a permission-based system where potentially dangerous operations require user authorization.

### Tool Call Lifecycle


**Sources:** [crates/acp_thread/src/acp_thread.rs900-1100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/acp_thread.rs#L900-L1100>)

### Tool Call Data Structure


**Sources:** [crates/acp_thread/src/acp_thread.rs175-460](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/acp_thread.rs#L175-L460>)

### Built-in Tool Types

**File Operations:**

  * Read file contents
  * Write/edit files (produces `Diff` entities)
  * List directory contents


**Terminal Execution:**

  * Run shell commands
  * Produces `Terminal` entities with output capture
  * Requires authorization for most commands


**Code Intelligence:**

  * Get diagnostics
  * Format code
  * Search workspace


**Context Servers:**

  * MCP (Model Context Protocol) server integration
  * Dynamic tool loading from extensions


**Sources:** [crates/acp_thread/src/acp_thread.rs592-670](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/acp_thread.rs#L592-L670>) [crates/acp_thread/src/terminal.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/terminal.rs#L1-L200>) [crates/acp_thread/src/diff.rs1-200](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/diff.rs#L1-L200>)

### Authorization Workflow

`AcpThread` maintains authorization state and provides methods for user interaction:


**Permission Policies:**

  * `AllowOnce` \- Approve this single tool call
  * `AllowAlways` \- Remember decision for future calls of this tool
  * Auto-approval based on tool safety level and user settings


**Sources:** [crates/acp_thread/src/acp_thread.rs1100-1300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/acp_thread.rs#L1100-L1300>)

### Diff and Terminal Entities

**Diff Entity:** Represents a file modification proposed by the agent.


Diffs are rendered in the UI with syntax highlighting and allow users to:

  * Review changes
  * Accept/reject individual hunks
  * Apply all changes at once


**Terminal Entity:** Represents terminal command execution.


Terminal output is captured and displayed in the UI. Users can:

  * View live output
  * Send additional input
  * Move terminals to background when complete


**Sources:** [crates/acp_thread/src/diff.rs1-300](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/diff.rs#L1-L300>) [crates/acp_thread/src/terminal.rs1-250](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/terminal.rs#L1-L250>)

## Mention System and Context

The mention system allows users to attach rich context to messages using `@` syntax. Each mention is represented by a `MentionUri` and resolved to actual content.

### MentionUri Types


**URI Schemes:**

  * `file://` \- Local file system paths
  * `zed://` \- Zed-specific resources (threads, rules, symbols)
  * `http://`, `https://` \- Web URLs for fetching


**Sources:** [crates/acp_thread/src/mention.rs15-294](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/mention.rs#L15-L294>)

### MentionSet Management

`MentionSet` tracks all mentions in a message editor and resolves them to content:


**Resolution Process:**

  1. User types `@` and selects a completion
  2. `MentionSet.confirm_mention_completion()` inserts a crease in the editor
  3. Background task resolves the mention to actual content
  4. When sending, `MentionSet.contents()` gathers all resolved mentions


**Sources:** [crates/agent_ui/src/mention_set.rs1-500](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/mention_set.rs#L1-L500>)

### Content Block Conversion

When sending to the agent, mentions are converted to `acp::ContentBlock` based on capabilities:

**With Embedded Context Support:**


**Without Embedded Context Support:**


The agent server is responsible for resolving resource links if embedded context is not supported.

**Sources:** [crates/agent_ui/src/acp/message_editor.rs355-456](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/acp/message_editor.rs#L355-L456>)

### Mention URI Parsing and Serialization

`MentionUri::parse()` converts URI strings to structured types:


**Line Number Convention:**

  * URIs use 1-based line numbers (L1:10)
  * Internal representation uses 0-based line numbers (0..=9)


**Sources:** [crates/acp_thread/src/mention.rs52-294](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/acp_thread/src/mention.rs#L52-L294>)

### Context Capabilities

Agents declare their context capabilities during initialization:


The UI adapts based on capabilities:

  * Image pasting only enabled if `image` is true
  * Embedded context sent as full text if `embedded_context` is true
  * Otherwise, resource links are sent for agent to resolve


**Sources:** [crates/agent_servers/src/acp.rs227-229](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_servers/src/acp.rs#L227-L229>)

## Legacy Agent Thread System

The legacy system refers to the text-based assistant threads that predate ACP. These use a different architecture:

**Key Differences:**

  * Not based on external processes
  * Use language model API directly (e.g., Anthropic API, OpenAI API)
  * Managed through `TextThread` and `TextThreadEditor` entities
  * Use slash commands for tools instead of ACP tool protocol


**Components:**

  * `TextThread` \- Conversation state
  * `TextThreadEditor` \- Editor-based UI
  * `assistant_text_thread` crate - Core implementation
  * Integrated directly with `LanguageModelRegistry`


This system is still available and accessible through `AgentPanel` by selecting the "Text Thread" option.

**Sources:** [crates/agent_ui/src/agent_panel.rs569-602](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/agent_panel.rs#L569-L602>) [crates/agent_ui/src/text_thread_editor.rs1-100](<https://github.com/zed-industries/zed/blob/4109c9dd/crates/agent_ui/src/text_thread_editor.rs#L1-L100>)

Dismiss

Refresh this wiki

This wiki was recently refreshed. Please wait 7 days to refresh again.

### On this page

  * [AI Agent System](<#ai-agent-system>)
  * [Architecture Overview](<#architecture-overview>)
  * [High-Level Architecture](<#high-level-architecture>)
  * [Agent Communication Protocol (ACP)](<#agent-communication-protocol-acp>)
  * [Protocol Overview](<#protocol-overview>)
  * [Connection Lifecycle](<#connection-lifecycle>)
  * [AcpConnection Implementation](<#acpconnection-implementation>)
  * [Protocol Message Types](<#protocol-message-types>)
  * [Agent UI and Thread Management](<#agent-ui-and-thread-management>)
  * [AcpThread Entity](<#acpthread-entity>)
  * [Thread State Machine](<#thread-state-machine>)
  * [AcpThreadView](<#acpthreadview>)
  * [Message Editor](<#message-editor>)
  * [AgentPanel](<#agentpanel>)
  * [Agent Connection and Implementations](<#agent-connection-and-implementations>)
  * [AgentConnection Trait](<#agentconnection-trait>)
  * [Agent Server Implementations](<#agent-server-implementations>)
  * [ClaudeCode](<#claudecode>)
  * [Gemini](<#gemini>)
  * [Codex](<#codex>)
  * [Custom Agents](<#custom-agents>)
  * [Agent Server Comparison](<#agent-server-comparison>)
  * [Tool System](<#tool-system>)
  * [Tool Call Lifecycle](<#tool-call-lifecycle>)
  * [Tool Call Data Structure](<#tool-call-data-structure>)
  * [Built-in Tool Types](<#built-in-tool-types>)
  * [Authorization Workflow](<#authorization-workflow>)
  * [Diff and Terminal Entities](<#diff-and-terminal-entities>)
  * [Mention System and Context](<#mention-system-and-context>)
  * [MentionUri Types](<#mentionuri-types>)
  * [MentionSet Management](<#mentionset-management>)
  * [Content Block Conversion](<#content-block-conversion>)
  * [Mention URI Parsing and Serialization](<#mention-uri-parsing-and-serialization>)
  * [Context Capabilities](<#context-capabilities>)
  * [Legacy Agent Thread System](<#legacy-agent-thread-system>)
