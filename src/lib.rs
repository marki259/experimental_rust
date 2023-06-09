use std::thread::current;

use rand::{seq::SliceRandom, Rng};

#[derive(Debug)]
pub struct BivExperiment {
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

impl BivExperiment {
    pub fn new(
        n: Option<i64>,
        k: Option<i64>,
        x_treatment: Option<Vec<f64>>,
        x_control: Option<Vec<f64>>,
    ) -> BivExperiment {
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

        BivExperiment {
            n: n,
            k: k,
            n_comb: n_combs,
            x_treatment: x_treatment,
            x_control: x_control,
        }
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
            m_control,
        }
    }

    pub fn exact_pval(&self) -> f64 {
        let n = self.n;
        let k = self.k;

        let x_treatment = self.x_treatment.as_ref().unwrap().clone();
        let x_control = self.x_control.as_ref().unwrap().clone();

        let x_all: Vec<f64> = x_treatment.iter().chain(x_control.iter()).map(|&x| x as f64).collect();

        let means = self.mean();
        let observed_effect = means.m_treatment - means.m_control;

        let mut all_orders = vec![];
        let current_order = vec![];
        generate_all_orders(n, k, current_order, &mut all_orders);

        let mut counter: i64 = 0;

        for placebo in all_orders.iter() {
            let placebo = placebo.clone();
            let mut mask = placebo.iter();
            let mut mask_clone = mask.clone();

            let mut x_placebo_treatment: Vec<f64> = x_all.iter().map(|&x| x as f64).collect();
            x_placebo_treatment.retain(|_| *mask.next().unwrap());

            let mut x_placebo_control: Vec<f64> = x_all.iter().map(|&x| x as f64).collect();
            x_placebo_control.retain(|_| !mask_clone.next().unwrap());

            let placebo_experiment = BivExperiment::new(
                None,
                None,
                Some(x_placebo_treatment),
                Some(x_placebo_control),
            );

            let placebo_means = placebo_experiment.mean();

            let placebo_delta = placebo_means.m_treatment - placebo_means.m_control;

            if observed_effect.abs() > placebo_delta.abs() {
                counter += 1;
            } else {
                continue;
            }
        }

        1.0 - ((counter as f64).ln() - (self.n_comb as f64).ln()).exp()
    }

    pub fn approx_pval(&self, nsim: Option<i64>) -> f64 {
        let n = self.n;
        let k = self.k;

        let x_treatment = self.x_treatment.as_ref().unwrap().clone();
        let x_control = self.x_control.as_ref().unwrap().clone();

        let x_all: Vec<f64> = x_treatment.iter().chain(x_control.iter()).map(|&x| x as f64).collect();

        let means = self.mean();
        let observed_effect = means.m_treatment - means.m_control;

        let nsim = nsim.unwrap_or(1000);

        let mut counter: i64 = 0;

        for _ in 0..nsim {
            let placebo = generate_binary_treatments(n, k);

            let placebo = placebo.clone();
            let mut mask = placebo.iter();
            let mut mask_clone = mask.clone();

            let mut x_placebo_treatment: Vec<f64> = x_all.iter().map(|&x| x as f64).collect();
            x_placebo_treatment.retain(|_| *mask.next().unwrap());

            let mut x_placebo_control: Vec<f64> = x_all.iter().map(|&x| x as f64).collect();
            x_placebo_control.retain(|_| !mask_clone.next().unwrap());

            let placebo_experiment = BivExperiment::new(
                None,
                None,
                Some(x_placebo_treatment),
                Some(x_placebo_control),
            );

            let placebo_means = placebo_experiment.mean();

            let placebo_delta = placebo_means.m_treatment - placebo_means.m_control;

            if observed_effect.abs() > placebo_delta.abs() {
                counter += 1;
            } else {
                continue;
            }
        }

        1.0 - ((counter as f64).ln() - (nsim as f64).ln()).exp()

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

    let log_comb = log_factorial(n) - log_factorial(k) - log_factorial(n - k);

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

pub fn generate_all_orders(
    n: i64,
    k: i64,
    current_order: Vec<bool>,
    mut all_orders: &mut Vec<Vec<bool>>,
) {
    let current_len: i64 = current_order.len().try_into().unwrap();
    let current_sum: i64 = current_order.iter().map(|b| if *b { 1 } else { 0 }).sum();

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
