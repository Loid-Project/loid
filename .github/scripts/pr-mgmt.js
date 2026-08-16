/**
 * @param {Object} params
 * @param {import('@actions/github').GitHub} params.github
 * @param {import('@actions/github').context} params.context
 */
module.exports = async ({ github, context }) => {
    const pr = context.payload.pull_request;
    const action = context.payload.action;
    const labelsToAdd = [];
    const labelsToRemove = [
        "status: in-progress",
        "status: needs review",
        "status: ready",
        "status: needs triage",
    ];

    if (pr.draft) {
        labelsToAdd.push("status: in-progress");
    } else if (["opened", "reopened", "ready_for_review"].includes(action) && !pr.draft) {
        labelsToAdd.push("status: needs review");
    } else if (action === "closed" && pr.merged) {
        labelsToAdd.push("status: merged");
    } else if (action === "closed" && !pr.merged) {
        labelsToAdd.push("status: wontfix");
    }

    if (
        context.eventName === "pull_request_review" &&
        context.payload.review.state === "approved"
    ) {
        labelsToAdd.push("status: ready");
        const index = labelsToRemove.indexOf("status: ready");
        if (index > -1) labelsToRemove.splice(index, 1);
    }

    if (action === "opened" && !pr.draft && pr.requested_reviewers.length === 0) {
        // await github.rest.pulls.requestReviewers({
        //   owner: context.repo.owner, repo: context.repo.repo, pull_number: pr.number,
        //   reviewers: ['your-lead-dev']
        // });
    }

    if (action === "opened" && pr.assignees.length === 0) {
        await github.rest.issues.addAssignees({
            owner: context.repo.owner,
            repo: context.repo.repo,
            issue_number: pr.number,
            assignees: [pr.user.login],
        });
    }

    for (const label of labelsToRemove) {
        try {
            await github.rest.issues.removeLabel({
                owner: context.repo.owner,
                repo: context.repo.repo,
                issue_number: pr.number,
                name: label,
            });
        } catch (e) {}
    }

    // Add new status label
    if (labelsToAdd.length > 0) {
        await github.rest.issues.addLabels({
            owner: context.repo.owner,
            repo: context.repo.repo,
            issue_number: pr.number,
            labels: labelsToAdd,
        });
    }
};
