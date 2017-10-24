use tera::{Tera, Context};
use regex::Regex;
use std::fs::{create_dir_all,File};
use std::io::prelude::*;
use std::path::Path;
use git;

pub fn create(dir: &str, tmpl_dir: &str, name: &str, testing: &str, exec: bool, lib: bool) {

    let ctx  = context(name, testing, exec, lib);

    println!("Checking for git executable");
    if !git::check_executable().success() {
        panic!("Could not find git executable");
    }

    println!("Creating project directory");
    create_dir_all(dir).unwrap();

    println!("Generating files...");

    let tera = tera(tmpl_dir, testing, exec, lib);

    tera.templates.keys().for_each(|x| {
        let entry = tera.templates[x].clone();
        let name = &entry.name[..];
        let dst = dir.to_owned() + "/" + &get_destination(name);
        match tera.render(&entry.name[..], &ctx) {
            Ok(ref tmpl) => write(tmpl, &dst),
            Err(e)       => panic!("{:?}", e)
        };
    });
    
    println!("Initializing git repository");
    if !git::init(dir).success() {
        panic!("Error: Could not initialize git repository in {:?}", dir);
    }

    if testing != "" {
        println!("Adding catch submodule");
        if !git::submodule_add(
            dir,
            "https://github.com/philsquared/Catch.git",
            "test/catch").success() {
            panic!("Failed to add catch submodule");
        }
    }

    println!("Adding all files");
    if !git::add(dir, "*").success() {
        panic!("Failed to add files");
    }

    println!("Committing");
    if !git::commit(dir, "Initial commit").success() {
        panic!("Failed to commit");
    }
}


fn get_destination(template: &str) -> String {
    let re = Regex::new(r"^(?P<dst>[\w./-]+)\.[\w]+$").unwrap();
    let caps = re.captures(template).unwrap();
    caps["dst"].into()
}

fn write(content: &str, dst: &str) {
    let dst = Path::new(dst);
    println!("...generating {}", dst.to_str().unwrap());
    create_dir_all(dst.parent().unwrap()).unwrap();
    let mut file = File::create(dst).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn tera(tmpl_dir: &str, testing: &str, exec: bool, lib: bool) -> Tera {
    let tmpl_dir  = tmpl_dir.to_owned();
    let glob_all  = tmpl_dir.clone() + "/cpp/**/*.all";
    let glob_test = tmpl_dir.clone() + "/cpp/**/*.test";
    let glob_exec = tmpl_dir.clone() + "/cpp/**/*.exec";
    let glob_lib  = tmpl_dir.clone() + "/cpp/**/*.lib";
    
    let mut tera  = compile_templates!(&glob_all[..]);
    let test_tera = compile_templates!(&glob_test[..]);
    let exec_tera = compile_templates!(&glob_exec[..]);
    let lib_tera  = compile_templates!(&glob_lib[..]);
    
    if testing != "" {
        match tera.extend( &test_tera ) {
            Ok(_) => {},
            Err(e) => { panic!("{:?}", e); }
        }
    }

    if exec {
        match tera.extend( &exec_tera ) {
            Ok(_) => {},
            Err(e) => { panic!("{:?}", e); }
        }
    }
    
    if lib {
        match tera.extend( &lib_tera ) {
            Ok(_) => {},
            Err(e) => { panic!("{:?}", e); }
        }
    }
    tera
}

fn context(name: &str, testing: &str, exec: bool, lib: bool) -> Context {
   let mut context = Context::new();
    context.add("name", &name);
    context.add("testing", &testing);
    context.add("executable", &exec);
    context.add("library", &lib);
    context
}
