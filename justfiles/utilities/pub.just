# Publish to crates.io (bumps version, creates tag, publishes)
# Usage: just pub
# Example: just pub

pub:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Checking git status..."
    if ! git diff --quiet --exit-code || ! git diff --cached --quiet --exit-code; then
        echo "Found uncommitted changes. Committing all changes before release..."
        git add .
        git commit -m "chore: prepare for release"
        echo "Committed all pending changes"
    else
        echo "Git working directory is clean"
    fi

    just bump-version

    NEW_VERSION=$(awk -F'"' '/^version = "[0-9]+\.[0-9]+\.[0-9]+"/ { print $2; exit }' Cargo.toml)
    echo "Release version: $NEW_VERSION"

    cargo check --quiet
    echo "Updated Cargo.lock"

    echo "Committing version bump..."
    git add Cargo.toml README.md skills/ratkit/SKILL.md src/lib.rs
    if git ls-files --error-unmatch Cargo.lock >/dev/null 2>&1; then
        git add Cargo.lock
    fi

    if git diff --cached --quiet --exit-code; then
        echo "No staged version changes found after bump-version"
        exit 1
    fi

    git commit -m "chore: bump version to $NEW_VERSION"
    echo "Committed version bump"

    echo "Creating git tag v$NEW_VERSION..."
    git tag "v$NEW_VERSION"
    echo "Created tag v$NEW_VERSION"

    if git remote | grep -q .; then
        echo "Pushing to remote..."
        git push
        git push --tags
        echo "Pushed commits and tags"
    else
        echo "No git remote configured. Skipping push."
        echo "To add a remote: git remote add origin <url>"
    fi

    echo "Publishing to crates.io..."
    cargo publish

    echo "Published version $NEW_VERSION to crates.io"
