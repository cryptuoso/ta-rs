use std::fmt;
use ta::{errors::Result, indicators::Maximum, Close, High, Low, Next, Period, Reset};

use super::AverageDirectionalIndex;

/// Maximum Average Directional Index (MaxADX)
///
/// Returns the highest value of ADX in a given time frame.
///
/// A direction indicator, originally developed by J. Welles Wilder. The
/// average directional movement index is an N-sample smoothed moving average of
/// a combination of positive & negative directional indicator (DI) values.
///
/// # Parameters
/// * `period` - size of the time frame (integer greater than 0).
/// * `adx_period` - Smoothing period (samples) of SDM and ATR (nonzero integer) used in the DIs.
///
///
/// # Links
///
/// * [Averager directional movement index, Wikipedia](https://en.wikipedia.org/wiki/Average_directional_movement_index)
#[doc(alias = "MaxADX")]
#[derive(Debug, Clone)]
pub struct MaximumAverageDirectionalIndex {
  previous: f64,
  adx: AverageDirectionalIndex,
  maximum: Maximum,
}

impl MaximumAverageDirectionalIndex {
  pub fn new(period: usize, adx_period: usize) -> Result<Self> {
    Ok(Self {
      previous: 0.0,
      adx: AverageDirectionalIndex::new(adx_period)?,
      maximum: Maximum::new(period)?,
    })
  }
}

impl<T: High + Low + Close> Next<&T> for MaximumAverageDirectionalIndex {
  type Output = f64;

  fn next(&mut self, input: &T) -> Self::Output {
    let current_adx = self.adx.next(input);

    let current = self.maximum.next(current_adx);

    self.previous = current;

    current
  }
}

impl Reset for MaximumAverageDirectionalIndex {
  fn reset(&mut self) {
    self.previous = 0.0;
    self.adx.reset();
    self.maximum.reset();
  }
}

impl Default for MaximumAverageDirectionalIndex {
  fn default() -> Self {
    Self::new(14, 14).unwrap()
  }
}

impl fmt::Display for MaximumAverageDirectionalIndex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "MaxADX({}, {})",
      self.maximum.period(),
      self.adx.period()
    )
  }
}
