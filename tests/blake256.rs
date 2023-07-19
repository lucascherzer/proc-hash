use proc_hash::include_blake256;

#[docify::export]
#[test]
fn test_blake256() {
    assert_eq!(
        include_blake256!("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
        "cc173d3132f002f0b0b71420f41ada74339f778a4c73d1dab23642191a35a8eb"
    );
}
