# global vars
# $entities --- An associative array from entity name -> nothing right now
# $inEntity --- 1 if currently matching the fields of the entity, 0 otherwise
# $entityName --- The current entity we are parsing for
BEGIN {
	inEntity = 0
	entityName = ""
	exitState = 0
}

# new foo
/^new|update\s(\w)+.*{/ {
	entityName = $2
	setEntity()
}

/}/ {
	if (inEntity == 1) {
		inEntity = 0
		entityName = ""
	}
}

/\w+:/ {
	if (inEntity == 1) {
		sub(/,/, "", $4)
		_type = formatType($4)
		_field = sprintf("%s %s\n", $1, _type)
		_fieldEntry = definedFields[entityName][$1]
		if ((_fieldEntry != _type) && (_fieldEntry != "")) {
			print "~~~~ERROR!~~~~"
			print "Schema type change!"
			print "\n~~~~INFO~~~~"
			printf "The type of field: '%s' on entity '%s' has changed type in your source! This is always a mistake!\n", _fieldEntry, entityName
			print "\n~~~~CONTEXT~~~~"
			printf "Last Known Type: %s, \nUpdated to: %s, \nError on line %s", _fieldEntry, _type, NR
			exitState = 1
			exit 1
		}
		definedFields[entityName][$1] = _type
		entities[entityName] = entities[entityName] _field
	}
}

END {
	if (exitState != 0) {
		exit exitState
	}
	for (ent in entities) {
		printf "type %s @entity {\n", ent
		printf "  id: ID!\n"
		for (field in definedFields[ent]) {
			printf "  %s %s\n", field, definedFields[ent][field]
		}
		printf "}\n"
		#printf "### %s ###\n", ent
		#print entities[ent]
	}
}


# Converts a type into the graphql syntax
function formatType(typeName, splitLen, arr)
{
	splitLen = split(typeName, arr, ":")
	lfield = arr[1]
	# Replace instances of Address with Bytes
	lfield = gensub(/Address/, "String", "g", lfield)
	if (arr[2] == "") {
		return lfield
	} else {
		return sprintf("%s!", arr[1])
	}
}

# Adds an entity to the global store
function setEntity(value)
{
	if (inEntity == 0) {
		inEntity = 1
		value = entities[entityName]
		if (value == "") {
			entities[entityName] = value
		}
	}
}
