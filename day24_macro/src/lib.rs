use day24_shared::{Instruction, Variable};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::fs::File;
use std::io::{BufRead, BufReader};
use syn::{parse_macro_input, DeriveInput};

mod asm;

#[proc_macro_derive(AluProgram, attributes(alu_program))]
pub fn alu_program_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let attr = input
        .attrs
        .get(0)
        .expect("#[alu_program(\"input.txt\")] missing");
    let lit: syn::LitStr = attr
        .parse_args()
        .expect("argument should be a string with the file path");

    let input_file = lit.value();
    let instructions = read_instructions_from_file(&input_file).unwrap();

    // Build the trait implementation
    let mut one = asm::impl_macro(&input, &instructions);
    let two = impl_macro(&input, &instructions, &Variable::W, &Variable::Z);
    one.extend(two);
    one
}

fn read_instructions_from_file(file: &str) -> Result<Vec<Instruction>, String> {
    let file = File::open(file).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|x| x.map_err(|e| e.to_string())?.parse::<Instruction>())
        .collect::<Result<_, String>>()?;

    Ok(instructions)
}

fn impl_macro(
    ast: &DeriveInput,
    instructions: &[Instruction],
    input_var: &Variable,
    keep_var: &Variable,
) -> TokenStream {
    let name = &ast.ident;
    let mut instructions_functions = Vec::new();

    let mut chunks = Vec::new();
    let mut lines = Vec::new();
    for instruction in instructions.iter() {
        if instruction.is_input() {
            if lines.is_empty() {
                continue;
            }
            chunks.push(lines);
            lines = Vec::new();
        } else {
            lines.push(instruction);
        }
    }
    chunks.push(lines);

    for (i, chunk) in chunks.into_iter().enumerate() {
        let func_name = Ident::new(&format!("calculate_{}", i), Span::call_site());
        let input_one = Ident::new(&input_var.to_string(), Span::call_site());
        let input_two = Ident::new(&keep_var.to_string(), Span::call_site());
        let mut instructions_code = Vec::new();
        let mut amount_of_inputs = 0;
        for instruction in chunk {
            instructions_code.push(asm::generate_instruction_code_with_opts(
                &mut amount_of_inputs,
                instruction,
            ));
        }

        let let_vars: Vec<_> = Variable::all()
            .into_iter()
            .filter(|v| v != input_var && v != keep_var)
            .map(|v| {
                let var = Ident::new(&v.to_string(), Span::call_site());

                quote! {
                    let #var = 0;
                }
            })
            .collect();

        let calculate = quote! {
            pub fn #func_name(#input_one: i64, #input_two: i64) -> i64 {
                #(#let_vars)*
                #(#instructions_code)*
                #input_two
            }
        };
        instructions_functions.push(calculate)
    }

    let lines: Vec<_> = (0..instructions_functions.len())
        .map(|i| {
            let func_name = Ident::new(&format!("calculate_{}", i), Span::call_site());
            quote! { #i => Self::#func_name(a, b), }
        })
        .collect();
    let global_func = quote! {
        pub fn calculate_n(i: usize, a: i64, b: i64) -> i64 {
            match i {
                #(#lines)*
                _ => unreachable!()
            }
        }
    };

    let gen = quote! {
        impl #name {
            #(#instructions_functions)*
            #global_func
        }
    };
    gen.into()
}
