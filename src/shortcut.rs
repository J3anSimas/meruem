use crate::error::{self, Error};

pub struct ShortcutList {
    paths: [Option<String>; 5],
}

impl ShortcutList {
    pub fn new(paths_input: [&str; 5]) -> Result<Self, Error> {
        for p in paths_input {
            if p != "" && !Self::validate_path(p) {
                return Err(Error::InvalidPath {
                    path: p.to_string(),
                });
            }
        }
        let paths_vec: Vec<Option<String>> = paths_input
            .into_iter()
            .map(|x| if x != "" { Some(x.to_string()) } else { None })
            .collect();

        let paths: [Option<String>; 5] = paths_vec.try_into().unwrap();
        Ok(Self { paths })
    }
    pub fn list(&self) -> &[Option<String>; 5] {
        return &self.paths;
    }
    pub fn set(&mut self, position: usize, path: String) -> Result<(), Error> {
        if position > 4 {
            return Err(Error::OutOfBounds { idx: position });
        }
        if !Self::validate_path(path.as_str()) {
            return Err(Error::InvalidPath { path });
        }
        self.paths[position] = Some(path);
        Ok(())
    }
    pub fn get(&self, position: usize) -> Result<&Option<String>, Error> {
        if position > self.paths.len() - 1 {
            return Err(Error::OutOfBounds { idx: position });
        }
        return Ok(&self.paths[position]);
    }
    pub fn reset(&mut self) {
        self.paths = [None, None, None, None, None];
    }
    fn validate_path(path: &str) -> bool {
        let path = std::path::Path::new(path);
        println!("{:?}", path.as_os_str());
        path.is_dir()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn list() {
        let shortcut_list: ShortcutList = match ShortcutList::new(["C:", "C:\\Users", "", "", ""]) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };
        let result = shortcut_list.list();
        assert_eq!(
            result,
            &[
                Some(String::from("C:")),
                Some(String::from("C:\\Users")),
                None,
                None,
                None
            ]
        )
    }
    #[test]
    fn set() {
        let mut shortcut_list: ShortcutList = match ShortcutList::new(["", "", "", "", ""]) {
            Ok(result) => result,
            Err(_) => panic!(""),
        };
        shortcut_list.set(0, "C:".to_string());
        assert_eq!(
            shortcut_list.list(),
            &[Some(String::from("C:")), None, None, None, None]
        )
    }
    #[test]
    fn get() {
        let mut shortcut_list: ShortcutList = match ShortcutList::new(["C:", "", "", "", ""]) {
            Ok(result) => result,
            Err(_) => panic!(""),
        };
        let result = match shortcut_list.get(0) {
            Ok(result) => result,
            Err(_) => panic!(""),
        };
        let result = match result {
            Some(res) => res,
            None => panic!(""),
        };
        assert_eq!(result, &String::from("C:"));
    }
}
