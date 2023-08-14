include database/Makefile

.PHONY: run

run:
	mold --run cargo run src/main.rs
