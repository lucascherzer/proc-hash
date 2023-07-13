use proc_hash::include_sha3;

#[docify::export]
#[test]
fn test_sha3() {
    assert_eq!(
        include_sha3!("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
        "2adde7e6a52bda4b4573f46d78507e6c317e72fce63327c220b95c6b7246b951b21e309ed24a4e0e3084a809528cc8e2aa2b9cc050c07bcc821068e7d866df59"
    );
}
