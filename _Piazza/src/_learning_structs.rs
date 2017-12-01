#![allow(non_snake_case)]
#![cfg(test)]

struct Person<'LNames>
{
    firstName: &'LNames str,
    lastName: &'LNames str,
}

#[test]
fn Empty_struct_requires_braces_when_creating_instance() {
    struct Empty {};

    let x = Empty {};
}


//TODO: How to instantiate this?
#[test]
fn Struct_can_have_str_field_but_then_cannot_be_instantiated() {
    struct MyBox
    {
        name: str,
    };


    //     let myBox = MyBox {
    //         name: "hello",
    //     };
}


#[test]
fn Creating_instance_with_str_name_property() {
    let gossda = Person
        {
            firstName: "Marcin?",
            lastName: "Gozdera",
        };
}

#[test]
fn Owing_struct_immutably() {
    let gossda = Person {
        firstName: "Marcin?",
        lastName: "Gozdera",
    };



    //gossda.firstName = "Pan Gossda";
    //gossda = Person {firstName: "Boxer", lastName: "Bokserski"};
}

#[test]
fn Owing_struct_mutably() {
    let mut gossda = Person
        {
            firstName: "Marcin?",
            lastName: "Gozdera",
        };

    gossda.firstName = "Pan Gossda";

    gossda = Person
        {
            firstName: "Boxer",
            lastName: "Bokserski",
        };
}


#[test]
fn Reference_of_a_reference_is_flattened_when_field_is_accessed() {
    let gossda = Person
        {
            firstName: "Marcin?",
            lastName: "Gozdera",
        };

    let gossdaRef = &gossda;
    let gossdaRefRef = &gossdaRef;

    let firstName: &str = gossdaRefRef.firstName;
}

#[test]
fn Even_same_object_cannot_own_another_object_by_two_references() {
    struct DoubleReference<'a>
    {
        left: Person<'a>,
        right: Person<'a>
    }

    let gossda = Person
        {
            firstName: "Marcin?",
            lastName: "Gozdera",
        };

//    let doubleRef = DoubleReference { left: gossda, right: gossda };
}


