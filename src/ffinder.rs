use std::env;
use std::mem;
use colored::*;
use std::process::exit;
use std::thread;



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

    let (h, mut count):(f32, u8) = ((c.len() as f32 / 2.0).round(), 0);
    let half = h as u8;
    
    for s1 in c {
        for s2 in &p {
            if s1.to_uppercase()==s2.to_uppercase(){ //case insensitive comparison
                count +=1;
            }
        }
    }

    mem::forget(p);
    if count>half { 
        return true;
    }
    false
}


fn ffinder(base_dir:String, prmtr:&'static str, e:bool, h:bool) -> std::io::Result<()>{ // prmtr:String
    let mut handle_vec = vec![];
    let pth = std::fs::read_dir(&base_dir)?;
    for p in pth {
        let p2 = p?.path().clone();
        if p2.is_dir() {
            if !h{ //search doesn't include hidden directories
                let sstring:String = get_fname(p2.display().to_string());
                let slice:String = sstring[..1].to_string();
                if slice != ".".to_string() {
                    let handle = thread::spawn(move || {
                        ffinder(p2.display().to_string(),prmtr,e,h);
                    });
                    handle_vec.push(handle);
                    
                }
            }
            else {//search include hidden directories
                let handle2 = thread::spawn(move || {
                    ffinder(p2.display().to_string(),prmtr,e,h);
                });
                handle_vec.push(handle2);
            } 
        }
        else {
            if compare(rmv_underline(get_fname(p2.display().to_string())),rmv_underline(prmtr.to_string()),e){
                let handle3 = thread::spawn(move || {
                    if compare(rmv_underline(get_fname(p2.display().to_string())),rmv_underline(prmtr.to_string()),e){
                        println!("File found at: {}",p2.display().to_string().blue());
                    }
                });
                handle_vec.push(handle3);
            }
        }
    }
    for h in handle_vec{
        h.join().unwrap();
    }
    Ok(())
}


fn main(){
    let i = input();
    ffinder(i.base_dir,string_to_static_str(i.prmtr),i.extension,i.hidden);
}
