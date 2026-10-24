
# Publish the crate
[group('publish')]
publish: build test
  cargo publish -p slint-themekit

