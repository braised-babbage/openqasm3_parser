#!/bin/env sh

# See mkgenerated.sh

echo
echo ======================= Running write_syntax_kinds_enum once ===============
echo

cargo test write_syntax_kinds_enum

#echo ======================= Running write_syntax_kinds_enum() twice ===============
# echo >>>>>>>>>>>>>>> Running write_syntax_kinds_enum() twice
# echo

# cargo test write_syntax_kinds_enum

# rustfmt crates/oq3_syntax/src/ast/generated/_new_nodes.rs
# rustfmt crates/oq3_parser/src/syntax_kind/_syntax_kind_enum.rs
# rustfmt crates/oq3_syntax/src/ast/generated/_new_tokens.rs

