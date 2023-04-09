struct Experiment {
    n: i64, 
    k: i64, 
}

pub fn factorial(x: u128) -> u128 {
    (1..=x).product()
}

pub fn generate_all_orders(n: u64, k: u64, current_order: Vec<u64>, mut all_orders: &mut Vec<Vec<u64>>) {
    let current_len: u64 = current_order.len().try_into().unwrap();
    let current_sum: u64 = current_order.iter().sum();

    let mut current_order1 = current_order.clone();
    let mut current_order2 = current_order.clone();

    if current_len == n {
        if current_sum == k {
            all_orders.push(current_order);
        }
        return;
    }
    if (n - current_len) < (k - current_sum) {
        return;
    }

    current_order1.push(1);
    current_order2.push(0);

    generate_all_orders(n, k, current_order1, &mut all_orders);
    generate_all_orders(n, k, current_order2, &mut all_orders);
    
}