use vanth::Node;

#[test]
fn test_entity_count_zero() {
    let mut node = Node::new();
    assert_eq!(node.entity_count(), 0);
}
