# Disable automatic .git:
# cargo new --vcs none project_name

# Cargo run by default runs everything in debug mode.
run:
	cargo run

# To skip debugging checks.
run-release:
	cargo run --release

# Creating a new library.
new-lib:
	cargo new --lib lib_name

# Run tests.
test:
	cargo test
# Run tests for the entire workspace.
test-all:
	cargo test --all

# Document the program.
doc:
	cargo doc

# Run our cli user management tool.
# cargo run --manifest-path userman/Cargo.toml -- --help
# cargo run --manifest-path userman/Cargo.toml -- list
# cargo run -- add --username mantou --password baga --limited true
# cargo run -- delete mantou
# cargo run -- change-password adam baga

# Adding a specific crate
# cargo add tokio

# To add everything from the crate:
# cargo add tokio -F full