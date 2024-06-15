use gtk4::gdk::{prelude::*, Display, Monitor};
use gtk4::gio::Settings;

pub fn monitor() -> Monitor {
  let display = Display::default().unwrap();
  let monitor = display.monitors().item(0);
  monitor.expect("no monitors").downcast_ref::<Monitor>().unwrap().to_owned()
}

pub fn scaling_factor() -> f32 {
  let monitor_scaling = monitor().scale_factor();
  let text_scaling = Settings::new("org.gnome.desktop.interface");
  let text_scaling = text_scaling.double("text-scaling-factor");

  (monitor_scaling as f64 * text_scaling) as f32
}
