use std::{
    io::ErrorKind,
    process::{Command, Output},
};

pub fn git(args: &[&str]) -> Result<Output, String> {
    let commandline = format!("git {}", args.join(" "));
    let mut command = Command::new("git");

    args.iter().for_each(|arg| {
        command.arg(arg);
    });

    let output = command.output().map_err(|err| match err.kind() {
        ErrorKind::NotFound => "git command not found".to_string(),
        _ => format!("failed to run '{}': {}", commandline, err)
            .trim()
            .into(),
    })?;

    Ok(output)
}
