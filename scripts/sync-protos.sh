#!/usr/bin/env bash
set -euo pipefail

PKG="@teacinema/contracts"
VERSION="${1:?usage: sync-protos.sh <version, e.g. 1.5.1>}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DST="$(cd "$SCRIPT_DIR/.." && pwd)/proto"

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

cd "$TMP"
npm init -y >/dev/null
npm install --no-audit --no-fund --silent "${PKG}@${VERSION}"

PKG_DIR="$(pwd)/node_modules/${PKG}"

REL_PATHS="$(node -e '
  const fs = require("fs");
  const path = require("path");
  const c = require("@teacinema/contracts");
  const p = c.PROTO_PATHS || (c.default && c.default.PROTO_PATHS);
  if (!p) { console.error("PROTO_PATHS not found"); process.exit(1); }

  function findPkgRoot(startFile) {
    let dir = path.dirname(startFile);
    while (dir !== path.dirname(dir)) {
      const pj = path.join(dir, "package.json");
      if (fs.existsSync(pj)) {
        try {
          if (JSON.parse(fs.readFileSync(pj, "utf8")).name === "@teacinema/contracts") return dir;
        } catch (_) {}
      }
      dir = path.dirname(dir);
    }
    return null;
  }

  const pkgRoot = findPkgRoot(require.resolve("@teacinema/contracts"));
  if (!pkgRoot) { console.error("package root not found"); process.exit(1); }

  const protoRoot = path.join(pkgRoot, "proto");
  const out = [];
  for (const v of Object.values(p)) {
    const rel = path.relative(protoRoot, v);
    if (!rel.startsWith("..") && !path.isAbsolute(rel) && v.endsWith(".proto")) {
      out.push(rel.split(path.sep).join("/"));
    }
  }
  process.stdout.write(out.join("\n"));
')"

if [ -z "$REL_PATHS" ]; then
  echo "❌ no .proto paths resolved" >&2
  exit 1
fi

rm -rf "$DST"
mkdir -p "$DST"

while IFS= read -r rel; do
  [ -z "$rel" ] && continue
  mkdir -p "$DST/$(dirname "$rel")"
  cp "$PKG_DIR/proto/$rel" "$DST/$rel"
  echo "copied proto/$rel"
done <<< "$REL_PATHS"

echo "${PKG}@${VERSION}" > "$DST/.source"
echo "✅ synced $(find "$DST" -name '*.proto' | wc -l) file(s) from $(cat "$DST/.source")"
