use experimental_rust::Experiment;


fn main() {
    let n = Some(20);
    let k = Some(10);

    let experiment = Experiment::new(n, k, None, None);

    println!("{:?}", experiment);
}