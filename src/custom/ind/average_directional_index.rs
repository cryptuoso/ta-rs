use std::cmp::Ordering;
use std::fmt;
use ta::{errors::Result, Close, High, Low, Next, Period, Reset};

/// Average Directional Index (ADX)
///
/// A direction indicator, originally developed by J. Welles Wilder. The
/// average directional movement index is an N-sample smoothed moving average of
/// a combination of positive & negative directional indicator (DI) values.
///
/// # Parameters
///
/// * `period` - Smoothing period (samples) of SDM and ATR (nonzero integer)
///   used in the DIs.
///
/// # Links
///
/// * [Averager directional movement index, Wikipedia](https://en.wikipedia.org/wiki/Average_directional_movement_index)
#[doc(alias = "ADX")]
#[derive(Debug, Clone)]
pub struct AverageDirectionalIndex {
  previous: f64,
  period: usize,
  processed: usize,
  per: f64,
  invper: f64,
  adx: f64,
  atr: f64,
  dmup: f64,
  dmdown: f64,
  prev_high: f64,
  prev_low: f64,
  prev_close: f64,
}

impl AverageDirectionalIndex {
  pub fn new(period: usize) -> Result<Self> {
    Ok(Self {
      period,
      previous: 0.0,
      processed: 0,
      per: ((period as f64) - 1.0) / (period as f64),
      invper: 1.0 / (period as f64),
      adx: 0.0,
      atr: 0.0,
      dmup: 0.0,
      dmdown: 0.0,
      prev_high: 0.0,
      prev_low: 0.0,
      prev_close: 0.0,
    })
  }
}

impl Period for AverageDirectionalIndex {
  fn period(&self) -> usize {
    self.period
  }
}

impl<T: High + Low + Close> Next<&T> for AverageDirectionalIndex {
  type Output = f64;

  fn next(&mut self, input: &T) -> Self::Output {
    self.processed += 1;

    let mut result = 0.0;
    if self.processed < self.period {
      let truerange: f64;
      {
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
        truerange = v;
      }
      self.atr += truerange;
      let mut dp: f64;
      let mut dm: f64;
      {
        dp = input.high() - self.prev_high;
        dm = self.prev_low - input.low();
        if dp < 0.0 {
          dp = 0.0;
        } else if dp > dm {
          dm = 0.0;
        }
        if dm < 0.0 {
          dm = 0.0;
        } else if dm > dp {
          dp = 0.0;
        }
      }
      self.dmup += dp;
      self.dmdown += dm;
    }

    if self.processed == self.period - 1 {
      let di_up = self.dmup / self.atr;
      let di_down = self.dmdown / self.atr;
      let dm_diff = (di_up - di_down).abs();
      let dm_sum = di_up + di_down;
      let dx = dm_diff / dm_sum * 100.0;
      self.adx += dx;
    }

    if self.processed >= self.period {
      let truerange;
      {
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
        truerange = v;
      }

      self.atr = self.atr * self.per + truerange;

      let mut dp;
      let mut dm;

      {
        dp = input.high() - self.prev_high;
        dm = self.prev_low - input.low();
        if dp < 0.0 {
          dp = 0.0;
        } else if dp > dm {
          dm = 0.0;
        }
        if dm < 0.0 {
          dm = 0.0;
        } else if dm > dp {
          dp = 0.0;
        }
      }
      self.dmup = self.dmup * self.per + dp;
      self.dmdown = self.dmdown * self.per + dm;

      let di_up = self.dmup / self.atr;
      let di_down = self.dmdown / self.atr;
      let dm_diff = (di_up - di_down).abs();
      let dm_sum = di_up + di_down;
      let dx = dm_diff / dm_sum * 100.0;

      match (self.processed - self.period).cmp(&(self.period - 2)) {
        Ordering::Less => {
          self.adx += dx;
        }
        Ordering::Equal => {
          self.adx += dx;
          result = self.adx * self.invper;
        }
        _ => {
          self.adx = self.adx * self.per + dx;
          result = self.adx * self.invper;
        }
      }
    }

    self.prev_high = input.high();
    self.prev_low = input.low();
    self.prev_close = input.close();

    if self.processed >= self.period {
      self.previous = result;
    }

    result
  }
}

impl Reset for AverageDirectionalIndex {
  fn reset(&mut self) {
    self.previous = 0.0;
    self.processed = 0;
    self.adx = 0.0;
    self.atr = 0.0;
    self.dmup = 0.0;
    self.dmdown = 0.0;
    self.prev_high = 0.0;
    self.prev_low = 0.0;
    self.prev_close = 0.0;
  }
}

impl Default for AverageDirectionalIndex {
  fn default() -> Self {
    Self::new(14).unwrap()
  }
}

impl fmt::Display for AverageDirectionalIndex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ADX({})", self.period())
  }
}
