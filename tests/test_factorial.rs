use experimental_rust;

#[test]
fn fac_of_three() {
    let x: u128 = 3;
    let fact = experimental_rust::factorial(x);

    assert_eq!(6, fact);
}

#[test]
fn fac_of_zero() {
    let x = 0; 
    let fact = experimental_rust::factorial(x);

    assert_eq!(1, fact);
}