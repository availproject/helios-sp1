# Helios SP1 program

## Build the program from the program dir run:
```bash
cd program && cargo prove build --elf-name sp1-helios-elf --docker --output-directory ../elf
```

## To get the verification key run:
```bash
cargo run --bin vkey
```