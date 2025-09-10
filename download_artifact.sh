#!/bin/bash

# Ensure GitHub CLI is installed
if ! command -v gh &> /dev/null; then
    echo "GitHub CLI (gh) is not installed. Please install it first."
    exit 1
fi

# Define ENV
REPO="chlaty/chlaty-cli"
VERSION="0.1.0"
OUTPUT_DIR="artifacts"


# Clean output directory if it exist
rm -rf "$OUTPUT_DIR"

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"



# List of workflows to process
WORKFLOWS=(
  "linux-build.yml"
  "macos-build.yml"
  "windows-build.yml"
)

for WORKFLOW in "${WORKFLOWS[@]}"; do
    echo "🔍 Checking latest run for $WORKFLOW..."

    RUN_ID=$(gh run list --repo "$REPO" --workflow "$WORKFLOW" --limit 10 --json status,databaseId -q '.[] | select(.status == "completed") | .databaseId' | head -n 1)

    if [ -z "$RUN_ID" ]; then
        echo "⚠️ No completed run found for $WORKFLOW. Skipping."
        continue
    fi

    echo "📦 Downloading artifacts from run ID: $RUN_ID ($WORKFLOW)"
    gh run download "$RUN_ID" --repo "$REPO" --dir "$OUTPUT_DIR"
done

# Flatten files from subfolders into output directory
echo "🧹 Flattening files into $OUTPUT_DIR"
find "$OUTPUT_DIR" -mindepth 2 -type f -exec mv -n {} "$OUTPUT_DIR/" \;
find "$OUTPUT_DIR" -mindepth 1 -type d -empty -delete

# Generate manifest.json
echo "📝 Generating manifest.json..."

declare -A targets=(
  ["windows/x86_64"]="chlaty-cli-x86_64-pc-windows-msvc.exe"
  ["windows/i686"]="chlaty-cli-i686-pc-windows-msvc.exe"
  ["windows/aarch64"]="chlaty-cli-aarch64-pc-windows-msvc.exe"
  ["linux/x86_64"]="chlaty-cli-x86_64-unknown-linux-musl"
  ["linux/aarch64"]="chlaty-cli-aarch64-unknown-linux-musl"
  ["macos/x86_64"]="chlaty-cli-x86_64-apple-darwin"
  ["macos/aarch64"]="chlaty-cli-aarch64-apple-darwin"
)

# Build manifest as a jq-compatible JSON object
jq_manifest="{}"
for key in "${!targets[@]}"; do
  IFS="/" read -r platform arch <<< "$key"
  file="${targets[$key]}"
  path="$OUTPUT_DIR/$file"
  [ -f "$path" ] || continue
  sha256=$(sha256sum "$path" | awk '{print $1}')
  url="https://github.com/chlaty/chlaty-cli/releases/download/$VERSION/$file"
  jq_manifest=$(echo "$jq_manifest" | jq --arg p "$platform" --arg a "$arch" --arg f "$url" --arg s "$sha256" '
    .[$p][$a] = {file: $f, sha256: $s}
  ')
done

# Save pretty-printed manifest
echo "$jq_manifest" | jq '.' > "$OUTPUT_DIR/manifest.json"

echo "✅ All artifacts downloaded and manifest.json created in $OUTPUT_DIR"
