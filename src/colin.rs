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
const X: [u8; 16] = [
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
const Y: [u8; 16] = [
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
const INPUTS: [[u8; 16]; 2] = [X, Y];
