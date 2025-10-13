# Helios SP1 program

## Build the program from the program dir run:
```bash
cd program && cargo prove build --elf-name sp1-helios-elf --docker --tag v5.2.1 --output-directory ../elf
```

## To get the verification key run:
```bash
cargo run --bin vkey
```