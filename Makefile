release-macos-arm64:
	@cargo build --release --target aarch64-apple-darwin
	@strip target/aarch64-apple-darwin/release/metamask
	@CARGO_BUNDLE_SKIP_BUILD=1 cargo bundle --release --target aarch64-apple-darwin

release-macos-x86_64:
	@cargo build --release --target x86_64-apple-darwin
	@strip target/x86_64-apple-darwin/release/metamask
	@CARGO_BUNDLE_SKIP_BUILD=1 cargo bundle --release --target x86_64-apple-darwin

.PHONY: release-macos-arm64 release-macos-x86_64
