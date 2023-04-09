struct Experiment {
    n: i64, 
    k: i64, 
}

// pub fn count_combinations(n: &i128, k: &i128) -> i128 {
//     let n = n.clone() as f64;
//     let k = k.clone() as f64;

//     log_comb = (1..=n).sum() - (n - k).iter().log().sum() - k.iter().log().sum();

// }

pub fn factorial(x: u64) -> u128 {
    let s = log_factorial(x);

    let s = s.exp().round() as u128;
    
    s
}

pub fn log_factorial(x: u64) -> f64 {
    let mut s: f64 = 0.0;

    for i in 1..=x {
        let mut j = i as f64;
        j = j.ln();
        s += j;
    }

    s
}

pub fn generate_all_orders(n: i64, k: i64, current_order: Vec<i64>, mut all_orders: &mut Vec<Vec<i64>>) {
    let current_len: i64 = current_order.len().try_into().unwrap();
    let current_sum: i64 = current_order.iter().sum();

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