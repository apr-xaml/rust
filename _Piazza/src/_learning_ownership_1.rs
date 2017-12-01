#![allow(non_snake_case)]

#![cfg(test)]

#[test]
fn Cannot_reassign_immutable_variable() {
    let x = 1;
    //x = 3;
    assert_eq!(x, 1);
}


#[test]
fn Can_reassign_mutable_variable() {
    let mut x = 1;
    x = 3;
    assert_eq!(x, 3);
}

#[test]
fn Can_create_explicit_var_1() {
    let x: i32 = 1;
}

#[test]
fn Can_create_explicit_var_2() {
    let x: &i32 = &1;
}

#[test]
fn Can_create_explicit_var_3() {
    let x = 1;
    let expected = &3;
}

//Can function return reference of just created object? NO!
