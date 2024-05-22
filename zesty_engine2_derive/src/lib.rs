use std::str::FromStr;

use proc_macro;
use proc_macro2::{TokenStream, Literal};
use quote::quote;
use syn::{Ident, DeriveInput, parse_macro_input, Meta, MetaList, parse2};


// Type alias so that I don't get confused
type OldTokenStream = proc_macro::TokenStream;

// It is expected for the Matrix struct to be in the format
// `Matrix[num]`, where `num` is the size of the matrix
#[proc_macro_derive(Matrix, attributes(matrix))]
pub fn derive(input: OldTokenStream) -> OldTokenStream {
    let DeriveInput { ident, attrs, .. } = parse_macro_input!(input);

    let matrix_size_literal = match &attrs[0].meta {
        Meta::List(MetaList { tokens, .. }) => parse2::<Literal>(tokens.clone()).unwrap(),
        _ => panic!("WTF")
    };

    let matrix_size = matrix_size_literal
        .to_string()
        .parse::<usize>()
        .unwrap();

    let identity_matrix_str = TokenStream::from_str(
        &generate_identity_matrix(&ident, matrix_size)
    ).unwrap();

     println!("{}", generate_identity_matrix(&ident, matrix_size));

    let output = quote! {
        impl Matrix for #ident {
            const SIZE: usize = #matrix_size;
            const IDENTITY: Self = #identity_matrix_str;
        }
    };

    output.into()
}

fn generate_identity_matrix(ident: &Ident, size: usize) -> String {
    let mut output = format!("{}([", ident.to_string());

    for row in 0..size {
        output += "[";

        for col in 0..size {
            output += if row == col { "1.0" } else { "0.0" };
            output += if col == size-1 { "" } else { ", " };
        }

        output += "]";
        output += if row == size-1 { "" } else { ", " };
    }

    output += "])";

    output
}
