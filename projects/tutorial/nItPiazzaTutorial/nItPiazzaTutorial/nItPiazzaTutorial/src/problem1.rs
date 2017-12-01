#![allow(non_snake_case)]

pub fn sum(slice: &[i32], default: i32) -> i32 {
    let mut iSum = default;

    for iVal in slice {
        let val = iVal;
        iSum = iSum + val;
    }

    iSum
}


pub fn distinct(vec: &Vec<i32>) -> Vec<i32> {
    let mut distinctVec: Vec<i32> = Vec::new();

    for iVal in vec {
        let hasAlreadyBeenSeen = distinctVec.contains(iVal);

        if hasAlreadyBeenSeen {
            continue;
        } else {
            let valueToPush: i32 = *(iVal);
            distinctVec.push(valueToPush);
        }
    }

    distinctVec
}





pub fn filter(vec: &Vec<i32>, oxPred: &(Fn(i32) -> bool)) -> Vec<i32> {
    
    let mut filtered:Vec<i32> = Vec::new();

    for iElem in vec {
        let predRes = oxPred(*iElem);

        if predRes {
            filtered.push(*iElem);
        }
    }

    return filtered;  
}
