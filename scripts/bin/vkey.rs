use anyhow::Result;
use sp1_sdk::blocking::{Prover, ProverClient};
use sp1_sdk::{Elf, HashableKey, ProvingKey};

const HELIOS_ELF: &[u8] = include_bytes!("../../elf/sp1-helios-elf");

fn main() -> Result<()> {
    let client = ProverClient::builder().mock().build();
    let pk = client
        .setup(Elf::Static(HELIOS_ELF))
        .expect("Failed to setup prover");
    println!("SP1 Helios Verifying Key: {:?}", pk.verifying_key().bytes32());
    Ok(())
}
