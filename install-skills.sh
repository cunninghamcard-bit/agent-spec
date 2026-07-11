#!/usr/bin/env bash
set -euo pipefail

SKILL_DIR="${HOME}/.claude/skills"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "=== docwright skills installer ==="
echo

# Step 1: Install CLI
if command -v docwright &>/dev/null; then
  CURRENT=$(docwright --version 2>/dev/null || echo "unknown")
  echo "[ok] docwright CLI already installed: ${CURRENT}"
else
  echo "[..] Installing docwright CLI via cargo..."
  if command -v cargo &>/dev/null; then
    cargo install docwright
    echo "[ok] docwright CLI installed"
  else
    echo "[!!] cargo not found. Install Rust first: https://rustup.rs"
    echo "     Then run: cargo install docwright"
    exit 1
  fi
fi

echo

# Step 2: Install skills
mkdir -p "${SKILL_DIR}"

for skill in docwright-sdd docwright-research docwright-tool-first docwright-authoring docwright-estimate; do
  SRC="${SCRIPT_DIR}/skills/${skill}"
  DST="${SKILL_DIR}/${skill}"

  if [ ! -d "${SRC}" ]; then
    echo "[skip] ${skill} — not found in ${SCRIPT_DIR}/skills/"
    continue
  fi

  # Copy (overwrite) to ensure latest version
  rm -rf "${DST}"
  cp -r "${SRC}" "${DST}"
  echo "[ok] ${skill} → ${DST}"
done

echo
echo "Done. All docwright skills are ready for Claude Code."
echo "Verify with: ls ~/.claude/skills/docwright-*"
