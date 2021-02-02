MAKE=make
CARGO=cargo

BENCHES=

all: rebuild-protos

rebuild-protos:
	@$(CARGO) fmt && BUILD_PROTO=1 $(CARGO) build

fmt:
	@echo "Running cargo fmt..."
	@$(CARGO) fmt --all -v

lint:
	@echo "Running cargo check & clippy..."
	@cargo check && cargo clippy --all-targets --all-features --tests --benches -- -D warnings

bench: $(BENCHES)

$(BENCHES):
	@$(CARGO) bench --bench $@

test:
	@$(CARGO) test
