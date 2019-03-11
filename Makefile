
.PHONY: all
all: build

.PHONY: debug
debug:
	sudo cargo run 

.PHONY: build
build:
	cargo build --release
