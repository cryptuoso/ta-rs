use std::cmp;
use std::fmt;
use ta::Close;
use ta::{errors::Result, High, Low, Next, Period, Reset};

use super::AverageDirectionalIndex;

#[napi(object, js_name = "ChanADXOutput")]
pub struct ChanADXOutput {
  pub value: u32,
  pub high: f64,
  pub low: f64,
}

/// Channel Average Directional Index (ChanADX)
///
/// TODO: description
///
/// # Parameters
/// * `period` - size of the time frame (integer greater than 0).
/// * `adx_period` - Smoothing period (samples) of SDM and ATR (nonzero integer) used in the DIs.
/// * `ratio`
///

#[doc(alias = "ChanADX")]
#[derive(Debug, Clone)]
pub struct ChannelAverageDirectionalIndex {
  period: usize,
  ratio: u16,
  adx: AverageDirectionalIndex,
  adx_series: Vec<f64>,
  high_series: Vec<f64>,
  low_series: Vec<f64>,
  value: u32,
  high: f64,
  low: f64,
}

impl ChannelAverageDirectionalIndex {
  pub fn new(period: usize, adx_period: usize, ratio: u16) -> Result<Self> {
    Ok(Self {
      period,
      ratio,
      adx: AverageDirectionalIndex::new(adx_period)?,
      adx_series: Vec::with_capacity(adx_period + period),
      high_series: Vec::with_capacity(period),
      low_series: Vec::with_capacity(period),
      value: 0,
      high: 0.0,
      low: 0.0,
    })
  }
}

impl<T: High + Low + Close> Next<&T> for ChannelAverageDirectionalIndex {
  type Output = ChanADXOutput;

  fn next(&mut self, input: &T) -> Self::Output {
    let current_adx = self.adx.next(input);

    self.high_series.push(input.high());
    self.low_series.push(input.low());
    if self.high_series.len() > self.period {
      self.high_series.remove(0);
    }
    if self.low_series.len() > self.period {
      self.low_series.remove(0);
    }

    if current_adx > 0.0 {
      self.adx_series.push(current_adx);
      if self.adx_series.len() > self.period {
        self.adx_series.remove(0);
      }

      if self.high_series.len() == self.period
        && self.low_series.len() == self.period
        && self.adx_series.len() == self.period
      {
        self.value = cmp::max((self.ratio as f64 / current_adx).trunc() as u32, 1);
        if self.value < self.high_series.len().try_into().unwrap() {
          self.high = *self.high_series[self.high_series.len() - (self.value as usize)..]
            .iter()
            .max_by(|a, b| a.total_cmp(b))
            .unwrap();
          self.low = *self.low_series[self.low_series.len() - (self.value as usize)..]
            .iter()
            .min_by(|a, b| a.total_cmp(b))
            .unwrap();
        } else {
          self.high = 0.0;
          self.low = 0.0;
        }
      }
    } else {
      self.value = 0;
      self.high = 0.0;
      self.low = 0.0;
    }

    Self::Output {
      value: self.value,
      high: self.high,
      low: self.low,
    }
  }
}

impl Reset for ChannelAverageDirectionalIndex {
  fn reset(&mut self) {
    self.adx.reset();
    self.high_series.clear();
    self.low_series.clear();
    self.high = 0.0;
    self.low = 0.0;
  }
}

impl Default for ChannelAverageDirectionalIndex {
  fn default() -> Self {
    Self::new(14, 14, 3).unwrap()
  }
}

impl fmt::Display for ChannelAverageDirectionalIndex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "ChannelAverageDirectionalIndex({}, {}, {})",
      self.period,
      self.adx.period(),
      self.ratio
    )
  }
}
