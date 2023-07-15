use proc_hash::include_whirlpool;

#[docify::export]
#[test]
fn test_whirlpool() {
    assert_eq!(
        include_whirlpool!("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
        "7a973293a065f4adc6a8dc6355f91054b03a85cd6bacf60bb81066d59316e654b3bc56562061178b56fee41e3ced5d2e153a47014ca2d5acb854cfc3cdcea57a"
    );
}
