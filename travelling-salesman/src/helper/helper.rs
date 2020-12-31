use rand::{Rng,thread_rng};
pub fn select_index(weights:&[f64]) -> usize {
    let max = weights.last().unwrap();
    let r:f64 = thread_rng().gen_range(0.0,*max);
    weights.iter().rposition(|&w| w<r).unwrap()
}
