use rand::{Rng, seq::SliceRandom};

#[derive(Debug)]
pub struct Experiment {
    n: i64, 
    k: i64,
    n_comb: u128,  
}

impl Experiment {
    pub fn new(n: i64, k: i64) -> Experiment {
        let n_combs = count_combinations(n, k);

        Experiment { n: n, k: k, n_comb: n_combs }
    }
}

pub fn generate_binary_treatments(n: i64, k: i64) -> Vec<bool> {
    let n_u: usize = n.try_into().unwrap();

    let mut treatment_vector = vec![false; n_u];
    let mut index_vector: Vec<i64> = (0..n).collect();

    let mut rng = rand::thread_rng();

    for _ in 0..k {
        let choice = *index_vector.choose(&mut rng).unwrap();
        let index = choice as usize;

        treatment_vector[index] = true;

        index_vector.retain(|&x| x != choice);
    }

    treatment_vector
}

pub fn count_combinations(n: i64, k: i64) -> u128 {
    let n = n.clone() as u64;
    let k = k.clone() as u64;

    let log_comb = log_factorial(n) - log_factorial(k) - log_factorial(n-k);

    let val = log_comb.exp().round() as u128;    

    val
}

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