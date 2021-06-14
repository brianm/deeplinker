use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};
use serde_json;
use serde::de::DeserializeOwned;
use std::io::Cursor;


macro_rules! scripty {
    ($($name:ident),*) => {
        $(
            pub fn $name<T>() -> Result<T, Box<dyn Error>> 
                where T: DeserializeOwned
            {
                let script = include_str!(concat!(stringify!($name), ".applescript"));
                let mut child = Command::new("/usr/bin/osascript")
                    .arg("-s")
                    .arg("s")
                    .arg("-")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;
                let input = child.stdin.as_mut().unwrap();
                input.write_all(script.as_bytes())?;
                drop(input); // close input to allow child to finish reading

                let output = child.wait_with_output()?;
                let out = output.stdout;
                let r: T = serde_json::from_reader(Cursor::new(out))?;
                Ok(r)
            }
        )*
    };
}

scripty!(front_app, chrome, safari);
