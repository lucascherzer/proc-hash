use proc_hash::include_md5;

#[docify::export]
#[test]
fn test_md5() {
    assert_eq!(
        include_md5!("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
        "75170fc230cd88f32e475ff4087f81d9"
    )
}
