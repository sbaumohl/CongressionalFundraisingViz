use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

// SpecificyEntity should take the entity's path and create a basic access function
// CustomFunction should allow one to pass a custom function and add it on to the queryRoot, allowing me to add extra shit
#[proc_macro_derive(CustomizedQueryRoot, attributes(SpecifyEntity, CustomFunction))]
pub fn derive_entity_access(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}