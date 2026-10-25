# The Lucide release the icons come from (https://lucide.dev)
LUCIDE_VERSION := "1.61.0"

# Downloads, unmodified, every icon ui/icons.slint references from the pinned Lucide release
[group('icons')]
icons:
  #!/usr/bin/env bash
  set -euo pipefail
  ui=crates/slint-themekit/ui
  base="https://unpkg.com/lucide-static@{{LUCIDE_VERSION}}"
  names=$(grep -o 'icons/[a-z0-9-]*\.svg' "$ui/icons.slint" | sed 's|^icons/||; s|\.svg$||' | sort -u)
  # Download everything first, so a failed download leaves the icons as they were.
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' EXIT
  for name in $names; do
    curl -fsSL "$base/icons/$name.svg" -o "$tmp/$name.svg"
  done
  curl -fsSL "$base/LICENSE" -o "$tmp/LICENSE"
  rm -f "$ui"/icons/*.svg "$ui/icons/LICENSE"
  mv "$tmp"/* "$ui/icons/"
  echo "icons: $(echo $names | wc -w) from Lucide {{LUCIDE_VERSION}}"
