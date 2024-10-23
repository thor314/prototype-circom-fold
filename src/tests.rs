use std::collections::HashMap;

use log::debug;
use proofs::program::{
    self,
    data::{
        CircuitData, FoldInput, InstructionConfig, NotExpanded, Online, ProgramData, R1CSType,
        SetupData, WitnessGeneratorType,
    },
};
use serde_json::{json, Value};

const ENTRY_EXTERNAL_R1CS: &[u8] = include_bytes!("entry.r1cs");
const ENTRY_WITNESS_GENERATOR: &[u8] = include_bytes!("entry.bin");
const JSON_MAX_ROM_LENGTH: usize = 35;

const MUL_X: (&str, [u8; 16]) = ("X", [
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
]);

const MUL_Y: (&str, [u8; 16]) = ("Y", [
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
]);

const AES_BYTES: [u8; 50] = [0; 50];

const AES_PLAINTEXT: (&str, [u8; 320]) = ("plainText", [
    72, 84, 84, 80, 47, 49, 46, 49, 32, 50, 48, 48, 32, 79, 75, 13, 10, 99, 111, 110, 116, 101,
    110, 116, 45, 116, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110,
    47, 106, 115, 111, 110, 59, 32, 99, 104, 97, 114, 115, 101, 116, 61, 117, 116, 102, 45, 56, 13,
    10, 99, 111, 110, 116, 101, 110, 116, 45, 101, 110, 99, 111, 100, 105, 110, 103, 58, 32, 103,
    122, 105, 112, 13, 10, 84, 114, 97, 110, 115, 102, 101, 114, 45, 69, 110, 99, 111, 100, 105,
    110, 103, 58, 32, 99, 104, 117, 110, 107, 101, 100, 13, 10, 13, 10, 123, 13, 10, 32, 32, 32,
    34, 100, 97, 116, 97, 34, 58, 32, 123, 13, 10, 32, 32, 32, 32, 32, 32, 32, 34, 105, 116, 101,
    109, 115, 34, 58, 32, 91, 13, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 123, 13, 10, 32,
    32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 34, 100, 97, 116, 97, 34, 58, 32, 34,
    65, 114, 116, 105, 115, 116, 34, 44, 13, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
    32, 32, 32, 34, 112, 114, 111, 102, 105, 108, 101, 34, 58, 32, 123, 13, 10, 32, 32, 32, 32, 32,
    32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 34, 110, 97, 109, 101, 34, 58, 32, 34, 84, 97, 121,
    108, 111, 114, 32, 83, 119, 105, 102, 116, 34, 13, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
    32, 32, 32, 32, 32, 125, 13, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 125, 13, 10, 32,
    32, 32, 32, 32, 32, 32, 93, 13, 10, 32, 32, 32, 125, 13, 10, 125,
]);

//
#[test]
fn test_setup() {
    let setup_data = SetupData {
        r1cs_types:              vec![R1CSType::Raw(ENTRY_EXTERNAL_R1CS.to_vec())],
        witness_generator_types: vec![WitnessGeneratorType::Raw(ENTRY_WITNESS_GENERATOR.to_vec())],
        max_rom_length:          JSON_MAX_ROM_LENGTH,
    };

    debug!("Setting up `Memory`...");
    let public_params = program::setup(&setup_data);

    debug!("Creating ROM");
    let rom_data = HashMap::from([(String::from("entry"), CircuitData { opcode: 0 })]);

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
        rom: rom.to_vec(),
        initial_nivc_input,
        inputs,
        witnesses: vec![],
    }
    .into_expanded();
    debug!("program_data.inputs: {:?}, {:?}", program_data.inputs.len(), program_data.inputs[15]);

    let recursive_snark = program::run(&program_data);
}
