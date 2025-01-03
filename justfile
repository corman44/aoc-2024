# Use `just work day-01 part1` to work on the specific binary for a specific day's problems
work day part:
    cargo watch -w {{day}} -x "check -p {{day}}" -s "just test {{day}} {{part}}" -s "just lint {{day}}" -s "just bench {{day}} {{part}}"
www-watch:
   RUST_LOG=info cargo +nightly leptos watch --project www
www-build:
   cargo +nightly leptos build --project www --release
lint day:
    cargo clippy -p {{day}}
test day part:
    cargo test -p {{day}} {{part}} -- --nocapture
bench-all:
    cargo bench -q >> "benchmarks.txt"
bench day part:
    cargo bench --bench {{day}}-bench {{part}} >> bench/{{day}}.bench.txt
flamegraph day part:
    cargo flamegraph --profile flamegraph --root --package {{day}} --bin {{part}} -o flamegraphs/{{day}}--{{part}}.svg
dhat day part:
    cargo run --profile dhat --features dhat-heap --package {{day}} --bin {{part}}
run day part:
    cargo run --package {{day}} --bin {{part}}
run-relase day part:
    cargo run --package {{day}} --bin {{part}} --release
# creates day directory and fills inputs
create day:
    cargo generate --path ./template --name {{day}}
    just get-input {{day}}
# fill text inputs from AOC
get-input day:
    cargo run --package day-libs --bin input-filler -- --day {{day}} --cwd "{{justfile_directory()}}"