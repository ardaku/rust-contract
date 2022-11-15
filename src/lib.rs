extern crate proc_macro;
use proc_macro::TokenStream;
use std::collections::HashMap;
use proc_macro2::Literal;
use proc_macro_error::{abort};
use proc_macro_error::abort_call_site;
use proc_macro_error::proc_macro_error;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{Token, Type};

#[allow(dead_code)]
enum ContractType {
    Vary,
    None,
    Some,
    Ok,
    Err,
    True,
    False,
    Panic,
    SameType(String),
}

struct Contract {
    inputs: HashMap<String, ContractType>,
    output: ContractType,
}


/// Contract annotation, using a [procedural macro attribute](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
/// # Syntax:
/// ```rust
/// #[contract(param1 = value1 -> output1, •••)]
/// fn my_function(param1: type1) -> output1type {
///    /* function body */
/// }
/// ```
///
/// Output can be any of:
///
/// * `vary` (any value),
/// * `none` (as in Option::None),
/// * `some` (as in Option::Some),
/// * `ok` (as in Result::Ok),
/// * `err` (as in Result::Err)
/// * `true` (as in boolean true)
/// * `false` (as in boolean false)
/// * `panic` (function calls `panic!()`)
/// * Value of type `output1type` (as in `return output1;`)
///
/// Inputs can be any of these except for `panic`.
///
///
/// # Examples
/// ```rust
/// #[contract("x = vary -> vary", "x = 0 -> 1", "x = 1 -> 1")]
/// pub fn factorial(x: u32) -> u32 {
///     if x == 0 || x == 1 {
///         1
///     } else {
///         x * factorial(x - 1)
///     }
/// }
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn contract(params: TokenStream, item: TokenStream) -> TokenStream {
    let contracts_in: Punctuated<Literal, Token![,]> =
        syn::parse_macro_input!(params with Punctuated<Literal, Token![,]>::parse_terminated);

    let item_clone = item.clone();
    let return_type = match syn::parse_macro_input!(item_clone as syn::ItemFn).sig.output {
        syn::ReturnType::Default => abort_call_site!("Function must have a non-unit return type"),
        syn::ReturnType::Type(_, ty) => *ty,
    };

    if let syn::Type::Path(ref ty) = return_type {
        if ty.to_token_stream().to_string() == "()" {
            abort!(return_type, "Function must have a non-unit return type");
        }
    } else {
        abort!(return_type, "FUNCTION MUST HAVE A NON-UNIT RETURN TYPE");
    }

    let item_clone = item.clone();
    let params = syn::parse_macro_input!(item_clone as syn::ItemFn).sig.inputs;
    let params = if params.is_empty() {
        abort_call_site!("No contract(s) specified");
    } else {
        params
    };

    let mut contracts: Vec<Contract> = vec![];
    let mut vary_vary_vary_contract = Contract {
        inputs: HashMap::new(),
        output: ContractType::Vary,
    };

    for param in &params {
        let param = match param {
            syn::FnArg::Typed(param) => param,
            _ => abort_call_site!("Function must have typed parameters"),
        };
        let param_name = match param.pat.as_ref() {
            syn::Pat::Ident(param_name) => param_name.ident.to_string(),
            _ => abort_call_site!("Function must have named parameters 1"),
        };
        let param_type = match param.ty.as_ref() {
            syn::Type::Path(param_type) => param_type.to_token_stream().to_string(),
            _ => abort_call_site!("Function must have named parameters 2"),
        };
        vary_vary_vary_contract.inputs.insert(param_name, ContractType::Vary);
    }

    for contract in contracts_in {
        // HashMap<param_name, type>
        let mut param_types: HashMap<String, Type> = HashMap::new();
        for param in &params {
            if let syn::FnArg::Typed(syn::PatType { pat, ty, .. }) = param {
                if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = &**pat {
                    param_types.insert(ident.to_string(), *ty.clone());
                } else {
                    let some_if_placeholder = if let syn::Pat::Wild(_) = &**pat {
                        Some("Placeholder parameter names are not allowed in functions with contracts")
                    } else {
                        None
                    };
                    abort!(param, "Invalid parameter name";
                        help = "Parameter names must be valid identifiers";
                        help =? some_if_placeholder;
                    );
                }
            } else {
                abort_call_site!("Function parameters must be named");
            }
        }

        let mut contract_out = Contract {
            inputs: HashMap::new(),
            output: ContractType::Vary,
        };

        let contract = contract.to_string();
        let mut contract = contract.split("->");
        let inputs = contract.next().unwrap();

        let mut inputs = inputs.split(",");

        let output = contract.next().unwrap();
        match output.trim() {
            "vary" => contract_out.output = ContractType::Vary,
            "none" => contract_out.output = ContractType::None,
            "some" => contract_out.output = ContractType::Some,
            "ok" => contract_out.output = ContractType::Ok,
            "err" => contract_out.output = ContractType::Err,
            "true" => contract_out.output = ContractType::True,
            "false" => contract_out.output = ContractType::False,
            "panic" => contract_out.output = ContractType::Panic,
            _ => {
                // Either `ContractType::SameType` or an error
                let output_type = match syn::parse_str::<Type>(output.trim()) {
                    Ok(output_type) => output_type,
                    Err(_) => abort_call_site!("Invalid contract output type")
                };
                if let syn::Type::Verbatim(ref ty) = output_type {
                    if ty.to_string() == "()" {
                        abort!(return_type, "Function must have a non-unit return type");
                    } else if ty.is_empty() {
                        abort!(return_type, "Function must have a non-unit return type");
                    } else if ty.to_string() == return_type.to_token_stream().to_string() {
                        contract_out.output = ContractType::SameType(output.trim().to_owned());
                    } else {
                        abort!(return_type, "Contract output type must match function return type");
                    }
                }
            }
        }
    }

    item
}
