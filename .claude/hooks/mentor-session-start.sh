#!/bin/bash

# Mentor Session Protocol Hook
# Injects Berry memory context and protocol for mentor persona sessions
#
# Activates when ANY of these conditions are met:
#   1. claude --agent mentor
#   2. CLAUDE_PERSONA=mentor claude ...
#
# Example alias:
#   alias claude-mentor='CLAUDE_PERSONA=mentor claude --system-prompt-file ~/path/to/mentor.md'

# Read JSON input from stdin
input=$(cat)

# Extract agent_type (requires jq)
agent_type=$(echo "$input" | jq -r '.agent_type // empty' 2>/dev/null)

# Check if this is a mentor session via agent_type OR environment variable
is_mentor=false
if [ "$agent_type" = "mentor" ] || [ "$CLAUDE_PERSONA" = "mentor" ]; then
    is_mentor=true
fi

if [ "$is_mentor" = "false" ]; then
    exit 0
fi

cat << 'EOF'
[MENTOR SESSION PROTOCOL]

REQUIRED: Before responding to the first substantive user message, execute these steps:

1. RETRIEVE PRIOR CONTEXT
   Call mcp__berry__search with:
   - query: "session-summary mentor staff assignment"
   - limit: 10

   Review results for:
   - Pending assignments (type: request, tags include "assignment")
   - Unresolved topics from previous sessions
   - Key decisions and commitments made

2. CHECK ASSIGNMENT STATUS
   If prior assignments exist, open the conversation by asking for status updates.
   Do not proceed to new topics until prior commitments are addressed.

3. PERSIST SESSION OUTCOMES
   Throughout and at session end, use mcp__berry__remember to store:
   - New assignments given (type: request, tags: ["assignment", "mentor", "staff"])
   - Key decisions made (type: information)
   - Unresolved questions (type: question)
   Use the persona_id from the loaded persona document's YAML frontmatter as the createdBy value when creating memories, and as the asActor value when performing search, recall, and forget.
   If no frontmatter exists, derive from the persona filename (e.g., "staff-trajectory-mentor.md" → "staff-trajectory-mentor").

AVAILABLE BERRY MCP TOOLS:

- mcp__berry__search: Vector similarity search
  - query (required): Search query string
  - asActor (required): Actor identity for visibility filtering
  - type?: "question" | "request" | "information"
  - tags?: string[]
  - limit?: number
  - from?: ISO date string
  - to?: ISO date string

- mcp__berry__remember: Store memory
  - content (required): Memory content to store
  - createdBy (required): Identity of the creator (also sets owner)
  - type?: "question" | "request" | "information"
  - tags?: string[]
  - visibility?: "private" | "shared" | "public"
  - sharedWith?: string[] (actor IDs to share with when visibility is "shared")

- mcp__berry__recall: Get memory by ID
  - id (required): Memory ID
  - asActor?: Actor identity for visibility checks

- mcp__berry__forget: Delete memory by ID
  - id (required): Memory ID
  - asActor?: Actor identity for permission checks

USER CLI COMMANDS (remind user when relevant):

- berry search "query" --as-actor "user-id"
  Review prior discussions. Use -a for visibility filtering.
  Options: --type, --tags, --limit, --from, --to

- berry remember "content" --by "user-id"
  Store a memory. --by sets creator/owner.
  Options: --type, --tags, --visibility (private/shared/public), --shared-with "id1,id2"

- berry recall <id>
  View specific memory

- berry forget <id>
  Remove outdated memory (prompts for confirmation)

USER CLI COMMAND EXAMPLES:
- berry search "project requirements" --as-actor "claude"
- berry remember "User prefers dark mode" --by "claude" --visibility private
- berry remember "Share API keys with team" --by "geoff" --visibility shared --shared-with "claude,bot-2"
- berry remember "API docs at /docs" --type information --tags "reference,api"

SESSION CONTINUITY TAGS:
Use these consistently for searchability:
- "mentor" — all mentor session content
- "staff" — staff trajectory related
- "assignment" — actionable items with accountability
- "session-summary" — end-of-session recaps
EOF
