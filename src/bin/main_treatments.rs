use experimental_rust::generate_binary_treatments;

fn main() {
    let (n, k) = (20, 5);

    let treatment_vector = generate_binary_treatments(n, k);

    println!("{:?}", treatment_vector);
}