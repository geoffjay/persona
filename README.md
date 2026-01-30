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

## Getting Started

### Prerequisites

- [OpenCode](https://opencode.ai)
- [Berry](https://github.com/geoffjay/berry) - Memory management with MCP interface

### Installation

There are multiple ways to install most of the dependencies, this is just one example for macOS only:

```bash
brew install opencode
brew install geoffjay/tap/berry
brew install geoffjay/tap/persona
```

### Setup

#### Berry

The persona project uses Berry to manage context, and Berry requires a Chroma database to store the context. You can
either use a local database or a cloud-hosted database, using a cloud database is probably the easiest way to get
started with a persistent database. This can be created at [trychroma.com](https://trychroma.com).

```bash
brew services start geoffjay/tap/berry
curl http://localhost:4114/health
```

#### OpenCode

The persona user interface only supports OpenCode at this time, for it to work you need to authenticate with one of the
OpenCode providers. Instructions on doing this are available at [opencode.ai](https://opencode.ai/docs/providers/), but
typically this only involves executing the command `opencode auth login` and following the prompts for the desired
provider.

### Session Hook

For Claude Code use this is located at `.claude/hooks/mentor-session-start.sh`, and for OpenCode at
`.opencode/plugin/mentor-session-start.ts`.

Injects the memory protocol at session start:

- Instructs the agent to search for prior context
- Defines memory persistence patterns
- Documents available Berry tools and CLI commands

## Memory Tagging Conventions

Use consistent tags for searchability:

| Tag               | Purpose                              |
| ----------------- | ------------------------------------ |
| `mentor`          | All mentor session content           |
| `staff`           | Staff trajectory related             |
| `assignment`      | Actionable items with accountability |
| `session-summary` | End-of-session recaps                |

## License

MIT
