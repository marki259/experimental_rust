use experimental_rust::Experiment;


fn main() {
    let n: i64 = 20;
    let k: i64 = 10;

    let experiment = Experiment::new(n, k);

    println!("{:?}", experiment);
}