#![deny(clippy::all)]
#![allow(clippy::upper_case_acronyms)]

use ::ta::{Close, High, Low, Open, Volume};

#[macro_use]
extern crate napi_derive;

#[napi(object)]
#[derive(Debug, Clone, PartialEq)]
pub struct Candle {
  pub time: i64,
  pub open: f64,
  pub high: f64,
  pub low: f64,
  pub close: f64,
  pub volume: f64,
}

impl Open for Candle {
  fn open(&self) -> f64 {
    self.open
  }
}

impl High for Candle {
  fn high(&self) -> f64 {
    self.high
  }
}

impl Low for Candle {
  fn low(&self) -> f64 {
    self.low
  }
}

impl Close for Candle {
  fn close(&self) -> f64 {
    self.close
  }
}

impl Volume for Candle {
  fn volume(&self) -> f64 {
    self.volume
  }
}

pub mod ta;
pub mod yata;
