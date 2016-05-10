use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io;
use std::fs;
extern crate regex;
use regex::Regex;







fn word_match(word: &str) -> bool {
    println!("Type \"{}\" to continue...", word);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read your input!");
    let strng = format!("(?i){}", word);
    let meta_match = Regex::new(strng.as_str()).unwrap();
    let b = meta_match.is_match(&input);
    //println!("{}",b);
    b
}

fn count_files(folder: &str) -> Option<u16> {
    let p = Path::new(folder);
    if p.is_dir() {
        let mut paths = fs::read_dir(&folder)
                            .expect("Unable to read the files in the directory :( -- perhaps the \
                                     folder permissions are wrong?");
        Some(paths.by_ref().count() as u16)
    } else {
        None
    }
}

fn make_choice(vs: Vec<(u16, String)>) -> String {
    println!("Pick the number of the file that you would like to work on: ");
    for v in &vs {
        println!("{} ->  {}", v.0, v.1);
    }
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let index = input.parse::<u8>().expect("wanted a number");

    vs[(index as usize) - 1].1.to_owned()
}
//


// if a folder location isn't provided, the default folder location is the current directory (i.e. the directory from which this is called)
fn make_folder(name: &str, optional_folder_location: Option<&str>) -> Result<String, String> {
    // let mut st = String::new();
    match optional_folder_location {
        Some(s) => {
            let n_st = s.to_string() + name;
            try!(fs::create_dir(n_st.to_owned()).map_err(|e| e.to_string()));
            Ok(format!("{}", n_st))
        }
        None => {
            let n_st = ".".to_owned() + name;
            try!(fs::create_dir(n_st.to_owned()).map_err(|e| e.to_string()));
            Ok(format!("{}", n_st))
        }
    }


}

fn print_first_line_of_file(file: &str) {
    let path = Path::new(file);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {

            println!("{}",s.lines().nth(0).unwrap());


        }
    }

}


// prints the files in a directory/ letting the user know if it is not a directory
fn show_files(folder: &str) -> Vec<(u16, String)> {
    let mut v: Vec<(u16, String)> = Vec::new();

    let p = Path::new(folder);

    if p.is_dir() {
        let paths = fs::read_dir(&folder).unwrap();
        println!("Which data file would you like to work on?:\n-------------------------");

        let mut choice = 1u16;

        for path in paths {
            // put choice in a vector of tuples (choice number, filename)
            let disp = format!("{}", path.unwrap().path().display());
            println!("{} :: {}", choice, disp);
            v.push((choice, disp.to_owned()));
            choice += 1;
        }
        println!("-------------------------")

    } else {
        println!("{} is not a directory", folder);
    }
    v //return the vector of tuples of the choices we can make!
}

#[derive(Debug)]
struct Data {
    field_names: Vec<String>,
    data_file: String,
    data: Vec<Vec<String>>,
    has_meta_data: bool,
    has_data: bool,
    delimiter: String,
    project_name: String,
}

impl Data {
    fn initialize(&self) {

        let has_data: bool;
        let has_meta_data: bool;
        let mut wait_for_data = false;

        // check for existance of ./data and ./meta folder
        match count_files("./meta") {
            Some(_) => {
                    has_meta_data = true
                },

            None => has_meta_data = false,
        }


        match count_files("./data") {
            Some(a) => {
                if a == 0 {
                    has_data = true;
                    wait_for_data = true
                } else {
                    has_data = true
                }
            }
            None => has_data = false,
        }

        if !has_data && !has_meta_data {
            println!("It seems that you are new to this program. Welcome! I'll give you a quick \
                      intro. All data that you want to import should be placed in the data \
                      folder that has been just been created in the same directory as this \
                      program.");
            match make_folder("/data", None) {
                Err(e) => {
                    println!("Error trying to make the data folder: {}", e);
                }
                Ok(s) => {
                    println!("{} folder successfully created! Go ahead and add a data file.",
                             s)
                }
            }
             //if there are no files in the data folder this should be true

            while wait_for_data == true {
                if word_match("data added") {
                    match count_files("./data") {
                        Some(a) => {
                            if a == 0 {
                                println!("Make sure you put the data in the correct folder");
                                wait_for_data = true
                            } else {
                                wait_for_data = false
                            }
                        }
                        None => wait_for_data = false,
                    }
                }
            }


        }



        if !has_meta_data && has_data {
            // need to choose a data file to load ... have them pick from the data directory
            let v: Vec<(u16, String)> = show_files("./data");
            self.data_file = make_choice(v);
            
            print_first_line_of_file(choice.as_str());
            self.data = Data::load(&choice);
            // println!("{:?}", v[0]);


        }

 

    }



    fn load(data_file: &String) -> Vec<Vec<String>> {
        // we will collect field_names, delimiter
        let mut c: Vec<Vec<String>> = Vec::new();



        // Create a path to the desired file
        let path = Path::new(data_file.as_str());
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that describes the error
            Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
            Ok(_) => {

                println!("{} data loaded!", display);

                for l in s.lines() {
                    let x = l.split(",").into_iter().map(|z| z.to_owned()).collect::<Vec<String>>();
                    c.push(x);

                }
            }
        }
        c //this is the data that we want to return
    }

}






fn main() {

    // show_files("../blah");

    Data::initialize();



}
