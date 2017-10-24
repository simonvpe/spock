#[macro_use] extern crate assert_cli;
extern crate tempdir;

static WITHOUT_PROJECT_NAME_OUTPUT: &'static str
    = "error: The following required arguments were not provided:
    <name>
    --exec
    --lib

USAGE:
    spock create [FLAGS] [OPTIONS] <lang> <name> --exec --lib

For more information try --help
";

static WITHOUT_EXEC_LIB_OUTPUT: &'static str
    = "error: The following required arguments were not provided:
    --exec
    --lib

USAGE:
    spock create [FLAGS] [OPTIONS] <lang> <name> --exec --lib

For more information try --help
";

#[cfg(test)]
mod cpp {
    use assert_cli::Assert;
    use tempdir;
    use std::path::Path;
    use WITHOUT_PROJECT_NAME_OUTPUT;
    use WITHOUT_EXEC_LIB_OUTPUT;
    
    #[test]
    fn create_without_project_name() {
        Assert::main_binary()
            .with_args(&["create", "cpp"])
            .fails()
            .and()
            .stderr()
            .contains(WITHOUT_PROJECT_NAME_OUTPUT)
            .unwrap()
    }
    
    #[test]
    fn create_without_exec_lib() {
        Assert::main_binary()
            .with_args(&["create", "cpp", "proj"])
            .fails()
            .and()
            .stderr()
            .contains(WITHOUT_EXEC_LIB_OUTPUT)
            .unwrap()
    }

    #[test]
    fn create_exec_without_tests() {
        let src_dir = tempdir::TempDir::new("spock").unwrap();
        let src_path = src_dir.path().to_str().unwrap();

        let cmd = &["target/debug/spock",
                    "--templates", "./templates",
                    "create", "-C", src_path, "cpp", "exec-proj", "--exec"];
        Assert::command(cmd).unwrap();

        assert_is_git_repo(src_path);
        assert_no_untracked_files(src_path);

        let build_dir = tempdir::TempDir::new("spock-build").unwrap();
        let build_path = build_dir.path().to_str().unwrap();
        
        assert_cmake(src_path, build_path);
        assert_make(build_path);

        assert_run_executable(build_path);
    }

    fn assert_is_git_repo(path: &str) {
        Assert::command(&["git", "-C", path, "status"]).unwrap();
    }

    fn assert_no_untracked_files(path: &str) {
        Assert::command(&["git", "-C", path, "status", "--porcelain"])
            .stdout().is("")
            .unwrap();
    }

    fn assert_cmake(src_path: &str, build_path: &str) {
        Assert::command(&["cmake", src_path])
            .current_dir(build_path)
            .unwrap();
    }

    fn assert_make(build_path: &str) {
        Assert::command(&["make", "all"])
            .current_dir(build_path)
            .unwrap();
    }

    fn assert_run_executable(build_path: &str) {
        Assert::command(&["./src/exec-proj"])
            .current_dir(build_path)
            .unwrap();
    }
}
