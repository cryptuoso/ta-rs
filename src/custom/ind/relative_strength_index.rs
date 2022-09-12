use std::fmt;
use ta::{errors::Result, Close, Next, Period, Reset};

/// The relative strength index (RSI).
///
/// It is a momentum oscillator,
/// that compares the magnitude of recent gains
/// and losses over a specified time period to measure speed and change of price
/// movements of a security. It is primarily used to attempt to identify
/// overbought or oversold conditions in the trading of an asset.
///
/// The oscillator returns output in the range of 0..100.
///
/// # Parameters
///
/// * `period` - number of periods (integer greater than 0).
///
#[doc(alias = "RSI")]
#[derive(Debug, Clone)]
pub struct RelativeStrengthIndex {
  previous: f64,
  period: usize,
  processed: usize,
  per: f64,
  smooth_up: f64,
  smooth_down: f64,
  prev_input: f64,
}

impl RelativeStrengthIndex {
  pub fn new(period: usize) -> Result<Self> {
    Ok(Self {
      period,
      previous: 0.0,
      processed: 0,
      per: 1.0 / (period as f64),
      smooth_up: 0.0,
      smooth_down: 0.0,
      prev_input: 0.0,
    })
  }
}

impl Period for RelativeStrengthIndex {
  fn period(&self) -> usize {
    self.period
  }
}

#[deny(clippy::comparison_chain)]
impl Next<f64> for RelativeStrengthIndex {
  type Output = f64;

  fn next(&mut self, input: f64) -> Self::Output {
    self.processed += 1;

    let mut result = 0.0;
    if self.processed < self.period {
      let upward = match input > self.prev_input {
        true => input - self.prev_input,
        false => 0.0,
      };

      let downward = match input < self.prev_input {
        true => self.prev_input - input,
        false => 0.0,
      };
      self.smooth_up += upward;
      self.smooth_down += downward;
    };

    if self.processed == self.period - 1 {
      self.smooth_up = self.smooth_up / (self.period as f64);
      self.smooth_down = self.smooth_down / (self.period as f64);
      result = 100.0 * self.smooth_up / (self.smooth_up + self.smooth_down);
    }

    if self.processed >= self.period {
      let upward = match input > self.prev_input {
        true => input - self.prev_input,
        false => 0.0,
      };

      let downward = match input < self.prev_input {
        true => self.prev_input - input,
        false => 0.0,
      };
      self.smooth_up = (upward - self.smooth_up) * self.per + self.smooth_up;
      self.smooth_down = (downward - self.smooth_down) * self.per + self.smooth_down;
      result = 100.0 * (self.smooth_up / (self.smooth_up + self.smooth_down));
    }

    self.prev_input = input;
    if self.processed >= self.period {
      self.previous = result;
    }

    result
  }
}

impl<T: Close> Next<&T> for RelativeStrengthIndex {
  type Output = f64;

  fn next(&mut self, input: &T) -> Self::Output {
    self.next(input.close())
  }
}

impl Reset for RelativeStrengthIndex {
  fn reset(&mut self) {
    self.previous = 0.0;
    self.processed = 0;
    self.smooth_up = 0.0;
    self.smooth_down = 0.0;
    self.prev_input = 0.0;
  }
}

impl Default for RelativeStrengthIndex {
  fn default() -> Self {
    Self::new(14).unwrap()
  }
}

impl fmt::Display for RelativeStrengthIndex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ATR({})", self.period())
  }
}
