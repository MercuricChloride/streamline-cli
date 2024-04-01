#!/usr/bin/env sh
set -e

# Get all uses of new/update for entities
awk '
BEGIN { ORS = "\n\n" }
/\y(new|update)\y\s+(\w+|_+)\s+(.*)\s*\{/, /}/ {
  print gensub(/^\s+/, "", "g", $0)
}
' src/TraderJoe.strm \
    | awk -f scripts/awk/graphql/create_schema.awk \
    > build/schema.graphql

echo "Wrote schema to 'schema.graphql!'"
