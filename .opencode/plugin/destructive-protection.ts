import type { Plugin } from "@opencode-ai/plugin";

/**
 * Destructive command patterns that should be blocked or warned about.
 * These patterns match dangerous operations that could cause data loss.
 */
const DESTRUCTIVE_PATTERNS = [
  // File/directory deletion
  { pattern: /\brm\s+(-[a-zA-Z]*f[a-zA-Z]*\s+)?.*/, name: "rm (force delete)" },
  { pattern: /\brm\s+-[a-zA-Z]*r[a-zA-Z]*/, name: "rm -r (recursive delete)" },
  { pattern: /\brmdir\b/, name: "rmdir" },
  { pattern: /\bunlink\b/, name: "unlink" },

  // Git destructive operations
  { pattern: /\bgit\s+push\s+.*--force\b/, name: "git push --force" },
  { pattern: /\bgit\s+push\s+-f\b/, name: "git push -f" },
  { pattern: /\bgit\s+reset\s+--hard\b/, name: "git reset --hard" },
  { pattern: /\bgit\s+clean\s+-[a-zA-Z]*f/, name: "git clean -f" },
  { pattern: /\bgit\s+checkout\s+--\s+\./, name: "git checkout -- ." },
  { pattern: /\bgit\s+stash\s+drop\b/, name: "git stash drop" },
  { pattern: /\bgit\s+branch\s+-[dD]\b/, name: "git branch delete" },
  { pattern: /\bgit\s+rebase\b/, name: "git rebase" },

  // Database operations
  {
    pattern: /\bDROP\s+(DATABASE|TABLE|INDEX|SCHEMA)\b/i,
    name: "DROP database object",
  },
  { pattern: /\bTRUNCATE\s+TABLE\b/i, name: "TRUNCATE TABLE" },
  { pattern: /\bDELETE\s+FROM\b.*(?!WHERE)/i, name: "DELETE without WHERE" },

  // System-level destructive commands
  { pattern: /\bmkfs\b/, name: "mkfs (format filesystem)" },
  { pattern: /\bdd\s+.*of=\/dev\//, name: "dd to device" },
  { pattern: /\bshred\b/, name: "shred" },
  { pattern: /\bwipe\b/, name: "wipe" },

  // Process/service disruption
  { pattern: /\bkillall\b/, name: "killall" },
  { pattern: /\bpkill\b/, name: "pkill" },
  { pattern: /\bkill\s+-9\b/, name: "kill -9" },
  {
    pattern: /\bsystemctl\s+(stop|disable|mask)\b/,
    name: "systemctl stop/disable",
  },
  { pattern: /\bservice\s+\S+\s+stop\b/, name: "service stop" },

  // Container/infrastructure destruction
  { pattern: /\bdocker\s+rm\b/, name: "docker rm" },
  { pattern: /\bdocker\s+rmi\b/, name: "docker rmi" },
  { pattern: /\bdocker\s+system\s+prune\b/, name: "docker system prune" },
  { pattern: /\bdocker-compose\s+down\s+-v\b/, name: "docker-compose down -v" },
  { pattern: /\bkubectl\s+delete\b/, name: "kubectl delete" },

  // Dangerous redirects/overwrites
  { pattern: />\s*\/dev\/(sd|hd|nvme)/, name: "redirect to disk device" },
  { pattern: /:\s*>\s*\S+/, name: "truncate file with :>" },

  // Chmod/chown dangers
  { pattern: /\bchmod\s+(-R\s+)?777\b/, name: "chmod 777" },
  { pattern: /\bchown\s+-R\s+.*\s+\//, name: "recursive chown on root paths" },

  // npm/package manager destructive
  {
    pattern: /\bnpm\s+cache\s+clean\s+--force\b/,
    name: "npm cache clean --force",
  },
];

/**
 * Paths that are especially dangerous to modify
 */
const PROTECTED_PATHS = [
  /^\/$/, // Root
  /^\/etc\//, // System config
  /^\/usr\//, // System programs
  /^\/bin\//, // Essential binaries
  /^\/sbin\//, // System binaries
  /^\/boot\//, // Boot files
  /^\/var\/lib\//, // Variable state
  /^~\/\.(bash|zsh|profile)/, // Shell configs
  /^\/home\/[^/]+\/\.(bash|zsh|profile)/, // Shell configs
];

/**
 * Check if a command targets protected paths
 */
function targetsProtectedPath(command: string): string | null {
  for (const pathPattern of PROTECTED_PATHS) {
    if (pathPattern.test(command)) {
      const match = command.match(pathPattern);
      return match ? match[0] : "protected path";
    }
  }
  return null;
}

/**
 * Check if a command matches any destructive pattern
 */
function matchesDestructivePattern(
  command: string,
): { matched: boolean; name: string } | null {
  for (const { pattern, name } of DESTRUCTIVE_PATTERNS) {
    if (pattern.test(command)) {
      return { matched: true, name };
    }
  }
  return null;
}

export const DestructivePreventionPlugin: Plugin = async () => {
  return {
    "tool.execute.before": async (input, output) => {
      // Only check bash/shell commands
      if (input.tool !== "bash" && input.tool !== "shell") {
        return;
      }

      const command = output.args.command;
      if (!command || typeof command !== "string") {
        return;
      }

      // Check for destructive command patterns
      const destructiveMatch = matchesDestructivePattern(command);
      if (destructiveMatch) {
        throw new Error(
          `Blocked destructive command: "${destructiveMatch.name}"\n` +
            `Command: ${command}\n\n` +
            `This command could cause data loss or system damage. ` +
            `If you're sure you want to run this, ask the user to execute it manually.`,
        );
      }

      // Check for protected paths
      const protectedPath = targetsProtectedPath(command);
      if (protectedPath) {
        throw new Error(
          `Blocked command targeting protected path: ${protectedPath}\n` +
            `Command: ${command}\n\n` +
            `Modifying system paths can cause serious problems. ` +
            `If this is intentional, ask the user to execute it manually.`,
        );
      }

      // Additional check for write tool targeting sensitive files
      if (input.tool === "write") {
        const filePath = output.args.filePath;
        if (filePath && typeof filePath === "string") {
          const protectedFilePath = targetsProtectedPath(filePath);
          if (protectedFilePath) {
            throw new Error(
              `Blocked write to protected path: ${protectedFilePath}\n` +
                `Path: ${filePath}\n\n` +
                `Writing to system paths can cause serious problems.`,
            );
          }
        }
      }
    },
  };
};
