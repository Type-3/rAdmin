#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro]
pub fn b(input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as syn::ExprStruct);
    let ident = &input_struct.path;
    let field_names = input_struct.fields.iter().map(|value| &value.member);
    let field_values = input_struct.fields.iter().map(|value| &value.expr);

    let output = quote! {
        #ident::builder()
        #(
            .#field_names(#field_values)
        )*
            .build()
    };
    output.into()
}

#[proc_macro_derive(Permission, attributes(name, prefix))]
pub fn derive_permission(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    let ident = derive_input.ident;

    let mut attr_name = None;
    let mut attr_name_prefix = None;

    for attr in derive_input.attrs {
        if let Some(path) = attr.path.get_ident() {
            let ident_string = format!("{}", path);
            match ident_string.as_ref() {
                "name" => {
                    if let Ok(string) = attr.parse_args::<syn::LitStr>() {
                        attr_name = Some(string.value());
                    }
                }
                "prefix" => {
                    if let Ok(string) = attr.parse_args::<syn::LitStr>() {
                        attr_name_prefix = Some(string.value());
                    }
                }
                _ => {}
            }
        }
    }

    let _name = attr_name.unwrap_or_else(|| ident_to_snake_case(&format!("{}", ident)));
    let prefix = attr_name_prefix
        .map(|item| format!("{}.", item))
        .unwrap_or_else(String::new);
    let name = format!("{}{}", prefix, _name);
    let tokens = quote! {
        impl PermissionDef for #ident {
            const NAME: &'static str = #name;
        }
    };
    tokens.into()
}

#[proc_macro_derive(Role, attributes(name))]
pub fn derive_role(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    let ident = derive_input.ident;

    let mut attr_name = None;

    for attr in derive_input.attrs {
        if let Some(path) = attr.path.get_ident() {
            let ident_string = format!("{}", path);
            if let "name" = ident_string.as_ref() {
                if let Ok(string) = attr.parse_args::<syn::LitStr>() {
                    attr_name = Some(string.value());
                }
            }
        }
    }

    let name = attr_name.unwrap_or_else(|| ident_to_snake_case(&format!("{}", ident)));
    let tokens = quote! {
        impl RoleDef for #ident {
            const NAME: &'static str = #name;
        }
    };
    tokens.into()
}

fn ident_to_snake_case(input: &str) -> String {
    use case::CaseExt;
    input.to_snake().replace("_", ".")
}

#[proc_macro_attribute]
pub fn from_similar(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as syn::ItemStruct);
    let args = parse_macro_input!(args as syn::Path);

    let ident = input_struct.ident.clone();

    let fields = match &input_struct.fields {
        syn::Fields::Named(fields) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let field_name = fields.iter().map(|field| &field.ident);

    let output = quote! {
        #input_struct
        impl From<#args> for #ident {
            fn from(f: #args) -> #ident {
                #ident {
                    #(
                        #field_name: f.#field_name,
                    )*
                }
            }
        }
    };
    output.into()
}
