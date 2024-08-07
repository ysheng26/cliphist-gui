use std::{
    fs::{self, create_dir, remove_dir_all},
    path::PathBuf,
    thread::sleep,
    time::Duration,
};

use copypasta::{osx_clipboard::OSXClipboardContext, ClipboardProvider};
use directories::UserDirs;

pub(crate) const DIR_NAME: &str = ".cliphist";

pub(crate) fn get_cliphist_path() -> PathBuf {
    let user_dir = UserDirs::new().unwrap();
    let home_dir = user_dir.home_dir();
    PathBuf::from(home_dir).join(DIR_NAME)
}

pub(crate) fn reset_cliphist() {
    let cliphist_path = get_cliphist_path();
    if cliphist_path.exists() {
        remove_dir_all(cliphist_path).unwrap();
    }

    create_dir(get_cliphist_path()).unwrap();
    fs::write(get_cliphist_path().join("0"), "").unwrap();
}

pub(crate) fn set_clipboard(s: String) {
    let mut clipboard = OSXClipboardContext::new().unwrap();

    clipboard.set_contents(s).unwrap();
}

pub(crate) fn poll_every(duration: Duration) {
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
        let (prev_content, prev_id) = get_prev_clipboard_content(&cliphist_path);

        if curr_content != prev_content {
            let next_id = prev_id.parse::<i64>().unwrap() + 1;
            let next_id = next_id.to_string();
            let next_id_path = cliphist_path.join(next_id);
            fs::write(next_id_path, curr_content).unwrap();
        }
        sleep(duration);
    });
}

pub(crate) fn get_prev_clipboard_content(cliphist_path: &PathBuf) -> (String, String) {
    let cliphist_path_clone = cliphist_path.clone();
    let mut xs = Vec::<String>::new();

    let paths = fs::read_dir(cliphist_path).unwrap();
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
        .map(|x| x.to_string())
        .unwrap();

    (
        fs::read_to_string(cliphist_path_clone.join(largest_id.clone())).unwrap(),
        largest_id,
    )
}

pub(crate) fn get_all_clipboard_content(cliphist_path: &PathBuf) -> Vec<String> {
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
