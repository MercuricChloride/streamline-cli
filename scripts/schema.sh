#!/usr/bin/env sh
set -e

SCRIPTS="$HOME/streamline-cli/scripts"

# Get all uses of new/update for entities
gawk '
BEGIN { ORS = "\n\n" }
/\y(new|update)\y\s+(\w+|_+)\s+(.*)\s*\{/, /}/ {
  print gensub(/^\s+/, "", "g", $0)
}
' "$1" \
    | gawk -f "$SCRIPTS/awk/graphql/entities.awk" \
    > schema.graphql

echo "Wrote schema to 'schema.graphql!'"
