#![allow(dead_code)]
//! Implementation of merkle_tree.

use alloc::string::String;

use alloc::vec::Vec;

use casper_contract::contract_api::runtime;

use casper_types::ApiError;
use tiny_keccak::Hasher;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Position {
    Left = 0,
    Right = 1,
}

impl Position {
    pub fn from_u8(value: u8) -> Position {
        match value {
            0 => Position::Left,
            1 => Position::Right,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

/// Verify leaf is in the tree
pub fn verify(root: String, leaf: String, proof: Vec<(String, u8)>) {
    let converted_proof: Vec<(Vec<u8>, Position)> = proof
        .iter()
        .map(|proof| {
            (
                hex::decode(proof.0.clone()).unwrap(),
                Position::from_u8(proof.1),
            )
        })
        .collect();

    let leaf_bytes: &[u8] = leaf.as_bytes();

    let mut keccak = tiny_keccak::Keccak::v256();
    let mut result: [u8; 32] = Default::default();
    keccak.update(leaf_bytes);
    keccak.finalize(&mut result);

    let mut computed_hash: &[u8; 32] = &result;
    let mut result: [u8; 32] = Default::default();
    for proof_item in converted_proof {
        match proof_item.1 {
            Position::Right => {
                let mut keccak = tiny_keccak::Keccak::v256();
                keccak.update(computed_hash);
                keccak.update(&proof_item.0);
                keccak.finalize(&mut result);
            }
            Position::Left => {
                let mut keccak = tiny_keccak::Keccak::v256();
                keccak.update(&proof_item.0);
                keccak.update(computed_hash);
                keccak.finalize(&mut result);
            }
        }
        computed_hash = &result;
    }

    let computed_root = hex::encode(*computed_hash);

    if !computed_root.eq(&root) {
        runtime::revert(ApiError::PermissionDenied);
    }
}
