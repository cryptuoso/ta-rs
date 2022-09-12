use std::fmt;
use ta::{errors::Result, Close, High, Low, Next, Period, Reset};

/// Average true range (ATR).
///
/// A technical analysis volatility indicator, originally developed by J. Welles Wilder.
/// The average true range is an N-day smoothed moving average of the true range values.
/// This implementation uses exponential moving average.
///
/// # Parameters
///
/// * `period` - smoothing period of EMA (integer greater than 0)
///
#[doc(alias = "ATR")]
#[derive(Debug, Clone)]
pub struct AverageTrueRange {
  previous: f64,
  period: usize,
  processed: usize,
  per: f64,
  sum: f64,
  truerange: f64,
  val: f64,
  prev_close: f64,
}

impl AverageTrueRange {
  pub fn new(period: usize) -> Result<Self> {
    Ok(Self {
      period,
      previous: 0.0,
      processed: 0,
      per: 1.0 / (period as f64),
      sum: 0.0,
      truerange: 0.0,
      val: 0.0,
      prev_close: 0.0,
    })
  }
}

impl Period for AverageTrueRange {
  fn period(&self) -> usize {
    self.period
  }
}

#[deny(clippy::comparison_chain)]
impl<T: High + Low + Close> Next<&T> for AverageTrueRange {
  type Output = f64;

  fn next(&mut self, input: &T) -> Self::Output {
    self.processed += 1;

    if self.processed == 1 {
      self.sum += input.high() - input.low();
    }

    let mut result = 0.0;
    if self.processed < self.period {
      let l = input.low();
      let h = input.high();
      let c = self.prev_close;
      let ych = (h - c).abs();
      let ycl = (l - c).abs();
      let mut v = h - l;
      if ych > v {
        v = ych;
      }
      if ycl > v {
        v = ycl;
      }
      self.truerange = v;
      self.sum += self.truerange;
    }

    if self.processed == self.period - 1 {
      self.val = self.sum / self.period as f64;
      result = self.val;
    }

    if self.processed >= self.period {
      let l = input.low();
      let h = input.high();
      let c = self.prev_close;
      let ych = (h - c).abs();
      let ycl = (l - c).abs();
      let mut v = h - l;
      if ych > v {
        v = ych;
      }
      if ycl > v {
        v = ycl;
      }
      self.truerange = v;
      self.val = (self.truerange - self.val) * self.per + self.val;
      result = self.val;
    }

    self.prev_close = input.close();

    if self.processed >= self.period {
      self.previous = result;
    }

    result
  }
}

impl Reset for AverageTrueRange {
  fn reset(&mut self) {
    self.previous = 0.0;
    self.sum = 0.0;
    self.processed = 0;
    self.truerange = 0.0;
    self.val = 0.0;
    self.prev_close = 0.0;
  }
}

impl Default for AverageTrueRange {
  fn default() -> Self {
    Self::new(14).unwrap()
  }
}

impl fmt::Display for AverageTrueRange {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ATR({})", self.period())
  }
}
