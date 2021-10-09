use anyhow::Result;
use std::{
    env::args,
    io::{Read, Write},
    process::{exit, Command, Stdio},
};

fn main() -> Result<()> {
    let mut args = args();
    args.next().unwrap();
    let program = match args.next() {
        Some(program) => program,
        None => {
            eprintln!("You must provide at least one argument");
            exit(1);
        }
    };
    let args = args.collect::<Vec<_>>();

    let located = Command::new("pacman")
        .arg("-Fq")
        .arg(format!("/usr/bin/{}", program))
        .output()?;

    let stdout = String::from_utf8(located.stdout)?;
    let mut choices = stdout.lines().collect::<Vec<_>>();

    let choice = match choices.len() {
        0 => {
            eprintln!("Could not find a matching package");
            exit(1);
        }
        1 => choices[0].to_string(),
        _ => {
            let mut proc = Command::new("fzf")
                .args(&["--preview", "pacman -Si {}", "--info", "hidden"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;
            let mut stdin = proc.stdin.take().unwrap();
            choices.push("");
            stdin.write(&choices.join("\n").into_bytes())?;

            let mut output = Vec::new();
            proc.stdout.take().unwrap().read_to_end(&mut output)?;
            let status = proc.wait()?;
            if !status.success() {
                exit(1);
            }
            let choice = String::from_utf8(output)?;
            choice[0..choice.len() - 1].to_string()
        }
    };

    let package = {
        let mut iter = choice.split("/");
        iter.next().unwrap();
        iter.next().unwrap()
    };

    let was_installed = Command::new("pacman")
        .arg("-Q")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg(package)
        .status()?
        .success();
    if !was_installed {
        if !Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg(package)
            .status()?
            .success()
        {
            exit(1);
        };
    }

    let status = Command::new(program).args(args).status()?;

    if !was_installed {
        Command::new("sudo")
            .arg("pacman")
            .arg("-R")
            .arg("--noconfirm")
            .arg(package)
            .status()?;
    }

    exit(status.code().unwrap_or(1));
}
