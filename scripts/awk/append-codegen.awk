// {
	print "if in_repl() {"
	printf "  generate_yaml(\"%s/streamline.yaml\");\n", $0
	printf "  generate_rust(\"%s/src/streamline.rs\");\n", $0
	print "}\n"
}
