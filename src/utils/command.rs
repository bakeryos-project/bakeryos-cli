use std::process::{Command, Stdio};

pub fn create_command<S, A, S2>(program: S, args: A) -> Command
where
    S: AsRef<std::ffi::OsStr>,
    A: AsRef<[S2]>,
    S2: AsRef<std::ffi::OsStr>,
{
    let mut cmd = Command::new(program);
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    for arg in args.as_ref() {
        cmd.arg(arg);
    }

    cmd
}
