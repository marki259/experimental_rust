use experimental_rust::BivExperiment;

fn main() {
    // let n = Some(20);
    // let k = Some(10);

    let x_treatment = vec![10, 5, 5, 2, 3, 5, 9, 10];
    let x_treatment = x_treatment.iter().map(|&x| x as f64).collect();
    let x_treatment = Some(x_treatment);
    
    let x_control = vec![1, 0, 1, 2, 3, 9, 0, 1, 3];
    let x_control = x_control.iter().map(|&x| x as f64).collect();
    let x_control = Some(x_control);


    let experiment = BivExperiment::new(None, None, x_treatment, x_control);

    println!("{:?}", experiment);

    println!("{:#?}", experiment.mean());

    println!("{:#?}", experiment.exact_pval());

    println!("{:#?}", experiment.approx_pval(None));
}

