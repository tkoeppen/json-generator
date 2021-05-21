use std::fs::{metadata, Metadata, File, OpenOptions, create_dir_all};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use crate::sender::Sender;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::error::GenError;

/// the struct which implements the Sender trait and allows
/// to save a generated json to folder
/// It includes the internal state to generate the index of the files
pub struct FolderSender {
    path: String,
    idx: usize,
}

impl FolderSender {
    pub fn new(path: String) -> Self {
        match metadata(path.clone()) {
            Ok(m) => {
                if !m.is_dir() {
                    panic!(format!("the output path {} to the file should point to a folder.", path));
                }
            }
            Err(e) =>
                if !Path::new(path.as_str()).exists() {
                    match create_dir_all(path.as_str()) {
                        Ok(_) => (),
                        Err(e) => panic!("error occurred while creating or open the file:{}", e.to_string()),
                    }
                } else { panic!("the error occurred with the output file: {}", e.to_string()) }
        }
        debug!("the folder sender with the path {} has been created successfully", path);
        FolderSender { path, idx: 0 }
    }
}

impl Sender for FolderSender {
    fn send(&mut self, json: String) -> Result<String, GenError> {
        let mut pb = PathBuf::new();
        pb.push(self.path.as_str());
        pb.push(format!("json_{}.json", self.idx).as_str());

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(pb.as_path())
            .expect("problem with a file");


        if let Err(e) = file.write_all(json.into_bytes().as_slice()) {
            Err(GenError::new_with_in_sender(format!("error while appending to a file: {}", e.to_string()).as_str()))
        } else {
            let res = format!("the item {} has been saved in the folder: {}", self.idx, self.path);
            self.idx += 1;
            Ok(res)
        }
    }
}


/// the struct which implements the Sender trait and allows
/// to append a generated json to file
pub struct FileSender {
    path: String
}


impl FileSender {
    pub fn new(path: String) -> Self {
        match metadata(path.clone()) {
            Ok(m) => {
                if m.is_dir() {
                    panic!(format!("the output path {} should point to a file not to a folder.", path));
                }
            }
            Err(_) => match create_file(path.as_str()) {
                Ok(_) => (),
                Err(str) => panic!(str),
            }
        }

        debug!("the file sender with the path {} has been created successfully", path);
        FileSender { path }
    }
}

fn create_file(path: &str) -> Result<(), GenError> {
    if !Path::new(path).exists() {
        match File::create(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(GenError::new_with_in_sender(
                format!("error occurred while creating or open the file:{}", e.to_string())
                    .as_str())),
        }
    } else { Ok(()) }
}

impl Sender for FileSender {
    fn send(&mut self, json: String) -> Result<String, GenError> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.path.as_str())
            .expect("the file to append should be there");


        if let Err(e) = file.write_all(json.into_bytes().as_slice()) {
            Err(GenError::new_with_in_sender(format!("error occurred while appending to the file: {}", e.to_string())
                .as_str()))
        } else {
            Ok(format!("the item has been saved to the file: {}", self.path))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sender::file::{FileSender, FolderSender};
    use crate::sender::Sender;

    #[test]
    fn file_sender_test() {
        match FileSender::new(r#"C:\projects\json-generator\jsons\test.txt"#.to_string())
            .send("test".to_string()) {
            Ok(_) => (),
            Err(_) => panic!("!"),
        }
    }

    #[test]
    fn folder_sender_test() {
        match FolderSender::new(r#"C:\projects\json-generator\jsons\t"#.to_string())
            .send("test!!".to_string()) {
            Ok(_) => (),
            Err(_) => panic!("!"),
        }
    }
}