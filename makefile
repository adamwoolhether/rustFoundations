# Disable automatic .git:
# cargo new --vcs none project_name

# Cargo run by default runs everything in debug mode.
run:
	cargo run

# To skip debugging checks:
run-release:
	cargo run --release