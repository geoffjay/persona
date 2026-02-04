# Persona

Personas are markdown files that define the identity and communication style of an AI agent. They guide the AI in conversations by establishing expertise, voice, behavioral guidelines, and boundaries.

## File Structure

Personas are stored in the `personas/` directory as markdown files with YAML frontmatter:

```
personas/
  personal-assistant.md
  systems-designer.md
  tech-review-professional.md
```

## Frontmatter Fields

Each persona file must begin with YAML frontmatter between `---` delimiters:

```yaml
---
persona_id: my-persona
avatar_url: https://example.com/avatar.png
knowledgebase_dir: ./my-persona-kb/
---
```

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `persona_id` | string | Unique identifier for the persona. Used internally for routing and state management. Should be lowercase with hyphens (e.g., `personal-assistant`). |

### Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `avatar_url` | string | URL to an avatar image displayed in the persona list and conversations. Supports any image URL. |
| `knowledgebase_dir` | string | Path to a directory containing markdown files for this persona's knowledgebase. Can be relative (resolved from persona file's directory) or absolute. |

## Persona Name

The persona's display name is extracted from the first `# Heading` in the markdown content. If no heading is found, the filename is used with hyphens converted to spaces and title-cased.

Example:
```markdown
---
persona_id: personal-assistant
---

# Personal Assistant

You are a professional executive assistant...
```

This persona will display as "Personal Assistant" in the UI.

## Knowledgebase

Each persona can have an associated knowledgebase - a directory of markdown files containing reference material, notes, and documentation specific to that persona.

### Setting Up a Knowledgebase

1. Create a directory for the knowledgebase files:
   ```bash
   mkdir personas/my-persona-kb/
   ```

2. Add the `knowledgebase_dir` field to your persona's frontmatter:
   ```yaml
   ---
   persona_id: my-persona
   knowledgebase_dir: ./my-persona-kb/
   ---
   ```

3. Add markdown files to the knowledgebase directory:
   ```
   personas/my-persona-kb/
     meeting-notes.md
     project-overview.md
     workflow-preferences.md
   ```

### Knowledgebase Features

- **Viewing**: Knowledgebase entries appear in the Memory page sidebar beneath Berry when a persona has a configured knowledgebase
- **Editing**: Click any entry to open it in a slideout editor with markdown syntax highlighting and line numbers
- **Saving**: Changes can be saved back to disk using the Save button

### Path Resolution

- **Relative paths** (recommended): Resolved from the persona file's parent directory
  ```yaml
  knowledgebase_dir: ./my-persona-kb/
  ```

- **Absolute paths**: Used as-is
  ```yaml
  knowledgebase_dir: /Users/me/knowledgebases/my-persona/
  ```

If the specified directory doesn't exist, a warning is logged and the knowledgebase is disabled for that persona.

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

## Example Persona

```markdown
---
persona_id: code-reviewer
avatar_url: https://gravatar.com/avatar/abc123?d=robohash
knowledgebase_dir: ./code-reviewer-kb/
---

# Code Reviewer

## Core Identity

You are a senior software engineer specializing in code review...

## Communication Style

- Provide constructive feedback
- Reference specific line numbers
- Suggest improvements with examples

## Boundaries

- Focus on code quality, not personal preferences
- Respect existing architectural decisions
```
