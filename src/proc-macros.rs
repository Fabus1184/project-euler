use proc_macro::{Literal, TokenStream, TokenTree};

#[proc_macro]
pub fn include_http(url: TokenStream) -> TokenStream {
    let url = syn::parse_macro_input!(url as syn::LitStr).value();

    let text = reqwest::blocking::get(url).unwrap().text().unwrap();

    TokenStream::from_iter(std::iter::once(TokenTree::Literal(Literal::string(
        text.as_str(),
    ))))
}
