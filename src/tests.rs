// //! Demo: fold two invocations of `circuits/gmul::GhashMul`, each multiplying (X=1)(Y=1).
// use std::{collections::HashMap, path::PathBuf};

// use log::debug;
// use serde_json::{json, Value};
// use tk_program::ProgramTrace;

// // circom circuit compilation artifacts
// const GMUL_R1CS: &[u8] = include_bytes!("entry.r1cs");
// const GMUL_WITNESS_GENERATOR: &[u8] = include_bytes!("entry.bin");
// const JSON_MAX_ROM_LENGTH: usize = 35; // TODO(TK 2024-10-23): doc
// const MAX_ROM_LENGTH: usize = 35; // TODO(TK 2024-10-23): doc

// // X = Y = X * Y = 1
// const X: [u8; 16] = [
//     0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
// 0x00, ];
// const Y: [u8; 16] = [
//     0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
// 0x00, ];
// const INPUTS: [[u8; 16]; 2] = [X, Y];

// /// Setup data for a program to fold, including:
// /// - the R1CS layout of the circuits to fold, loaded as bytes from a Circom r1cs file
// /// - the type of witnesses generated (wasm, raw bytes, etc.)
// /// - the maximum length of the ROM (?) TODO(TK 2024-10-23): unsure
// #[derive(Clone, Debug)]
// pub struct SetupData {
//     pub r1cs_types:              Vec<R1CSType>,
//     pub witness_generator_types: Vec<WitnessGeneratorType>,
//     // TODO(TK 2024-10-23): how to obtain?
//     pub max_rom_length:          usize,
// }

// impl Default for SetupData {
//     fn default() -> Self {
//         Self {
//             r1cs_types:              vec![R1CSType::Raw(GMUL_R1CS.to_vec())],
//             witness_generator_types: vec![WitnessGeneratorType::Raw(
//                 GMUL_WITNESS_GENERATOR.to_vec(),
//                 // GMUL_WITNESS_GENERATOR.to_vec(),
//             )],
//             max_rom_length:          MAX_ROM_LENGTH,
//         }
//     }
// }

// impl SetupData {
//     // ref: https://github.com/pluto/web-prover/blob/main/proofs/src/tests/witnesscalc.rs#L31
//     // ref: https://github.com/pluto/web-prover/blob/main/proofs/src/tests/mod.rs#L160
//     fn run(self) -> tk_program::ProgramTrace {
//         debug!("load the external inputs and rom data");
//         // gunk
//         let private_input =
//             vec![("external".to_string(), json!(INPUTS))].into_iter().collect::<HashMap<_, _>>();

//         let instruction_config = InstructionConfig { name: "GhashMul".to_string(), private_input
// };

//         // two instances of the gfmul program to fold into one another:
//         let rom = vec![instruction_config.clone(), instruction_config];

//         // let public_params = program::setup(&self);
//         // let program_data = ProgramData::<Online, NotExpanded> {
//         //   public_params,
//         //   setup_data,
//         //   template_name // rom_data,
//         //   rom,
//         //   initial_nivc_input: INIT_PUBLIC_INPUT.to_vec(),
//         //   inputs: HashMap::new(),
//         //   witnesses: vec![],
//         // }
//         // .into_expanded();

//         // debug!("run snark");
//         // let recursive_snark = program::run(&program_data);

//         // debug!("return data and snark");
//         // ProgramTrace{program_data, recursive_snark}
//         todo!()
//     }
// }

// /// R1CS may be loaded by raw bytes or from filepath
// #[derive(Clone, Debug)]
// pub enum R1CSType {
//     // TODO(TK 2024-10-23): clean: if we're not using it, remove it
//     // File {
//     //     path: PathBuf,
//     // },
//     Raw(Vec<u8>),
// }

// // TODO(TK 2024-10-23): clean, doc
// #[derive(Clone, Debug)]
// pub enum WitnessGeneratorType {
//     // #[serde(skip)]
//     Raw(Vec<u8>),
//     // #[serde(rename = "wasm")]
//     // Wasm { path: String, wtns_path: String },
//     // #[serde(rename = "circom-witnesscalc")]
//     // CircomWitnesscalc { path: String },
//     // #[serde(rename = "browser")] // TODO: Can we merge this with Raw?
//     // Browser,
//     // #[serde(rename = "mobile")]
//     // Mobile { circuit: String },
//     // TODO: Would prefer to not alloc here, but i got lifetime hell lol
//     // #[serde(skip)]
//     // RustWitness(fn(&str) -> Vec<F<G1>>),
// }

// pub mod tk_program {
//     //! ref: https://github.com/pluto/web-prover/blob/main/proofs/src/program/data.rs
//     //! ref: https://github.com/pluto/web-prover/blob/main/proofs/src/lib.rs
//     use std::collections::HashMap;

//     use log::debug;
//     use proving_ground::{
//         provider::{hyperkzg::EvaluationEngine, Bn256EngineIPA, Bn256EngineKZG, GrumpkinEngine},
//         spartan::batched::BatchedRelaxedR1CSSNARK,
//         supernova::{snark::CompressedSNARK, PublicParams, RecursiveSNARK, TrivialCircuit},
//         traits::{Engine, Group},
//     };
//     use serde_json::{json, Value};

//     use self::{setup_typestate::*, witness_typestate::*};
//     use super::SetupData;

//     // TODO(TK 2024-10-23): doc: pasted from Arecibo
//     pub type E1 = Bn256EngineIPA;
//     pub type E2 = GrumpkinEngine;
//     pub type G1 = <E1 as Engine>::GE;
//     pub type G2 = <E2 as Engine>::GE;
//     // pub type EE1 = EvaluationEngine<halo2curves::bn256::Bn256, E1>;
//     pub type EE1 = proving_ground::provider::ipa_pc::EvaluationEngine<E1>;
//     pub type EE2 = proving_ground::provider::ipa_pc::EvaluationEngine<E2>;
//     pub type S1 = BatchedRelaxedR1CSSNARK<E1, EE1>;
//     pub type S2 = BatchedRelaxedR1CSSNARK<E2, EE2>;
//     pub type F<G> = <G as Group>::Scalar;

//     // TODO(TK 2024-10-23): doc
//     // thor wrote this one
//     pub struct ProgramTrace {
//         pub program_data:    ProgramData<Online, Expanded>,
//         pub recursive_snark: RecursiveSNARK<E1>,
//     }

//     // TODO(TK 2024-10-23): doc
//     pub struct ProgramData<S: SetupStatus, W: WitnessStatus> {
//         pub public_params:      S::PublicParams,
//         pub setup_data:         SetupData,
//         pub rom_data:           HashMap<String, CircuitData>,
//         pub rom:                Vec<InstructionConfig>,
//         pub initial_nivc_input: Vec<u64>,
//         pub inputs:             W::PrivateInputs,
//         pub witnesses:          Vec<Vec<F<G1>>>, // TODO: Ideally remove this
//     }

//     // type nonsense, kindof overkill
//     pub mod setup_typestate {
//         //! ProgramData setup typestate: {Online, Offline}
//         use super::*;
//         pub trait SetupStatus {
//             type PublicParams;
//         }
//         pub struct Online;
//         impl SetupStatus for Online {
//             type PublicParams = PublicParams<E1>;
//         }
//         pub struct Offline;
//         impl SetupStatus for Offline {
//             // type PublicParams = PathBuf;
//             type PublicParams = Vec<u8>;
//         }
//     }

//     pub mod witness_typestate {
//         //! witness typestate: {Expanded, NotExpanded}
//         use super::*;

//         pub trait WitnessStatus {
//             type PrivateInputs;
//         }
//         pub struct Expanded;
//         impl WitnessStatus for Expanded {
//             type PrivateInputs = Vec<HashMap<String, Value>>;
//         }
//         pub struct NotExpanded;
//         impl WitnessStatus for NotExpanded {
//             type PrivateInputs = HashMap<String, FoldInput>;
//         }
//     }

//     // TODO(TK 2024-10-23): doc
//     #[derive(Clone, Debug)]
//     pub struct CircuitData {
//         pub opcode: u64,
//     }

//     // TODO(TK 2024-10-23): doc
//     #[derive(Clone, Debug)]
//     pub struct InstructionConfig {
//         pub name:          String,
//         pub private_input: HashMap<String, Value>,
//     }

//     // TODO(TK 2024-10-23): doc
//     /// Circuit input?
//     #[derive(Clone, Debug)]
//     pub struct FoldInput {
//         // #[serde(flatten)]
//         pub value: HashMap<String, Vec<Value>>,
//     }

//     impl FoldInput {
//         /// Split `value` into `freq` chunks and return a vector of `HashMap`s
//         pub fn split_values(&self, freq: usize) -> Vec<HashMap<String, Value>> {
//             let mut res = vec![HashMap::new(); freq];

//             for (key, value) in &self.value {
//                 // debug!("key: {:?}, freq: {}, value_len: {}", key, freq, value.len());
//                 assert_eq!(value.len() % freq, 0);
//                 let chunk_size = value.len() / freq;
//                 let chunks: Vec<Vec<Value>> =
//                     value.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect();
//                 for i in 0..freq {
//                     res[i].insert(key.clone(), json!(chunks[i].clone()));
//                 }
//             }

//             res
//         }
//     }
// }

// #[test]
// fn test_setup() {
//     let data = SetupData::default();
//     let ProgramTrace { program_data, recursive_snark } = data.run();

//     todo!()
//     // let final_mem = [
//     //   F::<G1>::from(37),
//     //   F::<G1>::from(484),
//     //   F::<G1>::from(6),
//     //   F::<G1>::from(0),
//     //   F::<G1>::from(1),
//     //   F::<G1>::from(2),
//     //   F::<G1>::from(0),
//     //   F::<G1>::from(1),
//     //   F::<G1>::from(2),
//     //   F::<G1>::from(u64::MAX),
//     //   F::<G1>::from(u64::MAX),
//     //   F::<G1>::from(u64::MAX),
//     //   F::<G1>::from(u64::MAX),
//     // ];
//     // assert_eq!(&final_mem.to_vec(), proof.zi_primary());
// }
