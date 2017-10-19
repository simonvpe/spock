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

### Arch Linux

1. Download the PKGBUILD file and from the same directory run
   ``` bash
   makepkg -i
   ```

### Other Linux Platforms

1. Install rust

2. Install using cargo
   ``` bash
   cargo install --git https://github.com/simonvpe/spock
   ```
   
3. Add the stuff to your path as instructed by the installation

4. Clone the repo and install the templates
   ``` bash
   git clone https://github.com/simonvpe/spock && cd spock
   sudo install -d -m 755 /usr/share/spock
   sudo cp -R templates/cpp /usr/share/spock/
   sudo chmod -R 755 /usr/share/spock
   
Contributing
------------

Pull requestes are accepted. The most appreciated PR's are the following
- Enhancing the interface of the `cpp` module.
- Error handling
