build:
	cargo build $(BUILD_FLAGS)

build-release:
	$(MAKE) build BUILD_FLAGS=--release

publish:
	cargo publish

test:
	cargo test

install:
	cargo install --path=crates/dionysus-web
