use std::fs;
use std::env::args;
use std::process;
use std::io::*;

struct Extract;

impl Extract {
    fn extract() -> Result<()> {
        let argc: Vec<String> = args().collect();

        if argc[1] == "-h" {
            println!("will do this later");
            process::exit(1)
        } else if argc[1] == "--help" {
            println!("will do this later");
            process::exit(1)
        }

        let file = &argc[1];

        // let metadata = metadata(file)?;

        let mut name: Vec<&str>  = file.split('.').collect();

        if name[1] != "apk" {
            println!("the file that you provided isn't a Android Package, please provide a .apk file");
            process::exit(1)
        }

        name[1] = ".zip";

        let newname = String::from(name[0]) + name[1];

        println!("{}", newname);

        let backupath = String::from("./backup/") + file;

        fs::copy(file, backupath)?;

        Ok(())

    }
}

fn main() {
    let _ext = Extract::extract();
}
