#[macro_use]
extern crate tera;
extern crate clap;
extern crate regex;

use clap::{App, Arg, SubCommand};

mod cpp;
mod git;

fn main() {
    let matches = App::new("spock")
        .about("A package manager")
        .author("Simon Pettersson <simon.v.pettersson@gmail.com>")
        .arg(Arg::with_name("templates")
             .help("the location of the templates to use")
             .takes_value(true)
             .long("templates")
             .short("t"))
        .subcommand(SubCommand::with_name("create")
                    .about("creates a new package")
                    .arg(Arg::with_name("lang")
                         .help("the language to use")
                         .required(true)
                         .index(1)
                    )
                    .arg(Arg::with_name("name")
                         .help("the name of the project")
                         .required(true)
                         .index(2)
                    )
                    .arg(Arg::with_name("testing")
                         .help("use a test framework")
                         .takes_value(true)
                         .long("testing")
                    )
                    .arg(Arg::with_name("dir")
                         .help("the location of the project")
                         .takes_value(true)
                         .long("dir")
                         .short("C"))
                    .arg(Arg::with_name("exec")
                         .help("this project is an executable")
                         .takes_value(false)
                         .long("exec")
                         .short("e")
                         .required(true)
                         .conflicts_with("lib"))
                    .arg(Arg::with_name("lib")
                         .help("this project is a library")
                         .takes_value(false)
                         .long("lib")
                         .short("l")
                         .required(true)
                         .conflicts_with("exec"))
        )
        .get_matches();
    
    let evt = match matches.subcommand() {
        
        ("create", Some(sub_matches)) => Event::Create {
            dir:  sub_matches.value_of("dir").unwrap_or("./"),
            tmpl_dir: matches.value_of("templates").unwrap_or("/usr/share/spock"),
            lang: sub_matches.value_of("lang").unwrap(),
            name: sub_matches.value_of("name").unwrap(),
            testing: match sub_matches.value_of("testing") {
                Some(x) => x,
                None => ""
            },
            exec: sub_matches.is_present("exec"),
            lib: sub_matches.is_present("lib")
        },
        (&_, _) => Event::Invalid
    };

    handle_event(evt);
}

enum Event<'a> {
    Create {
        dir: &'a str,
        tmpl_dir: &'a str,
        lang: &'a str,
        name: &'a str,
        testing: &'a str,
        exec: bool,
        lib: bool
    },
    Invalid
}

fn handle_event(evt: Event) {
    match evt {
        Event::Create { dir, tmpl_dir, lang, name, testing, exec, lib } => {
            match lang {
                "cpp" | "c++" | "C++" => {
                    cpp::create(dir, tmpl_dir, name, testing, exec, lib);
                },
                _ => println!("Bad language \"{}\"", lang)
            }
            
        },

        Event::Invalid => {
            println!("Invalid subcommand");
        }
    }
}
