use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Fields, ItemStruct, PathArguments, Type, TypePath,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

// A small struct used to parse an inline struct definition
// from inside the #[flatten(...)] attribute.
struct InlineStructArg {
    inline_struct: ItemStruct,
}

impl Parse for InlineStructArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let inline_struct: ItemStruct = input.parse()?;
        Ok(InlineStructArg { inline_struct })
    }
}

#[proc_macro_attribute]
pub fn flatten_struct(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the struct that the attribute is applied to
    let input_ast = parse_macro_input!(input as ItemStruct);
    let struct_ident = &input_ast.ident;

    let mut new_fields = vec![];

    // Iterate over the original struct's fields
    if let Fields::Named(fields_named) = &input_ast.fields {
        for field in fields_named.named.iter() {
            let field_ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;

            // Check if the field has #[flatten(...)] attribute
            let maybe_attr = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("flatten"));

            if let Some(attr) = maybe_attr {
                // Parse the inline struct from the attribute arguments
                let args: InlineStructArg = attr
                    .parse_args()
                    .expect("Failed to parse #[flatten(...)] inline struct");
                let inline_struct = args.inline_struct;

                let prefix = field_ident.to_string();

                // Check if the field is a Vec<T>
                let is_vec = matches!(ty,
                    Type::Path(TypePath { path, .. })
                    if path.segments.last().map_or(false, |seg|
                        seg.ident == "Vec" &&
                        matches!(seg.arguments, PathArguments::AngleBracketed(ref args)
                            if matches!(args.args.first(), Some(syn::GenericArgument::Type(_)))
                        )
                    )
                );

                if is_vec {
                    // Add a count field for Vecs
                    let count_ident = format_ident!("{}_count", prefix);
                    new_fields.push(quote! {
                        #count_ident: Option<usize>,
                    });
                }

                // Extract fields from the inline struct
                if let Fields::Named(fields_named) = inline_struct.fields {
                    for f in fields_named.named.iter() {
                        let sf_ident = &f.ident;
                        let sf_ty = &f.ty;
                        let flat_name = format_ident!("{}_{}", prefix, sf_ident.as_ref().unwrap());

                        if is_vec {
                            new_fields.push(quote! {
                                #flat_name: Option<Vec<#sf_ty>>,
                            });
                        } else {
                            new_fields.push(quote! {
                                #flat_name: Option<#sf_ty>,
                            });
                        }
                    }
                }
            } else {
                // Preserve normal fields unchanged
                new_fields.push(quote! {
                    #field_ident: #ty,
                });
            }
        }
    }

    // Preserve original attributes and visibility
    let attrs = &input_ast.attrs;
    let vis = &input_ast.vis;

    // Emit a new struct with the flattened fields, injecting Builder derivation
    let expanded = quote! {
        #(#attrs)*
        #[derive(Builder)]
        #[builder(private, build_fn(validate = "Self::validate"))]
        #vis struct #struct_ident {
            #(#new_fields)*
        }
    };

    TokenStream::from(expanded)
}
