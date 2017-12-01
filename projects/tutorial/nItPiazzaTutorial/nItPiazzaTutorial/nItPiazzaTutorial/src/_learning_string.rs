#![allow(non_snake_case)]
#![cfg(test)]


use std::ops::Add;

#[test]
fn String_literal_is_considered_to_be_strref() {
    let x: &str = "hello";
}

#[test]
fn Object_strref_can_be_cloned_and_thus_considered_owned() {
    let strref = "hello";
    let x: String = strref.to_string();
}


#[test]
fn It_is_not_possible_at_all_to_have_str_variable() {
    //The one below simply does not compile
    // let name: str = *("hello");

}


#[test]
fn StringRef_can_be_treated_as_strref() {
    let StringRef = &("hello".to_string());
    let strref: &str = StringRef;
}


#[test]
fn String_can_be_mutated_but_the_original_one_is_destroyed() {
    let StringRef: String = String::new();
    let extended = StringRef.add("hello");

    assert_eq!("hello", extended);
}


fn _borrowStringAndReturnNewString(x: &String) -> String {
    return String::from("new String");
}

fn _borrowStringAndReturnSliceOfIt(x: &String) -> &str {
    return &x[2..];
}


fn _borrowStringAndReturnSliceOfBoth<'a, 'b>(x: &'a String, y: &'b String) -> (&'a str, &'b str) {
    return (&x[2..], &y[2..]);
}



#[test]
fn StringSlice_keeps_original_string_immutable() {
    let mut x = String::from("hello world");
    let mut y = String::from("nasty team of Devs!");

    let slice = _borrowStringAndReturnSliceOfBoth(&x, &y);

    //x.clear();
}
