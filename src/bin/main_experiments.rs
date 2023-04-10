use experimental_rust::Experiment;

fn main() {
    // let n = Some(20);
    // let k = Some(10);

    let x_treatment = vec![3, 2, 1];
    let x_treatment = x_treatment.iter().map(|&x| x as f64).collect();
    let x_treatment = Some(x_treatment);
    
    let x_control = vec![3, 0, 0, 2];
    let x_control = x_control.iter().map(|&x| x as f64).collect();
    let x_control = Some(x_control);


    let experiment = Experiment::new(None, None, x_treatment, x_control);

    println!("{:?}", experiment);
}

