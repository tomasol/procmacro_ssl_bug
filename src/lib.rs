#[proc_macro]
pub fn make_answer(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _ = openssl::ssl::SslMethod::tls();
    item
}
