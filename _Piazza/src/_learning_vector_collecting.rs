#![allow(non_snake_case)]
#![cfg(test)]


struct XSample {}

#[test]
fn Creating_vector_of_references()
{


    //    fn _createVector() -> Vec<&XSample>
    //    {
    //        let x = XSample {};
    //
    //        let vec = vec![x];
    //
    //        vec
    //    }
    //
    //    let vec = _createVector();
}


#[test]
fn Collecting_vector_of_values()
{

    //    let vec = vec![XSample {}];
    //
    //
    //    let collected = vec.collect();
}

#[test]
fn Collecting_vector_of_ints_with_type_specification_on_right_side()
{
    let vec: Vec<u32> = vec![1, 2, 3];


    let collected = vec.iter().collect::<Vec<_>>();
}


#[test]
fn Collecting_vector_of_references()
{

    let vec: Vec<XSample> = vec![XSample {}];


    let collected: Vec<_> = vec.iter().collect();
}
