# Persona

A framework for stateful AI-assisted conversations with persistent memory. Combines Claude Code personas with Berry's
vector-searchable memory to enable context-aware mentoring and professional development guidance across multiple
sessions.

## Overview

This project demonstrates how persistent memory can augment specialized AI personas to create continuous, accountable
collaboration. Rather than starting each AI session from scratch, Persona maintains context about previous discussions,
decisions, assignments, and follow-ups.

### Key Features

- **Custom AI Personas**: System prompts that transform Claude into specialized roles (e.g., Staff Trajectory Mentor)
- **Persistent Memory**: Vector-searchable context via Berry MCP integration
- **Session Continuity**: Hooks that automatically retrieve prior context at session start
- **Assignment Tracking**: Structured memory types for questions, requests, and information

## Prerequisites

- One of the following:
  - [Claude Code](https://github.com/anthropics/claude-code)
  - [OpenCode](https://opencode.ai)
- [Berry](https://github.com/geoffjay/berry) - Memory management with MCP interface
- [asdf](https://asdf-vm.com/) - Runtime version manager
- [direnv](https://direnv.net/) - Automatic environment loading
- [direnv extensions](https://github.com/geoffjay/direnv-extensions) - Helpers for direnv

## Installation

```bash
# Clone the repository
git clone git@github.com:geoffjay/persona.git
cd persona

# Install runtimes
asdf install

# Install direnv extensions
git clone https://github.com/geoffjay/direnv-extensions.git ~/.config/direnv

# Allow direnv
direnv allow

# Configure environment
cp .env.example .env
# Edit .env with your Chroma credentials
```

## Usage

### Starting a Mentor Session

#### Claude Code

The project includes a shell alias for quick access:

```bash
cc-mentor-staff "What should I focus on for my Staff trajectory?"
```

Or explicitly specify the persona:

```bash
claude --system-prompt-file ./.claude/personas/staff-trajectory-mentor.md
```

#### OpenCode

The personas are used in an `opencode` session using the `--agent` flag:

```bash
opencode --agent staff-mentor
```

### Memory Operations

Store memories from the CLI:

```bash
# Log an assignment
berry remember "Review team's technical debt backlog" --type request --tags "assignment"

# Save a question for later discussion
berry remember "How do I measure Staff-level impact?" --type question

# Search prior context
berry search "session-summary mentor"

# View specific memory
berry recall <memory-id>

# Remove outdated memory
berry forget <memory-id>
```

### Session Workflow

1. **Start**: Launch a session with `cc-mentor-staff` or `opencode --agent`
2. **Context Load**: The session hook automatically searches for prior assignments and context
3. **Accountability**: If pending assignments exist, the mentor asks for status before proceeding
4. **Work**: Engage in context-aware discussion
5. **Persist**: Key decisions, new assignments, and questions are stored in Berry

## Configuration

### Berry MCP

#### Claude Code (`.mcp.json`)

Configure the Berry memory server for Claude Code integration:

```json
{
  "mcpServers": {
    "berry": {
      "command": "berry",
      "args": ["mcp"]
    }
  }
}
```

#### OpenCode (`.opencode/opencode.jsonc`)

Configure the Berry memory server for OpenCode integration:

```json
{
  "mcp": {
    "berry": {
      "type": "local",
      "command": ["berry", "mcp"],
      "enabled": true
    }
  }
}
```

### Session Hook (`.claude/hooks/mentor-session-start.sh`)

Injects the memory protocol at session start:

- Instructs Claude to search for prior context
- Defines memory persistence patterns
- Documents available Berry tools and CLI commands

### Environment (`.envrc`)

Sets up the development environment:

- Loads `.env` secrets
- Adds bun to PATH
- Creates the `cc-mentor-staff` alias

## Memory Tagging Conventions

Use consistent tags for searchability:

| Tag               | Purpose                              |
| ----------------- | ------------------------------------ |
| `mentor`          | All mentor session content           |
| `staff`           | Staff trajectory related             |
| `assignment`      | Actionable items with accountability |
| `session-summary` | End-of-session recaps                |

## Creating New Personas

Use the included command to create additional personas:

```bash
claude /create-persona
```

This guides you through defining:

- Identity and expertise
- Voice and communication style
- Behavioral guidelines
- Interaction patterns
- Boundaries and scope

## Dependencies

### Runtimes (via asdf)

- bun 1.3.5
- nodejs 24.2.0
- uv 0.7.19

### Services

- **Berry** - Memory management (MCP server)
- **Chroma** - Vector database backend (cloud or local)
- **Ollama** (optional) - Local LLM inference

## License

MIT
