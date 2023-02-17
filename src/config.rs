use crate::error::{self, Error};
enum ConfigCommand {
    Set { position: usize, path: String },
    Get { position: usize },
    List,
    Reset,
}

impl ConfigCommand {
    fn new(args: Vec<String>) -> Result<Self, Error> {
        let main_comm = match args.get(1) {
            Some(res) => res,
            None => return Err(_),
        };
        parse
    }
}
