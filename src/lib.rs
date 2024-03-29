use blake2::{Blake2b512, Blake2s256, Digest};
use proc_macro::TokenStream;
use quote::quote;
use sha3::Sha3_512;
use syn::LitStr;
use whirlpool::Whirlpool;

/// Simply converts a `TokenStream` to a `LitStr`
macro_rules! ts_to_litstr {
    ($item:ident) => {{
        let arg = syn::parse_macro_input!($item as LitStr);
        arg.value()
    }};
}
/// Returns a `String` of the hash of type `$algo`.
/// Only works with hash types implementing the `Digest` trait defined in [digest](https://crates.io/crates/digest)
macro_rules! rscrypto_hash {
    ($algo:ty, $val:ident) => {{
        let data = $val.as_bytes();
        let mut hasher = <$algo>::new();

        hasher.update(data);

        let res = hasher.finalize();
        format!("{:x}", res)
    }};
}

/// Takes a string and replaces it with it's [MD5](https://en.wikipedia.org/wiki/MD5) hash at compile time
/// # Example
#[doc = docify::embed_run!("tests/md5.rs", test_md5)]
/// # Security Warning
/// MD5 [should be considered cryptographically broken](https://www.kb.cert.org/vuls/id/836068) and be avoided for security critical
/// applications if possible.
#[proc_macro]
pub fn include_md5(item: TokenStream) -> TokenStream {
    let val = ts_to_litstr!(item);
    let hash = md5::compute(val);
    let hash = format!("{:x}", hash);
    let out = quote! {
        {
            const HASH: &str = #hash;
            HASH
        }
    };
    out.into()
}

/// Takes a string and replaces it with it's [Blake2s256](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE2) hash at compile time
/// # Example
#[doc = docify::embed_run!("tests/blake512.rs", test_blake512)]
#[proc_macro]
pub fn include_blake256(item: TokenStream) -> TokenStream {
    let val = ts_to_litstr!(item);
    let hash = rscrypto_hash!(Blake2s256, val);
    let out = quote! {
        {
            const HASH: &str = #hash;
            HASH
        }
    };
    out.into()
}

/// Takes a string and replaces it with it's [Blake2b512](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE2) hash at compile time
/// # Example
#[doc = docify::embed_run!("tests/blake512.rs", test_blake512)]
#[proc_macro]
pub fn include_blake512(item: TokenStream) -> TokenStream {
    let val = ts_to_litstr!(item);
    let hash = rscrypto_hash!(Blake2b512, val);
    let out = quote! {
        {
            const HASH: &str = #hash;
            HASH
        }
    };
    out.into()
}

/// Takes a string and replaces it with it's [SHA-3](https://en.wikipedia.org/wiki/SHA-3) hash at compile time
/// # Example
#[doc = docify::embed_run!("tests/sha3.rs", test_sha3)]
#[proc_macro]
pub fn include_sha3(item: TokenStream) -> TokenStream {
    let val = ts_to_litstr!(item);
    let hash = rscrypto_hash!(Sha3_512, val);

    let out = quote! {
        {
            const HASH: &str = #hash;
            HASH
        }
    };
    out.into()
}

/// Takes a string and replaces it with it's [Whirlpool](https://en.wikipedia.org/wiki/Whirlpool_(hash_function)) hash at compile time
/// # Example
#[doc = docify::embed_run!("tests/whirlpool.rs", test_whirlpool)]
#[proc_macro]
pub fn include_whirlpool(item: TokenStream) -> TokenStream {
    let val = ts_to_litstr!(item);
    let hash = rscrypto_hash!(Whirlpool, val);

    let out = quote! {
        {
            const HASH: &str = #hash;
            HASH
        }
    };
    out.into()
}
