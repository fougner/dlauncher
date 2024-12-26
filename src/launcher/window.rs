use std::sync::{Arc, Mutex};

use gtk4::glib::{idle_add_local, ControlFlow, Propagation, clone};
use gtk4::{gdk::{prelude::*}, prelude::*, Builder, Entry, Box as GtkBox, ScrolledWindow, Window as GtkWindow, EventControllerFocus, EventControllerKey, gdk};
use gtk4::gdk::Display;
use log::{debug, error};

use crate::entry::calc_entry::CalcEntry;
use crate::fuzzy::get_matching_blocks;
use crate::{
  entry::{app_entry::AppEntry, script_entry::ScriptEntry, ResultEntry},
  extension::{Extension, ExtensionExitCode},
  fuzzy::MatchingBlocks,
  launcher::{
    navigation::Navigation,
    result::ResultWidget,
    util::{
      app::App,
      config::Config,
      display::{monitor, scaling_factor},
    query_history::QueryHistory,
      recent::Recent,
    },
  },
  script::Script,
  util::{matches_app, matches_script},
};

#[derive(Debug, Clone)]
pub struct Window {
  /// Window state, contains mutatable data.
  pub state: WindowState,
  /// GTK builder of the window.
  pub builder: Builder,
  /// Navigation utility, which controls which results are selected, and shown, etc.
  pub navigation: Arc<Mutex<Navigation>>,
  /// GTK Window
  pub window: GtkWindow,
  /// The Dlauncher main configuration usually stored in ~/.config/dlauncher/config.toml
  pub config: Config,
  /// A list of enabled extensions that are running
  pub extensions: Vec<Extension>,
}

#[derive(Debug, Clone)]
pub struct WindowState {
  /// A list of desktop entries/apps that are eligible to be shown in the results.
  pub apps: Arc<Mutex<Vec<AppEntry>>>,
  /// A list of recent apps that are shown when there is no query or to determine which result
  /// should be displayed above another.
  pub recents: Arc<Mutex<Vec<Recent>>>,
  /// Query History
  pub query_history: Arc<QueryHistory>,
  /// Scripts
  pub scripts: Arc<Vec<Script>>,
}

#[derive(Debug, Clone)]
pub struct Result {
  pub entry: ResultEntry,
  pub match_: Vec<usize>,
}

impl Window {
  pub fn new(application: &gtk4::Application, config: &Config) -> Self {
    let apps = Arc::new(Mutex::new(App::all()));
    let recents = Arc::new(Mutex::new(Recent::all(&config.recents())));
    let scripts = Arc::new(Script::all(config));
    let dlauncher_str = include_str!("../../data/ui/DlauncherWindow.ui");

    let builder = Builder::new();
    builder.add_from_string(dlauncher_str).unwrap();

    let window: GtkWindow = builder
      .object("dlauncher_window")
      .expect("Couldn't get window");
/* 
    let visual = gtk4::prelude::GtkWindowExt::screen(&window)
      .unwrap()
      .rgba_visual();
    if let Some(visual) = visual {
      window.set_visual(Some(&visual));
    }*/

    window.set_application(Some(application));

    let query_history = Arc::new(QueryHistory::new(config.clone()));

    let mut sel = Self {
      state: WindowState {
        apps,
        scripts,
        recents,
        query_history: query_history.clone(),
      },
      builder,
      navigation: Arc::new(Mutex::new(Navigation::new(query_history))),
      window,
      config: config.clone(),
      extensions: vec![],
    };

    sel.extensions = sel.config.extensions(&sel);

    sel
  }

  fn styles(&self) {
    let provider = gtk4::CssProvider::new();
    provider
      .load_from_path(
        self
          .config
          .theme()
          .compile_css()
          .as_os_str()
          .to_str()
          .unwrap(),
      );

    // Add the provider to the default screen
    gtk4::style_context_add_provider_for_display(
      &Display::default().expect("Could not connect to a display."),
      &provider,
      gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    /*gtk4::StyleContext::add_provider(
      &gtk4::prelude::GtkWindowExt::screen(&self.window).unwrap(),
      &provider,
      gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );*/

    gtk4::StyleContext::add_provider(
      &self.window.style_context(),
      &provider,
      gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let mut child = self.window.first_child();
    while let Some(widget) = child {
      child = widget.next_sibling();
      gtk4::StyleContext::add_provider(
        &widget.style_context(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
      );
    }

    /*if let Some(visual) = self.window.visual() {
      self.window.set_visual(Some(&visual));
    }*/
  }

  fn fix_window_width(&self) {
    let (width, height) = self.window.size_request();
    self.window.set_size_request(width + 2, height);
  }

  fn position_window(&self) {
    let monitor = monitor();
    let geo = monitor.geometry();
    let max_height = geo.height() as f32 - (geo.height() as f32 * 0.15) - 100.0;
    let window_width = 500.0 * scaling_factor() + 100.0;

    self
      .window
      .set_property("width-request", window_width as i32);
    let result_box_scroll_container: ScrolledWindow =
      self.builder.object("result_box_scroll_container").unwrap();
    result_box_scroll_container.set_property("max-content-height", max_height as i32);

    let _x = geo.width() as f32 * 0.5 - window_width * 0.5 + geo.x() as f32;
    let _y = geo.y() as f32 + geo.height() as f32 * 0.12;

    //self.window.move_(20, 20).expect("failed to move window");
  }

  pub fn toggle_window(&self) {
    if self.window.is_visible() {
      self.hide_window();
    } else {
      self.show_window();
    }
  }

  /// Show the GTK window, and refresh the apps and recents.
  pub fn show_window(&self) {
    self.styles();
    self.window.present();
    self.position_window();
    self.fix_window_width();

    self.show_results(vec![], false);
    self.window.grab_focus();

    let input: Entry = self.builder.object("input").expect("Couldn't get input");
    if self.config.launcher.clear_input {
      input.set_text("");
    }
    input.grab_focus();
  }

  pub fn hide_window(&self) {
    self.window.hide();

    let state = self.state.clone();
    let config_recents = self.config.recents();
/*
    idle_add_local(move || {
      let mut apps = state.apps.lock().unwrap();
      let mut recents = state.recents.lock().unwrap();
      *apps = App::all();
      *recents = Recent::all(&config_recents);

      // For some reason the mutex doesn't go out of scope and get automatically dropped so i had to do this.
      drop(apps);
      drop(recents);

      ControlFlow::Break
    });*/

  }

  /// Add a result widget to the results box
  ///
  /// Useful for extensions that don't want to clear the entire result box, but just want to add a
  /// result widget to the end of the list.
  pub fn append_result(&self, result: ResultWidget) {
    let mut navigation = self.navigation.lock().unwrap();

    self.add_one_to_results(&result);
    navigation.results.push(result);
    navigation.set_indicies();

    let results = navigation.results.clone();

    results.iter().for_each(|r| r.setup());
  }

  /// Show multiple result widgets in the results box. This will clear the results box first then
  /// add the results.
  ///
  /// Useful for when extensions want to show their own results without app results.
  ///
  /// When override is true, it will act like there are no results and show nothing. When it is
  /// false, it will show the recent apps (override should be `true` for all extensions!).
  pub fn show_results(&self, results: Vec<ResultWidget>, override_: bool) {
    let scroll_box: ScrolledWindow = self.builder.object("result_box_scroll_container").unwrap();

    if override_ && results.is_empty() {
      scroll_box.hide();
      return;
    } else {
      scroll_box.show();
    }

    let mut results = if results.is_empty() {
      let mut res = self
        .state
        .recents
        .lock()
        .unwrap()
        .iter()
        .map(|recent| recent.to_result(self.clone(), self.state.apps.clone()))
        .filter(|result| result.is_some())
        .flatten()
        .collect::<Vec<ResultWidget>>();

      if res.is_empty() {
        scroll_box.hide();
      }

      if res.len() > self.config.launcher.frequent_apps as usize {
        res.truncate(self.config.launcher.frequent_apps as usize);
      }

      res
    } else {
      results
    };

    let mut navigation = self.navigation.lock().unwrap();

    self.add_to_results(&results);
    navigation.results = results;
    navigation.set_indicies();

    let input: Entry = self.builder.object("input").expect("Couldn't get input");
    navigation.select_default(&input.text());

    results = navigation.results.clone();
    results.iter().for_each(|r| r.setup());
  }

  fn add_one_to_results(&self, result: &ResultWidget) {
    let result_box: GtkBox = self
      .builder
      .object("result_box")
      .expect("Couldn't get result_box");

    let object: GtkBox = result.builder.object("item-frame").unwrap();
    result_box.append(&object);

    result_box.set_margin_top(3);
    result_box.set_margin_bottom(10);

    //let _scroll_box: ScrolledWindow = self.builder.object("result_box_scroll_container").unwrap();
  }

  fn add_to_results(&self, apps: &Vec<ResultWidget>) {
    let result_box: gtk4::Box = self
      .builder
      .object("result_box")
      .expect("Couldn't get result_box");

    let mut child = result_box.first_child();
    while let Some(widget) = child {
        child = widget.next_sibling();
        result_box.remove(&widget);
    }

    if !apps.is_empty() {
      for app in apps {
        let object: GtkBox = app.builder.object("item-frame").unwrap();
        result_box.append(&object);
      }

      result_box.set_margin_top(3);
      result_box.set_margin_bottom(10);

      let provider = gtk4::CssProvider::new();
      provider
        .load_from_path(
          self
            .config
            .theme()
            .compile_css()
            .as_os_str()
            .to_str()
            .unwrap(),
        );

      gtk4::StyleContext::add_provider(
        &result_box.style_context(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
      );

      let mut child = result_box.first_child();
      while let Some(widget) = child {
        child = widget.next_sibling();
        gtk4::StyleContext::add_provider(
          &widget.style_context(),
          &provider,
          gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
      }

      //let _scroll_box: ScrolledWindow = self.builder.object("result_box_scroll_container").unwrap();
    }
  }

  fn handle_key_press_event(&self,
    _controller: &EventControllerKey,
    keyval: gdk::Key,
    _keycode: u32,
    _state: gdk::ModifierType,
  ) -> Propagation {
    let mut navigation = self.navigation.lock().unwrap();
    let input: Entry = self.builder.object("input").expect("Couldn't get input");

    if let Some(keycode) = keyval.name() {
      let keycode = keycode.to_string();
      let custom = self.config.keybinds();

      if keycode == custom.result_up {
        navigation.go_up();
      } else if keycode == custom.result_down {
        navigation.go_down();
      } else if keycode == custom.open {
        if let Some(selected) = navigation.selected {
          let entry = &navigation.results[selected as usize].entry;
          if !input.text().is_empty() {
            self
              .state
              .query_history
              .save_query(input.text(), entry.name());
            debug!("Saved query_history {}: {}", input.text(), entry.name());
          }

          if self.config.main.daemon {
            self.hide_window();
            entry.execute(self.clone());
          } else {
            entry.execute(self.clone());
            std::process::exit(0);
          }
        }
      } else if keycode == custom.close {
        if self.config.main.daemon {
          self.hide_window();
        } else {
          std::process::exit(0);
        }
      }
    }

    input.grab_focus_without_selecting();
    Propagation::Proceed
  }

  fn connect_changed(&self, input: &Entry) {
    let rawquery = input.text();
    let query = rawquery.trim_start();
    //input.set_text(text);
    debug!("connect_changed() inputtext={:?}", query);

    let mut results = Vec::new();

    if query.is_empty() {
      self.show_results(vec![], false);
    } else {
      let mut unsort = Vec::new();
      let apps = self.state.apps.lock().unwrap();
      for app in apps.iter() {
        if let Some((match_, score)) = matches_app(app, query, self.config.main.least_score) {
          unsort.push((ResultEntry::App(app.clone()), self.clone(), match_, score));
        }
      }

      let mut ns = fasteval::EmptyNamespace;
      if let Ok(evaluated) = fasteval::ez_eval(query, &mut ns) {
        debug!("Ok mathresult {}: {}", query, evaluated);
        let entry = ResultEntry::Calc(CalcEntry::new_from_result(query.to_string(), evaluated));
        let res = evaluated.to_string();
        let res = res.as_str();
        let match_ = get_matching_blocks(res, res);
        unsort.push((entry, self.clone(), match_, 1000));
      }

      for script in self.state.scripts.iter() {
        if let Some((match_, score)) = matches_script(script, query, self.config.main.least_score) {
          unsort.push((
            ResultEntry::Script(ScriptEntry::new(script.clone())),
            self.clone(),
            match_,
            score,
          ));
        }
      }

      let mut unsort: Vec<(ResultEntry, Window, MatchingBlocks, usize)> = unsort
        .into_iter()
        .filter(|x| x.3 > x.1.config.main.least_score)
        .collect();

      unsort.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap());

      results.extend(
        unsort
          .into_iter()
          .map(|(entry, window, match_, _)| ResultWidget::new(entry, window, match_)),
      );

      if results.len() > 9 {
        results.truncate(9);
      }

      self.show_results(results, true);

      for ext in &self.extensions {
        match ext.on_input(query) {
          ExtensionExitCode::Error(err) => {
            error!("[{}] An error occurred on `on_input`: {}", ext.name, err)
          }
          _ => {}
        }
      }
    }
  }

  pub fn build_ui(&self) {
    if !self.config.main.daemon {
      self.show_window();
    }

    let input: Entry = self.builder.object("input").expect("Couldn't get input");
    let body: gtk4::Box = self.builder.object("body").unwrap();
    body.style_context().add_class("no-window-shadow");
    let w = self.clone();
    let focus_controller = EventControllerFocus::new();
    focus_controller.connect_leave(move |_controller| {
      if w.config.launcher.hide_on_focus_lost {
        w.window.hide();
      }
    });
    self.window.add_controller(focus_controller);
    let w = self.clone();
    let key_controller = EventControllerKey::new();
    key_controller.connect_key_pressed(move |controller, keyval, keycode, state| {
        //println!("Key pressed: keyval={} keycode={} state={:?}", keyval, keycode, state);
        w.handle_key_press_event(controller, keyval, keycode, state);
        Propagation::Proceed
    });
    self.window.add_controller(key_controller);

    let th = self.clone();
    input.connect_changed(move |entry| th.connect_changed(entry));
  }
}

#[cfg(test)]
mod tests {
  use std::hint::black_box;
  use brunch::{Bench, Benches};
  use super::*;

  #[test]
  fn bench_matches_app() {

    let min_score =60;

    let apps=  App::all();

    let mut nucleo = nucleo::Matcher::new(nucleo::Config::DEFAULT.match_paths());
    let skim = fuzzy_matcher::skim::SkimMatcherV2::default();

    // TODO: unicode?
    let needles = ["firefoxkkkkkkkkkkkkkkkkkkkkkkkk", "firfx"];
    // Announce that we've started.
    ::std::eprint!("\x1b[1;38;5;199mStarting:\x1b[0m Running benchmark(s). Stand by!\n\n");
    let mut benches = Benches::default();
    // let mut scores = Vec::with_capacity(paths.0.len());
    for needle in needles {
      println!("running {needle:?}...");
      benches.push(Bench::new(format!("nucleo {needle:?}")).run(|| {

        let mut unsort = Vec::new();
        for app in apps.iter() {
          let appname = app.name.clone();
          if let Some((match_, score)) = black_box(matches_app(app, needle, min_score)) {
            unsort.push((appname, match_, score));
          }
        }
        // scores.clear();
        // scores.extend(paths.0.iter().filter_map(|haystack| {
        /*for haystack in &paths.0 {
            black_box(
                nucleo.fuzzy_match(haystack.slice(..), Utf32Str::Ascii(needle.as_bytes())),
            );
        }*/
        // }));
        // scores.sort_unstable();
      }));
      /*benches.push(Bench::new(format!("skim {needle:?}")).run(|| {
          for haystack in &paths.1 {
              let res = skim.fuzzy_match(haystack, needle);
              let _ = black_box(res);
          }
      }));*/
    }
    benches.finish();

  }
}
