use std::{
    collections::HashSet,
    fs::{self, create_dir, remove_dir_all},
    path::PathBuf,
    thread::sleep,
    time::Duration,
};

use copypasta::{osx_clipboard::OSXClipboardContext, ClipboardProvider};
use directories::UserDirs;

pub const DIR_NAME: &str = ".cliphist";

pub fn get_cliphist_path() -> PathBuf {
    let user_dir = UserDirs::new().unwrap();
    let home_dir = user_dir.home_dir();
    PathBuf::from(home_dir).join(DIR_NAME)
}

pub fn reset_cliphist() {
    let cliphist_path = get_cliphist_path();
    if cliphist_path.exists() {
        remove_dir_all(cliphist_path).unwrap();
    }

    create_dir(get_cliphist_path()).unwrap();
    fs::write(get_cliphist_path().join("0"), "dummy").unwrap();
}

pub fn set_clipboard(s: String) {
    let mut clipboard = OSXClipboardContext::new().unwrap();

    clipboard.set_contents(s).unwrap();
}

pub fn poll_every(duration: Duration) {
    std::thread::spawn(move || loop {
        // setup cliphist dir if not exist
        let cliphist_path = get_cliphist_path();
        if !cliphist_path.exists() {
            create_dir(cliphist_path).unwrap();
            fs::write(get_cliphist_path().join("0"), "").unwrap();
        }

        // start main loop
        let mut clipboard = OSXClipboardContext::new().unwrap();

        let curr_content = clipboard.get_contents().unwrap();
        let cliphist_path = get_cliphist_path();

        if !content_exists(&curr_content) {
            let next_id = get_next_id();
            let next_id_path = cliphist_path.join(next_id);
            fs::write(next_id_path, curr_content).unwrap();
        }

        sleep(duration);
    });
}

pub fn get_next_id() -> String {
    let mut xs = Vec::<String>::new();

    let paths = fs::read_dir(get_cliphist_path()).unwrap();
    for path in paths {
        if let Some(filename) = path.unwrap().path().file_name() {
            let filename = filename.to_str().unwrap();
            xs.push(filename.to_string());
        }
    }
    let largest_id = xs
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .max()
        .map(|x| (x + 1).to_string())
        .unwrap();

    largest_id
}

pub fn content_exists(content: &String) -> bool {
    let mut xs = Vec::<String>::new();

    let paths = fs::read_dir(get_cliphist_path()).unwrap();
    for path in paths {
        if let Some(filename) = path.unwrap().path().file_name() {
            let filename = filename.to_str().unwrap();
            xs.push(filename.to_string());
        }
    }

    // fs::read_to_string(cliphist_path_clone.join(largest_id.clone())).unwrap(),
    let hashset = HashSet::<String>::from_iter(
        xs.iter()
            .map(|id| fs::read_to_string(get_cliphist_path().join(id)).unwrap()),
    );

    hashset.contains(content)
}

// pub fn get_prev_clipboard_content(cliphist_path: &PathBuf) -> (String, String) {
//     let cliphist_path_clone = cliphist_path.clone();
//     let mut xs = Vec::<String>::new();

//     let paths = fs::read_dir(cliphist_path).unwrap();
//     for path in paths {
//         if let Some(filename) = path.unwrap().path().file_name() {
//             let filename = filename.to_str().unwrap();
//             xs.push(filename.to_string());
//         }
//     }
//     let largest_id = xs
//         .iter()
//         .map(|x| x.parse::<i64>().unwrap())
//         .max()
//         .map(|x| x.to_string())
//         .unwrap();

//     (
//         fs::read_to_string(cliphist_path_clone.join(largest_id.clone())).unwrap(),
//         largest_id,
//     )
// }

pub fn get_all_clipboard_content(cliphist_path: &PathBuf) -> Vec<String> {
    let cliphist_path_clone = cliphist_path.clone();
    let mut xs = Vec::<String>::new();

    let paths = fs::read_dir(cliphist_path).unwrap();
    for path in paths {
        if let Some(filename) = path.unwrap().path().file_name() {
            let filename = filename.to_str().unwrap();
            xs.push(filename.to_string());
        }
    }
    xs.sort_by(|a, b| b.parse::<i64>().unwrap().cmp(&a.parse::<i64>().unwrap()));

    xs.iter()
        .map(|x| fs::read_to_string(cliphist_path_clone.join(x)).unwrap())
        .collect()
}

/*
create folder in home directory which has files that represent history
increasing by id

poll every 500ms and compare with x.get_contents(), if not same add

only keep last N results


*/
