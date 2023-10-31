extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(LightEnum)]
pub fn generate_light_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Enum(data_enum) = &input.data else {
        panic!("LightEnum can only be derived for enums");
    };

    let orig_enum_name = &input.ident;

    let new_enum_name = syn::Ident::new(&format!("{}Light", orig_enum_name), orig_enum_name.span());

    let light_variants = data_enum.variants.iter().map(|variant| &variant.ident);
    let light_variants_cloned = light_variants.clone();

    let generated_code = quote! {
        #[derive(Debug, PartialEq, Eq, Clone)]
        enum #new_enum_name {
            #(
                #light_variants,
            )*
        }

        impl #orig_enum_name {
            fn to_light(&self) -> #new_enum_name {
                match self {
                    #(
                        #orig_enum_name::#light_variants_cloned(_) => #new_enum_name::#light_variants_cloned,
                    )*
                }
            }
        }
    };

    generated_code.into()
}
