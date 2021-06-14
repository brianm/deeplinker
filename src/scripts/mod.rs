use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::from_utf8;

macro_rules! scripty {
    ($($name:ident),*) => {
        $(
            pub fn $name() -> Result<String, Box<dyn Error>> {
                let script = include_str!(concat!(stringify!($name), ".applescript"));
                let mut child = Command::new("/usr/bin/osascript")
                    .arg("-")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;
                // .expect("failed to execute process");
                let child_stdin = child.stdin.as_mut().unwrap();
                child_stdin.write_all(script.as_bytes())?;
                // Close stdin to finish and avoid indefinite blocking
                drop(child_stdin);

                let output = child.wait_with_output()?;
                let out = output.stdout;
                Ok(String::from(from_utf8(&out)?.trim()))
            }
        )*
    };
}

scripty!(front_app, chrome, safari);
