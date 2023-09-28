# Rgrep

This is a project to replicate `grep` functionalities using Rust. It is a basic implementation that only takes into account the following options:

### Options

`RUST_LOG=warn,info cargo run -- <pattern> <path> -i <ignore files> -r <recursive dir search> -c <case sensitive>`

[-i]: These are comma delimited strings. If the file path matches this list, it will ignore searching it. This includes directories and files.

- Default = ""

[-r]: Recursive flag is used to traverse subdirectories from the root path. If it is true, all subdirectories will be searched for the matching pattern.

- Default = false

[-c]: Case sensitivity is enforced by default. The pattern is affected by this flag. If case sensitivity is false, then the search will ignore case sensitivity in the file lines and the pattern, broadening the matching results.

- Default = false

### Example

`RUST_LOG=warn,info cargo run -- Const ../../ -i node_modules,.git -r`

- Searches everything at path `../../` for `Const`, while ignoring the node_modules files/folders and recursively checking all subdirectories.
