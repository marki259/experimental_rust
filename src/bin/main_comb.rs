use experimental_rust::BivExperiment;


fn main() {
    let n = Some(20);
    let k = Some(10);

    let experiment = BivExperiment::new(n, k, None, None);

    println!("{:?}", experiment);
}