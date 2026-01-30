# Development

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

## Dependencies

### Runtimes (via asdf)

- bun 1.3.5
- nodejs 24.2.0
- uv 0.7.19

### Environment (`.envrc`)

Sets up the development environment:

- Loads `.env` secrets
- Adds bun to PATH
- Creates the `cc-mentor-staff` alias

### Services

Persona uses the following services:

- **Berry** - Memory management (MCP server)
- **Chroma** - Vector database backend (cloud or local)
- **Ollama** (optional) - Local LLM inference

In development how you use these services is up to you.

## Configuration

### Berry MCP

Use of the Berry MCP server is optional, but recommended for context-aware conversations.

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
