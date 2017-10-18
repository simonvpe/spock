use tera::{Tera, Context};
use regex::Regex;
use std::fs::{create_dir_all,File};
use std::io::prelude::*;
use std::path::Path;

use git;

pub fn create(dir: &str, tmpl_dir: &str, name: &str, testing: &str) {
    let templates = vec![
        ("cpp/CMakeLists.txt.all", indoc!("
            cmake_minimum_required(VERSION 3.8)
            project({{ name }})

            if(NOT CMAKE_BUILD_TYPE)
                set(CMAKE_BUILD_TYPE Release)
            endif()

            set(CMAKE_CXX_FLAGS \"-Wall -Wextra\")
            set(CMAKE_CXX_FLAGS_DEBUG \"-g\")
            set(CMAKE_CXX_FLAGS_RELEASE \"-O3\")

            add_subdirectory(src)
            {% if testing != \"\" %}
            add_subdirectory(test)
            {% endif %}
        ")),   
        ("cpp/test/CMakeLists.txt.test", indoc!("
            enable_testing()

            # Prepare Catch library for other executables
            set(CATCH_INCLUDE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/Catch/include)
            set(MR_INCLUDE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../include)

            add_library(Catch INTERFACE)

            target_include_directories(Catch INTERFACE ${CATCH_INCLUDE_DIR})
            target_include_directories(Catch INTERFACE ${MR_INCLUDE_DIR})

            # Make test executable
            set(TEST_SOURCES 
                ${CMAKE_CURRENT_SOURCE_DIR}/testrunner.cpp
                ${CMAKE_CURRENT_SOURCE_DIR}/example.cpp
            )
            add_executable(tests ${TEST_SOURCES})
            target_link_libraries(tests Catch)
            target_compile_features(tests PRIVATE cxx_std_17)

            add_test(all tests)
        ")),
        ("cpp/test/testrunner.cpp.test", indoc!("
            #define CATCH_CONFIG_MAIN
            #include \"catch.hpp\"
        ")),
        ("cpp/test/example.cpp.test", indoc!("
            #include \"catch.hpp\"
            SCENARIO(\"Test Scenario\") {
                 GIVEN(\"an int\") {
                     int x = 5;
                     THEN(\"check that it is 5\") {
                         CHECK( x == 5 );
                     }
                 }
            }
        "))
    ];

    let dir  = Path::new(dir);
    let tera = tera(&templates);
    let ctx  = context(name, testing);

    println!("Checking for git executable");
    if !git::check_executable().success() {
        panic!("Could not find git executable");
    }

    println!("Creating project directory");
    create_dir_all(dir.to_str().unwrap()).unwrap();

    println!("Checking destination");
    if git::check_git_repository(dir.to_str().unwrap()).success() {
        panic!("Seems to already be a valid git repository");
    }

    println!("Generating files...");
    
    templates.iter()
        .filter(|x| testing != "" || get_tag(x.0) != "test")
        .for_each(|x| {
            let src = x.0;
            let sdir = dir.to_str().unwrap().to_owned() + "/";            
            let path = x.0.to_string().replace("cpp/", &sdir);
            let dst = sdir + &get_destination(x.0);

            match tera.render(&src, &ctx) {
                Ok(ref x) => write(x, Path::new(&dst)),
                Err(e)    => panic!("{:?}", e)
            }
        });

    println!("Initializing git repository");
    if !git::init(dir.to_str().unwrap()).success() {
        panic!("Error: Could not initialize git repository in {:?}", dir);
    }

    if testing != "" {
        println!("Adding catch submodule");
        if !git::submodule_add(
            dir.to_str().unwrap(),
            "https://github.com/philsquared/Catch.git",
            "test/catch").success() {
            panic!("Failed to add catch submodule");
        }
    }
}

fn get_tag(template: &str) -> String {
    let re = Regex::new(r"^cpp/[\w./-]+\.(?P<tag>[\w]+)$").unwrap();
    let caps = re.captures(template).unwrap();
    caps["tag"].into()
}

fn get_destination(template: &str) -> String {
    let re = Regex::new(r"^cpp/(?P<dst>[\w./-]+)\.[\w]+$").unwrap();
    let caps = re.captures(template).unwrap();
    caps["dst"].into()
}

fn write(content: &String, dst: &Path) {
    println!("...generating {}", dst.to_str().unwrap());
    create_dir_all(dst.parent().unwrap()).unwrap();
    let mut file = File::create(dst).unwrap();
    file.write_all(content.as_bytes());
}

fn tera(templates: &Vec<(&str,&str)>) -> Tera {
   let mut tera = Tera::default();
    match tera.add_raw_templates(templates.clone()) {
        Err(e) => panic!("ERROR {}", e),
        _ => {}
    }
    tera
}

fn context(name: &str, testing: &str) -> Context {
   let mut context = Context::new();
    context.add("name", &name);
    context.add("testing", &testing);
    context
}
