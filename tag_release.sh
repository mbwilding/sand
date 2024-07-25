#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Function to extract the version from Cargo.toml
get_version_from_cargo_toml() {
    grep "^version" Cargo.toml | head -1 | awk -F\" '{print $2}'
}

# Change to the directory containing your repository (optional)
# cd /path/to/your/repo

# Get version from Cargo.toml
VERSION=$(get_version_from_cargo_toml)

if [ -z "$VERSION" ]; then
    echo "Version not found in Cargo.toml"
    exit 1
fi

# Get the latest commit hash
COMMIT_HASH=$(git rev-parse HEAD)

echo "Tagging commit $COMMIT_HASH with version $VERSION..."

# Create an annotated tag
git tag -a "v$VERSION" $COMMIT_HASH -m "Tagging version $VERSION"

# Push the tag to the remote repository
git push origin "v$VERSION"

echo "Tag v$VERSION pushed successfully."
