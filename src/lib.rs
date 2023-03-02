use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};

#[inline]
pub fn find_bounds_custom(m: &BTreeMap<u32, String>, i: u32) -> (Option<(&u32, &String)>, Option<(&u32, &String)>) {

    let mut lower_bound: Option<(&u32, &String)> = None;
    let mut higher_bound: Option<(&u32, &String)> = None;

    if m.is_empty() {
        return (lower_bound, higher_bound);
    }

    // First check against first map value
    let first = m.first_key_value().unwrap(); // safe to unwrap()

    if i < *first.0 {
        higher_bound = Some(first);
        return (lower_bound, higher_bound);
    }
    if i == *first.0 {
        lower_bound = Some(first);
        higher_bound = Some(first);
        return (lower_bound, higher_bound);
    }

    // Then check against second map value
    let last = m.last_key_value().unwrap(); // safe to unwrap()

    if i > *last.0 {
        lower_bound = Some(last);
        return (lower_bound, higher_bound);
    }
    if i == *last.0 {
        lower_bound = Some(last);
        higher_bound = Some(last);
        return (lower_bound, higher_bound);
    }

    // Then iter over
    // We are in between two states, find bounds
    for (mi, ms) in m.iter() {
        if *mi <= i {
            lower_bound = Some((mi, ms));
        }
        if *mi >= i && higher_bound.is_none() {
            higher_bound = Some((mi, ms));
            break;
        }
    }

    (lower_bound, higher_bound)
}

#[inline]
pub fn find_bounds_range(m: &BTreeMap<u32, String>, i: u32) -> (Option<(&u32, &String)>, Option<(&u32, &String)>) {
    (m.range(
        (Unbounded, Included(i))
    ).last(),
    m.range(
        (Included(i), Unbounded)
    ).next())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_bounds() {

        // u32: ts, String: state
        let btm : BTreeMap<u32, String> = BTreeMap::from([
            (1, "Defined".to_string()),
            (4, "Started".to_string()),
            (6, "LockedIn".to_string()),
            (8, "Active".to_string()),
        ]);

        // Find lower bound & higher bound in btreemap using range function

        let around_0 = find_bounds_range(&btm, 0);
        println!("around 0: {around_0:?}");
        let around_5 = find_bounds_range(&btm, 5);
        println!("around 5: {around_5:?}");
        let around_9 = find_bounds_range(&btm, 9);
        println!("around 9: {around_9:?}");
        // edge case: empty btreemap
        let btm0 = BTreeMap::default();
        let around_e = find_bounds_range(&btm0, 9);
        println!("[empty btreemap] around e: {around_e:?}");
        // edge case: exact value
        let around_6 = find_bounds_range(&btm, 6);
        println!("[exact] around 6: {around_6:?}");

        // Find lower & higher bounds using custom function

        let around_0c = find_bounds_custom(&btm, 0);
        println!("around 0c: {around_0c:?}");
        let around_5c = find_bounds_custom(&btm, 5);
        println!("around 5c: {around_5c:?}");
        let around_9c = find_bounds_custom(&btm, 9);
        println!("around 9c: {around_9c:?}");
        // edge case: empty btreemap
        let btm0 = BTreeMap::default();
        let around_ec = find_bounds_custom(&btm0, 9);
        println!("[empty btreemap] around ec: {around_ec:?}");
        // edge case: exact value
        let around_6c = find_bounds_custom(&btm, 6);
        println!("[exact] around 6c: {around_6c:?}");

        // Check they are the same
        assert_eq!(around_0, around_0c);
        assert_eq!(around_5, around_5c);
        assert_eq!(around_9, around_9c);
        assert_eq!(around_e, around_ec);
        assert_eq!(around_6, around_6c);
    }
}
