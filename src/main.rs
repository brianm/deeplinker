use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::from_utf8;
fn main() -> Result<(), Box<dyn Error>> {
    
    let app_name = front_app()?;
    println!("'{}'", app_name);

    let output = match &app_name[..] {
        "Safari" | "Webkit" => safari()?,
        "Google Chrome" |"Google Chrome Canary" | "Chromium" => chrome()?,
        _ => String::from("unknown application"),
    };

    println!("{}", output);

    Ok(())
}

// fn front_app() -> Result<String, Box<dyn Error>> {
//     let script = include_str!("scripts/front_app.applescript");
//     let mut child = Command::new("/usr/bin/osascript")
//         .arg("-")
//         .stdin(Stdio::piped())
//         .stdout(Stdio::piped())
//         .spawn()?;

//     let child_stdin = child.stdin.as_mut().unwrap();
//     child_stdin.write_all(script.as_bytes())?;

//     drop(child_stdin);

//     let output = child.wait_with_output()?;
//     let out = output.stdout;
//     Ok(String::from(from_utf8(&out)?.trim()))
// }

macro_rules! scripty {
    ($($name:ident),*) => {
        $(
        fn $name() -> Result<String, Box<dyn Error>> {
            let script = include_str!(concat!("scripts/", stringify!($name), ".applescript"));
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
