#!/usr/bin/env bash
set -euo pipefail

if [ -n "$(git status --porcelain)" ]; then
	printf 'Working tree has local changes. Commit or stash before publishing.\n' >&2
	exit 1
fi

version="$(awk -F '"' '/^version[[:space:]]*=/{print $2; exit}' Cargo.toml)"

if [ -z "$version" ]; then
	printf 'Could not read version from Cargo.toml\n' >&2
	exit 1
fi

tag="v$version"

local_exists=false
remote_exists=false

if git rev-parse -q --verify "refs/tags/$tag" >/dev/null; then
	local_exists=true
fi

if git ls-remote --tags origin "refs/tags/$tag" | grep -q .; then
	remote_exists=true
fi

if [ "$local_exists" = true ] || [ "$remote_exists" = true ]; then
	printf 'Tag %s already exists.\n' "$tag"
	read -r -p "Delete and recreate tag $tag? [y/N]: " answer

	case "$answer" in
	y | Y | yes | YES)
		;;
	*)
		printf 'Cancelled.\n'
		exit 1
		;;
	esac

	if [ "$local_exists" = true ]; then
		git tag -d "$tag"
	fi

	if [ "$remote_exists" = true ]; then
		git push origin ":refs/tags/$tag"
	fi
fi

git tag "$tag"
git push origin "$tag"

printf 'Published %s\n' "$tag"
