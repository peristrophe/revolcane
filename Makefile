
.PHONY: run

run:
	-rm -rf ./target
	mold --run cargo run src/main.rs
