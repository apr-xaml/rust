#![allow(non_snake_case)]

use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::Values;



// pub fn goDuplicate<T>(vec: &Vec<T>) -> (Vec<T>, Vec<T>) {
//     let mut vec1 = Vec::new();
//     let mut vec2 = Vec::new();

//     for iElem in vec {
//         let x1 = *iElem;
//         let x2 = *iElem;

//         vec1.push(x1);
//         vec2.push(x2);
//     }

//     (vec1, vec2)
// }




pub fn goWhere<'a, T>(vec: &Vec<&'a T>, oxPred: &(Fn(&T) -> bool)) -> Vec<&'a T> {
    let mut filtered = Vec::new();

    for iElem in vec {
        let isOk = oxPred(iElem);

        if !isOk {
            continue;
        } else {
            let mut toAdd = *iElem;

            filtered.push(toAdd);
        }
    }

    filtered
}


pub struct Group<'LifeKey, 'LifeVal, TKey: 'LifeKey, TValue: 'LifeVal>
where
    TKey: Eq + Hash,
    'LifeVal: 'LifeKey,
{
    key: &'LifeKey TKey,
    values: Vec<&'LifeVal TValue>,
}


pub type OxGetKeyRef<TElem, TKey> = (Fn(&TElem) -> &TKey);


pub fn goGroupBy<'LifeKey, 'LifeElem, TElem: 'LifeElem, TKey: 'LifeKey>(
    vec: &Vec<TElem>,
    oxGetKeyRef: &OxGetKeyRef<TElem, TKey>,
) -> Vec<Group<'LifeKey, 'LifeElem, TKey, TElem>>
where
    TKey: Eq + Hash,
    'LifeElem: 'LifeKey,
{
    let mut groupsByKeyMap: HashMap<&TKey, Group<TKey, TElem>> = HashMap::new();

    for iE in vec {
        let key: &TKey = oxGetKeyRef(iE);

        let groupMaybe: Option<_> = groupsByKeyMap.get(key);

        let group: &Group<TKey, TElem> = match groupMaybe {
            Some(group) => group,
            None => {
                let newEmptyGroup: Group<TKey, TElem> = Group {
                    key: key,
                    values: vec![],
                };
                //let res = groupsByKeyMap.insert(key, newEmptyGroup);
                //How the fuck compiler knows that 'newEmptyGroup' is not a dangling reference???
                &newEmptyGroup
            }
        };
    }

    //let values: Vec<Group<'LifeKey, 'LifeElem, &'LifeKey TKey, &'LifeElem TElem>> =
    let valuesIter: Values<&TKey, Group<TKey, TElem>> = groupsByKeyMap.values();
 
    let values: Vec<Group<TKey, TElem>> = valuesIter.collect();

    values
}
