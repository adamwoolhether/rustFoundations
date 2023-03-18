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