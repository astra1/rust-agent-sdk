use std::char::ToLowercase;

#[proc_macro]
pub fn toFuncName(reqType: String) -> String {
    let slice = reqType[..reqType.rfind(".") + 1];
    ToLowercase(slice.chars().nth(0).unwrap()) + slice[..1]
}
