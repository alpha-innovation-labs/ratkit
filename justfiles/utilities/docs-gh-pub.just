# Build and publish docs to GitHub Pages
# Usage: just docs-gh-pub
# Example: just docs-gh-pub

docs-gh-pub:
	#!/usr/bin/env bash
	set -euo pipefail

	if ! command -v bun >/dev/null 2>&1; then
		echo "ERROR: bun is required. Install it from https://bun.sh and retry."
		exit 1
	fi

	if ! git remote get-url origin >/dev/null 2>&1; then
		echo "ERROR: git remote 'origin' is required to publish to gh-pages."
		exit 1
	fi

	ORIGIN_URL=$(git remote get-url origin)
	REMOTE_INFO=$(printf '%s\n' "$ORIGIN_URL" | sed -E 's#(git@[^:]+:|https?://[^/]+/)([^/]+)/([^/.]+)(\.git)?$#\2 \3#')
	if [[ "$REMOTE_INFO" == "$ORIGIN_URL" ]]; then
		echo "ERROR: Unable to derive owner/repository from origin URL: $ORIGIN_URL"
		exit 1
	fi

	OWNER=$(printf '%s\n' "$REMOTE_INFO" | cut -d' ' -f1)
	REPO_NAME=$(printf '%s\n' "$REMOTE_INFO" | cut -d' ' -f2)
	BASE_PATH="/$REPO_NAME"
	PAGES_URL="https://$OWNER.github.io/$REPO_NAME/"

	echo "Building static docs for GitHub Pages..."
	echo "Base path: $BASE_PATH"
	(
		cd docs
		GH_PAGES=1 GH_PAGES_BASE_PATH="$BASE_PATH" bun run build
	)

	if [[ ! -d docs/out ]]; then
		echo "ERROR: docs/out was not produced by the static build."
		exit 1
	fi

	touch docs/out/.nojekyll

	PUBLISH_WORKTREE=$(mktemp -d)
	cleanup() {
		git worktree remove --force "$PUBLISH_WORKTREE" >/dev/null 2>&1 || true
		rm -rf "$PUBLISH_WORKTREE"
	}
	trap cleanup EXIT

	if git show-ref --verify --quiet refs/heads/gh-pages; then
		git worktree add --force "$PUBLISH_WORKTREE" gh-pages >/dev/null
	elif git ls-remote --exit-code --heads origin gh-pages >/dev/null 2>&1; then
		git worktree add --force -b gh-pages "$PUBLISH_WORKTREE" origin/gh-pages >/dev/null
	else
		git worktree add --force -b gh-pages "$PUBLISH_WORKTREE" >/dev/null
	fi

	shopt -s dotglob nullglob
	for path in "$PUBLISH_WORKTREE"/*; do
		name=$(basename "$path")
		if [[ "$name" == ".git" ]]; then
			continue
		fi
		rm -rf "$path"
	done
	shopt -u dotglob nullglob

	cp -R docs/out/. "$PUBLISH_WORKTREE"/

	(
		cd "$PUBLISH_WORKTREE"
		git add --all

		if git diff --cached --quiet --exit-code; then
			echo "No documentation changes detected. Skipping commit and push."
			exit 0
		fi

		git commit -m "docs: publish GitHub Pages"
		git push origin gh-pages
	)

	echo "GitHub Pages publication complete: $PAGES_URL"
