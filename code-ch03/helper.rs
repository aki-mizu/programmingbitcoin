//! Small Chapter 3 helper utilities.

use sha2::{Digest, Sha256};

/// Runs one test callback, mirroring the source chapter's helper.
pub fn run(test: impl FnOnce()) {
    test();
}

/// Returns two rounds of SHA-256 for the supplied bytes.
pub fn hash256(value: &[u8]) -> [u8; 32] {
    let first = Sha256::digest(value);
    let second = Sha256::digest(first);
    second.into()
}

#[cfg(test)]
mod tests {
    use super::{hash256, run};

    #[test]
    fn runs_the_supplied_callback() {
        let mut was_run = false;
        run(|| was_run = true);
        assert!(was_run);
    }

    #[test]
    fn hashes_twice_with_sha256() {
        assert_eq!(
            hash256(b"Programming Bitcoin!"),
            [
                0x96, 0x9f, 0x60, 0x56, 0xaa, 0x26, 0xf7, 0xd2, 0x79, 0x5f, 0xd0, 0x13, 0xfe, 0x88,
                0x86, 0x8d, 0x09, 0xc9, 0xf6, 0xae, 0xd9, 0x69, 0x65, 0x01, 0x6e, 0x19, 0x36, 0xae,
                0x47, 0x06, 0x0d, 0x48,
            ]
        );
    }
}
