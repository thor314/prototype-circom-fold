//! Demo: fold two invocations of `circuits/gmul::GhashMul`, each multiplying (X=1)(Y=1).
use std::{collections::HashMap, path::PathBuf};

use log::debug;
use proofs::{program::data::*, E1, *};
use proving_ground::supernova::RecursiveSNARK;
// {Online, ProgramData, R1CSType, SetupData, WitnessGeneratorType};
use serde_json::{json, Value};

// circom circuit compilation artifacts
const GMUL_R1CS: &[u8] = include_bytes!("entry.r1cs");
const GMUL_WITNESS_GENERATOR: &[u8] = include_bytes!("entry.bin");
const JSON_MAX_ROM_LENGTH: usize = 35; // TODO(TK 2024-10-23): doc

// what is this colin i haven't any idea
// "what circuits are being dispatched"
// if you think of supernova as a rom machine, then what it does is start at idx 0,
// execute that circuit, passes the state into the next circuit
// whatever the next circuit will be is decided  by that rom
//
// if we have 2 circuits
// idx is 0,1
// then the rom is [0,1,0,1,0,1]
//
// you should think of the rom as what opcodes will be used,
// and those op codes are the circuits and their private inputs

/// The maximum number of different circuits that can be used in a supernova configuration
///
/// i.e. (MAX_ROM_LENGTH-1) is the maxmimum number of folds that can be performed
/// on N=MAX_ROM_LENGTH circuits
const MAX_ROM_LENGTH: usize = 5; // TODO(TK 2024-10-23): doc

// The Mul to perform: X = Y = 1 * 1 = 1
const X: [u8; 16] = [
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
const Y: [u8; 16] = [
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
const INPUTS: [[u8; 16]; 2] = [X, Y];

fn setup_data() -> SetupData {
    SetupData {
        r1cs_types:              vec![R1CSType::Raw(GMUL_R1CS.to_vec())],
        witness_generator_types: vec![WitnessGeneratorType::Raw(GMUL_WITNESS_GENERATOR.to_vec())],
        max_rom_length:          MAX_ROM_LENGTH,
    }
}

fn run_entry(setup_data: SetupData) -> (ProgramData<Online, Expanded>, RecursiveSNARK<E1>) {
    let private_input = [("X".to_string(), json!(X)), ("Y".to_string(), json!(Y))]
        .iter()
        .cloned()
        .collect::<HashMap<String, Value>>();

    // generate public params for the circuit from the setup data
    let public_params = program::setup(&setup_data);

    let rom_data = HashMap::from([("GMUL".to_string(), CircuitData { opcode: 0 })]);

    let rom = vec![InstructionConfig { name: "GMUL".to_string(), private_input }];

    let program_data = ProgramData::<Online, NotExpanded> {
        public_params,
        setup_data,
        rom_data,
        rom,
        // initialize step_in:
        initial_nivc_input: vec![0],
        inputs: HashMap::new(),
        witnesses: vec![],
    }
    .into_expanded();
    let recursive_snark = program::run(&program_data);
    (program_data, recursive_snark)
}

// X * Y = 1 * 1 = 1
#[test]
fn test_just_once() {
    let setup_data = setup_data();
    let (_, proof) = run_entry(setup_data);
    let mem = [
        // step_out
        F::<G1>::from(0),
        F::<G1>::from(1),
    ];

    assert_eq!(&mem.to_vec(), proof.zi_primary());
}
