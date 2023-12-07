use gtk::{
  gdk_pixbuf::{Pixbuf, PixbufLoader},
  prelude::*,
};

use crate::launcher::util::icon::load_icon;

#[derive(Debug, Clone)]
pub struct CalcEntry {
  calc: Calc,
}

#[derive(Debug, Clone)]
pub struct Calc {
  pub query: String,
  pub result: String,
}

static CALC_IMG: &str = include_str!("../../data/icons/calculator.xpm");

impl CalcEntry {
  pub fn new(calc: Calc) -> Self {
    Self { calc }
  }
  pub fn new_from_result(query: String, result: f64) -> Self {
    Self {
      calc: Calc {
        query: query,
        result: result.to_string(),
      },
    }
  }

  pub fn name(&self) -> &str {
    &self.calc.result
  }

  pub fn desc(&self) -> &str {
    &self.calc.query
  }

  pub fn icon(&self) -> Pixbuf {
    let loader = PixbufLoader::new();
    loader.write(CALC_IMG.as_bytes()).unwrap();
    loader.close().unwrap();
    loader
      .pixbuf()
      .unwrap()
      .scale_simple(40, 40, gtk::gdk_pixbuf::InterpType::Bilinear)
      .unwrap()
  }
}
