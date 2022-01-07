#[cfg(not(windows))]
use rustix::fd::{FromRawFd, IntoRawFd};
use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::Command;

mod inter;

pub use inter::{InterType, InterTypeable, InterVal};

pub enum Convention {
    Implicit,
}

#[inline]
pub fn make_command<S: AsRef<OsStr>>(
    program: S,
    args: &[InterVal],
    envs: &[(String, InterVal)],
    convention: Convention,
) -> Command {
    let program = program.as_ref();
    _make_command(program, args, envs, convention)
}
fn _make_command(
    program: &OsStr,
    args: &[InterVal],
    envs: &[(String, InterVal)],
    convention: Convention,
) -> Command {
    let mut command = Command::new(program);
    let mut fds = Vec::new();
    let mut next_fd = 3; // For 0, 1, and 2, use `Command::stdin`, `stdout`, and `stderr`.

    // This is just a placeholder for future ideas.
    match convention {
        Convention::Implicit => {}
    }

    for arg in args {
        match arg {
            InterVal::S8(i) => command.arg(i.to_string()),
            InterVal::U8(i) => command.arg(i.to_string()),
            InterVal::S16(i) => command.arg(i.to_string()),
            InterVal::U16(i) => command.arg(i.to_string()),
            InterVal::S32(i) => command.arg(i.to_string()),
            InterVal::U32(i) => command.arg(i.to_string()),
            InterVal::S64(i) => command.arg(i.to_string()),
            InterVal::U64(i) => command.arg(i.to_string()),
            InterVal::F32(f) => {
                command.arg(f.map(|f| f.to_string()).unwrap_or_else(|| "NaN".to_owned()))
            }
            InterVal::F64(f) => {
                command.arg(f.map(|f| f.to_string()).unwrap_or_else(|| "NaN".to_owned()))
            }
            InterVal::Bool(b) => command.arg(b.to_string()),
            InterVal::Char(c) => command.arg(c.to_string()),
            InterVal::String(s) => command.arg(&**s),
            InterVal::Handle(handle) => {
                fds.push((handle.clone(), next_fd));
                command.arg(next_fd.to_string());
                next_fd += 1;
                &mut command
            }
            other => todo!("arg lowering for {:?}", other),
        };
    }

    for (key, val) in envs {
        match val {
            InterVal::S8(i) => command.env(key, i.to_string()),
            InterVal::U8(i) => command.env(key, i.to_string()),
            InterVal::S16(i) => command.env(key, i.to_string()),
            InterVal::U16(i) => command.env(key, i.to_string()),
            InterVal::S32(i) => command.env(key, i.to_string()),
            InterVal::U32(i) => command.env(key, i.to_string()),
            InterVal::S64(i) => command.env(key, i.to_string()),
            InterVal::U64(i) => command.env(key, i.to_string()),
            InterVal::F32(f) => command.env(
                key,
                f.map(|f| f.to_string()).unwrap_or_else(|| "NaN".to_owned()),
            ),
            InterVal::F64(f) => command.env(
                key,
                f.map(|f| f.to_string()).unwrap_or_else(|| "NaN".to_owned()),
            ),
            InterVal::Bool(b) => command.env(key, b.to_string()),
            InterVal::Char(c) => command.env(key, c.to_string()),
            InterVal::String(s) => command.env(key, &**s),
            InterVal::Handle(handle) => {
                fds.push((handle.clone(), next_fd));
                command.env(key, next_fd.to_string());
                next_fd += 1;
                &mut command
            }
            other => todo!("env lowering for {:?}", other),
        };
    }

    if !fds.is_empty() {
        #[cfg(unix)]
        unsafe {
            command.pre_exec(move || {
            for (fd, target) in &fds {
                let target = rustix::io::OwnedFd::from_raw_fd(*target);

                // FIXME: Handle the case where targets alias fds.
                #[cfg(not(windows))]
                rustix::io::dup2(&**fd, &target)?;
                #[cfg(windows)]
                todo!("what we really want is OwnedFd::try_clone; finish https://github.com/rust-lang/rust/pull/88794");

                // Intentionally leak into the child.
                let _ = target.into_raw_fd();
            }
            Ok(())
        });
        }

        #[cfg(not(unix))]
        todo!("passing fds on non-unix")
    }

    command
}
