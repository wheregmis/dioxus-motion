use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Fields, Meta};

fn get_transition_from_attrs(attrs: &[Attribute]) -> Option<String> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("transition"))
        .and_then(|attr| {
            if let Ok(Meta::Path(path)) = attr.parse_args::<Meta>() {
                path.get_ident().map(|ident| ident.to_string())
            } else {
                None
            }
        })
}

#[proc_macro_derive(MotionTransitions, attributes(transition))]
pub fn derive_route_transitions(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let variants = match input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("MotionTransitions can only be derived for enums"),
    };

    let transition_match_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let transition = get_transition_from_attrs(&variant.attrs)
            .map(|t| format_ident!("{}", t))
            .unwrap_or(format_ident!("Fade"));

        match &variant.fields {
            Fields::Named(fields) => {
                let field_patterns = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! { #name: _ }
                });
                quote! {
                    Self::#variant_ident { #(#field_patterns,)* } => TransitionVariant::#transition
                }
            }
            Fields::Unnamed(_) => {
                quote! { Self::#variant_ident(..) => TransitionVariant::#transition }
            }
            Fields::Unit => {
                quote! { Self::#variant_ident {} => TransitionVariant::#transition }
            }
        }
    });

    let component_match_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let component_name = &variant.ident;

        match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                quote! {
                    Self::#variant_ident { #(ref #field_names,)* } => {
                        rsx! { #component_name { #(#field_names: #field_names.clone(),)* } }
                    }
                }
            }
            Fields::Unnamed(_) => {
                quote! { Self::#variant_ident(..) => rsx! { #component_name {} } }
            }
            Fields::Unit => {
                quote! { Self::#variant_ident {} => rsx! { #component_name {} } }
            }
        }
    });

    let expanded = quote! {
        impl AnimatableRoute for  #name {
            fn get_transition(&self) -> TransitionVariant {
                match self {
                    #(#transition_match_arms,)*
                    _ => TransitionVariant::Fade,
                }
            }

            fn get_component(&self) -> Element {
                match self {
                    #(#component_match_arms,)*
                }
            }
        }


    };

    TokenStream::from(expanded)
}
