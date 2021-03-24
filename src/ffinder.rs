use std::env;
use std::mem;
use colored::*;
use std::process::exit;
use walkdir::{DirEntry, WalkDir};


struct Input{
    prmtr: String, //parameter being searched
    base_dir: String, //Base directory to start search
    extension:bool, //user included the file extension or not (.something)//
    hidden:bool, //search or not in hidden directories
}

fn input()->Input{
    let mut args: Vec<String> = env::args().collect();
    let (mut b, mut p) = (get_home(),String::new());
    let (mut c, mut e, mut h) = (false,false,false);
    args.remove(0);
    for s in args{
        if s == "-h"{
            h=true;
        }
        else if s == "-c" && !c {
            c=true;
            b=env::current_dir().unwrap().display().to_string();
        }
        else if s.contains("/"){//USER SPECIFIED CUSTOM DIR
            b=s;
        }
        else{
            if s.contains("."){ //user specified the file extension!
                e=true;
            }
            if p.is_empty(){
                p = s;
            }
            else{
                p = p + " " + &s;
            }
        }
    }
    if p.len()==0{
        println!("{}","Please specify a paramater!".red().bold().italic());
        exit(0);
    }
    let inp = Input {
        prmtr: p,
        base_dir: b,
        extension: e,
        hidden: h,
    };
    inp
}

fn get_home()-> String{
    let mut s =  String::new(); 
    match home::home_dir() {
        Some(path) =>{
            s = path.display().to_string();
        } 
        None =>(),
    }
    s
}

fn rmv_underline(s: String) -> String{
    let rplc: [&str; 6] = ["-", "_", ",","(",")","..."];
    let mut result = s.clone();
    for i in 0..rplc.len() {
        result = result.replace(rplc[i], " ");
    }
    result
}

fn get_fname(stri: String) -> String{
    let mut s = stri.clone();
    let mut i = 0;
    if s.chars().last().unwrap()=='/'{
        s.pop();
    }
    if let Some(n) = s.rfind("/"){
        i = n;
    }
    s[i+1..].to_string()
}

fn rmv_extension(stri: String) -> String{
    let mut s = stri.clone();
    let mut i = 0;
    if s.chars().last().unwrap()=='/'{
        s.pop();
    }
    if let Some(n) = s.rfind("."){
        i = n;
    }
    s[..i].to_string()
}

fn vectorize(stri: String,ext:bool) -> Vec<String> { //create a vector in which every element is a single word from string
    let mut vec: Vec<String> = Vec::new();
    let stri = stri.split(" ");
    for s in stri {
        vec.push(s.to_string());
    }
    if ext {
        let size = vec.len()-1;
        let s = vec[size].clone();
        match s.find(".") {
            Some(n) => {
                let mut slice:String = s[..n].to_string();
                vec[size]=slice;
                slice = s[n..].to_string();
                vec.push(slice);
            }
            None => (),
        }
        mem::forget(s);
    }
    vec
} 

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn compare(mut current:String,prmtr:String,  ext:bool) -> bool {
    if !ext { //If user didn't specified extension, remove it from file being compared
        current = rmv_extension(current);
    }
    if !prmtr.contains(" ") { //Current file is compound of a single word
        if prmtr.to_uppercase()==current.to_uppercase() {
            return true;
        }
        else {
            return false;
        }
    }
    let (p,c) = (vectorize(prmtr,ext),vectorize(current,ext));
    
    if ext && p[p.len()-1]!=c[c.len()-1]{//user gave extension, and the current extension doesn't match.
        return false;
    }
    let (size, mut count):(usize, usize) = (p.len(), 0);
    
    for s1 in c {
        for s2 in &p {
            if s1.to_uppercase()==s2.to_uppercase(){ //case insensitive comparison
                count +=1;
            }
        }
    }
    mem::forget(p);
    if count>=size { 
        return true;
    }
    false
}

fn is_hidden(entry: &DirEntry,h:bool) -> bool {
    if !h{
        return entry.file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false);
    }
    else{
        return false;
    }
}

fn ffinder(base_dir:String, prmtr:&'static str, e:bool, h:bool){ 
    let walker = WalkDir::new(base_dir).follow_links(true).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e,h)).filter_map(|e| e.ok()) {
        let p2 = entry.path().clone();
        if p2.is_file(){
            if compare(rmv_underline(get_fname(p2.display().to_string())),rmv_underline(prmtr.to_string()),e){
                println!("File found at: {}",p2.display().to_string().blue());
            }
        }
    }
}   

fn main(){
    let i = input();
    let inp =string_to_static_str(rmv_underline(i.prmtr));
    ffinder(i.base_dir,inp,i.extension,i.hidden);
}
