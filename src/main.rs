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
    let meta_match = Regex::new(strng.as_str()).expect("words to match or not match");
    let b = meta_match.is_match(&input);
    //println!("{}",b);
    b
}

fn count_files(folder: &str) -> Option<u16> {
    let p = Path::new(folder);
    if p.is_dir() {
        let mut count = 0;
        let paths = fs::read_dir(&folder)
                            .expect("Unable to read the files in the directory :( -- perhaps the \
                                     folder permissions are wrong?");
        //only count the files that have content in them
        for path in paths{
            let temp_file_name = format!("{}",path.expect("print").path().display());
            if is_empty_file(&temp_file_name) {continue} else {count += 1}
            
            
        }
        Some(count)
        //Some(paths.by_ref().count() as u16)
    } else {
        None
    }
}

macro_rules! parse_input {
    ($t:ty) => {{
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<$t>() {
        Ok(i) => Some(i),
        Err(..) => None,
    }
    }};
}




fn make_choice(vs: Vec<(u16, String)>) -> String {
    
    println!("\n-----------\nchoices\n-----------");
    for v in &vs {
        println!("{} ->  {}", v.0, v.1);
    }
        println!("-----------\n");    
        println!("Pick the number of the file that you would like to work on: ");

    let mut index = parse_input!(u16);
    
    while index.is_none() {
        index = parse_input!(u16);
    }
    let num_index = index.expect("the index to be a number");
    //println!("{:?}", index);
    
    vs[(num_index  - 1) as usize].1.to_string()
}
//

fn is_empty_file(file: &str) -> bool {
    let path = Path::new(file);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {
            //println!("lines of s.lines: {}",s.lines().count());
            if s.lines().count() == 0{
                true
            }
            else{
                false
            }

        }
    }
}

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
            //println!("lines of s.lines: {}",s.lines().count());
            if s.lines().count() == 0{
                println!("This file doesn't even have one line!")
            }
            else{
            
            println!("{}",s.lines().nth(0).expect("print_first_line_of_file to work"));
            }

        }
    }

}


// prints the files in a directory/ letting the user know if it is not a directory
fn collect_files(folder: &str) -> Vec<(u16, String)> {
    let mut v: Vec<(u16, String)> = Vec::new();

    let p = Path::new(folder);

    if p.is_dir() {
        let paths = fs::read_dir(&folder).expect("collect_files to work");
        //println!("Which data file would you like to work on?:\n-------------------------");

        let mut choice = 1u16;

        for path in paths {
            // put choice in a vector of tuples (choice number, filename)
            let disp = format!("{}", path.expect("path in paths to work").path().display());
            if is_empty_file(&disp){
                continue
            }
            else{
            //println!("{} :: {}", choice, disp);
            v.push((choice, disp.to_owned()));
            choice += 1;
            }
        }
        //println!("-------------------------")

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
    fn initialize() {

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
                println!("Inside the ./data folder there are {} non-empty file(s)", a);
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
            println!("It seems that you are new to this program. Welcome! This program is written to be highly intuitive for the user (you). If you have any questions please post them on the Rusty_ML github page.");
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




        }
        
            while wait_for_data == true {
                println!("All data that you want to import should be placed in the data folder that exists **in the location the place where this program was called from**");
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

        let choice : String;
        //let mut data : vec<vec<String>>;

        if !has_meta_data && has_data {
            // need to choose a data file to load ... have them pick from the data directory
            let v: Vec<(u16, String)> = collect_files("./data");
            choice = make_choice(v);
            print_first_line_of_file(&choice);
            
            //right here we want to ask the user to specify the data headers and delimiter based on the printed first line
            
            
            //data = Data::load(&self.data_file);
            // println!("{:?}", v[0]);


        }

 

    }


/*
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

*/ 

}






fn main() {

    // show_files("../blah");

    Data::initialize();



}
