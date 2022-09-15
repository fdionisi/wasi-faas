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
                let input = std::io::stdin()
                    .lines()
                    .fold(vec![], |result, l| [&result, l.unwrap().as_bytes()].concat());

                <wasi_faas_interface::Request as wasi_faas_interface::Binary>::de(&input)
            };

            let resp = #fn_ident(req);
            let resp = wasi_faas_interface::Binary::ser(&resp);

            let mut stdout = std::io::stdout().lock();
            std::io::Write::write_all(&mut stdout, &resp)?;

            Ok(())
        }
    )
    .into()
}
