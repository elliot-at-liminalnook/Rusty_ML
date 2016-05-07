use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io;
use std::fs;
extern crate regex;
use regex::Regex;





#[derive(Debug)]
struct Data {
    field_names : Vec<String>,
    data_file : String,
    data : Vec<Vec<String>>,
    has_meta_data :   bool,
    has_data : bool,
    delimiter : String,

}

fn word_match(word : &str) -> bool{
let mut input = String::new();
io::stdin()
    .read_line(&mut input)
    .expect("failed to read your input!");
let strng = format!("(?i){}",word);
let meta_match = Regex::new(strng.as_str()).unwrap();
meta_match.is_match(&input)
}

fn count_files(folder : &str) -> Option<u16>{
    let p = Path::new(folder);
    if p.is_dir(){
        let  mut paths = fs::read_dir(&folder).unwrap();
    Some(paths.by_ref().count() as u16)
    }
    else{
        None
    }
}


//prints the files in a directory/ letting the user know if it is not a directory
fn show_files(folder : &str){
    let p = Path::new(folder);

    if p.is_dir(){
        let  paths = fs::read_dir(&folder).unwrap();
println!("Which data file would you like to work on?:\n-------------------------");

    let mut choice   = 1;

    for path in paths {
        //put choice in a vector of tuples (choice number, filename)
        println!("{} :: {}", choice, path.unwrap().path().display());
        choice += 1;
    }
    println!("-------------------------")

    }

    else {println!("{} is not a directory", folder);}

}

impl Data  {
    fn initialize() -> Data {

        let has_data : bool;
        let has_meta_data : bool;

        //check for existance of ./data and ./meta folder
        match count_files("./meta"){
            Some(a) => {if a == 0 {has_meta_data = false; } else {has_meta_data = true }},
            None => {has_meta_data = false}
        }

        match count_files("./data"){
            Some(a) => {if a == 0 {has_data = false; } else {has_data = true }},
            None => {has_data = false}
        }

        if !has_data && !has_meta_data {
            println!("It seems that you are new to this program! Welcome!\nI'll give a quick intro.\n All data that you want to import should be placed in the .\\data folder that has been created in the same directory as this program.\n After you put some data in this folder, please re-run this program!");
        }

        //need to choose a filename to load ... have them pick from the data directory

        if !has_meta_data && has_data{
            data = load()
        }

        let mut d =  Data { field_names : Vec<String>,
            data_file_name : String,
            data : Vec<Vec<String>>,
            has_meta_data :   has_meta_data,
            has_data : has_data };

    }


    fn load(data_file : &str ) -> Vec<Vec<String>>{
        //we will collect field_names, delimiter
        let mut c :  Vec<Vec<String>> = Vec::new();



        // Create a path to the desired file
        let path = Path::new(data_file);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display,
                                                       Error::description(&why)),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display,
                                                       Error::description(&why)),
            Ok(_) => {

                println!("{} data loaded!", display);

                for l in s.lines(){
                    let x = l.split(",").into_iter().map(|c| c.to_owned()).collect::<Vec<String>>();
                    c.push(x);

            }
            },
        }
        c //this is the data that we want to return
        }


}





fn main() {

    show_files("./src");
    //show_files("../blah");

    Data::initialize();
    Data::load();



}
