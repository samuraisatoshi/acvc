use std::collections::BTreeSet;

use crate::Hash;

pub fn missing_hashes(offered: &[Hash], present: &[Hash]) -> Vec<Hash> {
    let have: BTreeSet<Hash> = present.iter().copied().collect();
    let mut seen = BTreeSet::new();
    let mut missing = Vec::new();
    for hash in offered {
        if seen.insert(*hash) && !have.contains(hash) {
            missing.push(*hash);
        }
    }
    missing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_keeps_offer_order_and_drops_what_is_present() {
        let first = Hash::blob(b"one");
        let second = Hash::blob(b"two");
        let third = Hash::blob(b"three");
        let missing = missing_hashes(
            &[second, first, second, third],
            &[first, Hash::blob(b"other")],
        );
        assert_eq!(missing, vec![second, third]);
    }
}
