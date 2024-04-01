#!/usr/bin/env sh

(cd docs/ && rhai-doc)

brave docs/dist/index.html
