use std::{
    env::{self},
    fs::{DirEntry, copy},
    path::{Path, PathBuf},
};

use id3::{Tag, TagLike, Version};

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
            let tem_dir = copy_to_tem_dir(&PathBuf::from(&path_to_second_folder), &entries2);
            
            let tag1 = Tag::read_from_path(PathBuf::from(&path_to_first_folder).join(entries1.file_name())).unwrap();
            let mut tag2 = Tag::read_from_path(&tem_dir).unwrap();
        
            copy_tag(&tag1, &mut tag2, &metadata_key);
            tag2.write_to_path(&tem_dir, Version::Id3v23).unwrap();
            
        }
    }
}

fn copy_to_tem_dir(path: &PathBuf, entry: &DirEntry) -> PathBuf {
    let original_file = PathBuf::from(path).join(entry.file_name());
    let temp_file = std::env::temp_dir().join(entry.file_name());
    copy(original_file, &temp_file).unwrap();
    temp_file
}

fn copy_tag(tag1: &Tag, tag2: &mut Tag, key: &str) {
    if tag1.title() == tag2.title() {
        let lyric_frame = tag1
            .frames()
            .find(|f| f.id() == key)
            .map(|f| f.clone())
            .unwrap();
        tag2.add_frame(lyric_frame);
    }
}

fn lyric_tag_exists(tag: &Tag, metadata: &str) -> bool {
    tag.frames().any(|f| f.id() == metadata)
}

fn lyric_tag_equal(tag1: Tag, tag2: Tag) -> bool {
    tag1.lyrics().eq(tag2.lyrics())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_copy_tag() {
        let example1 =
            PathBuf::from_str("/home/raina/Projects/music-tag-transfer/test_files/with_lyrics")
                .unwrap();
        let example2 =
            PathBuf::from_str("/home/raina/Projects/music-tag-transfer/test_files/without_lyrics")
                .unwrap();
        for entries1 in std::fs::read_dir(&example1).unwrap() {
            let entries1 = entries1.unwrap();
            for entries2 in std::fs::read_dir(&example2).unwrap() {
                let entries2 = entries2.unwrap();
                let tem_dir = copy_to_tem_dir(&PathBuf::from(&example2), &entries2);
                
                let tag1 = Tag::read_from_path(PathBuf::from(&example1).join(entries1.file_name())).unwrap();
                let mut tag2 = Tag::read_from_path(&tem_dir).unwrap();
                
                assert!(!lyric_tag_exists(&tag2, "Lyrics"));
                copy_tag(&tag1, &mut tag2, "Lyrics");
                assert!(lyric_tag_exists(&tag2, "Lyrics"));

                tag2.write_to_path(&tem_dir, Version::Id3v23).unwrap();
            }
        }
    }
}
