fn main() {
    let a = vec![3, 0, 1, 2];
    let bool = vec![true, false, false, true];
    let mut mask = bool.iter();

    let mut mask_clone = mask.clone();

    let mut b = a.clone();
    b.retain(|_| *mask.next().unwrap());

    println!("{:?}", b);

    let mut c = a.clone();
    c.retain(|_| !mask_clone.next().unwrap());

    println!("{:?}", c);
}