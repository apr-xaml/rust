#![allow(non_snake_case)]
#![cfg(test)]

use std::collections::HashMap;


#[test]
fn Creating_map_of_char_u32()
{
    let counts = "ABCDE"
        .chars()
        .map(|c| (c, 0_i32))
        .collect::<HashMap<_, _>>();
}

#[test]
fn Creating_map_of_u32_str_1()
{
    let vec = vec![("One", 1u32), ("Two", 2u32), ("Three", 3u32)];

    let map = vec
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();

    let len = vec.len();
}


#[test]
fn Creating_map_of_u32_str_2()
{
    let vec = vec![("One", 1u32), ("Two", 2u32), ("Three", 3u32)];

    let map = vec
        .into_iter()
        .collect::<HashMap<_, _>>();

    //Error: used of moved value
    //let len = vec.len();
}

#[test]
fn Collecting_values_of_map_of_u32_str()
{
    let vec = vec![("One", 1u32), ("Two", 2u32), ("Three", 3u32)];

    let map: HashMap<&str, u32> = vec
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();

    let vals = map
        .values()
    ;

    let valsVec = vals
        .map(|x| x.clone())
        .collect::<Vec<u32>>();
}


#[test]
fn Collecting_values_of_map_of_u32_MyRecord_1()
{
    struct MyRecord { id: u32 }

    let vec = vec![("One", MyRecord { id: 1 }), ("Two", MyRecord { id: 2 }), ("Three", MyRecord { id: 3 })];

    {
        let map: HashMap<&str, &MyRecord> = vec
            .iter()
            .map(|x| (x.0, &(x.1)))
            .collect();

        let vals = map
            .values();


        let valsVec: Vec<&&MyRecord> = vals
            .collect();
    }
}



#[test]
fn Collecting_values_of_map_of_u32_MyRecord_2()
{
    struct MyRecord { id: u32 }

    let vec = vec![("One", MyRecord { id: 1 }), ("Two", MyRecord { id: 2 }), ("Three", MyRecord { id: 3 })];


    let map: HashMap<&str, MyRecord> = vec
        .into_iter()
        .map(|x| (x.0, x.1))
        .collect();

    let vals = map.into_iter().map(|x| x.1);


    let valsVec: Vec<MyRecord> = vals
        .collect();
}
