#[proc_macro_derive(EmptyStruct)]
pub fn derive_empty_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;
    quote::quote! {
        impl ::see_migration_test_helpers::EmptyStruct for #name {
            #[inline]
            fn new() -> Self {
                Self
            }
        }
    }
    .into()
}
