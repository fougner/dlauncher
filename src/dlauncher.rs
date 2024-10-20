use dbus_crossroads::{Context, Crossroads, IfaceBuilder};
use gtk4::{
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

fn main() {
  init_logger();
  debug!("Starting dlauncher...");
  gtk4::init().expect("Failed to initialize GTK.");

  let config = Config::read();
  let application = gtk4::Application::builder().application_id(APP_ID).build();

  application.connect_activate(move |application| {
    let windows = Window::new(application, &config);
    windows.build_ui();
    info!("Started dlauncher v{PKG_VERSION}");

    if !config.main.daemon {
      // skip initializing a dbus interface if daemon isn't enabled & show window
      windows.window.show();
      info!("Running in non-daemon mode");
    }
  });

  application.run();
}
