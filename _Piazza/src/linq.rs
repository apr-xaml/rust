#![allow(non_snake_case)]

use std::collections::HashMap;
use std::hash::Hash;




pub type OxGetKeyRef<TElem, TKey> = (Fn(&TElem) -> &TKey);


pub struct GroupWithRefs<'a, TKey, TValue>
    where
        TKey: Eq + Hash,
        TKey: 'a,
        TValue: 'a

{
    key: &'a TKey,
    values: Vec<&'a TValue>,
}




pub fn goWhere<'a, T>(vec: &Vec<&'a T>, oxPred: &(Fn(&T) -> bool)) -> Vec<&'a T> {
    let mut filtered = Vec::new();

    for iElem in vec {
        let isOk = oxPred(iElem);

        if !isOk {
            continue;
        } else {
            let toAdd = *iElem;

            filtered.push(toAdd);
        }
    }

    filtered
}





fn _addOrUpdateGroup<'map, 'mapKeyOrElem, TKey, TElem>(map: &'map mut HashMap<&'mapKeyOrElem TKey, GroupWithRefs<'mapKeyOrElem, TKey, TElem>>, key: &'mapKeyOrElem TKey, value: &'mapKeyOrElem TElem)
    where TKey: Eq + Hash, 'mapKeyOrElem: 'map,

{
    let groupMaybe = map.get_mut(key);

    match groupMaybe
        {
            Some(g) =>
                {
                    g.values.push(value)
                }
            None =>
                {
                    let _newG = GroupWithRefs { key, values: vec![value] };
                }
        }
}

pub fn goGroupBy<'a, TElem, TKey>(owingVec: &'a Vec<TElem>, oxGetKeyRef: &'a OxGetKeyRef<TElem, TKey>) -> Vec<GroupWithRefs<'a, TKey, TElem>>
    where
        TKey: Eq + Hash,

{
    let mut groupsByKeyMap: HashMap<&TKey, GroupWithRefs<TKey, TElem>> = HashMap::new();

    for iElemRef in owingVec
        {
            let keyRef: &TKey = oxGetKeyRef(iElemRef);
            _addOrUpdateGroup(&mut groupsByKeyMap, keyRef, iElemRef)
        }

    let values = groupsByKeyMap
        .into_iter()
        .map(|x| x.1);


    let valuesVec: Vec<GroupWithRefs<'a, TKey, TElem>> = values.collect();

    valuesVec
}
