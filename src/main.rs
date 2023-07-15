mod git;

use git::git;
use once_cell::sync::Lazy;
use semver::Version;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Commandline {
    #[structopt(flatten)]
    subcommand: Subcommands,

    #[structopt(flatten)]
    root_options: RootOptions,
}

#[derive(StructOpt, Debug)]
struct RootOptions {}

#[derive(StructOpt, Debug)]
enum Subcommands {
    Init(InitOptions),
    Switch(SwitchOptions),
}

#[derive(StructOpt, Debug)]
struct InitOptions {}

#[derive(StructOpt, Debug)]
struct SwitchOptions {}

fn main() {
    let commandline = Commandline::from_args();

    match run(commandline) {
        Ok(()) => exit(0),
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    }
}

fn run(commandline: Commandline) -> Result<(), String> {
    must_have_new_git_command()?;
    must_in_git_repository()?;

    match commandline.subcommand {
        Subcommands::Init(options) => init(commandline.root_options, options),
        Subcommands::Switch(options) => switch(commandline.root_options, options),
    }
}

fn must_have_new_git_command() -> Result<(), String> {
    let output = git(&["--version"])?;

    if !output.status.success() {
        return Err(format!(
            "failed to run 'git --version': {}",
            String::from_utf8(output.stderr).unwrap().trim()
        ));
    }

    static LEAST_GIT_VERSION: Lazy<Version> = Lazy::new(|| Version::parse("2.30.0").unwrap());

    let version_line = String::from_utf8(output.stdout)
        .map_err(|err| format!("git version output is not utf8: {}", err))?;
    let version = Version::parse(
        version_line
            .split_whitespace()
            .nth(2)
            .ok_or(format!("git version output is not valid: {}", version_line))?,
    )
    .map_err(|err| format!("git version output is not valid: {}", err))?;

    if version < *LEAST_GIT_VERSION {
        return Err(format!(
            "git version must be at least {}, but {} is found",
            *LEAST_GIT_VERSION, version
        ));
    }

    Ok(())
}

fn must_in_git_repository() -> Result<(), String> {
    let output = git(&["rev-parse", "--is-inside-work-tree"])?;

    if !output.status.success() {
        return Err("current directory is not a git repository".to_string());
    }

    Ok(())
}

fn init(_root_options: RootOptions, _options: InitOptions) -> Result<(), String> {
    Ok(())
}

fn switch(_root_options: RootOptions, _options: SwitchOptions) -> Result<(), String> {
    Ok(())
}
