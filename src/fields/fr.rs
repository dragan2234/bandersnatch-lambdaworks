
//! Scalar field of bandersantch.

use lambdaworks_math::{
    field::{
        element::FieldElement,
        fields::montgomery_backed_prime_fields::{IsModulus, MontgomeryBackendPrimeField},
    },
    unsigned_integer::element::U256,
};

#[derive(Clone, Debug)]
pub struct FrConfig;

/// Modulus of bandersnatch subgroup
impl IsModulus<U256> for FrConfig {
    const MODULUS: U256 = U256::from_hex_unchecked(
        "1CFB69D4CA675F520CCE760202687600FF8F87007419047174FD06B52876E7E1",
    );
}

/// FrField using MontgomeryBackend for bandersnatch
pub type FrField = MontgomeryBackendPrimeField<FrConfig, 4>;
/// FrElement using MontgomeryBackend for bandersnatch
pub type FrElement = FieldElement<FrField>;
