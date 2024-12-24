#[derive(Debug, PartialEq, Eq)]
pub struct PrecisonLossError;

/// Try to convert an i128 to an f64 without loosing precision
pub fn try_to_f64(v: i128) -> Result<f64, PrecisonLossError> {
    let attempt = v as f64;
    match attempt as i128 == v {
        true  => Ok(attempt),
        false => Err(PrecisonLossError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion() {
        assert_eq!(try_to_f64(42_i128), Ok(42.0))
    }

    #[test]
    fn precisionlosserror() {
        assert_eq!(try_to_f64(123456789123456789), Err(PrecisonLossError))
    }
}