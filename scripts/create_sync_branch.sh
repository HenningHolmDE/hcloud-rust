#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e
# Treat unset variables as an error when substituting.
set -u
# Pipelines return the exit status of the last command to exit with a non-zero status, or zero if no command exited with a non-zero status.
set -o pipefail

# --- Configuration ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_FILE="${SCRIPT_DIR}/config.sh"
PROJECT_ROOT=$(pwd)

# --- Script Logic ---

# 1. Initialization & Validation
echo "INFO: Starting sync branch creation script."

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <target_spec_version>"
    exit 1
fi

TARGET_SPEC_VERSION=$1
SYNC_BRANCH_NAME="sync/${TARGET_SPEC_VERSION}"

if [ ! -f "$CONFIG_FILE" ]; then
    echo "ERROR: Configuration file not found at ${CONFIG_FILE}"
    exit 1
fi

source "$CONFIG_FILE"

echo "INFO: Sourced configuration from ${CONFIG_FILE}"

# Validate that required configuration variables from config.sh are non-empty.
: "${HCLOUD_OPENAPI_VERSION:?HCLOUD_OPENAPI_VERSION is not set in config.sh}"
: "${HCLOUD_OPENAPI_URL:?HCLOUD_OPENAPI_URL is not set in config.sh}"

# --- Derived & Local Variables ---

# A sed-compatible regular expression to find and replace the version line in the spec file.
# For JSON: "version": "1.2.3" -> "version": "in-sync"
VERSION_PATTERN='s/"version": ".*"/"version": "in-sync"/'

# The relative path to the specification file within its repository.
# This is assumed to be at the root of the repository.
SPEC_FILE_SOURCE_PATH="openapi/hcloud.json"
SPEC_FILE_DESTINATION_PATH="${DOWNLOAD_DIR}/hcloud_working.json"

# The path to the script that runs the code generation.
CODEGEN_SCRIPT_PATH="scripts/generate_api_code.sh"
export HCLOUD_OPENAPI_JSON="$SPEC_FILE_DESTINATION_PATH"

SOURCE_PATH="src/ .openapi-generator/FILES"

echo "INFO: All required configuration variables are present."

if git rev-parse --verify "$SYNC_BRANCH_NAME" >/dev/null 2>&1; then
    echo "ERROR: Branch ${SYNC_BRANCH_NAME} already exists."
    exit 1
fi

git checkout -b "$SYNC_BRANCH_NAME"
echo "INFO: Created and checked out new branch: ${SYNC_BRANCH_NAME}"

SPEC_REPO_DIR="${PROJECT_ROOT}/${DOWNLOAD_DIR}/spec_repo"
echo "INFO: Cloning spec repository ${HCLOUD_OPENAPI_REPO_URL} into ${SPEC_REPO_DIR}"
# Remove existing directory to ensure a clean clone
rm -rf "$SPEC_REPO_DIR"
git clone "$HCLOUD_OPENAPI_REPO_URL" "$SPEC_REPO_DIR"

SHORT_COMMIT_HASH=$(cd "$SPEC_REPO_DIR" && git rev-parse --short "$TARGET_SPEC_VERSION")

echo "INFO: Generating list of commits from ${HCLOUD_OPENAPI_VERSION} to ${TARGET_SPEC_VERSION}."
COMMIT_HASHES=$(cd "$SPEC_REPO_DIR" && git log --reverse --pretty=format:"%H" "${HCLOUD_OPENAPI_VERSION}..${TARGET_SPEC_VERSION}")

if [ -z "$COMMIT_HASHES" ]; then
    echo "WARNING: No new commits found between ${HCLOUD_OPENAPI_VERSION} and ${TARGET_SPEC_VERSION}."
    exit 0
else
    # 2. Initial Commit
    echo "INFO: Creating initial commit based on version ${HCLOUD_OPENAPI_VERSION}."

    # Get the spec file content for the current version
    (cd "$SPEC_REPO_DIR" && git show "${HCLOUD_OPENAPI_VERSION}:${SPEC_FILE_SOURCE_PATH}") > "${PROJECT_ROOT}/${SPEC_FILE_DESTINATION_PATH}"
    echo "INFO: Updated local spec file from version ${HCLOUD_OPENAPI_VERSION}."

    # Patch the version to "in-sync"
    sed -i -E "$VERSION_PATTERN" "${PROJECT_ROOT}/${SPEC_FILE_DESTINATION_PATH}"
    echo "INFO: Patched spec file version to 'in-sync'."

    # Run code generation
    echo "INFO: Running code generation script for initial commit..."
    (cd "$PROJECT_ROOT" && bash "$CODEGEN_SCRIPT_PATH")

    # Commit the initial state
    git add $SOURCE_PATH
    git commit -m "[spec-sync-meta] Start sync process based on ${HCLOUD_OPENAPI_VERSION}"
    echo "INFO: Created initial commit."

    # 3. Iterative Commit Generation
    for hash in $COMMIT_HASHES; do
        echo "INFO: Processing spec commit ${hash}..."

        # a. Fetch the original commit message
        COMMIT_MSG=$(cd "$SPEC_REPO_DIR" && git log -n 1 --pretty=format:%s "${hash}")

        # b. Get the content of the spec file and save it
        (cd "$SPEC_REPO_DIR" && git show "${hash}:${SPEC_FILE_SOURCE_PATH}") > "${PROJECT_ROOT}/${SPEC_FILE_DESTINATION_PATH}"
        echo "INFO: Updated local spec file from commit ${hash}."

        # c. Apply a patch to the local spec file
        sed -i -E "$VERSION_PATTERN" "${PROJECT_ROOT}/${SPEC_FILE_DESTINATION_PATH}"
        echo "INFO: Patched spec file version to 'in-sync'."

        # d. Execute the code generation script
        echo "INFO: Running code generation script from ${PROJECT_ROOT}..."
        (cd "$PROJECT_ROOT" && bash "$CODEGEN_SCRIPT_PATH")

        # e. Stage the changes and commit
        git add $SOURCE_PATH
        if git diff --cached --quiet; then
            echo "INFO: No changes to commit for spec ${hash}."
        else
            echo "INFO: Committing changes for spec ${hash}."
            git commit -m "[spec-sync] ${COMMIT_MSG}"
        fi
    done
fi

# 4. Final Sync Commit
echo "INFO: Generating final sync commit for version ${TARGET_SPEC_VERSION}."

# Get the final version of the spec file
(cd "$SPEC_REPO_DIR" && git show "${TARGET_SPEC_VERSION}:${SPEC_FILE_SOURCE_PATH}") > "${PROJECT_ROOT}/${SPEC_FILE_DESTINATION_PATH}"
echo "INFO: Updated local spec file to final version ${TARGET_SPEC_VERSION}."

# Execute the code generation script one last time
echo "INFO: Running code generation script for the final time from ${PROJECT_ROOT}..."
(cd "$PROJECT_ROOT" && bash "$CODEGEN_SCRIPT_PATH")

# Stage all changes
git add $SOURCE_PATH

# Create a final commit
COMMIT_MESSAGE="[spec-sync-meta] Finalize spec at version ${TARGET_SPEC_VERSION}"
echo "INFO: Creating final commit: \"${COMMIT_MESSAGE}\""
git commit -m "$COMMIT_MESSAGE"

# 5. Cleanup & Output
echo "INFO: Cleaning up spec repository directory..."
rm -rf "$SPEC_REPO_DIR"

# 6. Patch config.sh to overwrite the assignment of HCLOUD_OPENAPI_VERSION
echo "INFO: Patching config.sh to set HCLOUD_OPENAPI_VERSION to ${SHORT_COMMIT_HASH}."
sed -i "s/^HCLOUD_OPENAPI_VERSION=.*/HCLOUD_OPENAPI_VERSION=${SHORT_COMMIT_HASH}/" "$CONFIG_FILE"

git add "$CONFIG_FILE"
git commit -m "[spec-sync-meta] Update HCLOUD_OPENAPI_VERSION to ${SHORT_COMMIT_HASH}"

echo "INFO: Successfully created sync branch ${SYNC_BRANCH_NAME}."
echo "SUCCESS: Script finished."