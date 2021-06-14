use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use json5;
use std::str::from_utf8;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeepLink {
    pub link: String,
    pub title: String
}

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
                let r: T = json5::from_str(&from_utf8(&out)?)?;
                Ok(r)
            }
        )*
    };
}

scripty!(front_app, chrome, safari);
