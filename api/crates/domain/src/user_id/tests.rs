use std::collections::HashSet;

use super::*;

#[test]
fn reads_back_the_value_it_was_built_with() {
    let small = UserId::from_u128(7);
    let largest = UserId::from_u128(u128::MAX);

    assert_eq!(small.as_u128(), 7);
    assert_eq!(largest.as_u128(), u128::MAX);
}

#[test]
fn ids_are_equal_by_value() {
    let id = UserId::from_u128(7);
    let same = UserId::from_u128(7);
    let other = UserId::from_u128(8);

    assert_eq!(id, same);
    assert_ne!(id, other);
}

#[test]
fn equal_ids_collapse_in_a_set() {
    let ids = HashSet::from([UserId::from_u128(7), UserId::from_u128(7)]);

    assert_eq!(ids.len(), 1);
}
