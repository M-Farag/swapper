//! The swapper library: A library for editing files contents by swapping parts
//! You can define any piece of text to be replaced by another piece of text


use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "swapper", about = "A library for editing files contents by swapping string  parts")]
pub struct Arguments {
    #[structopt(short="p", long="path", help="The path to the file to be edited")]
    file_path: String,
    #[structopt(short="o", long="origin", help="The string to be replaced")]
    origin_string: String,
    #[structopt(short="n", long="new", help="The string to replace the origin string")]
    new_string: String,
}

impl Arguments {
    pub fn new() -> Self {
        Arguments::from_args()
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn origin_string(&self) -> &str {
        &self.origin_string
    }

    pub fn new_string(&self) -> &str {
        &self.new_string
    }
    
}