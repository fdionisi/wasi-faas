use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, Ident, ItemFn};

fn token_stream_with_error(mut tokens: TokenStream, error: Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}

#[proc_macro_attribute]
pub fn handler(_: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };

    if input.sig.ident != "main" {
        let msg = "the main function cannot accept arguments";
        let e = Error::new_spanned(&input.sig.ident, msg);
        return token_stream_with_error(item, e);
    }

    if input.sig.inputs.len() != 1 {
        let msg = "the main function must accept only one argument";
        let e = Error::new_spanned(&input.sig.ident, msg);
        return token_stream_with_error(item, e);
    }

    let fn_ident = Ident::new("_handler", input.sig.ident.span());
    let fn_inputs = input.sig.inputs.iter();
    let fn_outputs = &input.sig.output;
    let fn_body = &input.block;

    quote!(
        fn #fn_ident(#(#fn_inputs,)*) #fn_outputs
            #fn_body


        fn main() -> std::io::Result<()> {
            let req = {
                let mut args = std::env::args();

                let method = args.next().unwrap();
                let path = args.next().unwrap();

                let mut builder = wasi_faas_sdk::http::request::Builder::new()
                    .method(method.as_str())
                    .uri(path);

                for (k, v) in std::env::vars() {
                    builder = builder.header(k, v);
                }

                let body = std::io::stdin()
                    .lines()
                    .fold(vec![], |result, l| [&result, l.unwrap().as_bytes()]
                    .concat());

                let body = wasi_faas_sdk::bytes::Bytes::from(body);

                builder
                    .body(body)
                    .unwrap()
            };

            let res = #fn_ident(req);

            let res = wasi_faas_sdk::http::HttpInboundResponse {
                status: res.status().as_u16(),
                body: res.body().to_vec(),
                headers: res.headers()
                    .iter()
                    .map(|(k, v)| (k.to_string(), String::from(v.to_str().unwrap())))
                    .collect(),
            };

            let mut stdout = std::io::stdout().lock();
            std::io::Write::write_all(&mut stdout, &res.ser())?;

            Ok(())
        }
    )
    .into()
}
