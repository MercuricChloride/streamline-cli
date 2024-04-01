# This file is responsible for converting the graphql entity syntax to the
# appropriate entity change function calls for the streamline runtime

# The general pattern is:
# given an entry like: `new Token id {...`
# `new` -> create_entity("$2", "$3"
# `update` -> update_entity(entity_name, entity_id, fields)
# `delete` -> delete_entity(entity_name, entity_id)

# The fields are updated similarly,
# given an entry like: `token: obj.token as BigInt,`
# We should convert it into:
# `field_change("$1", $2, "$3")`

# global vars
# $entities --- An associative array from entity name -> nothing right now
# $inEntity --- 1 if currently matching the fields of the entity, 0 otherwise
# $entityName --- The current entity we are parsing for
BEGIN {
	inEntity = 0
	entityName = ""
	entityId = ""
	actionType = ""
	exitState = 0
}

/\y(new|update)\y\s+(\w+|_+).*{/ {
	if ($1 == "new") {
		actionType = "create"
	} else {
		actionType = "update"
	}
	printf "%s_entity(\"%s\", %s, [\n", actionType, $2, $3
	inEntity = 1
}

/\ydelete\y\s(\w)+.*{/ {
	printf "delete_entity(\"%s\", \"%s\"),\n", $2, $3
}

/}/ {
	if (inEntity == 1) {
		inEntity = 0
		print "])"
		next
	}
}

/\w+:/ {
	if (inEntity == 1) {
		# Remove the colon
		sub(/:/, "", $1)
		sub(/,/, "", $4)
		_value = $2
		printf "  field_change(\"%s\", %s, \"%s\"),\n", $1, $2, $4
	}
}

// {
	if (inEntity == 0) {
		print $0
	}
}
