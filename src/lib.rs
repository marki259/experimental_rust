use rand::{Rng, seq::SliceRandom};

#[derive(Debug)]
pub struct Experiment {
    n: i64, 
    k: i64,
    n_comb: u128,  
    x_treatment: Option<Vec<f64>>,
    x_control: Option<Vec<f64>>,
}

#[derive(Debug)]
pub struct BivMean {
    m_treatment: f64,
    m_control: f64,
}

impl Experiment {
    pub fn new(n: Option<i64>, k: Option<i64>, x_treatment: Option<Vec<f64>>, x_control: Option<Vec<f64>>) -> Experiment {
        let n = match n {
            Some(x) => x,
            _ => {
                let n_treatment = x_treatment.as_ref().unwrap().len();
                let n_control = x_control.as_ref().unwrap().len() as i64;

                (n_treatment as i64) + n_control
            }
        };

        let k = match k {
            Some(x) => x,
            _ => {
                let n_treatment = x_treatment.as_ref().unwrap().len();

                (n_treatment as i64)
            }
        };

        let n_combs = count_combinations(n, k);

        Experiment { n: n, 
            k: k, 
            n_comb: n_combs, 
            x_treatment: x_treatment, 
            x_control: x_control }
    }

    pub fn mean(&self) -> BivMean {
        let x_treatment = self.x_treatment.as_ref().unwrap();
        let x_control = self.x_control.as_ref().unwrap(); 

        let n_treatment = x_treatment.len() as f64;
        let n_control = x_control.len() as f64;

        let mut m_treatment: f64 = x_treatment.iter().sum::<f64>();
        let mut m_control: f64 = x_control.iter().sum::<f64>();

        m_treatment /= n_treatment;        
        m_control /= n_control;

        BivMean {
            m_treatment, 
            m_control
        }
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

pub fn generate_all_orders(n: i64, k: i64, current_order: Vec<bool>, mut all_orders: &mut Vec<Vec<bool>>) {
    let current_len: i64 = current_order.len().try_into().unwrap();
    let current_sum: i64 = current_order.iter().map(|b| if *b {1} else {0}).sum();

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

    current_order1.push(true);
    current_order2.push(false);

    generate_all_orders(n, k, current_order1, &mut all_orders);
    generate_all_orders(n, k, current_order2, &mut all_orders);
    
}