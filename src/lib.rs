#![deny(clippy::all)]
#![allow(clippy::upper_case_acronyms)]

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

pub mod custom;
pub mod ta;
pub mod yata;
