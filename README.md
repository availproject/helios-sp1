# Helios SP1 program


## Build the program from the program dir run:
```bash
cargo prove build --docker --tag v4.0.0 --elf-name sp1-helios-elf --output-directory ../elf 
```

## To get the verification key run:
```bash
cargo run --bin vkey
```