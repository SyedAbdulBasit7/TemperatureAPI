#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::io::Read;

#[get("/")]

pub mod fn CallingAPI()->String{
    let path=Path::new("api.json"); 
    let display= path.display();

    println!("{:?} {:?}",path,display);


    let mut file=match File::create(path){
        Ok(file)=>file,
        Err(_)=>panic!("Could not Create the File"),
    };
    match reqwest::get("api"){ //api link will be goes here
        Ok(mut response)=>{
            match response.text(){
                Ok(text)=> match file.write_all(text.as_byte()){
                    Ok(_)=>println!("Data Write in File"),
                    Err(_)=>panic!("The Error is this")
                },
                Err(_)=>panic!("Server is not responding")
            }
        },
        Err(_)=>panic!("Server could not established the connection")
    }

    let mut file = match File::open(&path){
        Ok(file)=> file,
        Err(_)=>panic!("The file open Error {}")
    };

    let mut buffer =String::new();
    file.read_to_string(&mut buffer).unwrap();

    
    let json= Json::from_str(&buffer).unwrap();

    let result= format!("The Temerature is {}",json.find_path(&["main"]).unwrap());
    result
}


