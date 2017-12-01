#![allow(non_snake_case)]
#![cfg(test)]


use std::ops::Add;

#[test]
fn Empty_struct_requirec_braces_wjen_creating_instance() {
    struct Empty {};

    let x = Empty {};
}


//TODO: How to instatntiate this?
#[test]
fn Struct_can_have_str_field_but_then_cannot_be_instantiated() {
    struct MyBox {
        name: str,
    };


    // let myBox = MyBox {
    //     name: "hello".to_string(),
    // };
}


#[test]
fn Creating_instance_with_str_name_property() {
    struct Person<'LNames> {
        firstName: &'LNames str,
        lastName: &'LNames str,
    };

    let gossda = Person {
        firstName: "Marcin?",
        lastName: "Gozdera",
    };
}

#[test]
fn Owing_struct_immutably() {
    struct Person<'LNames> {
        firstName: &'LNames str,
        lastName: &'LNames str,
    };

    let gossda = Person {
        firstName: "Marcin?",
        lastName: "Gozdera",
    };



    //gossda.firstName = "Pan Gossda";
    //gossda = Person {firstName: "Boxer", lastName: "Bokserski"};
}

#[test]
fn Owing_struct_mutably() {
    struct Person<'LNames> {
        firstName: &'LNames str,
        lastName: &'LNames str,
    };

    let mut gossda = Person {
        firstName: "Marcin?",
        lastName: "Gozdera",
    };

    gossda.firstName = "Pan Gossda";

    gossda = Person {
        firstName: "Boxer",
        lastName: "Bokserski",
    };
}
