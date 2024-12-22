pub struct PrecisonLossError;

pub fn try_to_f64(v: i128) -> Result<f64, PrecisonLossError> {
    let attempt = v as f64;
    (attempt as i128 == v)
        .then_some(attempt)
        .ok_or(PrecisonLossError)
}
