#!/usr/env gawk

# match module defs
/^(mfn|sfn)\s+(\w+|_+)\s*\((.*)\)\s*\{/ {
	match($0, /^(mfn|sfn)\s+(\w+|_+)\s*\((.*)\)/, arr)
	kind = arr[1]
	name = arr[2]
	arg_list = arr[3]
	str_args = gensub(/((\w+|_+)(:\w+)?)/, "\"\\1\"", "g", arg_list)
	arg_list = gensub(/(:\w+)/, "", "g", arg_list)
	modules = (modules sprintf("\n  add_%s(\"%s\", [%s]);\n", kind, name, str_args))
	printf "fn %s(%s) {\n", name, arg_list
	next
}

#
# match method defs
/^(method)\s+(\w+|_+)\s*\((.*)\)\s*\{/ {
	printf gensub(/method/, "fn", "", $0)
	next
}

// {
	print
}

END {
	printf "if in_repl() {\n%s}", modules
}


function get_update_policy(last_arg, len, arr)
{
	len = split(last_arg, arr, ":")
	return arr[len]
}
