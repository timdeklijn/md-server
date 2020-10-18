use std::fs;
use std::path::{Path, PathBuf};

// Create a pathbuf vector with files in a path
fn list_files_in_directory(p: &Path) -> Vec<PathBuf> {
    let mut file_list = Vec::new();
    let entries = fs::read_dir(p).expect("Error reading directory");
    for entry in entries {
        let f = match entry {
            Ok(x) => x,
            Err(_) => panic!("Error unpacking file path"),
        };
        file_list.push(f.path());
    }
    file_list
}

pub fn create_content_list(folder: &String) -> Vec<Content> {
    let mut content_list = Vec::new();
    let folder_path = Path::new(&folder);
    for file in list_files_in_directory(&folder_path) {
        let content = Content::new(&file, folder.clone());
        content_list.push(content);
    }
    content_list
}

fn path_to_id(p: &PathBuf) -> String {
    p.file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap()
        .replace(".md", "")
}

#[derive(Debug)]
pub struct Content {
    pub parent: String,
    pub id: String,
    pub path: String,
}

impl Content {
    pub fn new(path: &PathBuf, parent: String) -> Self {
        Content {
            parent,
            id: path_to_id(&path),
            path: path.to_str().unwrap().to_string(),
        }
    }
}
// Compare two dircontent objects
impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent && self.id == other.id && self.path == other.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_files_in_directory() {
        let folder_name = "files";
        let p: PathBuf = [folder_name].iter().collect();
        let result = list_files_in_directory(&p);
        let expected: Vec<PathBuf> = vec![["files", "tst.md"].iter().collect()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_create_content() {
        let folder_name = "files";
        let p: PathBuf = [folder_name, "tst.md"].iter().collect();
        let result = Content::new(&p, folder_name.to_string());

        let expected = Content {
            parent: "files".to_string(),
            id: "tst".to_string(),
            path: "files/tst.md".to_string(),
        };

        assert_eq!(result, expected);
    }
}
