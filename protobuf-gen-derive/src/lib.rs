#![recursion_limit = "128"]

use protobuf_gen_extract as extract;

mod convert;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Fields, Item, ItemEnum, ItemStruct, TypePath};

use convert::ConversionGenerator;
use extract::Extract;

#[proc_macro_derive(ProtobufGen, attributes(protobuf_gen))]
pub fn derive_protobuf_gen(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as Item);

    match &item {
        Item::Struct(ItemStruct { attrs, .. }) | Item::Enum(ItemEnum { attrs, .. }) => {
            if let Some(proxy_mod) =
                syn_util::get_attribute_value::<String>(attrs, &["protobuf_gen", "proxy_mod"])
            {
                return generate_conversion_apis(
                    &item,
                    syn::parse_str(&proxy_mod).unwrap_or_else(|_| {
                        panic!("invalid proxy_mod attribyte: \"{}\"", proxy_mod)
                    }),
                )
                .into();
            }
        }
        _ => {}
    }
    TokenStream2::default().into()
}

fn generate_conversion_apis(item: &Item, proxy_mod: TypePath) -> TokenStream2 {
    let mut builder = ConversionGenerator { token_stream: TokenStream2::default(), proxy_mod };

    match item {
        Item::Struct(item_struct) => {
            protobuf_gen_extract::extract_message(&mut builder, item_struct);
        }
        Item::Enum(item_enum) => {
            if item_enum.variants.iter().all(|v| matches!(v.fields, Fields::Unit)) {
                builder.extract_enumerator(item_enum);
            } else {
                builder.extract_one_of(item_enum);

                for variant in &item_enum.variants {
                    protobuf_gen_extract::extract_nested_message(&mut builder, item_enum, variant);
                }
            }
        }
        _ => unreachable!(),
    }

    builder.token_stream
}
