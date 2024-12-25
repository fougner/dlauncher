use dbus_crossroads::{Context, Crossroads, IfaceBuilder};
use gtk4::{
  Application,
  glib,
  prelude::*,
};

use log::{debug, info};

use dlauncher::{
  launcher::{util::config::Config, window::Window},
  util::init_logger,
};

static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static APP_ID: &str = "net.launchpad.dlauncher";

fn main() -> glib::ExitCode {
  init_logger();
  debug!("Starting dlauncher...");

  let application = Application::builder().application_id(APP_ID).build();
  application.connect_activate(build_ui);
  application.run()
}

fn build_ui(app: &Application) {
  let config = Config::read();
  let windows = Window::new(app, &config);
  windows.build_ui();
  info!("Started dlauncher v{PKG_VERSION}");

  if !config.main.daemon {
    // skip initializing a dbus interface if daemon isn't enabled & show window
    windows.window.show();
    info!("Running in non-daemon mode");
  }

  windows.window.show();
}