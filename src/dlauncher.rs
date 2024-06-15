use dbus::blocking::Connection;
use dbus_crossroads::{Context, Crossroads, IfaceBuilder};
use gtk4::{
  glib,
  prelude::*,
};

use zbus::{blocking::Connection, zvariant::ObjectPath, proxy, Result};
use zbus::ConnectionBuilder;

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
    } else {
      debug!("Starting dbus interface");
      //let (tx, rx): (Sender<bool>, Receiver<bool>) =
      //    gtk::glib::MainContext::channel(gtk::glib::Priority::DEFAULT);


      std::thread::spawn(move || {
        let c = Connection::new_session().unwrap();
        c.request_name("com.dlauncher.server", false, true, false)
          .unwrap();
        let mut cr = Crossroads::new();

        let iface_token = cr.register(
          "com.dlauncher.server",
          |b: &mut IfaceBuilder<Sender<bool>>| {
            b.method(
              "OpenWindow",
              (),
              (),
              move |_: &mut Context, thread_tx, (): ()| {
                thread_tx.send(true).unwrap();
                Ok(())
              },
            );
          },
        );

        cr.insert("/open", &[iface_token], tx);
        cr.serve(&c).unwrap();
      });

      rx.attach(None, move |msg| {
        if msg {
          debug!("Received message from dbus interface, toggling window.");
          windows.toggle_window();
        }

        Continue
      });
    };
  });

  application.run();
}
