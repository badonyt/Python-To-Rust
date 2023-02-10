use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use std::collections::HashSet;
//use std::io::prelude::*;
use std::collections::HashMap;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn replace_all_occurrences(s: &str, target: &str, new: &str) -> Box<str> {
    let result = s.replace(target, new);
    result.into_boxed_str()
}
fn remove_first_char_from_str(s: &str, chars: char)-> &str{
    let mut final_length: &str = s;
    let mut i = 0;
    while i < s.len(){
        if final_length.chars().next().map(|c| c).unwrap_or(' ') == chars{
            
            final_length= &final_length[1..];

        }else{
            break;
        }
        i+=1
    }
    return final_length;
}
fn count_leading_spaces(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars() {
        if c != ' ' {
            break;
        }
        count += 1;
    }
    count
}
fn read_file_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.split(b'\n') {
        let line = line.unwrap();
        lines.push(String::from_utf8_lossy(&line).to_string());
    }

    lines
}

fn first_part_without_char(s: &str, c: char) -> &str {
    s.split(c).next().unwrap_or("")
}

fn write_to_file(filename: &str, vec: &Vec<String>) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    for line in vec {
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
    }

    Ok(())
}

fn has_no_repeated_characters(s: &str, ch: char) -> bool {
    let mut set = HashSet::new();

    for c in s.chars() {
        if c == ch {
            if set.contains(&ch) {
                return false;
            }
            set.insert(ch);
        }
    }

    true
}
fn check_var(line: &str)-> bool{
    //let s = "Hello, world!";
    let ch = '=';
    let v: Vec<&str> = line.split(' ').collect();
    if line.contains(ch) == true && v[0] != "def" && line.contains("!") == false && has_no_repeated_characters(line.clone(), ch) == true{
        return true;
        
    } else {
        return false;
    }

}
fn check_if_or_def(line: &str)-> &str{
    //let s = "Hello, world!";
    
    let v: Vec<&str> = line.split(' ').collect();
    println!("{:?}", v);
    if v.contains(&"if") == true{
        return "if";
    } else if v.contains(&"def") == true{
        return "def";
    }else{
        return "none";
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        std::process::exit(1);
    }
    if args[1] == "--help"{
        println!("GENERIC --HELP MESSAGE");
        std::process::exit(1);
    }
    let mut final_file = Vec::new();
    let file_path = &args[1];
    let lines = read_file_lines(file_path);
    let mut i = 0;
    //usize check
    let mut vars:HashMap<String, usize> = HashMap::new();
    //UNCLOSED
    let mut unclosed_if = 0;
    for line in lines.clone() {
        let mut notif: bool = false;
        println!("{}", count_leading_spaces(&line));
        //check var
        if check_var(&line) == true{
            println!("var incoming");
            let v: Vec<&str> = line.split('=').collect();
            println!("{:?}", vars);
            
            if vars.contains_key(v[0].trim()) { 
                println!("VAR but new");
                final_file.push(format!("{} = {}",v[0],&v[1]));
                let vs: Vec<&str> = lines[vars[v[0].trim()]].split('=').collect();
                //print_type_of(vars[&i]);
                final_file[vars[v[0].trim()]] = format!("let mut {} ={}",v[0], vs[1])
            } else {
                final_file.push(format!("let {} = {}",v[0],&v[1]));
                vars.insert(v[0].trim().to_owned(),i.try_into().unwrap(),);
            }
            
           

            
            
        }else if check_if_or_def(&line) == "if"{
            notif = true;
            unclosed_if += 1;
            let damn: &str = first_part_without_char(&line, ':');
            let damn: Box<str> = replace_all_occurrences(&damn, "or", "||").into();
            final_file.push(format!("{}{}", replace_all_occurrences(&*damn, "and", "&&") , '{'));
        }else if check_if_or_def(&line) == "def"{
            //unclosed_if +=1;
            
            let damn: &str = first_part_without_char(&line, ':');
            final_file.push(format!("{}{}", &damn , '{'));
        }else if remove_first_char_from_str(&line, ' ').split("(").next() == Some("print"){
            let mut parts = remove_first_char_from_str(&line, ' ').split("(");
            println!("dan");
            final_file.push(format!("println!({}",parts.nth(1).unwrap()));
        }
        
        
       
        if i != 0 {
            //print_type_of(
              //  &i);
            let yy = i - 1;
            if count_leading_spaces(&line) == 0  && count_leading_spaces(&lines[yy])>count_leading_spaces(&line){
            
                final_file.insert(i,format!("{}", "}"));   
                unclosed_if -=1;
            } 

            else if count_leading_spaces(&line) != 0 && notif == false && count_leading_spaces(&line)>count_leading_spaces(&lines[i + 1]){
                final_file.push(format!("{}", "}"));   
                unclosed_if -=1
            } 
        }
        i +=1;
    }
    let mut y =0 ;
    while y <unclosed_if{
        final_file.push(format!("{}", "}")); 
        y+=1
    }
    println!("{:?}", vars);
    println!("{:?}", final_file);
    if args.len() == 3 {
        write_to_file(&args[2], &final_file);
    }else{
        write_to_file("main.rs", &final_file);
    }
    
}