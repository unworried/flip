use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(VmInstruction, attributes(opcode))]
pub fn generate_vm_instruction_impl(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_opcode(&ast)
}

fn impl_opcode(ast: &syn::ItemEnum) -> TokenStream {
    let field_names = ast.variants.iter().map(|variant| &variant.ident);
    let field_values = ast.variants.iter().map(|variant| {
        for attr in variant.attrs.iter() {
            if attr.path().is_ident("opcode") {
                let value: syn::LitInt = attr.parse_args().unwrap();
                return value;
            }
        }
        syn::parse(quote! { 0 }.into()).unwrap()
    });
    quote! {
        #[repr(u8)]
        #[derive(Debug)]
        pub enum OpCode {
            #(#field_names = #field_values,)*
        }
    }
    .into()
}
