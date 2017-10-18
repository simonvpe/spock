extern crate tera;
extern crate clap;
#[macro_use]
extern crate indoc;
extern crate regex;

use clap::{App, Arg, SubCommand};

mod cpp;
mod git;

fn main() {
    let matches = App::new("spock")
        .about("A package manager")
        .author("Simon Pettersson <simon.v.pettersson@gmail.com>")
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
        )
        .get_matches();

    let evt = match matches.subcommand() {
        
        ("create", Some(sub_matches)) => Event::Create {
            dir:  sub_matches.value_of("dir").unwrap_or("./"),
            lang: sub_matches.value_of("lang").unwrap(),
            name: sub_matches.value_of("name").unwrap(),
            testing: match sub_matches.value_of("testing") {
                Some(x) => x,
                None => ""
            }
        },
        
        (&_, _) => Event::Invalid
    };

    handle_event(evt);
}

enum Event<'a> {
    Create { dir: &'a str, lang: &'a str, name: &'a str, testing: &'a str },
    Invalid
}

fn handle_event(evt: Event) {
    match evt {
        Event::Create { dir, lang, name, testing } => {
            match lang {
                "cpp" | "c++" | "C++" => {
                    cpp::create(dir, name, testing);
                },
                _ => println!("Bad language \"{}\"", lang)
            }
            
        },

        Event::Invalid => {
            println!("Invalid subcommand");
        }
    }
}
