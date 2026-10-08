use alloy_consensus::Header;
use alloy_primitives::{keccak256, B256};
use alloy_rlp::Decodable;
use helios_consensus_core::types::LightClientHeader;

/// Execution state root of a finalized light-client header.
///
/// Up to Fulu the light-client header carries the execution payload header, which holds the state
/// root. From Gloas (ePBS) it only commits to the execution block hash, so the RLP-encoded
/// execution block header must be supplied and must hash to that block hash before its state root
/// is trusted.
///
/// # Panics
///
/// Panics if a Gloas header comes without an execution block header, if the supplied header does
/// not hash to the committed block hash, if it is not a valid header, or if a pre-Gloas header has
/// no execution payload.
pub fn execution_state_root(
    header: &LightClientHeader,
    execution_block_header: Option<&[u8]>,
) -> B256 {
    match header {
        LightClientHeader::Gloas(gloas) => {
            let rlp = execution_block_header
                .expect("Gloas finalized header requires the execution block header.");
            assert_eq!(
                keccak256(rlp),
                gloas.execution_block_hash,
                "Execution block header does not hash to the finalized execution block hash."
            );
            let mut buf = rlp;
            let decoded = Header::decode(&mut buf).expect("Invalid execution block header RLP.");
            assert!(
                buf.is_empty(),
                "Trailing bytes after the execution block header."
            );
            decoded.state_root
        }
        other => *other
            .execution()
            .expect("Execution payload doesn't exist.")
            .state_root(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::b256;
    use helios_consensus_core::types::{LightClientHeaderBellatrix, LightClientHeaderGloas};

    /// Sepolia block 11869333 (after the Gloas fork): header RLP from `debug_getRawHeader`.
    const HEADER_HEX: &str = include_str!("../testdata/sepolia-11869333-header.rlp.hex");
    const BLOCK_HASH: B256 =
        b256!("227b1ede5189c4171dbc80bb4d8ba3933c8f5be244cc3764ab6fbbb80099cc2a");
    const STATE_ROOT: B256 =
        b256!("e5e8c0c4de7b7f94700723f546900ec9ac8b857e93d59d18fcf06cdd95886d7d");

    fn header_rlp() -> Vec<u8> {
        alloy_primitives::hex::decode(HEADER_HEX.trim()).unwrap()
    }

    fn gloas(block_hash: B256) -> LightClientHeader {
        LightClientHeader::Gloas(LightClientHeaderGloas {
            execution_block_hash: block_hash,
            ..Default::default()
        })
    }

    #[test]
    fn gloas_returns_state_root_of_matching_header() {
        let rlp = header_rlp();
        assert_eq!(keccak256(&rlp), BLOCK_HASH);
        assert_eq!(
            execution_state_root(&gloas(BLOCK_HASH), Some(&rlp)),
            STATE_ROOT
        );
    }

    #[test]
    #[should_panic(expected = "does not hash to the finalized execution block hash")]
    fn gloas_rejects_header_for_another_block() {
        let rlp = header_rlp();
        execution_state_root(&gloas(B256::repeat_byte(1)), Some(&rlp));
    }

    #[test]
    #[should_panic(expected = "requires the execution block header")]
    fn gloas_requires_header() {
        execution_state_root(&gloas(BLOCK_HASH), None);
    }

    #[test]
    #[should_panic]
    fn gloas_rejects_trailing_bytes() {
        let mut rlp = header_rlp();
        rlp.push(0);
        // The hash check runs first, so pin the hash to the padded bytes to reach the decoder.
        execution_state_root(&gloas(keccak256(&rlp)), Some(&rlp));
    }

    #[test]
    #[should_panic(expected = "Execution payload doesn't exist.")]
    fn pre_capella_header_has_no_execution_payload() {
        let header = LightClientHeader::Bellatrix(LightClientHeaderBellatrix::default());
        execution_state_root(&header, None);
    }
}
