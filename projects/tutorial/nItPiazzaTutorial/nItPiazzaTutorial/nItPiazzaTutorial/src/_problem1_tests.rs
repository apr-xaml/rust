#![allow(non_snake_case)]
#![cfg(test)]

use problem1::{distinct, filter, sum};


// use problem2::mat_mult;
// use problem3::sieve;
// use problem4::{hanoi, Peg};

//
// Problem 1
//

// Part 1
#[test]
fn Can_compute_sum_on_empty_slice() {
    let array = [];
    let default = 10;
    let res = sum(&array, default);
    assert_eq!(res, default);
}

#[test]
fn Can_compute_sum_on_one_element_slice() {
    let array = [1];
    let default = 10;
    let res = sum(&array, default);
    assert_eq!(res, 11);
}

#[test]
fn Can_compute_sum() {
    let array = [1, 2, 3, 4, 5];
    let res = sum(&array, 0);
    assert_eq!(res, 15);
}



#[test]
fn Can_distinct_items() {
    let vs = vec![1, 2, 2, 3, 4, 1];
    let actual = distinct(&vs);
    assert_eq!(actual, vec![1, 2, 3, 4]);
}





#[test]
fn test_filter_small() {
    
    fn even_predicate(x: i32) -> bool {
        (x % 2) == 0
    }


    let vs = vec![1, 2, 3, 4, 5];

    let actual = filter(&vs, &even_predicate);
    let expected = vec![2, 4];

    assert_eq!(actual, expected);
}

// //
// // Problem 2
// //

// #[test]
// fn test_mat_mult_identity() {
//     let mut mat1 = vec![vec![0.;3]; 3];
//     for i in 0..mat1.len() {
//         mat1[i][i] = 1.;
//     }
//     let mat2 = vec![vec![5.;3]; 3];
//     let result = mat_mult(&mat1, &mat2);
//     for i in 0..result.len() {
//         for j in 0..result[i].len() {
//             assert_eq!(result[i][j], mat2[i][j]);
//         }
//     }
// }

// //
// // Problem 3
// //

// #[test]
// fn test_sieve_basic() {
//     assert_eq!(vec![2,3,5,7,11], sieve(12));
// }

// //
// // Problem 4
// //

// #[test]
// fn test_hanoi_1_disks() {
//     let result = hanoi(1, Peg::A, Peg::B, Peg::C);
//     assert_eq!(vec![(Peg::A, Peg::C)], result);
//     assert_eq!(1, result.len());
// }
