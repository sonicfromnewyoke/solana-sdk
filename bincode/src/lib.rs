//! Contains a single utility function for deserializing from [bincode].
//!
//! [bincode]: https://docs.rs/bincode
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

use solana_instruction_error::InstructionError;

/// Deserialize with a limit based the maximum amount of data a program can expect to get.
/// This function should be used in place of direct deserialization to help prevent OOM errors
pub fn limited_deserialize<T, const L: usize>(
    instruction_data: &[u8],
) -> Result<T, InstructionError>
where
    T: serde_core::de::DeserializeOwned,
{
    let cfg = bincode::config::legacy()
        .with_limit::<L>()
        .with_fixed_int_encoding();

    bincode::serde::decode_from_slice(instruction_data, cfg)
        .map(|(value, _)| value)
        .map_err(|_| InstructionError::InvalidInstructionData)
}

#[cfg(test)]
pub mod tests {
    use {super::*, solana_system_interface::instruction::SystemInstruction};

    #[test]
    fn test_limited_deserialize_advance_nonce_account() {
        let item = SystemInstruction::AdvanceNonceAccount;
        let mut serialized =
            bincode::serde::encode_to_vec(&item, bincode::config::legacy()).unwrap();

        assert_eq!(
            serialized.len(),
            4,
            "`SanitizedMessage::get_durable_nonce()` may need a change"
        );

        assert_eq!(
            limited_deserialize::<SystemInstruction, 4>(&serialized).as_ref(),
            Ok(&item)
        );
        assert!(limited_deserialize::<SystemInstruction, 3>(&serialized).is_err());

        serialized.push(0);
        assert_eq!(
            limited_deserialize::<SystemInstruction, 4>(&serialized).as_ref(),
            Ok(&item)
        );
    }
}
