TARGET_DIR = target

.PHONY: all
all: build

.PHONY: build
build:
	cargo build


.PHONY: check
check:
	cargo check


.PHONY: fmt
fmt:
	cargo +nightly fmt


.PHONY: lint
lint:
	cargo +nightly fmt -- --check
	cargo clippy -- -D warnings


.PHONY: test
test:
	# cargo test --doc --all
	cargo test --all


.PHONY: clean
clean:
	rm -rf $(TARGET_DIR)
