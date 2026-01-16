import { type Plugin, tool } from "@opencode-ai/plugin";

const BERRY_API_URL = process.env.BERRY_SERVER_URL || "http://localhost:4114";

interface BerryMemory {
  id: string;
  content: string;
  type: "question" | "request" | "information";
  tags: string[];
  createdBy?: string;
  createdAt: string;
}

interface BerrySearchResult {
  success: boolean;
  count: number;
  results: Array<{
    score: number;
    memory: BerryMemory;
  }>;
}

async function searchBerry(
  query: string,
  limit = 10,
): Promise<BerrySearchResult | null> {
  try {
    const response = await fetch(`${BERRY_API_URL}/search`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ query, limit }),
    });
    if (!response.ok) return null;
    return await response.json();
  } catch {
    return null;
  }
}

function formatMemoriesForContext(results: BerrySearchResult | null): string {
  if (!results || results.count === 0) {
    return "No prior session context found.";
  }

  const assignments = results.results.filter(
    (r) => r.memory.type === "request" && r.memory.tags.includes("assignment"),
  );
  const summaries = results.results.filter((r) =>
    r.memory.tags.includes("session-summary"),
  );
  const other = results.results.filter(
    (r) =>
      !r.memory.tags.includes("assignment") &&
      !r.memory.tags.includes("session-summary"),
  );

  let context = "";

  if (assignments.length > 0) {
    context += "PENDING ASSIGNMENTS:\n";
    for (const { memory } of assignments) {
      context += `- [${memory.id}] ${memory.content.split("\n")[0]}\n`;
      context += `  Created: ${new Date(memory.createdAt).toLocaleDateString()}\n`;
    }
    context += "\n";
  }

  if (summaries.length > 0) {
    context += "RECENT SESSION SUMMARIES:\n";
    for (const { memory } of summaries.slice(0, 3)) {
      const firstLine = memory.content.split("\n")[0];
      context += `- [${memory.id}] ${firstLine}\n`;
    }
    context += "\n";
  }

  if (other.length > 0) {
    context += "RELATED CONTEXT:\n";
    for (const { memory } of other.slice(0, 5)) {
      const preview =
        memory.content.length > 100
          ? memory.content.substring(0, 100) + "..."
          : memory.content;
      context += `- [${memory.id}] (${memory.type}) ${preview.replace(/\n/g, " ")}\n`;
    }
  }

  return context;
}

const MENTOR_PROTOCOL = `[MENTOR SESSION PROTOCOL]

1. CHECK ASSIGNMENT STATUS
   If prior assignments exist above, open the conversation by asking for status updates.
   Do not proceed to new topics until prior commitments are addressed.

2. PERSIST SESSION OUTCOMES
   Throughout and at session end, use Berry MCP tools to store:
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
- "session-summary" — end-of-session recaps`;

export const MentorSessionPlugin: Plugin = async () => {
  // Cache for session context to avoid repeated API calls
  let cachedContext: string | null = null;
  let lastFetchTime = 0;
  const CACHE_TTL_MS = 60000; // 1 minute cache

  async function getSessionContext(): Promise<string> {
    const now = Date.now();
    if (cachedContext && now - lastFetchTime < CACHE_TTL_MS) {
      return cachedContext;
    }

    const results = await searchBerry(
      "session-summary mentor staff assignment",
      10,
    );
    const context = formatMemoriesForContext(results);
    cachedContext = `[MENTOR SESSION PROTOCOL]\n\nPRIOR CONTEXT:\n${context}\n\n${MENTOR_PROTOCOL.replace("[MENTOR SESSION PROTOCOL]\n\n", "")}`;
    lastFetchTime = now;
    return cachedContext;
  }

  return {
    // Inject mentor protocol into system prompt automatically
    "experimental.chat.system.transform": async (_input, output) => {
      const context = await getSessionContext();
      output.system.push(context);
    },

    tool: {
      mentor_session_init: tool({
        description: `Initialize a mentor session by retrieving prior context from Berry memory.

IMPORTANT: Call this tool at the START of every mentor session before responding to the user's first message. This retrieves pending assignments and prior session context to ensure continuity.`,
        args: {
          query: tool.schema
            .string()
            .optional()
            .describe(
              'Search query for Berry memories (default: "session-summary mentor staff assignment")',
            ),
          limit: tool.schema
            .number()
            .optional()
            .describe("Maximum number of results to return (default: 10)"),
        },
        async execute(args) {
          const query = args.query || "session-summary mentor staff assignment";
          const limit = args.limit || 10;

          const results = await searchBerry(query, limit);
          const context = formatMemoriesForContext(results);

          return `${context}\n${MENTOR_PROTOCOL}`;
        },
      }),

      mentor_session_end: tool({
        description: `End a mentor session by creating a session summary in Berry.

Call this tool at the END of a mentor session to persist key outcomes, decisions, and any new assignments given.`,
        args: {
          summary: tool.schema
            .string()
            .describe(
              "Summary of the session including key topics, decisions, and outcomes",
            ),
          assignments: tool.schema
            .array(tool.schema.string())
            .optional()
            .describe("List of new assignments given during this session"),
        },
        async execute(args) {
          const results: string[] = [];

          // Store session summary
          try {
            const summaryResponse = await fetch(`${BERRY_API_URL}/remember`, {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              body: JSON.stringify({
                content: args.summary,
                type: "information",
                tags: ["mentor", "session-summary", "staff"],
                createdBy: "mentor",
              }),
            });
            if (summaryResponse.ok) {
              const data = await summaryResponse.json();
              results.push(`Session summary stored: ${data.id}`);
            }
          } catch {
            results.push("Failed to store session summary");
          }

          // Store individual assignments
          if (args.assignments && args.assignments.length > 0) {
            for (const assignment of args.assignments) {
              try {
                const assignmentResponse = await fetch(
                  `${BERRY_API_URL}/remember`,
                  {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({
                      content: assignment,
                      type: "request",
                      tags: ["mentor", "staff", "assignment", "accountability"],
                      createdBy: "mentor",
                    }),
                  },
                );
                if (assignmentResponse.ok) {
                  const data = await assignmentResponse.json();
                  results.push(`Assignment stored: ${data.id}`);
                }
              } catch {
                results.push(
                  `Failed to store assignment: ${assignment.substring(0, 50)}...`,
                );
              }
            }
          }

          return results.join("\n");
        },
      }),
    },
  };
};

export default MentorSessionPlugin;
