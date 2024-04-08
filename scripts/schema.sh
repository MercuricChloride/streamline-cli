#!/usr/bin/env sh
set -e

SCRIPTS="$HOME/streamline-cli/scripts"

# Get all uses of new/update for entities
awk '
BEGIN { ORS = "\n\n" }
/\y(new|update)\y\s+(\w+|_+)\s+(.*)\s*\{/, /}/ {
  print gensub(/^\s+/, "", "g", $0)
}
' "$1" \
    | awk -f "$SCRIPTS/awk/graphql/create_schema.awk" \
    > schema.graphql

echo "Wrote schema to 'schema.graphql!'"
