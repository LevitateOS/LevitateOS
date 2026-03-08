#!/usr/bin/env bash
set -euo pipefail

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "error: run this script inside a git repository" >&2
  exit 1
fi

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

default_branch() {
  local branch=""

  git fetch origin --prune >/dev/null 2>&1 || return 1
  git remote set-head origin -a >/dev/null 2>&1 || true

  branch="$(git symbolic-ref --quiet --short refs/remotes/origin/HEAD 2>/dev/null | sed 's|^origin/||')"
  if [[ -z "$branch" ]]; then
    branch="$(git ls-remote --symref origin HEAD 2>/dev/null | awk '/^ref:/ {sub("refs/heads/", "", $2); print $2; exit}')"
  fi
  if [[ -z "$branch" ]]; then
    if git show-ref --verify --quiet refs/remotes/origin/main; then
      branch="main"
    elif git show-ref --verify --quiet refs/remotes/origin/master; then
      branch="master"
    fi
  fi

  [[ -n "$branch" ]] || return 1
  printf '%s\n' "$branch"
}

switch_to_default_branch() {
  local label="$1"
  local branch=""

  if ! branch="$(default_branch)"; then
    echo "warn: $label - could not determine default branch"
    return 0
  fi

  if ! git show-ref --verify --quiet "refs/remotes/origin/$branch"; then
    echo "warn: $label - origin/$branch not found"
    return 0
  fi

  git checkout -B "$branch" "origin/$branch" >/dev/null
  git branch --set-upstream-to="origin/$branch" "$branch" >/dev/null 2>&1 || true
  git pull --ff-only origin "$branch" >/dev/null

  echo "ok: $label -> $branch"
}

echo "syncing root repository..."
switch_to_default_branch "."

if [[ -f .gitmodules ]]; then
  echo "syncing submodules (recursive)..."
  git submodule sync --recursive
  git submodule update --init --recursive --jobs 8

  git submodule foreach --recursive '
    set -e
    default_branch() {
      local branch=""
      git fetch origin --prune >/dev/null 2>&1 || return 1
      git remote set-head origin -a >/dev/null 2>&1 || true
      branch="$(git symbolic-ref --quiet --short refs/remotes/origin/HEAD 2>/dev/null | sed "s|^origin/||")"
      if [[ -z "$branch" ]]; then
        branch="$(git ls-remote --symref origin HEAD 2>/dev/null | awk "/^ref:/ {sub(\"refs/heads/\", \"\", \\$2); print \\$2; exit}")"
      fi
      if [[ -z "$branch" ]]; then
        if git show-ref --verify --quiet refs/remotes/origin/main; then
          branch="main"
        elif git show-ref --verify --quiet refs/remotes/origin/master; then
          branch="master"
        fi
      fi
      [[ -n "$branch" ]] || return 1
      printf "%s\n" "$branch"
    }

    branch="$(default_branch || true)"
    if [[ -z "$branch" ]]; then
      echo "warn: $name - could not determine default branch"
      exit 0
    fi
    if ! git show-ref --verify --quiet "refs/remotes/origin/$branch"; then
      echo "warn: $name - origin/$branch not found"
      exit 0
    fi

    git checkout -B "$branch" "origin/$branch" >/dev/null
    git branch --set-upstream-to="origin/$branch" "$branch" >/dev/null 2>&1 || true
    git pull --ff-only origin "$branch" >/dev/null
    echo "ok: $name -> $branch"
  '
else
  echo "no .gitmodules found, skipping submodule sync"
fi

echo "done."
