use gtk4::{CellRendererPixbuf, gdk_pixbuf::{Pixbuf, PixbufLoader}, IconPaintable, Image, prelude::*};
use gtk4::builders::{ImageBuilder, PictureBuilder};
use gtk4::gdk::Surface;
use gtk4::gdk;
use gtk4::prelude::*;

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

  pub fn icon(&self) -> IconPaintable {

    let icon_theme = gtk4::IconTheme::for_display(&gdk::Display::default().unwrap());
    icon_theme.lookup_icon("calculator", &["image-missing"], 64, 1, gtk4::TextDirection::Ltr, gtk4::IconLookupFlags::FORCE_SYMBOLIC)

/*
    let loader = PixbufLoader::new();
    loader.write(CALC_IMG.as_bytes()).unwrap();
    loader.close().unwrap();
    let pb = loader
      .pixbuf();

    let mut im = Image::new();
    Image::set_from_pixbuf(&im, pb.as_ref());
    im.paintable().unwrap().downcast::<IconPaintable>().unwrap()*/
  }
}
