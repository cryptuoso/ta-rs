use std::fmt;
use ta::{errors::Result, indicators::StandardDeviation, Low, Next, Period, Reset};

use super::RelativeStrengthIndex;

/// FXHighBand
///
/// TODO: description
///
/// # Parameters
/// * `period` - size of the time frame (integer greater than 0).
/// * `rsi_period` - RSI period
/// * `modifier` - modifier
///

#[doc(alias = "ChanADX")]
#[derive(Debug, Clone)]
pub struct FXHighBand {
  modifier: u16,
  period: usize,
  std_dev: StandardDeviation,
  rsi: RelativeStrengthIndex,
  rsi_series: Vec<f64>,
  previous: f64,
}

impl FXHighBand {
  pub fn new(period: usize, rsi_period: usize, modifier: u16) -> Result<Self> {
    Ok(Self {
      modifier,
      period,
      std_dev: StandardDeviation::new(period)?,
      rsi: RelativeStrengthIndex::new(rsi_period)?,
      rsi_series: Vec::with_capacity(rsi_period + period),
      previous: 0.0,
    })
  }
}

impl<T: Low> Next<&T> for FXHighBand {
  type Output = f64;

  fn next(&mut self, input: &T) -> Self::Output {
    let current_rsi = self.rsi.next(input.low());
    self.rsi_series.push(current_rsi);
    if self.rsi_series.len() > self.period + self.rsi.period() {
      self.rsi_series.remove(0);
    }
    if self.rsi_series.len() == self.period + self.rsi.period() {
      self.std_dev.reset();

      let slice = self.rsi_series[self.rsi_series.len() - self.period..].iter();

      let mut std_dev_value: f64 = 0.0;
      for rsi_val in slice {
        std_dev_value = self.std_dev.next(*rsi_val);
      }
      self.previous = current_rsi / 30.0 + std_dev_value * 1.3185 + self.modifier as f64;

      return self.previous;
    }
    0.0
  }
}

impl Reset for FXHighBand {
  fn reset(&mut self) {
    self.rsi.reset();
    self.std_dev.reset();
    self.rsi_series.clear();
    self.previous = 0.0;
  }
}

impl Default for FXHighBand {
  fn default() -> Self {
    Self::new(14, 14, 3).unwrap()
  }
}

impl fmt::Display for FXHighBand {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "FXHighBand({}, {}, {})",
      self.period,
      self.rsi.period(),
      self.modifier
    )
  }
}
