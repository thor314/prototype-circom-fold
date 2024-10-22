use std::collections::HashMap;

use log::debug;
use proofs::program::{self, data::{CircuitData, InstructionConfig, R1CSType, SetupData, WitnessGeneratorType}};

const ENTRY_EXTERNAL_R1CS: &[u8] = include_bytes!("entry.r1cs");
const ENTRY_WITNESS_GENERATOR: &[u8] = include_bytes!("entry.bin");
const JSON_MAX_ROM_LENGTH: usize = 35;


#[test]
fn test_setup() {   
    let setup_data = SetupData {
        r1cs_types:              vec![
            R1CSType::Raw(ENTRY_EXTERNAL_R1CS.to_vec()),
        ],
        witness_generator_types: vec![
            WitnessGeneratorType::Raw(ENTRY_WITNESS_GENERATOR.to_vec()),
        ],
        max_rom_length: JSON_MAX_ROM_LENGTH,
    };

    debug!("Setting up `Memory`...");
    let public_params = program::setup(&setup_data);

    debug!("Creating ROM");
    let rom_data = HashMap::from([
      (String::from("entry"), CircuitData { opcode: 0 }),
    ]);

    let aes_rom_opcode_config = InstructionConfig {
        name:          String::from("AES_GCM_1"),
        private_input: HashMap::from([
          (String::from(AES_KEY.0), json!(AES_KEY.1)),
          (String::from(AES_IV.0), json!(AES_IV.1)),
          (String::from(AES_AAD.0), json!(AES_AAD.1)),
        ]),
      };

}