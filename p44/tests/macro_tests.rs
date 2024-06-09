use p44::hash_map;

#[test]
fn test_hash_map_macro() {
    let map = hash_map!(
        42 => true,
        64 => false,
        128 => true,
    );

    assert_eq!(map.get(&42), Some(&true));
    assert_eq!(map.get(&64), Some(&false));
    assert_eq!(map.get(&128), Some(&true));
    assert_eq!(map.len(), 3);
}
