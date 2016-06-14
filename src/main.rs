use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io;
use std::fs;
extern crate regex;
use regex::Regex;
extern crate ansi_term;
use ansi_term::Colour::Green;
use ansi_term::Style;
use std::process::Command;
use std::cmp;
extern crate bincode;
extern crate rustc_serialize;
use std::fmt;

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};
//


fn string_please() -> String {
    let mut temp_st = String::new();
    let mut good_result = false;

    while !good_result {
        let temp = io::stdin().read_line(&mut temp_st);
        if temp.is_ok() {
            good_result = true;
        } else {
            println!("please enter a series of letters and/or numbers:");
            temp_st = "".to_string();
        }

    }

    temp_st.trim().to_string()
}

fn clear() {
    // clear command line (for all platforms... hopefully)
    let output = Command::new("cls").output().unwrap_or_else(|_| {
        Command::new("clear").output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}\n Need to implement terminal clearing command \
                    for your operating system",
                   e)
        })
    });

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn word_match(word: &str, to_do: &str) -> bool {
    println!("Type \"{}\" {}", Green.underline().paint(word), to_do);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read your input!");
    let strng = format!("(?i){}", word);
    let meta_match = Regex::new(strng.as_str()).expect("words to match or not match");
    meta_match.is_match(&input)
}

fn count_files(folder: &str) -> Option<u16> {
    let p = Path::new(folder);
    if p.is_dir() {
        let mut count = 0;
        let paths = fs::read_dir(&folder)
            .expect("Unable to read the files in the directory :( -- perhaps the folder \
                     permissions are wrong?");
        // only count the files that have content in them
        for path in paths {
            let temp_file_name = format!("{}", path.expect("print").path().display());
            if is_empty_file(&temp_file_name) {
                continue;
            } else {
                count += 1
            }


        }
        Some(count)
        // Some(paths.by_ref().count() as u16)
    } else {
        None
    }
}

macro_rules! parse_input {
    ($t:ty,$min:expr,$max:expr) => {{
    if $min != $max {
    println!("Please enter a value greater or equal to: {} and less than or equal to: {}", $min, $max);
    }
    else {
        println!("Well, the only choice is {}", $min);

    }



    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<$t>() {
        Ok(i) => {
        if (i > $max) | (i < $min) {None}
        else {Some(i)}

        }
        Err(..) => None,
    }
    }};
}

macro_rules! parse_as {
    ($e:expr,$t:ty) => {{

    let trimmed = $e.trim();
    match trimmed.parse::<$t>() {
        Ok(i) => Some(i),


        Err(..) => None,
    }
    }};
}

fn make_choice(vs: &Vec<String>, explanation: &str) -> String {

    let max = vs.iter().count() as u16;

    println!("\n-----------\n{}\n-----------", Green.paint("choices"));
    for (i, st) in vs.iter().enumerate() {
        println!("{} ->  {}", i + 1, st);
    }
    println!("-----------\n");
    println!("{}", explanation);

    let mut index = parse_input!(u16, 1, cmp::min(std::u16::MAX, max));

    while index.is_none() {
        index = parse_input!(u16, 1, cmp::min(std::u16::MAX, max));

    }


    vs[(index.unwrap() - 1) as usize].to_string()
}


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
            // println!("lines of s.lines: {}",s.lines().count());
            if s.lines().count() == 0 {
                true
            } else {
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

fn print_first_line_of_file(file: &str) -> String {
    let mut line = String::new();
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
            // println!("lines of s.lines: {}",s.lines().count());
            if s.lines().count() == 0 {
                println!("This file doesn't even have one line!")
            } else {
                line = s.lines().nth(0).expect("print_first_line_of_file to work").to_string();
                println!("{}:\n{}\n", Green.paint("First line: "), line);
            }

        }
    }
    line
}

// prints the files in a directory/ letting the user know if it is not a directory
fn collect_files(folder: &str) -> Vec<String> {
    let mut v: Vec<(String)> = Vec::new();

    let p = Path::new(folder);

    if p.is_dir() {
        let paths = fs::read_dir(&folder).expect("collect_files to work");


        for path in paths {
            // put choice in a vector of tuples (choice number, filename)
            let disp = format!("{}", path.expect("path in paths to work").path().display());
            if is_empty_file(&disp) {
                continue;
            } else {
                v.push(disp.to_string());
            }
        }
        // println!("-------------------------")

    } else {
        println!("{} is not a directory", folder);
    }
    v
}

fn file_exists(file_name: &str) -> bool {
    Path::new(file_name).exists() && Path::new(file_name).is_file()
}

fn choose_another_file() -> String {
    println!("Please choose another file: ");
    let mut file_name = string_please();
    let mut exists = file_exists(&file_name);
    while !exists {
        println!("Please choose another file, {} does not exist: ", file_name);
        file_name = string_please();
        exists = file_exists(&file_name);
    }
    file_name
}

fn file_to_string(file: &str) -> String {
    let mut my_file = file.to_string();
    if file_exists(file) {} else {
        my_file = choose_another_file()
    }

    let path = Path::new(&my_file);
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
        Ok(_) => s,
    }
}

fn split_file_to_columns(file: &str, delimiter: &str) -> Vec<Vec<String>> {
    let temp = print_first_line_of_file(file);
    let count = split_over(&temp, delimiter).len();

    // split lines that don't have exactly "count" number of fields will be thrown away
    let s = file_to_string(file);

    let mut outter_vec: Vec<Vec<String>> = Vec::new();
    for _ in 0..count {
        outter_vec.push(Vec::new());
    }

    for line in s.lines() {
        let split = split_over(line, delimiter);
        if split.len() != count {
            println!("\"{}\" cannot be split correctly... Check the file ({}) for this line",
                     line.trim(),
                     file);
            continue;
        } else {
            for (inner_index, val) in split.iter().enumerate() {
                outter_vec[inner_index].push(val.to_string());
            }
        }
    }
    outter_vec

}




fn split_over<'a, 'b>(line: &'a str, delimiter: &'b str) -> Vec<&'a str> {
    line.split(delimiter).collect::<Vec<&str>>()
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
struct Column {
    total_row_number: u64,
    name: String,
    values: Vec<String>,
    is_numeric: bool,
    bad_rows: Option<Vec<usize>>, /* bad_rows starts at 0
                                   * max: Option<f64>,
                                   * min: Option<f64>, */
}

impl Column {
    fn new(data: Vec<String>, name: &str) -> Column {
        let name = name.to_string();
        let values = data;
        let mut is_numeric = false; //if the
        let first = values[0].to_owned();
        if values.len() > 0 {
            if let Some(_) = parse_as!({
                                           &first
                                       },
                                       f64) {



                println!("I've determined that the \"{}\" column is numeric. Is this correct?",
                         name);
                if !word_match("yes", "to confirm that the column is numeric.") {
                    is_numeric = false;
                } else {
                    is_numeric = true;
                }



            } else {
                println!("I've determined that the \"{}\" column contains text values as opposed \
                          to numeric values. Is this correct?",
                         name);
                if word_match("yes", "to confirm that the column is textual.") {
                    is_numeric = false;
                } else {
                    is_numeric = true;
                }
            }





        } else {
            println!("For some reason this column has no data.");
        }




        let mut bad_rows: Vec<usize> = Vec::new();


        for (ind, val) in values.iter().enumerate() {



            if is_numeric &
               parse_as!({
                             &val
                         },
                         f64)
                .is_none() {
                println!("\nFor column: {} Row number {} is not numeric. It has a value of: {}. \
                          For your convenience I'll mark this row and move on",
                         name,
                         ind,
                         val);
                bad_rows.push(ind);
            }
        }
        let r: Option<Vec<usize>>;
        if bad_rows.is_empty() {
            r = None;
        } else {
            r = Some(bad_rows);
        }
        let total_row_number = values.iter().len() as u64;

        Column {
            name: name,
            total_row_number: total_row_number,
            values: values,
            is_numeric: is_numeric,
            bad_rows: r,
        }


    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Data { ref field_names, ref project_name, ref data_file, .. } = *self;

        let width = field_names.iter().fold(0, |big, current| {
            if big >= current.len() {
                big
            } else {
                current.len()
            }
        }) + 3;

        let mut header = String::new();
        for name in field_names {
            let temp = format!("{n:<width$}| ", n = name, width = width);
            header = header + &temp;
        }

        try!(write!(f,
                    "Your project \"{}\" is based on data from the file \"{}\"..\nThe column \
                     names should look like: \n",
                    project_name,
                    data_file));

        write!(f, "{}", header)

    }
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
struct Data {
    field_names: Vec<String>,
    data_file: String,
    data: Vec<Column>,
    delimiter: String,
    project_name: String, // will be the file name of the meta-file that saves the data state
    bad_rows: Vec<usize>, // bad lines of the file
}

impl Data {
    fn initialize() -> Data {

        // clear terminal
        clear();

        let mut has_data: bool;
        let has_meta_data: bool;
        let mut wait_for_data = false;
        let mut data_file: String = "temp".to_string();
        let mut delimiter = String::new();
        let mut field_names: Vec<String> = Vec::new();
        let mut bad_rows: Vec<usize> = Vec::new();
        let mut data: Vec<Column> = Vec::new();
        let mut project_name = String::new();

        // check for existance of ./data and ./meta folder
        match count_files("./meta") {
            Some(_) => has_meta_data = true,

            None => has_meta_data = false,
        }


        match count_files("./data") {
            Some(a) => {
                println!("Inside the {} folder there are {} non-empty file(s)\n",
                         Green.paint("./data"),
                         Green.paint(a.to_string()));
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
            println!("It seems that you are new to this program. Welcome! This program is \
                      written to be highly intuitive for the user (you). If you have any \
                      questions please post them on the Rusty_ML github page.");
            match make_folder("/data", None) {
                Err(e) => {
                    println!("Error trying to make the data folder: {}", e);
                }
                Ok(s) => {
                    wait_for_data = true;
                    println!("\n{} folder successfully created! Go ahead and add a data file.\n",
                             s)
                }
            }
            // if there are no files in the data folder this should be true




        }

        while wait_for_data == true {
            println!("All data that you want to import should be placed in the data folder that \
                      exists in the location the place where this program was called from.");
            if word_match("data added", "to continue...") {
                match count_files("./data") {
                    Some(a) => {
                        if a == 0 {
                            println!("\nMake sure you put the data in the correct folder, also, \
                                      this program checks to see that the file is not empty.");
                            wait_for_data = true
                        } else {
                            wait_for_data = false;
                            has_data = true
                        }
                    }
                    None => wait_for_data = false,
                }
            }
        }

        let mut choice: String;
        // let mut data : vec<vec<String>>;

        if !has_meta_data && has_data {
            // need to choose a data file to load ... have them pick from the data directory

            clear();
            let v: Vec<String> = collect_files("./data");
            choice = make_choice(&v,
                                 "Pick the number of the file that you would like to work on: ");

            clear();

            println!("\n{}: {}.", Green.paint("You chose"), choice);

            while !word_match("continue",
                              "if this is the file you want to work with. Enter anything else \
                               if you want to pick another file.") {
                clear();
                choice = make_choice(&v,
                                     "Pick the number of the file that you would like to work \
                                      on: ");
                println!("\n{}: {}.", Green.paint("You chose"), choice);

            }

            data_file = choice.to_string();


            clear();

            println!("\nOkay, great! Now let's collect some information about this file. I'll \
                      provide a line from the data file and I want you to tell me some things \
                      about it.\n");

            let first_line = print_first_line_of_file(&choice);

            println!("What is the {} for the data (i.e. {}).",
                     Green.paint("delimiter"),
                     Style::new()
                         .bold()
                         .paint("the thing that separates the pieces of data -- usually a colon \
                                 or a comma"));

            let mut need_delimiter = true;

            let mut my_split: Vec<&str>;


            while need_delimiter {


                io::stdin().read_line(&mut delimiter).ok().expect("should read line...");

                delimiter = delimiter.to_string().trim().to_string();

                my_split = split_over(&first_line, &delimiter);

                clear();

                println!("Supposing that the {} is `{}` the first row will be parsed to look \
                          like:\n",
                         Green.paint("delimiter"),
                         delimiter);

                for (c, st) in my_split.iter().enumerate() {
                    println!("column {}: {}", c + 1, &st);
                }

                println!("\nMake sure this is {} because all subsequent rows will be parsed in a \
                          similar manner!\n",
                         Green.bold().paint("correct"));


                need_delimiter = !word_match("correct",
                                             "to indicate that you've entered the right \
                                              delimiter. Type anything else to pick a different \
                                              delimiter.");

                if need_delimiter {
                    clear();
                    println!("Alright, please enter a new delimiter:");
                    continue;

                }


                clear();
                println!("Okay, now for each of the columns, tell me the {} (i.e. {})\n",
                         Green.paint("label"),
                         Style::new()
                             .bold()
                             .paint("the name of what the data in the column actually \
                                     represents."));



                for (c, st) in my_split.iter().enumerate() {

                    println!("What is the name of column {} (with a value of {})? You might have \
                              to check to see where you got this data from in order to correctly \
                              name the columns.",
                             c + 1,
                             &st);

                    field_names.push(string_please());


                }




            }


            let v: Vec<Vec<String>> = split_file_to_columns(&data_file, &delimiter);


            for (i, col) in v.into_iter().enumerate() {
                let temp = Column::new(col, &field_names[i]);

                if temp.bad_rows.is_some() {
                    let mut temp_bad: Vec<usize> = temp.bad_rows.to_owned().unwrap();

                    bad_rows.append(&mut temp_bad);
                    bad_rows.sort();
                    bad_rows.dedup();
                    println!("{:?}", bad_rows);


                }

                data.push(temp);


            }





        }

        Data {
            field_names: field_names,
            data_file: data_file,
            data: data,
            delimiter: delimiter,
            project_name: "1".to_string(), /* will be the file name of the meta-file that saves the data state */
            bad_rows: bad_rows, // bad lines of the file
        }

    }

    fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let data: Vec<u8> = encode(self, SizeLimit::Infinite).expect("Should encode self...");
        let mut file = try!(File::create(path));
        file.write_all(&data).map_err(|e| e.into())

    }

    fn load(path: &str) -> Result<Data, std::io::Error> {
        let mut data: Vec<u8> = Vec::new();
        let mut file = try!(File::open(path));

        file.read_to_end(&mut data);
        let d: Data = decode(&data).expect("Should encode self...");
        Ok(d)
    }

    fn show(&self) {
        println!("{:?}", self);
    }
}






fn main() {

    // show_files("../blah");

    // let d = Data::initialize();
    let f: Data = Data::load("./project1").expect("please work");

    println!("{}", f);
    // d.save("./project1").expect("please work");
    // f.show();



}
