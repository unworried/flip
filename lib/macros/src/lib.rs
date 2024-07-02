use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(VmInstruction, attributes(opcode))]
pub fn generate_vm_instruction_impl(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_opcode(&ast)
}

fn get_type_name(ty: &syn::Type) -> String {
    if let syn::Type::Path(p) = ty {
        p.path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect()
    } else {
        panic!("unsupported type");
    }
}

fn impl_opcode(ast: &syn::ItemEnum) -> TokenStream {
    let field_names: Vec<_> = ast.variants.iter().map(|variant| &variant.ident).collect();

    let field_values = ast.variants.iter().map(|variant| {
        for attr in variant.attrs.iter() {
            if attr.path().is_ident("opcode") {
                let value: syn::LitInt = attr.parse_args().unwrap();
                return value;
            }
        }
        syn::parse(quote! { 0 }.into()).unwrap()
    });

    let field_u16_encoding: Vec<_> = ast
        .variants
        .iter()
        .map(|variant| {
            let name = &variant.ident;

            if let syn::Fields::Unit = &variant.fields {
                return quote! { Self::#name => OpCode::#name as u16 };
            }

            if let syn::Fields::Unnamed(fields) = &variant.fields {
                let types: Vec<_> = fields
                    .unnamed
                    .iter()
                    .map(|f| get_type_name(&f.ty))
                    .collect();
                
                let str_types: Vec<&str> = types.iter().map(AsRef::as_ref).collect();
                match &str_types[..] {
                    ["u8"] => {
                        quote! { Self::#name(u) => OpCode::#name as u16 | ((*u as u16) << 8) }
                    }
                    ["Register"] => { 
                        quote! { Self::#name(r) => OpCode::#name as u16 | (((*r as u16) & 0xf) << 8) }
                    }
                    
                    ["Register", "Register"] => {
                        quote! {
                            Self::#name(r1, r2) => OpCode::#name as u16 | (((*r1 as u16) & 0xf) << 8)
                                | (((*r2 as u16) & 0xf) << 12)
                        }
                    }
                    _ => panic!("invalid types: {:?}", types),
                }

            } else {
                panic!("fields must be unnamed in variant: {}", name);
            }
        })
        .collect();

    quote! {
        #[repr(u8)]
        #[derive(Debug)]
        pub enum OpCode {
            #(#field_names = #field_values,)*
        }

        impl FromStr for OpCode {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(stringify!(#field_names) => Ok(Self::#field_names),)*
                    _ => Err(format!("unknown opcode: {}", s)),
                }
            }
        }

        impl TryFrom<u8> for OpCode {
            type Error = String;

            fn try_from(b: u8) -> Result<Self, Self::Error> {
                match b {
                    #(x if x == Self::#field_names as u8 => Ok(Self::#field_names),)*
                    _ => Err(format!("unknown opcode: {:X}", b)),
                }
            }
        }

        impl Instruction {
            fn encode_r1(r: Register) -> u16 {
                (r as u16) & 0xf << 8
            }
            fn encode_r2(r: Register) -> u16 {
                (r as u16) & 0xf << 12
            }

            fn encode_num(u: u8) -> u16 {
                (u as u16) << 8
            }

            fn encode_rs(r1: Register, r2: Register) -> u16 {
                Self::encode_r1(r1) | Self::encode_r2(r2)
            }

            pub fn encode_u16(&self) -> u16 {
                match self {
                    #(#field_u16_encoding,)*
                }
            }
        }
    }
    .into()
}
