#!/bin/bash

REPO="chlaty/chlaty-cli"

# Loop through all pages of artifacts
page=1
while true; do
  echo "🔍 Fetching artifacts page $page..."
  artifacts=$(gh api repos/$REPO/actions/artifacts --paginate -q '.artifacts[].id')

  if [ -z "$artifacts" ]; then
    echo "✅ No more artifacts to delete."
    break
  fi

  for id in $artifacts; do
    echo "🗑️ Deleting artifact ID $id..."
    gh api --method DELETE repos/$REPO/actions/artifacts/$id
  done

  ((page++))
done

echo "✅ All artifacts deleted."