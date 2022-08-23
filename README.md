# bl602-hardfloat
Hard float support for the bl602

Must be on nightly (to build core for a custom target)

Run with hard float support (default custom target lives in ./cargo/config.toml):

```cargo build --release```

Run with soft float:

```cargo build --release --target riscv32imac-unknown-none-elf```
