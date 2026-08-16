/**
 * @param {Object} params
 * @param {import('@actions/github').GitHub} params.github
 * @param {import('@actions/github').context} params.context
 */
module.exports = async ({ github, context }) => {
    const issue = context.payload.issue;
    const title = issue.title.toLowerCase();
    const body = (issue.body || "").toLowerCase();
    const currentLabels = issue.labels.map((l) => l.name);
    const labelsToAdd = new Set();

    if (currentLabels.length === 0) {
        labelsToAdd.add("status: needs triage");
    }

    if (title.includes("bug") || title.includes("fix") || title.includes("crash")) {
        labelsToAdd.add("type: bug");
    }
    if (title.includes("feat") || title.includes("enhancement")) {
        labelsToAdd.add("type: feature");
        labelsToAdd.add("type: enhancement");
    }
    if (title.includes("doc") || title.includes("readme")) {
        labelsToAdd.add("type: documentation");
        labelsToAdd.add("area: documentation");
    }
    if (title.includes("perf") || title.includes("slow")) {
        labelsToAdd.add("type: performance");
    }
    if (title.includes("refactor") || title.includes("cleanup")) {
        labelsToAdd.add("type: refactor");
    }
    if (title.includes("security") || title.includes("cve") || title.includes("vulnerability")) {
        labelsToAdd.add("type: security");
        labelsToAdd.add("priority: critical");
    }
    if (title.includes("?")) {
        labelsToAdd.add("type: question");
    }

    if (title.includes("parser") || body.includes("parser")) labelsToAdd.add("area: parser");
    if (title.includes("lexer") || body.includes("lexer")) labelsToAdd.add("area: lexer");
    if (title.includes("transpile") || title.includes("js"))
        labelsToAdd.add("area: javascript transpiler");
    if (title.includes("env") || title.includes("setup")) labelsToAdd.add("area: dev environment");
    if (title.includes("std") || title.includes("library"))
        labelsToAdd.add("area: standard library");

    if (currentLabels.includes("good first issue") || labelsToAdd.has("good first issue")) {
        labelsToAdd.add("difficulty: easy");
        labelsToAdd.add("help wanted");
    }

    if (labelsToAdd.size > 0) {
        await github.rest.issues.addLabels({
            owner: context.repo.owner,
            repo: context.repo.repo,
            issue_number: issue.number,
            labels: Array.from(labelsToAdd),
        });
    }

    if (issue.assignees.length === 0) {
        await github.rest.issues.addAssignees({
            owner: context.repo.owner,
            repo: context.repo.repo,
            issue_number: issue.number,
            assignees: [issue.user.login],
        });
    }
};
