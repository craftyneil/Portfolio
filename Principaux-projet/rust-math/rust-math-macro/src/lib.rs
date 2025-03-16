#![deny(rustdoc::invalid_rust_codeblocks)]
#![feature(let_chains)]
#![feature(proc_macro_diagnostic)]
#![recursion_limit = "256"]
#![allow(unused)]

use proc_macro::{Diagnostic, TokenStream};
use quote::quote;
use syn::{
    FnArg, GenericParam, Generics, Ident, ImplItem, ImplItemFn, ItemImpl, PatType, Receiver,
    ReturnType, TypeParam, parse::Parse, parse_macro_input, punctuated::Punctuated,
    spanned::Spanned, token::Comma,
};

const FUNCTION_TEST_DATA_ATTRIBUTE_NAME: &str = "test_data";

fn get_generics(generics: Generics) -> Vec<TypeParam> {
    generics
        .params
        .into_iter()
        .filter_map(|generic| {
            if let GenericParam::Type(function) = generic {
                Some(function)
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug)]
struct TestedFns {
    generics: Vec<TypeParam>,
    functions: Vec<ImplItemFn>,
}

impl Parse for TestedFns {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let tested_impl_block: ItemImpl = input.parse()?;
        let generics = get_generics(tested_impl_block.generics);

        let functions = tested_impl_block
            .items
            .into_iter()
            .filter_map(|item| {
                if let ImplItem::Fn(function) = item
                    && function.attrs.iter().any(|attr| {
                        attr.path().is_ident(FUNCTION_TEST_DATA_ATTRIBUTE_NAME)
                            && attr.meta.require_list().is_ok()
                    })
                {
                    Some(function)
                } else {
                    None
                }
            })
            .collect();

        Ok(TestedFns {
            generics,
            functions,
        })
    }
}

#[derive(Debug)]
enum TestedMethod {
    StaticMethod {
        name: Ident,
        generics: Vec<TypeParam>,
        args: Vec<PatType>,
        return_type: ReturnType,
    },
    InstanceMethod {
        name: Ident,
        generics: Vec<TypeParam>,
        self_type: Receiver,
        args: Vec<PatType>,
        return_type: ReturnType,
    },
}

impl TryFrom<TestedFns> for Vec<TestedMethod> {
    type Error = Vec<Diagnostic>;
    fn try_from(mut value: TestedFns) -> Result<Self, Self::Error> {
        println!("{:#?}", &value);

        fn require_no_self(arg: FnArg, error_message: &str) -> Result<PatType, Diagnostic> {
            match arg {
                FnArg::Receiver(_) => Err(arg.span().unwrap().error(error_message)),
                FnArg::Typed(pat_type) => Ok(pat_type),
            }
        }

        let (values, parse_errors): (Vec<TestedMethod>, Vec<Diagnostic>) =
            value.functions.into_iter().map(|f| f.sig).map(|signature| {
                let name = signature.ident.clone();
                let mut generics = get_generics(signature.generics);
                generics.append(&mut value.generics);
                let return_type = signature.output.clone();

                if let Some(self_type) = signature.receiver() {
                    signature.inputs.pop();
                } else {
                }
            });

        if parse_errors.len() > 0 {
            return Err(parse_errors);
        }

        Ok(values)

        // .map(|mut signature| {
        // 	let name = signature.ident.clone();
        // 	let mut generics = get_generics(signature.generics.clone());
        // 	generics.append(&mut value.generics);
        // 	let return_type = signature.output.clone();
        // 	if let Some(self_type) = signature.receiver() {
        // 		TestedMethod::InstanceMethod {
        // 			name,
        // 			generics,
        // 			self_type: self_type.clone(),
        // 			args: {
        // 				signature.inputs.pop();
        // 				signature
        // 					.inputs
        // 					.clone()
        // 					.into_iter()
        // 					.map(|arg| {
        // 						match arg {
        // 							FnArg::Receiver(_) => break 'error Err(signature.span().unwrap().error(
        // 								"An instance method cannot have 2 or more self parameters"
        // 							)),
        // 							FnArg::Typed(arg_type) => arg_type
        // 						}
        // 					})
        // 					.collect()
        // 			},
        // 			return_type,
        // 		}
        // 	} else {
        // 		TestedMethod::StaticMethod {
        // 			name,
        // 			generics,
        // 			args: signature
        // 				.inputs
        // 				.clone()
        // 				.into_iter()
        // 				.map(|arg| {
        // 					if let FnArg::Typed(arg_type) = arg {
        // 						arg_type
        // 					} else {
        // 						break 'error Err(signature
        // 							.span()
        // 							.unwrap()
        // 							.error("Cannot have a self type in a static method."));
        // 					}
        // 				})
        // 				.collect(),
        // 			return_type,
        // 		}
        // 	}
        // })
    }
}

fn test_data() {}

#[proc_macro_attribute]
pub fn test_builder(attr: TokenStream, item: TokenStream) -> TokenStream {
    let all_methods = parse_macro_input!(item as TestedFns).try_into();

    if let Err(error) = all_methods {
        error.emit();
        return TokenStream::new();
    }

    let all_methods = all_methods.unwrap();

    // println!("{all_method:#?}");

    //let all_expanded = all_methods
    //    .iter()
    //    .map(|tested_method| match tested_method {
    //        TestedMethod::StaticMethod {
    //            name,
    //            generics,
    //            args,
    //            return_type,
    //        } => {
    //            let test_method_name = quote! {
    //                test_for_ #name _for_type_
    //            };
    //        }
    //        TestedMethod::InstanceMethod {
    //            name,
    //            generics,
    //            self_type,
    //            args,
    //            return_type,
    //        } => todo!(),
    //    })
    //    .collect::<Vec<TokenStream>>();

    // let expanded = quote! {};

    // TokenStream::from(expanded)
    TokenStream::new()
}

// // #[cfg(test)]
// mod tests {
//     #![allow(unused_imports)]
//     use crate::{test_builder, SequenceType};
//     use eager::eager;

//     //compile_error!(

//     //);

//     test_builder!(
//         @static_method SequenceType => new_arithmetic;
//         input_types = [
//             [u8] -> [SequenceType<u8>], [u16] -> [SequenceType<u16>],
//             [u32] -> [SequenceType<u32>], [u64] -> [SequenceType<u64>],
//             [u128] -> [SequenceType<u128>], [usize] -> [SequenceType<usize>],
//             [i8] -> [SequenceType<i8>], [i16] -> [SequenceType<i16>],
//             [i32] -> [SequenceType<i32>], [i64] -> [SequenceType<i64>],
//             [i128] -> [SequenceType<i128>], [isize] -> [SequenceType<isize>],
//             [f32] -> [SequenceType<f32>], [f64] -> [SequenceType<f64>]
//         ];
//         [
//             input_values = [1, 10, 6, 200];
//             expected_output_values = [
//                 // 1, 2, 3, 4
//                 // eager! { instance_of_type!(SequenceType::Arithmetic [1, 10, 6, 200]) }
//                 SequenceType::Arithmetic { reason: 1 },
//                 SequenceType::Arithmetic { reason: 10 },
//                 SequenceType::Arithmetic { reason: 6 },
//                 SequenceType::Arithmetic { reason: 200 }
//             ];
//         ]
//     );
// }
