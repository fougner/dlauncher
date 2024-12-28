use gtk4::IconPaintable;

use crate::{
  extension::{config::ExtensionConfig, ExtensionContext},
  launcher::{
    util::{config::Config},
    window::Window,
  },
};

pub mod app_entry;
pub mod calc_entry;
pub mod extension_entry;
pub mod script_entry;

#[derive(Debug, Clone)]
pub enum ResultEntry {
  App(app_entry::AppEntry),
  Extension(extension_entry::ExtensionEntry),
  Script(script_entry::ScriptEntry),
  Calc(calc_entry::CalcEntry),
  None,
}

impl ResultEntry {
  pub fn name(&self) -> &str {
    match self {
      ResultEntry::App(app) => &app.name,
      ResultEntry::Extension(ext) => &ext.name,
      ResultEntry::Script(script) => script.name(),
      ResultEntry::Calc(calc) => calc.name(),
      ResultEntry::None => "No results",
    }
  }

  pub fn description(&self) -> &str {
    match self {
      ResultEntry::App(app) => &app.description,
      ResultEntry::Extension(ext) => &ext.description,
      ResultEntry::Script(script) => script.desc(),
      ResultEntry::Calc(calc) => calc.desc(),
      ResultEntry::None => "No results found.",
    }
  }

  pub fn icon(&self) -> IconPaintable {
    match self {
      ResultEntry::App(app) => app.icon(),
      ResultEntry::Extension(ext) => ext.icon(),
      ResultEntry::Script(script) => script.icon(),
      ResultEntry::Calc(calc) => calc.icon(),
      ResultEntry::None => panic!("uh"),
    }
  }

  pub fn execute(&self, window: Window) {
    match self {
      ResultEntry::App(app) => app.execute(window),
      ResultEntry::Extension(ext) => {
        if let Some(on_enter) = ext.on_enter.as_ref() {
          let config = Config::read();
          on_enter(ExtensionContext {
            name: ext.extension_name.clone(),
            window,
            input: None,
            config: ExtensionConfig::new(&config, &ext.extension_name),
          })
        }
      }
      ResultEntry::Script(script) => script.run(),
      ResultEntry::Calc(_) => (),
      ResultEntry::None => (),
    }
  }
}
