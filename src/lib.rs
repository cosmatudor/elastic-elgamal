//! [ElGamal encryption] and related cryptographic protocols with pluggable crypto backend.
//!
//! # ⚠ Warnings
//!
//! While the logic in this crate relies on standard cryptographic assumptions
//! (complexity of discrete log and computational / decisional Diffie–Hellman problems
//! in certain groups), it has not been independently verified for correctness or absence
//! of side-channel attack vectors. **Use at your own risk.**
//!
//! ElGamal encryption is not a good choice for general-purpose public-key encryption
//! since it is vulnerable to [chosen-ciphertext attacks][CCA]. For security,
//! decryption operations should be limited on the application level.
//!
//! # Overview
//!
//! - [`Ciphertext`] provides ElGamal encryption. This and other protocols use
//!   [`PublicKey`], [`SecretKey`] and [`Keypair`] to represent participants' keys.
//! - Besides basic encryption, `PublicKey` also provides zero-knowledge proofs of
//!   [zero encryption](PublicKey::encrypt_zero()) and of
//!   [Boolean value encryption](PublicKey::encrypt_bool()). These are useful in higher-level
//!   protocols, e.g., re-encryption.
//! - Zero-knowledge range proofs for ElGamal ciphertexts are provided via [`RangeProof`]s
//!   and a high-level [`PublicKey` method](PublicKey::encrypt_range()).
//! - [`EncryptedChoice`] provides a way to encrypt a choice of one of `n` variants so that
//!   variant ciphertexts are additively homomorphic and have zero-knowledge proof of correctness.
//! - [`sharing`](crate::sharing) module exposes a threshold encryption scheme based
//!   on [Feldman's verifiable secret sharing][feldman-vss], including verifiable distributed
//!   decryption.
//!
//! # Backends
//!
//! [`group`](crate::group) module exposes a generic framework for plugging a [`Group`]
//! implementation into crypto primitives. It also provides several implementations:
//!
//! - [`Ristretto`] and [`Curve25519Subgroup`] implementations based on Curve25519 using
//!   [`curve25519-dalek`].
//! - [`Generic`] implementation allowing to plug in any elliptic curve group conforming to
//!   the traits specified by the [`elliptic-curve`] crate. For example,
//!   the secp256k1 curve can be used via the [`k256`] crate.
//!
//! # Crate features
//!
//! ## `serde`
//!
//! *(off by default)*
//!
//! Enables [`Serialize`](::serde::Serialize) / [`Deserialize`](::serde::Deserialize)
//! implementations for most types in the crate.
//! Group scalars, elements and wrapper key types are serialized to human-readable formats
//! (JSON, YAML, TOML, etc.) as strings that represent corresponding byte buffers using
//! base64-url encoding without padding. For binary formats, byte buffers are serialized directly.
//!
//! For complex types (e.g., participant states from the [`sharing`] module), self-consistency
//! checks are **not** performed on deserialization. That is, deserialization of such types
//! should only be performed from a trusted source or in the presence of additional integrity
//! checks.
//!
//! # Crate naming
//!
//! "Elastic" refers to pluggable backends, configurable params for threshold encryption,
//! and the construction of zero-knowledge [`RingProof`]s (a proof consists of
//! a variable number of rings, each of which consists of a variable number of admissible values).
//! `elastic_elgamal` is also one of [autogenerated Docker container names][docker-rng].
//!
//! [ElGamal encryption]: https://en.wikipedia.org/wiki/ElGamal_encryption
//! [CCA]: https://en.wikipedia.org/wiki/Chosen-ciphertext_attack
//! [feldman-vss]: https://www.cs.umd.edu/~gasarch/TOPICS/secretsharing/feldmanVSS.pdf
//! [`Group`]: crate::group::Group
//! [`Ristretto`]: crate::group::Ristretto
//! [`Curve25519Subgroup`]: crate::group::Curve25519Subgroup
//! [`curve25519-dalek`]: https://docs.rs/curve25519-dalek/
//! [`Generic`]: crate::group::Generic
//! [`elliptic-curve`]: https://docs.rs/elliptic-curve/
//! [`k256`]: https://docs.rs/k256/
//! [docker-rng]: https://github.com/moby/moby/blob/master/pkg/namesgenerator/names-generator.go

#![cfg_attr(not(feature = "std"), no_std)]
// Documentation settings.
#![doc(html_root_url = "https://docs.rs/elastic-elgamal/0.1.0")]
// Linter settings.
#![warn(missing_debug_implementations, missing_docs, bare_trait_objects)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
    clippy::doc_markdown
)]

mod encryption;
pub mod group;
mod keys;
mod proofs;
#[cfg(feature = "serde")]
mod serde;
pub mod sharing;

// Polyfill for `alloc` types.
mod alloc {
    #[cfg(not(feature = "std"))]
    extern crate alloc;

    #[cfg(not(feature = "std"))]
    pub use alloc::{string::ToString, vec, vec::Vec};
    #[cfg(feature = "std")]
    pub use std::{string::ToString, vec, vec::Vec};
}

pub use crate::{
    encryption::{Ciphertext, DiscreteLogTable, EncryptedChoice},
    keys::{Keypair, PublicKey, PublicKeyConversionError, SecretKey},
    proofs::{
        LogEqualityProof, PreparedRange, ProofOfPossession, RangeDecomposition, RangeProof,
        RingProof, RingProofBuilder,
    },
};
