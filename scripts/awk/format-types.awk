# This file is responsible for escaping and formatting certain types
# Things such as addresses, need to be wrapped in a special fn call
// {
	print gensub(/(0x[A-F0-9a-f]{40})/, "address(\"&\")", "g", $0)
}
