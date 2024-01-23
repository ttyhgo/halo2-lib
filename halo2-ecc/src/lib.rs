#![allow(clippy::too_many_arguments)]
#![allow(clippy::op_ref)]
#![allow(clippy::type_complexity)]
#![allow(unused_imports)]
#![allow(stable_features)]
#![feature(int_log)]
#![feature(trait_alias)]

pub mod bigint;
pub mod ecc;
pub mod fields;

pub mod bn254;
pub mod secp256k1;
pub mod secp256r1;

pub use halo2_base;
pub(crate) use halo2_base::halo2_proofs;
