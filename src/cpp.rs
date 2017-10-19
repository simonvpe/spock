use tera::{Tera, Context};
use regex::Regex;
use std::fs::{create_dir_all,File};
use std::io::prelude::*;
use std::path::Path;
use git;

pub fn create(dir: &str, tmpl_dir: &str, name: &str, testing: &str) {

    let ctx  = context(name, testing);

    println!("Checking for git executable");
    if !git::check_executable().success() {
        panic!("Could not find git executable");
    }

    println!("Creating project directory");
    create_dir_all(dir).unwrap();

    
    println!("Checking destination");
    if git::check_git_repository(dir).success() {
        panic!("Seems to already be a valid git repository");
    }

    println!("Generating files...");

    let tera = tera(tmpl_dir, testing);

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
}


fn get_destination(template: &str) -> String {
    let re = Regex::new(r"^(?P<dst>[\w./-]+)\.[\w]+$").unwrap();
    let caps = re.captures(template).unwrap();
    caps["dst"].into()
}

fn write(content: &String, dst: &str) {
    let dst = Path::new(dst);
    println!("...generating {}", dst.to_str().unwrap());
    create_dir_all(dst.parent().unwrap()).unwrap();
    let mut file = File::create(dst).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn tera(tmpl_dir: &str, testing: &str) -> Tera {
    let glob_all = tmpl_dir.to_owned() + "/cpp/**/*.all";
    let glob_test = tmpl_dir.to_owned() + "/cpp/**/*.test";

    let mut tera = compile_templates!(&glob_all[..]);
    let test_tera = compile_templates!(&glob_test[..]);
    if testing != "" {
        tera.extend( &test_tera );
    }
    tera
}

fn context(name: &str, testing: &str) -> Context {
   let mut context = Context::new();
    context.add("name", &name);
    context.add("testing", &testing);
    context
}
