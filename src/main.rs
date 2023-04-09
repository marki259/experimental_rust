use experimental_rust;

fn main() {
    let n: i64 = 20;
    let k: i64 = 10;
    let current_order: Vec<i64> = vec![];
    let mut all_orders: Vec<Vec<i64>> = vec![];

    experimental_rust::generate_all_orders(n, k, current_order, &mut all_orders);

    println!("{:?}", all_orders[0]);
}
