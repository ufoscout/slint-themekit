
# Run all tests
[group('test')]
test test_name="":
  cargo test run {{test_name}} --workspace --all-features 
