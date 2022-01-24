# bl602-hardfloat
Hard float support for the bl602

Must be on nightly (to build core for a custom target)

Run with hard float support (default custom target lives in ./cargo/config.toml):

```cargo run --release -Z build-std=core```

Run with soft float:

```cargo run --release --target riscv32imac-unknown-none-elf -Z build-std=core```
