/^(new|update)/, /\{$/ {
	printf "entity %s @entity {\n  id:ID!\n", $2
	next
}

/}/ {
	print $0
	next
}

/^(\w+|_+):/ {
	printf "  %s\n", formatType()
}


function formatType(lfield)
{
	lfield = $0
	# Remove the variable_value and 'as' usage
	lfield = gensub(/:\s+.*+\sas\s/, ": ", 1, lfield)
	# Replace instances of Address with Bytes
	lfield = gensub(/Address/, "Bytes", "g", lfield)
	# Replace Ref with
	lfield = gensub(/:Ref/, "!", "g", lfield)
	return lfield
}
