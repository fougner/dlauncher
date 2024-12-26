

use std::hint::black_box;
use brunch::{Bench, Benches};
use dlauncher::fuzzy::MatcherAlgo;
use dlauncher::launcher::util::app::App;
use dlauncher::util::matches_app;


fn main() -> () {

    gtk4::init().expect("Failed to initialize gtk");
    let min_score =60;

    let apps = App::all();

    for app in apps.iter() {
        let s = app.clone().name;
        if s.contains("Firefox") {
            if let Some((_mb, score)) = matches_app(app, "firfox", min_score, Some(MatcherAlgo::Skim)) {
                println!("firefox matches for skim algo: {:?}", score);
            }
        }
    }


    //let skim = fuzzy_matcher::skim::SkimMatcherV2::default();

    let needles = ["firefoxköööösuperlongstringthatdoesntmatchanythingkkkkkkkkkkkk", "firfx", "massomdöpning", "Filer", "ff"];
    let algos = [MatcherAlgo::Skim, MatcherAlgo::Simple];
    ::std::eprint!("\x1b[1;38;5;199mStarting:\x1b[0m Running benchmark(s). Stand by!\n\n");
    let mut benches = Benches::default();
    for needle in needles {
        for algo in &algos {
        println!("running {needle:?}...");
        benches.push(Bench::new(format!("{algo:?} {needle:?}")).run(|| {
            let mut unsort = Vec::new();
            for app in apps.iter() {
                let appname = app.name.clone();
                if let Some((match_, score)) = black_box(matches_app(app, needle, min_score, Some(algo.clone()))) {
                    unsort.push((appname, match_, score));
                }
            }
        }));
    }}
    benches.finish();

}
