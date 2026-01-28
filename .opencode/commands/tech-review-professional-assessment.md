# GitHub & Linear User Performance Assessment

## 1. Setup & Context

**Instructions for the AI:**
Before proceeding, ensure you have the following three variables. If any are missing, **STOP** and ask the user to provide them.

1.  **GITHUB_USERNAME** = `{REPLACE_WITH_USERNAME}`
2.  **GITHUB_ORG** = `{REPLACE_WITH_ORG}`
3.  **LINEAR_EMAIL** = `{REPLACE_WITH_EMAIL}`
4.  **TARGET_ROLE** = `{Manager OR IC}` (Used to select performance rubric)

**Context:**
Perform a comprehensive performance assessment for the **2025 Calendar Year** (2025-01-01 to 2025-12-31).

---

## 2. Data Collection Strategy

### A. GitHub Activity

**IMPORTANT: Execution Notes**

- **Permissions:** GitHub CLI commands require `required_permissions: ["all"]` to avoid TLS certificate errors in sandboxed environments
- **Rate Limits:** Run queries **sequentially** (not in parallel) to avoid GitHub's secondary rate limits
- If you receive a 403 rate limit error, **wait 60 seconds** and retry with exponential backoff (60s, 120s, 240s)
- Use `closed:` date filter (not `created:`) to match the analysis period accurately
- For code reviews, **start with the fallback approach** (Q4 sample) since rate limits are common
- If a query continues to fail after 3 retries, note it in the report and proceed with available data

#### Step 1: PRs Authored (run first)

```bash
# Use closed: filter to match the analysis period (PRs created in 2025 but closed in 2026 should be excluded)
gh search prs --owner="{GITHUB_ORG}" --limit 500 --json number,title,url,closedAt,repository \
  -- "is:merged" "closed:2025-01-01..2025-12-31" "author:{GITHUB_USERNAME}"
```

#### Step 1b: Count PRs by Quarter

After gathering PR data, count by quarter. If `jq` is available:

```bash
# Save Step 1 output to a file, then count by quarter:
cat prs.json | jq '[.[] | select(.closedAt >= "2025-01-01" and .closedAt < "2025-04-01")] | length'  # Q1
cat prs.json | jq '[.[] | select(.closedAt >= "2025-04-01" and .closedAt < "2025-07-01")] | length'  # Q2
cat prs.json | jq '[.[] | select(.closedAt >= "2025-07-01" and .closedAt < "2025-10-01")] | length'  # Q3
cat prs.json | jq '[.[] | select(.closedAt >= "2025-10-01" and .closedAt < "2026-01-01")] | length'  # Q4
```

If `jq` is not available, manually count PRs by scanning `closedAt` dates in the JSON output.

#### Step 2: Code Reviews (START WITH FALLBACK - rate limits are common)

_The GitHub Search API has aggressive secondary rate limits for review queries. Start with a single-quarter sample:_

```bash
# Get Q4 as a representative sample (most recent, likely to succeed)
gh search prs --owner="{GITHUB_ORG}" --limit 100 --json number,repository \
  -- "updated:2025-10-01..2025-12-31" is:pr reviewed-by:"{GITHUB_USERNAME}" -author:"{GITHUB_USERNAME}"
```

**Extrapolation:** Multiply Q4 count × 4 for annual estimate. **Note this estimation method in the report.**

_Optional: If rate limits allow, try quarterly queries with 60-second delays:_

```bash
# Q1 Reviews (wait 60s before running)
gh search prs --owner="{GITHUB_ORG}" --limit 200 --json number,repository,url \
  -- "updated:2025-01-01..2025-03-31" is:pr reviewed-by:"{GITHUB_USERNAME}" -author:"{GITHUB_USERNAME}"

# Q2 Reviews (wait 60s)
gh search prs --owner="{GITHUB_ORG}" --limit 200 --json number,repository,url \
  -- "updated:2025-04-01..2025-06-30" is:pr reviewed-by:"{GITHUB_USERNAME}" -author:"{GITHUB_USERNAME}"

# Q3 Reviews (wait 60s)
gh search prs --owner="{GITHUB_ORG}" --limit 200 --json number,repository,url \
  -- "updated:2025-07-01..2025-09-30" is:pr reviewed-by:"{GITHUB_USERNAME}" -author:"{GITHUB_USERNAME}"

# Q4 Reviews (wait 60s)
gh search prs --owner="{GITHUB_ORG}" --limit 200 --json number,repository,url \
  -- "updated:2025-10-01..2025-12-31" is:pr reviewed-by:"{GITHUB_USERNAME}" -author:"{GITHUB_USERNAME}"
```

#### Step 3: Issues Created

```bash
gh search issues --owner="{GITHUB_ORG}" --limit 100 --json number,title,url,createdAt \
  -- "created:2025-01-01..2025-12-31" "author:{GITHUB_USERNAME}"
```

### B. Linear Activity (MCP Queries)

**MCP Tool:** `linear_discover` with `action: "search-issues"`

_Note: Use the `LINEAR_EMAIL` variable. First, check that the Linear MCP server is available by listing MCP tools._

**1. Workload (Assigned Issues):**

```json
{
  "action": "search-issues",
  "assignee_emails": ["{LINEAR_EMAIL}"],
  "created_after": "2025-01-01T00:00:00Z",
  "created_before": "2025-12-31T23:59:59Z",
  "limit": 50
}
```

**2. Throughput (Completed Issues):**

```json
{
  "action": "search-issues",
  "assignee_emails": ["{LINEAR_EMAIL}"],
  "state_names": ["Done", "Completed", "Closed"],
  "updated_after": "2025-01-01T00:00:00Z",
  "updated_before": "2025-12-31T23:59:59Z",
  "limit": 50
}
```

**Fallback if MCP unavailable:**
If the Linear MCP server is not configured or returns errors, ask the user to provide:

- Screenshot or export of their Linear "My Issues" filtered to 2025
- Total count of assigned issues
- Total count of completed issues
- List of projects they contributed to

---

## 3. Assessment Framework

Analyze the gathered data against the following rubrics based on `TARGET_ROLE`.

### Velocity Standards

| Metric           | Manager Standard    | IC Standard            |
| ---------------- | ------------------- | ---------------------- |
| **Weekly PRs**   | > 1.4 (~73/year)    | > 3.5 (~182/year)      |
| **Code Reviews** | 400-800/year (Good) | > 800/year (Excellent) |

### Qualitative Analysis

- **Consistency:** Look for gaps in activity (e.g., low activity in Q2). correlate gaps with Linear tickets (was the user stuck on a complex ticket?).
- **Scope:** Did the user touch multiple repositories (Cross-team) or focus on one (Deep dive)?
- **Leadership:** Look for "Architecture" or "Refactor" in PR titles vs "Fix" or "Minor".

---

## 4. Output Deliverable

Please generate a report with the following structure:

### Executive Summary

- **Performance Rating:** (Based on the Standards table above)
- **Primary Strength:** (e.g., High Velocity, Deep Code Reviews)
- **Key Observation:** (e.g., "Velocity dipped in Q3, but Linear shows high complexity ticket ownership.")

### Detailed Metrics

| Category         | Q1  | Q2  | Q3  | Q4  | Total       | Target   | Status                            |
| ---------------- | --- | --- | --- | --- | ----------- | -------- | --------------------------------- |
| **PRs Merged**   |     |     |     |     | **{TOTAL}** | {TARGET} | :large_green_circle:/:red_circle: |
| **PRs Reviewed** |     |     |     |     | **{TOTAL}** | 400+     | :large_green_circle:/:red_circle: |

### Narrative Analysis

- **Code Quality & Focus:** (Analyze PR titles/descriptions)
- **Collaboration:** (Review ratio of PRs merged vs PRs reviewed)
- **Project Management:** (Linear completion rates and issue creation)
