#![allow(non_snake_case)]

#![cfg(test)]


fn _returnSameI32(x: i32) -> i32 {
    x
}


fn _returnDifferentI32(x: i32) -> i32 {
    1
}



fn _returnSameString()
{

}

#[test]
fn Can_read_variable_in_inner_block() {
    let x: i32 = 1;

    {
        let x2 = x;
        assert_eq!(x2, x);
    }

    assert_eq!(x, 1);
}


#[test]
fn Can_read_copyable_variable_in_function_that_owns_and_returns() {
    let x: i32 = 1;

    let xSame = _returnSameI32(x);

    assert_eq!(x, xSame);
}


#[test]
fn Can_read_copyable_variable_in_function_that_owns_and_does_not_return_it() {
    let x: i32 = 1;

    let xSame = _returnDifferentI32(x);

    assert_eq!(x, xSame);
}