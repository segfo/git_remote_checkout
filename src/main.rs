use std::process::Command;
mod cmdargs;
use cmdargs::*;
use std::env;
use std::io::Write;
fn main() {
    let arg = init_args();
    let workdir = env::current_dir().unwrap();
    exec_git(["fetch", "origin", arg.branch_name()].to_vec());
    exec_git(["checkout", arg.branch_name()].to_vec());
}

fn exec_git(git_args: Vec<&str>) {
    let cmd_output = Command::new("git")
        .args(&git_args)
        .output()
        .expect(&format!("gitコマンド({})が失敗しました", git_args[0]));
    std::io::stdout().write_all(&cmd_output.stdout).unwrap();
    std::io::stderr().write_all(&cmd_output.stderr).unwrap();
}
