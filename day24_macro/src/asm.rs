use day24_shared::{Instruction, Item};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

pub fn impl_macro(ast: &DeriveInput, instructions: &[Instruction]) -> TokenStream {
    let name = &ast.ident;
    let mut amount_of_inputs = 0;
    let mut instructions_code = Vec::new();
    for instruction in instructions.iter() {
        instructions_code.push(generate_instruction_code_with_opts(
            &mut amount_of_inputs,
            instruction,
        ));
    }

    // let inputs: Vec<_> = (0..amount_of_inputs).map(|_| quote! {i64}).collect();
    let calculate = quote! {
        pub fn calculate(input: &[i64]) -> i64 {
            let mut x = 0;
            let mut y = 0;
            let mut z = 0;
            let mut w = 0;
            #(#instructions_code)*
            z
        }
    };

    let gen = quote! {
        impl #name {
            #calculate
        }
    };
    gen.into()
}

#[allow(dead_code)]
fn generate_instruction_code_mut(input: &mut usize, instruction: &Instruction) -> TokenStream2 {
    use Instruction::*;

    let into_var = instruction
        .variables()
        .next()
        .expect("instruction needs the first argument to be a variable")
        .to_string();
    let ident = syn::Ident::new(&into_var, proc_macro2::Span::call_site());

    if instruction.is_input() {
        let index = syn::Index::from(*input);
        // let res = quote! {
        //     #ident = input.#index;
        // };
        let res = quote! {
            #ident = input[#index];
        };
        *input += 1;

        return res;
    }

    match instruction {
        // variables
        Add(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                #ident += #variable;
            }
        }
        Multiply(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                #ident *= #variable;
            }
        }
        Modulo(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                #ident %= #variable;
            }
        }
        Divide(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                #ident /= #variable;
            }
        }
        Equal(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                #ident = if #ident == #variable { 1 } else { 0 };
            }
        }
        // constants
        Add(_, Item::Value(value)) => {
            quote! {
                #ident += #value;
            }
        }
        Multiply(_, Item::Value(value)) => {
            quote! {
                #ident *= #value;
            }
        }
        Modulo(_, Item::Value(value)) => {
            quote! {
                #ident %= #value;
            }
        }
        Divide(_, Item::Value(value)) => {
            quote! {
                #ident /= #value;
            }
        }
        Equal(_, Item::Value(value)) => {
            quote! {
                #ident = if #ident == #value { 1 } else { 0 };
            }
        }

        _ => unreachable!(),
    }
}

#[allow(dead_code)]
fn generate_instruction_code(input: &mut usize, instruction: &Instruction) -> TokenStream2 {
    use Instruction::*;

    let into_var = instruction
        .variables()
        .next()
        .expect("instruction needs the first argument to be a variable")
        .to_string();
    let ident = syn::Ident::new(&into_var, proc_macro2::Span::call_site());

    if instruction.is_input() {
        let index = syn::Index::from(*input);
        // let res = quote! {
        //     #ident = input.#index;
        // };
        let res = quote! {
            let #ident = input[#index];
        };
        *input += 1;

        return res;
    }

    match instruction {
        // variables
        Add(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                let #ident = #ident + #variable;
            }
        }
        Multiply(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                let #ident = #ident * #variable;
            }
        }
        Modulo(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                let #ident = #ident % #variable;
            }
        }
        Divide(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                let #ident = #ident / #variable;
            }
        }
        Equal(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                let #ident = if #ident == #variable { 1 } else { 0 };
            }
        }
        // constants
        Add(_, Item::Value(value)) => {
            quote! {
                let #ident = #ident + #value;
            }
        }
        Multiply(_, Item::Value(value)) => {
            quote! {
                let #ident = #ident * #value;
            }
        }
        Modulo(_, Item::Value(value)) => {
            quote! {
                let #ident = #ident % #value;
            }
        }
        Divide(_, Item::Value(value)) => {
            quote! {
                let #ident = #ident / #value;
            }
        }
        Equal(_, Item::Value(value)) => {
            quote! {
                let #ident = if #ident == #value { 1 } else { 0 };
            }
        }

        _ => unreachable!(),
    }
}

pub fn generate_instruction_code_with_opts(
    input: &mut usize,
    instruction: &Instruction,
) -> TokenStream2 {
    use Instruction::*;

    let into_var = instruction
        .variables()
        .next()
        .expect("instruction needs the first argument to be a variable")
        .to_string();
    let ident = syn::Ident::new(&into_var, proc_macro2::Span::call_site());

    if instruction.is_input() {
        let index = syn::Index::from(*input);
        // let res = quote! {
        //     #ident = input.#index;
        // };
        let res = quote! {
            let #ident = input[#index];
        };
        *input += 1;

        return res;
    }

    match instruction {
        // variables
        Add(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                let #ident = #ident + #variable;
            }
        }
        Multiply(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                let #ident = #ident * #variable;
            }
        }
        Modulo(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                let #ident = #ident % #variable;
            }
        }
        Divide(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                let #ident = #ident / #variable;
            }
        }
        Equal(_, Item::Variable(var)) => {
            let variable = syn::Ident::new(&var.to_string(), proc_macro2::Span::call_site());

            quote! {
                let #ident = if #ident == #variable { 1 } else { 0 };
            }
        }
        // constants
        Add(_, Item::Value(0)) => {
            quote! {}
        }
        Add(_, Item::Value(value)) => {
            quote! {
                let #ident = #ident + #value;
            }
        }
        Multiply(_, Item::Value(0)) => {
            quote! {
                let #ident = 0;
            }
        }
        Multiply(_, Item::Value(value)) => {
            quote! {
                let #ident = #ident * #value;
            }
        }
        Modulo(_, Item::Value(value)) => {
            quote! {
                let #ident = #ident % #value;
            }
        }
        Divide(_, Item::Value(1)) => {
            quote! {}
        }
        Divide(_, Item::Value(value)) => {
            quote! {
                let #ident = #ident / #value;
            }
        }
        Equal(_, Item::Value(value)) => {
            quote! {
                let #ident = if #ident == #value { 1 } else { 0 };
            }
        }

        _ => unreachable!(),
    }
}
