use gtk4::{gdk_pixbuf::Pixbuf, prelude::RecentManagerExt, IconLookupFlags, IconTheme, TextDirection};

/// Get a themed icon's specific path on the filesystem.
pub fn get_icon_path(icon: &str, size: i32) -> String {
  let icon_theme = IconTheme::default();

  if icon.starts_with('/') {
    icon.to_string()
  } else
  {

    /*let themed_icon = icon_theme.lookup_icon(icon,&[&"dialog-question-symbolic"], size,1, TextDirection::None, IconLookupFlags::empty());
    themed_icon
      .filename()
      .unwrap()
      .to_string_lossy()
      .to_string()*/
    String::from("hello")
  }
}

/// Load a themed icon with a specified size.
pub fn load_icon(icon: &str, size: i32) -> Pixbuf {
  let icon_path = get_icon_path(icon, size);

  if let Ok(pixbuf) = Pixbuf::from_file_at_size(&icon_path, size, size) {
    pixbuf
  } else {
    default_pixbuf(size)
  }
}

pub fn default_pixbuf(size: i32) -> Pixbuf {
  let icon_path = get_icon_path("dialog-question-symbolic", size);

  Pixbuf::from_file_at_size(&icon_path, size, size).unwrap()
}
