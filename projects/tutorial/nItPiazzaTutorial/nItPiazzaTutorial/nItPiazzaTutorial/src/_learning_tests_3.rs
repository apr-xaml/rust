#![allow(non_snake_case)]

#![cfg(test)]


// fn _returnSameString(x: str) -> str {
//     x
// }


// fn _returnDifferentString(x: str) -> str {
//     "hello"
// }



// #[test]
// fn Can_read_variable_in_function_that_owns_and_returns() {
//     let x: i32 = 1;

//     let xSame = _returnSameString(x);

//     assert_eq!(x, xSame);
// }


// #[test]
// fn Can_read_variable_in_function_that_owns_and_does_not_return_it() {
//     let x: i32 = 1;

//     let xSame = _returnDifferentString(x);

//     assert_eq!(x, xSame);
// }