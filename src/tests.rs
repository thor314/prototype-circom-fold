//! Demo: fold two invocations of `circuits/gmul::GhashMul`, each multiplying (X=1)(Y=1).
use std::{collections::HashMap, path::PathBuf};

use log::debug;
use serde_json::{json, Value};

// circom circuit compilation artifacts
const GMUL_R1CS: &[u8] = include_bytes!("entry.r1cs");
const GMUL_WITNESS_GENERATOR: &[u8] = include_bytes!("entry.bin");
const JSON_MAX_ROM_LENGTH: usize = 35; // TODO(TK 2024-10-23): doc
const MAX_ROM_LENGTH: usize = 35; // TODO(TK 2024-10-23): doc

// X = Y = X * Y = 1
const X: (&str, [u8; 16]) = ("X", [
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
]);

const Y: (&str, [u8; 16]) = ("Y", [
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
]);

/// Setup data for a program to fold, including:
/// - the R1CS layout of the circuits to fold, loaded as bytes from a Circom r1cs file
/// - the type of witnesses generated (wasm, raw bytes, etc.)
/// - the maximum length of the ROM (?) TODO(TK 2024-10-23): unsure
#[derive(Clone, Debug)]
pub struct SetupData {
    pub r1cs_types:              Vec<R1CSType>,
    pub witness_generator_types: Vec<WitnessGeneratorType>,
    pub max_rom_length:          usize,
}

impl Default for SetupData {
    fn default() -> Self {
        Self {
            r1cs_types:              vec![R1CSType::Raw(GMUL_R1CS.to_vec())],
            witness_generator_types: vec![WitnessGeneratorType::Raw(
                GMUL_WITNESS_GENERATOR.to_vec(),
                // GMUL_WITNESS_GENERATOR.to_vec(),
            )],
            max_rom_length:          MAX_ROM_LENGTH,
        }
    }
}

impl SetupData {
    fn run(self) -> program::ProgramTrace {
        //
        todo!()
    }
}

/// R1CS may be loaded by raw bytes or from filepath
#[derive(Clone, Debug)]
pub enum R1CSType {
    // TODO(TK 2024-10-23): clean: if we're not using it, remove it
    // File {
    //     path: PathBuf,
    // },
    Raw(Vec<u8>),
}

// TODO(TK 2024-10-23): clean, doc
#[derive(Clone, Debug)]
pub enum WitnessGeneratorType {
    // #[serde(skip)]
    Raw(Vec<u8>),
    // #[serde(rename = "wasm")]
    // Wasm { path: String, wtns_path: String },
    // #[serde(rename = "circom-witnesscalc")]
    // CircomWitnesscalc { path: String },
    // #[serde(rename = "browser")] // TODO: Can we merge this with Raw?
    // Browser,
    // #[serde(rename = "mobile")]
    // Mobile { circuit: String },
    // TODO: Would prefer to not alloc here, but i got lifetime hell lol
    // #[serde(skip)]
    // RustWitness(fn(&str) -> Vec<F<G1>>),
}

pub mod program {
    use std::collections::HashMap;

    use log::debug;
    use proving_ground::{
        provider::{hyperkzg::EvaluationEngine, Bn256EngineIPA, Bn256EngineKZG, GrumpkinEngine},
        spartan::batched::BatchedRelaxedR1CSSNARK,
        supernova::{snark::CompressedSNARK, PublicParams, RecursiveSNARK, TrivialCircuit},
        traits::{Engine, Group},
    };
    use serde_json::{json, Value};

    use super::SetupData;

    // TODO(TK 2024-10-23): doc: pasted from Arecibo
    pub type E1 = Bn256EngineIPA;
    pub type E2 = GrumpkinEngine;
    pub type G1 = <E1 as Engine>::GE;
    pub type G2 = <E2 as Engine>::GE;
    // pub type EE1 = EvaluationEngine<halo2curves::bn256::Bn256, E1>;
    pub type EE1 = proving_ground::provider::ipa_pc::EvaluationEngine<E1>;
    pub type EE2 = proving_ground::provider::ipa_pc::EvaluationEngine<E2>;
    pub type S1 = BatchedRelaxedR1CSSNARK<E1, EE1>;
    pub type S2 = BatchedRelaxedR1CSSNARK<E2, EE2>;
    pub type F<G> = <G as Group>::Scalar;

    // TODO(TK 2024-10-23): doc
    pub struct ProgramTrace {
        program_data:    ProgramData<Online, Expanded>,
        recursive_snark: RecursiveSNARK<E1>,
    }

    // TODO(TK 2024-10-23): doc
    pub struct ProgramData<S: SetupStatus, W: WitnessStatus> {
        pub public_params:      S::PublicParams,
        pub setup_data:         SetupData,
        pub rom_data:           HashMap<String, CircuitData>,
        pub rom:                Vec<InstructionConfig>,
        pub initial_nivc_input: Vec<u64>,
        pub inputs:             W::PrivateInputs,
        pub witnesses:          Vec<Vec<F<G1>>>, // TODO: Ideally remove this
    }

    // Note, the below are typestates that prevent misuse of our current API.
    /// ProgramData setup typestate: {Online, Offline}
    pub trait SetupStatus {
        type PublicParams;
    }
    pub struct Online;
    impl SetupStatus for Online {
        type PublicParams = PublicParams<E1>;
    }
    pub struct Offline;
    impl SetupStatus for Offline {
        // type PublicParams = PathBuf;
        type PublicParams = Vec<u8>;
    }

    /// ProgramData witness typestate: {Expanded, NotExpanded}
    pub trait WitnessStatus {
        type PrivateInputs;
    }
    pub struct Expanded;
    impl WitnessStatus for Expanded {
        type PrivateInputs = Vec<HashMap<String, Value>>;
    }
    pub struct NotExpanded;
    impl WitnessStatus for NotExpanded {
        type PrivateInputs = HashMap<String, FoldInput>;
    }

    // TODO(TK 2024-10-23): doc
    #[derive(Clone, Debug)]
    pub struct CircuitData {
        pub opcode: u64,
    }

    #[derive(Clone, Debug)]
    pub struct InstructionConfig {
        pub name:          String,
        pub private_input: HashMap<String, Value>,
    }

    // TODO(TK 2024-10-23): doc
    /// Circuit input?
    #[derive(Clone, Debug)]
    pub struct FoldInput {
        // #[serde(flatten)]
        pub value: HashMap<String, Vec<Value>>,
    }

    impl FoldInput {
        // Iterate over the entries in self.value, using fold to accumulate results
        pub fn split_values(&self, freq: usize) -> Vec<HashMap<String, Value>> {
            self.value.iter().flat_map(|(key, value)| {
                debug!("key: {:?}, freq: {}, value_len: {}", key, freq, value.len());

                // Validate that the value can be evenly split into `freq` parts
                assert_eq!(value.len() % freq, 0, "value length must be divisible by freq");

                let chunk_size = value.len() / freq;

                // Create an iterator over the chunks and map each to a HashMap entry
                value.chunks(chunk_size).enumerate().map(move |(i, chunk)| {
                    let mut map = HashMap::new();
                    map.insert(key.clone(), json!(chunk.to_vec()));
                    (i, map)
                })
            })
            // Aggregate the maps into a vector of HashMaps, merging by index
            .fold(vec![HashMap::new(); freq], |mut acc, (i, map)|  {

                acc[i].extend(map); acc  } )
        }
    }
}

// TODO(TK 2024-10-23): unsure
// pub struct Entry {
// }

// #[test]
// fn test_setup() {
//     let setup_data = SetupData {
//         r1cs_types:              vec![R1CSType::Raw(ENTRY_EXTERNAL_R1CS.to_vec())],
//         witness_generator_types:
// vec![WitnessGeneratorType::Raw(ENTRY_WITNESS_GENERATOR.to_vec())],         max_rom_length:
// JSON_MAX_ROM_LENGTH,     };

//     debug!("Setting up `Memory`...");
//     let public_params = program::setup(&setup_data);

//     debug!("Creating ROM");
//     let rom_data = HashMap::from([(String::from("entry"), CircuitData { opcode: 0 })]);

//     let entry_rom_opcode_config = InstructionConfig {
//         name:          String::from("entry"),
//         private_input: HashMap::from([
//             (String::from(MUL_X.0), json!(MUL_X.1)),
//             (String::from(MUL_Y.0), json!(MUL_Y.1)),
//         ]),
//     };

//     debug!("Creating `private_inputs`...");
//     let mut rom = [entry_rom_opcode_config];

//     let inputs = HashMap::from([(String::from("AES_GCM_1"), FoldInput {
//         value: HashMap::from([(
//             String::from(AES_PLAINTEXT.0),
//             AES_PLAINTEXT.1.iter().map(|val| json!(val)).collect::<Vec<Value>>(),
//         )]),
//     })]);

//     let mut initial_nivc_input = AES_BYTES.to_vec();
//     initial_nivc_input.extend(AES_PLAINTEXT.1.iter());
//     initial_nivc_input.resize(4160, 0); // TODO: This is currently the `TOTAL_BYTES` used in
// circuits     let initial_nivc_input = initial_nivc_input.into_iter().map(u64::from).collect();
//     let program_data = ProgramData::<Online, NotExpanded> {
//         public_params,
//         setup_data,
//         rom_data,
//         rom: rom.to_vec(),
//         initial_nivc_input,
//         inputs,
//         witnesses: vec![],
//     }
//     .into_expanded();
//     debug!("program_data.inputs: {:?}, {:?}", program_data.inputs.len(),
// program_data.inputs[15]);

//     let recursive_snark = program::run(&program_data);
// }
