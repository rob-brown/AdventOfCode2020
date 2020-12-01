run:
	@cargo run

build:
	@cargo build

release:
	@cargo build --release

time: release
	@time target/release/advent_of_code_2020

format:
	@cargo fmt

