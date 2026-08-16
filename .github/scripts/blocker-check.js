/**
 * @param {Object} params
 * @param {import('@actions/core')} params.core
 * @param {import('@actions/github').context} params.context
 */
module.exports = async ({ core, context }) => {
    const labels = context.payload.pull_request.labels.map((l) => l.name);

    if (labels.includes("status: blocked")) {
        core.setFailed(
            "🚨 This PR is marked as 'status: blocked'. Resolve the blocker and remove the label before merging.",
        );
    }

    if (labels.includes("status: on-hold")) {
        core.setFailed("⏸️ This PR is currently 'status: on-hold'.");
    }

    console.log("No blocking labels found. Proceed.");
};
