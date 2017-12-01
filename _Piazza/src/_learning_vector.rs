#![allow(non_snake_case)]
#![cfg(test)]


#[test]
fn Creating_vector_of_ints() {
    let mut vec = Vec::new();

    let expected = vec![1];
    vec.push(1);

    assert_eq!(expected, vec);
}


#[test]
fn Macro_creates_mutable_vector_of_ints() {
    let mut vec = vec![1];
    vec.push(1);
}


#[test]
fn Mutating_reference_means_reference_and_object_can_be_mutated() {
    let mut vec = vec![1];
    vec.push(1);
    vec = vec![2];
}

#[test]
fn Immutable_reference_means_reference_and_object_cannot_be_mutated() {
    let vec = vec![1];
    //vec = vec![2];
    //vec.push(1);
}


#[test]
fn Shallow_vector_copy_gives_back_reference() {
    let vec = vec![1];

    let vecCopy = vec.clone();

    // vecCopy.push(2);
}

#[test]
fn Using_star_operator_can_obtain_ownership() {
    struct XSample {
        name: String,
    }

    let vec: Vec<XSample> = vec![
        XSample {
            name: String::from("Mr.Gozdera"),
        },
    ];


    for iEl in vec {
        let eleToAdd: XSample = iEl;
    }
}


