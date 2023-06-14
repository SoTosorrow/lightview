use std::{path::Path, sync::mpsc::Receiver};

use notify::{Watcher, RecommendedWatcher, Config, Event};
use notify::{RecursiveMode, Error, ReadDirectoryChangesWatcher};
// use notify_debouncer_mini::{new_debouncer, Debouncer, DebouncedEvent};

pub struct Notifyer {
    watcher: RecommendedWatcher,
    rev: Receiver<Result<Event, Error>>,
    // debouncer: Debouncer<ReadDirectoryChangesWatcher>,
    // rev: Receiver<Result<Vec<DebouncedEvent>, Vec<Error>>>,
}

impl Notifyer {

    pub fn new() -> Self {
        let (sed, rev) = std::sync::mpsc::channel();
        let watcher = RecommendedWatcher::new(sed, Config::default()).unwrap();
        // let debouncer = new_debouncer(std::time::Duration::from_secs(1), None, sed).unwrap();

        Notifyer { watcher, rev }
            // watcher: watcher,
    }

    // filter for RecommendedWatcher's multi notify
    // @todo RecommendedWatcher vs debouncer performance
    pub fn filter() {
        
    }

    pub fn watch(&mut self, root_path: &str) {
        self.watcher
            .watch(Path::new(root_path), RecursiveMode::Recursive)
            .expect("[Error] watch failed");
        // self.debouncer
        //     .watcher()
        //     .watch(Path::new(root_path), RecursiveMode::Recursive)
        //     .expect("[Error] watch failed");

        for res in &self.rev {
            match res {
                Ok(event) => println!("[Changed]: {:?}", event),
                Err(err) => println!("[Error]: {:?}", err),
                // Ok(events) => events.iter().for_each(|event| println!("{event:?}")),
                // Err(errors) => errors.iter().for_each(|err| println!("{err:?}")),
            }
        }
    }
}