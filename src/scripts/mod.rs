use json5;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::from_utf8;

macro_rules! scripty {
    ($($name:ident),*) => {
        $(
            #[allow(non_snake_case)]
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
                let r: T = json5::from_str(&from_utf8(&output.stdout)?)?;
                Ok(r)
            }
        )*
    };
}

scripty!(front_app, com_google_Chrome, com_apple_Safari);
