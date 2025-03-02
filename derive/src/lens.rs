// Adapted from Druid lens.rs

// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// use proc_macro2::{Ident, Span};
use quote::quote;
// use std::collections::HashSet;
use syn::{spanned::Spanned, Data, GenericParam, TypeParam};

use super::attr::{FieldKind, Fields, LensAttrs};

pub(crate) fn derive_lens_impl(
    input: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match &input.data {
        Data::Struct(_) => derive_struct(&input),
        Data::Enum(e) => Err(syn::Error::new(
            e.enum_token.span(),
            "Lens implementations cannot be derived from enums",
        )),
        Data::Union(u) => Err(syn::Error::new(
            u.union_token.span(),
            "Lens implementations cannot be derived from unions",
        )),
    }
}

fn derive_struct(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let struct_type = &input.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &input.data {
        Fields::<LensAttrs>::parse_ast(fields)?
    } else {
        return Err(syn::Error::new(
            input.span(),
            "Lens implementations can only be derived from structs with named fields",
        ));
    };

    if fields.kind != FieldKind::Named {
        return Err(syn::Error::new(
            input.span(),
            "Lens implementations can only be derived from structs with named fields",
        ));
    }

    let twizzled_name = if is_camel_case(&struct_type.to_string()) {
        let temp_name = format!("{}_derived_lenses", to_snake_case(&struct_type.to_string()));
        proc_macro2::Ident::new(&temp_name, proc_macro2::Span::call_site())
    } else {
        return Err(syn::Error::new(
            struct_type.span(),
            "Lens implementations can only be derived from CamelCase types",
        ));
    };
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut lens_ty_idents = Vec::new();
    let mut phantom_decls = Vec::new();
    let mut phantom_inits = Vec::new();

    for gp in input.generics.params.iter() {
        if let GenericParam::Type(TypeParam { ident, .. }) = gp {
            lens_ty_idents.push(quote! {#ident});
            phantom_decls.push(quote! {std::marker::PhantomData<*const #ident>});
            phantom_inits.push(quote! {std::marker::PhantomData});
        }
    }

    let lens_ty_generics = quote! {
        <#(#lens_ty_idents),*>
    };

    // Define lens types for each field
    let defs = fields.iter().filter(|f| !f.attrs.ignore).map(|f| {
        let field_name = &f.ident.unwrap_named();
        let struct_docs = format!(
            "Lens for the field `{field}` on [`{ty}`](super::{ty}).",
            field = field_name,
            ty = struct_type,
        );

        let fn_docs = format!(
            "Creates a new lens for the field `{field}` on [`{ty}`](super::{ty}). \
            Use [`{ty}::{field}`](super::{ty}::{field}) instead.",
            field = field_name,
            ty = struct_type,
        );

        quote! {
            #[doc = #struct_docs]
            #[allow(non_camel_case_types)]
            #[derive(Copy, Clone, Hash, PartialEq, Eq)]
            pub struct #field_name#lens_ty_generics(#(#phantom_decls),*);

            impl #lens_ty_generics #field_name#lens_ty_generics{
                #[doc = #fn_docs]
                pub const fn new()->Self{
                    Self(#(#phantom_inits),*)
                }
            }

            impl #lens_ty_generics std::fmt::Debug for #field_name#lens_ty_generics {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f,"{}:{}",stringify!(#struct_type), stringify!(#field_name))
                }
            }
        }
    });

    // let used_params: HashSet<String> = input
    //     .generics
    //     .params
    //     .iter()
    //     .flat_map(|gp: &GenericParam| match gp {
    //         GenericParam::Type(TypeParam { ident, .. }) => Some(ident.to_string()),
    //         _ => None,
    //     })
    //     .collect();

    // let gen_new_param = |name: &str| {
    //     let mut candidate: String = name.into();
    //     let mut count = 1usize;
    //     while used_params.contains(&candidate) {
    //         candidate = format!("{}_{}", name, count);
    //         count += 1;
    //     }
    //     Ident::new(&candidate, Span::call_site())
    // };

    //let func_ty_par = gen_new_param("F");
    //let val_ty_par = gen_new_param("V");

    let impls = fields.iter().filter(|f| !f.attrs.ignore).map(|f| {
        let field_name = &f.ident.unwrap_named();
        let field_ty = &f.ty;

        quote! {

            impl #impl_generics Lens for #twizzled_name::#field_name#lens_ty_generics #where_clause {

                type Source = #struct_type#ty_generics;
                type Target = #field_ty;

                fn view<'a>(&self, data: &'a#struct_type#ty_generics) -> &'a#field_ty {
                    &data.#field_name
                }
            }
        }
    });

    let associated_items = fields.iter().filter(|f| !f.attrs.ignore).map(|f| {
        let field_name = &f.ident.unwrap_named();
        let lens_field_name = f.attrs.lens_name_override.as_ref().unwrap_or(field_name);

        quote! {
            /// Lens for the corresponding field.
            pub const #lens_field_name: #twizzled_name::#field_name#lens_ty_generics = #twizzled_name::#field_name::new();
        }
    });

    let mod_docs = format!("Derived lenses for [`{}`].", struct_type);

    let expanded = quote! {
        #[doc = #mod_docs]
        pub mod #twizzled_name {
            #(#defs)*
            #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
            pub struct root;

            impl root {
                pub const fn new()->Self{
                    Self
                }
            }
        }

        #(#impls)*

        impl Lens for #twizzled_name::root {
            type Source = #struct_type#ty_generics;
            type Target = #struct_type#ty_generics;

            fn view<'a>(&self, source: &'a Self::Source) -> &'a Self::Target {
                source
            }
        }

        #[allow(non_upper_case_globals)]
        impl #impl_generics #struct_type #ty_generics #where_clause {
            #(#associated_items)*

            pub const root: #twizzled_name::root = #twizzled_name::root::new();
        }
    };

    Ok(expanded)
}

//I stole these from rustc!
fn char_has_case(c: char) -> bool {
    c.is_lowercase() || c.is_uppercase()
}

fn is_camel_case(name: &str) -> bool {
    let name = name.trim_matches('_');
    if name.is_empty() {
        return true;
    }

    // start with a non-lowercase letter rather than non-uppercase
    // ones (some scripts don't have a concept of upper/lowercase)
    !name.chars().next().unwrap().is_lowercase()
        && !name.contains("__")
        && !name.chars().collect::<Vec<_>>().windows(2).any(|pair| {
            // contains a capitalisable character followed by, or preceded by, an underscore
            char_has_case(pair[0]) && pair[1] == '_' || char_has_case(pair[1]) && pair[0] == '_'
        })
}

fn to_snake_case(mut str: &str) -> String {
    let mut words = vec![];
    // Preserve leading underscores
    str = str.trim_start_matches(|c: char| {
        if c == '_' {
            words.push(String::new());
            true
        } else {
            false
        }
    });
    for s in str.split('_') {
        let mut last_upper = false;
        let mut buf = String::new();
        if s.is_empty() {
            continue;
        }
        for ch in s.chars() {
            if !buf.is_empty() && buf != "'" && ch.is_uppercase() && !last_upper {
                words.push(buf);
                buf = String::new();
            }
            last_upper = ch.is_uppercase();
            buf.extend(ch.to_lowercase());
        }
        words.push(buf);
    }
    words.join("_")
}
