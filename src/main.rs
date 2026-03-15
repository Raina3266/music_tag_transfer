use std::{
    env::{self, Args},
    fs::DirEntry,
    path::PathBuf,
};

use id3::{Frame, Tag, TagLike};

// 4 args: program name, first
fn main() {
    let mut args = env::args().skip(1);
    let path_to_first_folder = args.next().unwrap();
    let path_to_second_folder = args.next().unwrap();
    let metadata_key = args.next().unwrap();
    
    for entries1 in std::fs::read_dir(&path_to_first_folder).unwrap() {
        let entries1 = entries1.unwrap();
        for entries2 in std::fs::read_dir(&path_to_second_folder).unwrap() {
            let entries2 = entries2.unwrap();
            let tag1 = Tag::read_from_path(entries1.path()).unwrap();
            let mut tag2 = Tag::read_from_path(entries2.path()).unwrap();
            if tag1.title() == tag2.title() {
                let lyric_frame = tag1
                    .frames()
                    .find(|f| f.id() == metadata_key)
                    .map(|f| f.clone()).unwrap();
                tag2.add_frame(lyric_frame);
            }
        }
    }
}

