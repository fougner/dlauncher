use std::path::PathBuf;

use gtk4::{
  gio::{AppInfo, DesktopAppInfo},
  glib::GString,
  prelude::*,
};
use log::debug;
use regex::Regex;

use crate::{entry::app_entry::AppEntry, launcher::util::icon::load_icon};
use crate::launcher::util::icon::load_gicon;

pub struct App;

impl App {
  pub fn all() -> Vec<AppEntry> {
    debug!("Reading apps");
    let mut results = Vec::new();

    let re = Regex::new(r"%[uUfFdDnNickvm]").unwrap();

    for app in AppInfo::all() {
      if !app.should_show() {
        continue;
      }

      if let Some(exec) = app.commandline() {
        let icon = if app.icon().is_none() {
          None
        } else {
          let st = IconExt::to_string(&app.icon().unwrap())
            .unwrap()
            .to_string();

          Some(load_gicon(app.icon().unwrap(), 64))
        };

        if let Some(file) = app.id() {
          let exec: Vec<String> =
            shell_words::split(&re.replace(&exec.display().to_string(), "")).unwrap();

          let terminal = if let Some(desktop) = DesktopAppInfo::new(&app.id().unwrap()) {
            desktop.boolean("Terminal")
          } else {
            false
          };

          results.push(AppEntry {
            name: app.display_name().to_string(),
            description: app
              .description()
              .unwrap_or_else(|| GString::from(""))
              .to_string(),
            file: PathBuf::from(file.to_string()),
            icon,
            exec,
            terminal,
          })
        }
      }
    }

    debug!("Read {} apps", results.len());

    results
  }
}
