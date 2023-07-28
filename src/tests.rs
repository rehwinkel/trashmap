use crate::TrashMap;

#[test]
fn test_insert_collsions() {
    let mut map = TrashMap::new();
    for i in 0..100 {
        map.insert(i, ());
    }
    for i in 0..100 {
        assert!(map.remove(&i));
    }
    assert!(!map.remove(&0));
    assert!(map.len() == 0);
}
