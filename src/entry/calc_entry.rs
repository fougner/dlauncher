use gtk::{
  gdk_pixbuf::Pixbuf,
  prelude::*,
};

use crate::{
  launcher::util::icon::load_icon,
};

#[derive(Debug, Clone)]
pub struct CalcEntry {
  calc: Calc,
}

#[derive(Debug, Clone)]
pub struct Calc {
  pub query: String,
  pub result: String,
}

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
    load_icon("org.gnome.Calculator", 40)
  }
}
