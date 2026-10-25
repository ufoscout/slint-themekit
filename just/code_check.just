# Formats the code
[group('code_check')]
fmt args="":
  cargo fmt --all {{args}}


# Runs clippy on the code
[group('code_check')]
clippy args="":
  cargo clippy --workspace --all-features --all-targets {{args}}


# Compiles the .slint files and fails on any Slint compiler warning
[group('code_check')]
slint_check:
  SLINT_COMPILER_DENY_WARNINGS=1 cargo check --workspace --all-features --all-targets


# Runs fmt, clippy and the Slint check on the code. It fails if any of the checks fail
[group('code_check')]
check_code:
    just fmt "--check"
    just clippy "-- -D warnings"
    just slint_check