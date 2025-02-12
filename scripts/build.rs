#[allow(unused_imports)]
use sp1_build::{build_program_with_args, BuildArgs};

fn main() {
    build_program_with_args("../program", BuildArgs {
        tag: "v4.0.0".to_string(),
        docker: true,
        elf_name: Some("sp1-helios-elf".to_string()),
        ..Default::default()
    });
}