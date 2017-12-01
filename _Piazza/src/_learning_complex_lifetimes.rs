#![allow(non_snake_case)]
#![cfg(test)]


fn _someFunc<'shorter, 'longer, TA, TB>(shorter: &'shorter TA, b: &'longer TB) where 'longer: 'shorter {}

struct MyRec {}


#[test]
fn Creating_vector_of_references()
{
    let longer = MyRec {};
    {
        let shorter = MyRec {};

        _someFunc(&longer, &shorter);
    }
}


fn anotherFunc<'a, 'b, T: 'a>(a: &'b T) {}

fn anotherFunc2<'a, 'b, T>(arg: &'b T) where T: 'a {}


