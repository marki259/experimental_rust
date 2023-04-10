use experimental_rust;

fn main() {
    let n: i64 = 20;
    let k: i64 = 10;
    let current_order: Vec<bool> = vec![];
    let mut all_orders: Vec<Vec<bool>> = vec![];

    experimental_rust::generate_all_orders(n, k, current_order, &mut all_orders);

    println!("{:?}", all_orders[3]);
}
