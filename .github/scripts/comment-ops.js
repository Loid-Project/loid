/**
 * @param {Object} params
 * @param {import('@actions/github').GitHub} params.github
 * @param {import('@actions/github').context} params.context
 */
module.exports = async ({ github, context }) => {
    const issue = context.payload.issue;
    const commenter = context.payload.comment.user.login;

    await github.rest.issues.addAssignees({
        owner: context.repo.owner,
        repo: context.repo.repo,
        issue_number: issue.number,
        assignees: [commenter],
    });

    await github.rest.issues.addLabels({
        owner: context.repo.owner,
        repo: context.repo.repo,
        issue_number: issue.number,
        labels: ["status: in-progress"],
    });

    try {
        await github.rest.issues.removeLabel({
            owner: context.repo.owner,
            repo: context.repo.repo,
            issue_number: issue.number,
            name: "status: needs triage",
        });
    } catch (e) {}
};
