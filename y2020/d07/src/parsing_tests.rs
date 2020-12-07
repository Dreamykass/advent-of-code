use crate::parsing::parse_bag_def;

#[test]
fn test1() {
    let bag = parse_bag_def("dotted black bags contain no other bags.");
    assert_eq!(bag.color, "dotted black");
    assert!(bag.inner_bags.is_empty());
}

#[test]
fn test2() {
    let bag = parse_bag_def("bright white bags contain 1 shiny gold bag.");
    assert_eq!(bag.color, "bright white");
    assert!(!bag.inner_bags.is_empty());
    assert_eq!(bag.inner_bags.len(), 1);
    assert_eq!(bag.inner_bags.first().unwrap(), "shiny gold");
}

#[test]
fn test3() {
    let bag = parse_bag_def("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.");
    assert_eq!(bag.color, "muted yellow");
    assert!(!bag.inner_bags.is_empty());
    assert_eq!(bag.inner_bags.len(), 11);
    assert!(bag.inner_bags.contains(&"shiny gold".to_string()));
    assert!(bag.inner_bags.contains(&"faded blue".to_string()));
}
