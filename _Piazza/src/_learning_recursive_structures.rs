//struct MyRecordRec<'a>
//{
//    pub id: u32,
//    pub name: &'a str,
//    pub next: Box<MyRecordRec<'a>>
//}


struct MyRecordRec2<'a>
{
    pub id: u32,
    pub name: &'a str,
    pub next: Box<Option<MyRecordRec2<'a>>>
}

struct ClosedLoop<'a>
{
    left: MyRecordRec2<'a>,
    right: MyRecordRec2<'a>
}


#[test]
fn Creating_circular_recursive_data_structure()
{
    let mut n1Mut = MyRecordRec2
        {
            id: 1,
            name: "n1",
            next: Box::new(None)
        };



    let n2 = MyRecordRec2
        {
            id: 2,
            name: "n2",
            next: Box::new(Some(n1Mut))
        };

    n1Mut.next = Box::new(Some(n2));
}


#[test]
fn Creating_and_freezing_circular_recursive_data_structure()
{
    let loopEntry =
        {
            let mut n1Mut = MyRecordRec2
                {
                    id: 1,
                    name: "n1",
                    next: Box::new(None)
                };


            let n2 = MyRecordRec2
                {
                    id: 2,
                    name: "n2",
                    next: Box::new(Some(n1Mut))
                };

            n1Mut.next = Box::new(Some(n2));

            n1Mut
        };
}


//#[test]
//fn Creating_finite_recursive_data_structure()
//{
//    let node = {
//        let mut n1Mut = MyRecordRec2
//            {
//                id: 1,
//                name: "n1Mut",
//                next: Box::new(None)
//            };
//
//
//        let n2 = MyRecordRec2
//            {
//                id: 2,
//                name: "n2",
//                next: Box::new(Some(n1Mut))
//            };
//
//        n1Mut.next = Box::new(Some(n2));
//
//        n1Mut
//    };
//}






