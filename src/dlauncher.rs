use dbus::blocking::Connection;
use dbus_crossroads::{Context, Crossroads, IfaceBuilder};

use gtk4::{glib, prelude::*, Application};

use dlauncher::{
  launcher::{util::config::Config, window::Window},
  util::init_logger,
};

use log::{debug, info, warn};

static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static APP_ID: &str = "net.launchpad.dlauncher";
static DBUS_NAME: &str = "com.dlauncher.server";

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
  } else {
    schedule_toggle(windows);
  }
}

fn schedule_toggle(window: Window) {
  debug!("Starting dbus interface");
  let main_context = glib::MainContext::default();
  let _guard = main_context.acquire().unwrap();

  let (tx, rx) = async_channel::bounded(1);

  std::thread::spawn(move || {
    let c = Connection::new_session().unwrap();
    c.request_name(DBUS_NAME, false, true, false)
      .unwrap();
    let mut cr = Crossroads::new();

    let iface_token = cr.register(
      DBUS_NAME,
      |b: &mut IfaceBuilder<async_channel::Sender<bool>>| {
        b.method(
          "OpenWindow",
          (),
          (),
          move |_: &mut Context, thread_tx, (): ()| {
            if thread_tx.send_blocking(true).is_err() {
              warn!("error sending message");
            }
            Ok(())
          },
        );
      },
    );

    cr.insert("/open", &[iface_token], tx);
    cr.serve(&c).unwrap();
  });

  main_context.spawn_local(async move {
    debug!("spawn_local for main context");
    while let Ok(command) = rx.recv().await {
      if command {
        debug!("Received message from dbus interface, toggling window.");
        window.toggle_window();
      }
    }
  });

}
