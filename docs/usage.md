# Usage

> [!NOTE]
> The persona project started out as a bunch of prompt files to test out system prompts for use with tasks that were not
> just coding. This is still relevant, but now that the focus is on use with the UI provided it's less likely to be
> running thes directly.

## Starting a Mentor Session

### Claude Code

The project includes a shell alias for quick access:

```bash
cc-mentor-staff "What should I focus on for my Staff trajectory?"
```

Or explicitly specify the persona:

```bash
claude --system-prompt-file ./.claude/personas/staff-trajectory-mentor.md
```

### OpenCode

The personas are used in an `opencode` session using the `--agent` flag:

```bash
opencode --agent staff-mentor
```

## Memory Operations

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

## Session Workflow

1. **Start**: Launch a session with `cc-mentor-staff` or `opencode --agent`
2. **Context Load**: The session hook automatically searches for prior assignments and context
3. **Accountability**: If pending assignments exist, the mentor asks for status before proceeding
4. **Work**: Engage in context-aware discussion
5. **Persist**: Key decisions, new assignments, and questions are stored in Berry
