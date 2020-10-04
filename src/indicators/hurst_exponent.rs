use std::fmt;

use crate::errors::*;
use crate::{Close, High, Low, Next, Reset};

// TODO: Explain what the Hursts Exponents are.

#[derive(Debug, Clone)]
pub struct HurstExponent {
    n: u32,
}

impl HurstExponent {
    pub fn new(n: u32) -> Result<Self> {
        match n {
            0..=9 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let h = HurstExponent { n };
                return Ok(h);
            }
        }
    }
}

impl Default for HurstExponent {
    fn default() -> Self {
        Self::new(10).unwrap()
    }
}

impl fmt::Display for HurstExponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HurstExponent()")
    }
}

impl Reset for HurstExponent {
    fn reset(&mut self) {}
}

impl Next<f64> for HurstExponent {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        return 0.00000;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    //test_indicator!(HurstExponent);

    #[test]
    fn test_new() {
        assert!(HurstExponent::new(10).is_ok());
        assert!(HurstExponent::new(0).is_err());
        assert!(HurstExponent::new(9).is_err());
    }
}
