use std::collections::HashMap;

use log::debug;
use proofs::program::{self, data::{CircuitData, FoldInput, InstructionConfig, R1CSType, SetupData, WitnessGeneratorType}};
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

    let entry_rom_opcode_config = InstructionConfig {
        name:          String::from("entry"),
        private_input: HashMap::from([
          (String::from(MUL_X.0), json!(MUL_X.1)),
          (String::from(MUL_Y.0), json!(MUL_Y.1)),
        ]),
      };

      debug!("Creating `private_inputs`...");
      let mut rom = [entry_rom_opcode_config];

      let inputs = HashMap::from([(String::from("AES_GCM_1"), FoldInput {
        value: HashMap::from([(
          String::from(AES_PLAINTEXT.0),
          AES_PLAINTEXT.1.iter().map(|val| json!(val)).collect::<Vec<Value>>(),
        )]),
      })]);
    
      let mut initial_nivc_input = AES_BYTES.to_vec();
      initial_nivc_input.extend(AES_PLAINTEXT.1.iter());
      initial_nivc_input.resize(4160, 0); // TODO: This is currently the `TOTAL_BYTES` used in circuits
      let initial_nivc_input = initial_nivc_input.into_iter().map(u64::from).collect();
      let program_data = ProgramData::<Online, NotExpanded> {
        public_params,
        setup_data,
        rom_data,
        rom,
        initial_nivc_input,
        inputs,
        witnesses: vec![],
      }
      .into_expanded();
      debug!("program_data.inputs: {:?}, {:?}", program_data.inputs.len(), program_data.inputs[15]);
    
      let recursive_snark = program::run(&program_data);

}