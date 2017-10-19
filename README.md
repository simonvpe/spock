spock
=========

Spock is a project generator for C++. It generates an *standard* shared library or executable project using CMake as its build tool. The project generation is configurable through commandline options.

Internals
---------

The core of spock is written in [Rust](https://github.com/rust-lang/rust), combined with the [Tera](https://github.com/Keats/tera) template engine and the [Clap](https://github.com/kbknapp/clap-rs) command line argument parser.

Every file generated is a [Tera](https://github.com/Keats/tera) template. Each template has an extension which is used to decide whether or not the template should be generated into the project depending on the command line options. The extensions are:

* **all** *is included in every project*
* **test** *is included if a test suite was selected*
* **lib** *is included if the project is a library*
* **exec** *is included if the project is an executable*

Installation
------------

For now you need to clone the repository and build it yourself using Cargo (this is described in the Cargo documentation).

Contributing
------------

Pull requestes are accepted. The most appreciated PR's are the following
- Enhancing the interface of the `cpp` module.
- Error handling
