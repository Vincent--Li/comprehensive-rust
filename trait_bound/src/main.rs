fn duplicate<T>(a: Option<T>) -> (Option<T>, Option<T>) 
where
  Option<T>: Clone,
{
    (a.clone(), a.clone())
}

#[derive(Debug,Clone)]
struct NotCloneable;

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(Some(foo));
    println!("{pair:?}");

    let nc = NotCloneable;
    let ncd = duplicate(Some(nc));
    println!("{ncd:?}");

    let none: Option<u32> = None;
    let noneDuplicate = duplicate(none);
    println!("{noneDuplicate:?}");
}