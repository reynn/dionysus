build:
	cargo build $(BUILD_FLAGS)

build-release:
	$(MAKE) build BUILD_FLAGS=--release

publish:
	cargo publish
