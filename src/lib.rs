use blake2::{Blake2b512, Digest};
use proc_macro::TokenStream;
use quote::quote;
use syn::LitStr;

/// Simply converts a `TokenStream` to a `LitStr`
macro_rules! ts_to_litstr {
    ($item:ident) => {{
        let arg = syn::parse_macro_input!($item as LitStr);
        arg.value()
    }};
}
/// Takes a string and replaces it with it's MD5 hash at compile time
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

/// Takes a string and replaces it with it's Blake2b512 hash at compile time
/// # Example
#[doc = docify::embed_run!("tests/blake512.rs", test_blake512)]
#[proc_macro]
pub fn include_blake512(item: TokenStream) -> TokenStream {
    let val = ts_to_litstr!(item);
    let hash = {
        let data = val.as_bytes();
        let mut hasher = Blake2b512::new();

        // write input message
        hasher.update(data);

        // read hash digest and consume hasher
        let res = hasher.finalize();
        format!("{:x}", res)
    };
    let out = quote! {
        {
            const HASH: &str = #hash;
            HASH
        }
    };
    out.into()
}
