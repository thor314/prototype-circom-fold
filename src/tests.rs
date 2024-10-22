use std::collections::HashMap;

use log::debug;
use proofs::program::{self, data::{CircuitData, InstructionConfig, R1CSType, SetupData, WitnessGeneratorType}};
use serde_json::json;

const ENTRY_EXTERNAL_R1CS: &[u8] = include_bytes!("entry.r1cs");
const ENTRY_WITNESS_GENERATOR: &[u8] = include_bytes!("entry.bin");
const JSON_MAX_ROM_LENGTH: usize = 35;

const MUL_X: (&str, [u8; 16]) =
    ("X", [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

const MUL_Y: (&str, [u8; 16]) =
    ("Y", [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

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
          (String::from(MUL_X.0), json!(MUL_X.1)),
          (String::from(MUL_Y.0), json!(MUL_Y.1)),
        ]),
      };

}