use std::fmt;
use ta::{errors::Result, Close, High, Low, Next, Period, Reset};

use super::AverageTrueRange;

#[napi(object, js_name = "RachSupTrendOutput")]
#[derive(Debug, Clone)]
pub struct RachSupTrendOutput {
  pub buy: i8,
  pub sell: i8,
  pub buy_entry: i8,
  pub sell_entry: i8,
}

/// RajandranSuperTrend (RST).
///
/// https://www.tradingview.com/script/baEa7vSy-Supertrend-V1-0-Buy-or-Sell-Signal/
///
/// # Parameters
///
/// * `period` - period of ATR (integer greater than 0 less than 100)
/// * `factor` (integer greater than 0 less than 100)
///
#[doc(alias = "RST")]
#[derive(Debug, Clone)]
pub struct RajandranSuperTrend {
  period: usize,
  factor: usize,
  processed: usize,
  previous: RachSupTrendOutput,
  atr: AverageTrueRange,
  prev_close: f64,
  trend_up: f64,
  trend_down: f64,
  trend: i8,
  tsl: f64,
  buy: i8,
  sell: i8,
  buy_entry: i8,
  sell_entry: i8,
}

impl RajandranSuperTrend {
  pub fn new(period: usize, factor: usize) -> Result<Self> {
    Ok(Self {
      period,
      factor,
      processed: 0,
      previous: RachSupTrendOutput {
        buy: 0,
        sell: 0,
        buy_entry: 0,
        sell_entry: 0,
      },
      atr: AverageTrueRange::new(period)?,
      prev_close: 0.0,
      trend_up: 0.0,
      trend_down: 0.0,
      trend: 0,
      tsl: 0.0,
      buy: 0,
      sell: 0,
      buy_entry: 0,
      sell_entry: 0,
    })
  }

  pub fn cross(prev_current: f64, prev_target: f64, current: f64, target: f64) -> bool {
    if prev_current <= prev_target && current > target {
      return true;
    }
    if prev_current >= prev_target && current < target {
      return true;
    }
    false
  }
}

impl Period for RajandranSuperTrend {
  fn period(&self) -> usize {
    self.period
  }
}

#[deny(clippy::comparison_chain)]
impl<T: High + Low + Close> Next<&T> for RajandranSuperTrend {
  type Output = RachSupTrendOutput;

  fn next(&mut self, input: &T) -> Self::Output {
    self.processed += 1;

    let atr = self.atr.next(input);

    if self.processed > self.period {
      let hl2 = (input.high() + input.low()) / 2.0;
      let up = hl2 - (atr * (self.factor as f64));
      let dn = hl2 + (atr * (self.factor as f64));

      let trend_up = if self.prev_close > self.trend_up {
        up.max(self.trend_up)
      } else {
        up
      };

      let trend_down = if self.prev_close < self.trend_down {
        dn.min(self.trend_down)
      } else {
        dn
      };

      let trend = if input.close() > self.trend_down {
        1
      } else if input.close() < self.trend_up {
        -1
      } else if self.trend == 0 {
        1
      } else {
        self.trend
      };

      let tsl = if trend == 1 {
        self.trend_up
      } else {
        self.trend_down
      };

      self.buy =
        if Self::cross(self.prev_close, self.tsl, input.close(), tsl) && input.close() > tsl {
          1
        } else {
          0
        };

      self.sell =
        if Self::cross(self.tsl, self.prev_close, tsl, input.close()) && input.close() < tsl {
          1
        } else {
          0
        };

      self.buy_entry = if trend == 1 && self.trend == -1 {
        trend
      } else {
        0
      };

      self.sell_entry = if trend == -1 && self.trend == 1 {
        trend
      } else {
        0
      };

      self.trend_up = trend_up;
      self.trend_down = trend_down;
      self.trend = trend;
      self.tsl = tsl;
    }

    let result = RachSupTrendOutput {
      buy: self.buy,
      sell: self.sell,
      buy_entry: self.buy_entry,
      sell_entry: self.sell_entry,
    };

    if self.processed >= self.period {
      self.previous = result.clone();
    }

    self.prev_close = input.close();

    result
  }
}

impl Reset for RajandranSuperTrend {
  fn reset(&mut self) {
    self.previous = RachSupTrendOutput {
      buy: 0,
      sell: 0,
      buy_entry: 0,
      sell_entry: 0,
    };
    self.processed = 0;
    self.atr.reset();
    self.prev_close = 0.0;
    self.trend_up = 0.0;
    self.trend_down = 0.0;
    self.trend = 0;
    self.tsl = 0.0;
    self.buy = 0;
    self.sell = 0;
    self.buy_entry = 0;
    self.sell_entry = 0;
  }
}

impl Default for RajandranSuperTrend {
  fn default() -> Self {
    Self::new(3, 7).unwrap()
  }
}

impl fmt::Display for RajandranSuperTrend {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "RST({},{})", self.period(), self.factor)
  }
}
