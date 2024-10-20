use gtk4::{gdk_pixbuf::{Pixbuf, PixbufLoader}, IconPaintable, Image, prelude::*};

use crate::{
  launcher::util::icon::load_icon,
  script::{Script, ScriptIcon},
};

#[derive(Debug, Clone)]
pub struct ScriptEntry {
  script: Script,
}

impl ScriptEntry {
  pub fn new(script: Script) -> Self {
    Self { script }
  }

  pub fn name(&self) -> &str {
    &self.script.meta.name
  }

  pub fn desc(&self) -> &str {
    &self.script.meta.desc
  }

  pub fn run(&self) -> () {
    self.script.run();
  }

  pub fn icon(&self) -> IconPaintable {
    match &self.script.meta.icon {
      ScriptIcon::Themed(value) => load_icon(&value, 40),
      ScriptIcon::Svg(value) => {

        let loader = PixbufLoader::new();
        loader.write(value.as_bytes()).unwrap();
        loader.close().unwrap();
        let pb = loader.pixbuf();

        let mut im = Image::new();
        Image::set_from_pixbuf(&im, pb.as_ref());
        im.paintable().unwrap().downcast::<IconPaintable>().unwrap()

      }
    }
  }
}
