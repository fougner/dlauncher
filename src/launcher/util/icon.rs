use gtk4::{IconLookupFlags, IconTheme, TextDirection, IconPaintable};
use gtk4::gio::Icon;
/*
/// Get a themed icon's specific path on the filesystem.
pub fn get_icon_path(icon: &str, size: i32) -> String {
  let icon_theme = IconTheme::default();

  if icon.starts_with('/') {
    icon.to_string()
  } else
  {

    let themed_icon = icon_theme.lookup_icon(icon,&["dialog-question-symbolic".into()], size,1, TextDirection::None, IconLookupFlags::empty());
    themed_icon.file().unwrap().parse_name().to_string()
  }
}
*/

/// Load a themed icon with a specified size.
pub fn load_icon(icon: &str, size: i32) -> IconPaintable {
  let icon_theme = IconTheme::default();
  icon_theme.lookup_icon(icon,&["dialog-question-symbolic"], size,1, TextDirection::None, IconLookupFlags::empty())
}

pub fn load_gicon(icon: Icon, size: i32) -> IconPaintable {
  let icon_theme = IconTheme::default();
  icon_theme.lookup_by_gicon(&icon, size,1, TextDirection::None, IconLookupFlags::empty())
}

/*pub fn default_pixbuf(size: i32) -> Pixbuf {
  let icon_path = get_icon_path("dialog-question-symbolic", size);

  Pixbuf::from_file_at_size(&icon_path, size, size).unwrap()
}
*/