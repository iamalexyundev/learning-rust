#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn main() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v1: Vec<i32> = vec![1, 2, 3];

    let added_v1 = v1.iter().map(|x| x + 1).collect::<Vec<_>>();
    println!("{:?}", added_v1)
}
