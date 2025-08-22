// pind - A simple keyboard shortcut runner in Rust
// 
// This program reads key events from all connected keyboards and executes
// configured shell commands when a matching key combination is pressed.
//
// Features:
// - Supports modifier keys (Ctrl, Shift, Alt, Meta) and common keys.
// - Loads key bindings from a configuration file (/etc/pind/pindrc).
// - Runs commands as a specified user using `runuser`.
// - Detects multiple keyboards and avoids duplicates.
// - Non-blocking key polling with a configurable delay (25ms by default).
//
// Usage:
//   sudo ./pind [optional:user]   # user argument specifies which user to run commands as
//
// Config file format (/etc/pind/pindrc):
//   # Lines starting with '#' are comments
//   <key_combination>:<command>
//   Example:
//     C+E:echo "Ctrl+Enter pressed"
//
// Dependencies:
// - evdev crate for reading keyboard input
// - std for filesystem, environment, threading, and process handling



use evdev::{AttributeSet, Device, KeyCode, enumerate};
use std::{env::{var,args},fs::read_to_string, path::PathBuf, process::{exit,Command, Stdio}, thread, time::Duration};

const DELAY  :u64  =  25; // 25ms
const CONFIG :&str =  "/etc/pind/pindrc";

fn error(title: &str, message: &str) 
{
    println!("[\x1b[33mE\x1b[0m] \x1b[31m{title}:\x1b[0m {message}.");
}

fn run(command: &str,  user: &str) 
{
    let shell = var("SHELL").unwrap_or_else(|_| "sh".to_string());
    Command::new("runuser")
        .arg("-u")
        .arg(user)
        .arg("--")
        .arg(&shell)
        .arg("-c")
        .arg(command)
        // .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn().unwrap_or_else(|e| {
            error("Command error: {}", &e.to_string());
            exit(1);
        });
}

fn key_to_keycode(input: String) -> AttributeSet<KeyCode> 
{
    let mut attribute_set = AttributeSet::new();

    for ch in input.chars() {
        match ch {
            'M' => attribute_set.insert(KeyCode::KEY_LEFTMETA),
            'C' => attribute_set.insert(KeyCode::KEY_LEFTCTRL),
            'S' => attribute_set.insert(KeyCode::KEY_LEFTSHIFT),
            'A' => attribute_set.insert(KeyCode::KEY_LEFTALT),
            'K' => attribute_set.insert(KeyCode::KEY_CAPSLOCK),
            '/' => attribute_set.insert(KeyCode::KEY_SLASH),
            '\\'=> attribute_set.insert(KeyCode::KEY_BACKSLASH),
            'T' => attribute_set.insert(KeyCode::KEY_TAB),
            'E' => attribute_set.insert(KeyCode::KEY_ENTER),
            '0' => attribute_set.insert(KeyCode::KEY_0),
            '1' => attribute_set.insert(KeyCode::KEY_1),
            '2' => attribute_set.insert(KeyCode::KEY_2),
            '3' => attribute_set.insert(KeyCode::KEY_3),
            '4' => attribute_set.insert(KeyCode::KEY_4),
            '5' => attribute_set.insert(KeyCode::KEY_5),
            '6' => attribute_set.insert(KeyCode::KEY_6),
            '7' => attribute_set.insert(KeyCode::KEY_7),
            '8' => attribute_set.insert(KeyCode::KEY_8),
            '9' => attribute_set.insert(KeyCode::KEY_9),
            'a' => attribute_set.insert(KeyCode::KEY_A),
            'b' => attribute_set.insert(KeyCode::KEY_B),
            'c' => attribute_set.insert(KeyCode::KEY_C),
            'd' => attribute_set.insert(KeyCode::KEY_D),
            'e' => attribute_set.insert(KeyCode::KEY_E),
            'f' => attribute_set.insert(KeyCode::KEY_F),
            'g' => attribute_set.insert(KeyCode::KEY_G),
            'h' => attribute_set.insert(KeyCode::KEY_H),
            'i' => attribute_set.insert(KeyCode::KEY_I),
            'j' => attribute_set.insert(KeyCode::KEY_J),
            'k' => attribute_set.insert(KeyCode::KEY_K),
            'l' => attribute_set.insert(KeyCode::KEY_L),
            'm' => attribute_set.insert(KeyCode::KEY_M),
            'n' => attribute_set.insert(KeyCode::KEY_N),
            'o' => attribute_set.insert(KeyCode::KEY_O),
            'p' => attribute_set.insert(KeyCode::KEY_P),
            'q' => attribute_set.insert(KeyCode::KEY_Q),
            'r' => attribute_set.insert(KeyCode::KEY_R),
            's' => attribute_set.insert(KeyCode::KEY_S),
            't' => attribute_set.insert(KeyCode::KEY_T),
            'u' => attribute_set.insert(KeyCode::KEY_U),
            'v' => attribute_set.insert(KeyCode::KEY_V),
            'w' => attribute_set.insert(KeyCode::KEY_W),
            'x' => attribute_set.insert(KeyCode::KEY_X),
            'y' => attribute_set.insert(KeyCode::KEY_Y),
            'z' => attribute_set.insert(KeyCode::KEY_Z),
            '.' => attribute_set.insert(KeyCode::KEY_DOT),
            ',' => attribute_set.insert(KeyCode::KEY_COMMA),
            ';' => attribute_set.insert(KeyCode::KEY_SEMICOLON),
            '\''=> attribute_set.insert(KeyCode::KEY_APOSTROPHE),
            '[' => attribute_set.insert(KeyCode::KEY_LEFTBRACE),
            ']' => attribute_set.insert(KeyCode::KEY_RIGHTBRACE),
            '-' => attribute_set.insert(KeyCode::KEY_MINUS),
            '=' => attribute_set.insert(KeyCode::KEY_EQUAL),
            '`' => attribute_set.insert(KeyCode::KEY_GRAVE),
            'U' => attribute_set.insert(KeyCode::KEY_UP),
            'D' => attribute_set.insert(KeyCode::KEY_DOWN),
            'L' => attribute_set.insert(KeyCode::KEY_LEFT),
            'R' => attribute_set.insert(KeyCode::KEY_RIGHT),
            _ => continue,
        }
    }
    attribute_set
}

fn is_keyboard(path: &PathBuf) -> bool 
{
    let dev = Device::open(path).unwrap();
    if let Some(keys) = dev.supported_keys() {
        if keys.contains(KeyCode::KEY_A) && keys.contains(KeyCode::KEY_ENTER) {
            return true;
        }
    }
    false
}

fn keyboards() -> Vec<PathBuf> 
{
    let mut devices: Vec<PathBuf> = vec![];

    for (path, _) in enumerate() {
        if is_keyboard(&path) {
            devices.push(path);
        }
    }
    devices
}


fn load_config(config:&str) -> Vec<(AttributeSet<KeyCode>, String)> 
{
    let mut binding: Vec<(AttributeSet<KeyCode>, String)> = vec![];
    let content = read_to_string(config).unwrap_or_else(|e| {
        error("config file", &e.to_string());
        exit(1)
    });
    for line in content.lines() {
        if !line.starts_with("#") && !line.trim().is_empty() {
            let mut l = line.splitn(2, ":");
            let key = l
                .next()
                .unwrap_or_else(|| {
                    error("Binding", "missing key");
                    exit(1)
                })
                .to_string();
            let cmd = l
                .next()
                .unwrap_or_else(|| {
                    error("Binding", "missing command");
                    exit(1)
                })
                .to_string();
            binding.push((key_to_keycode(key), cmd));
        }
    }
    binding
}

fn read_keys(kc: &Vec<(AttributeSet<KeyCode>, String)>, kbs: PathBuf,delay:u64,user:String) 
{
    let dev = Device::open(kbs).unwrap();
    let mut pressed = AttributeSet::new();

    while let Ok(event) = dev.get_key_state() {
        if event != pressed {
            pressed = event.clone();
            for (key, cmd) in kc {
                if event == *key {
                    run(&cmd, &user);
                }
            }
        }
        thread::sleep(Duration::from_millis(delay));
    }
}

fn main()
{
    let user = args().nth(1).unwrap_or_else(||{error("USER", "add your username in $USER");exit(1)});

    let binding = load_config(CONFIG);
    if binding.is_empty() {
        error("Binding", "no binding detected");
    }
    let keyboards = keyboards();
    if keyboards.is_empty() {
        error("Keyboard", "no keyboard detected");
    }

    for keyboard in keyboards {
        let bind  = binding.clone();
        let user_ = user.clone(); 
        let _ = std::thread::spawn(move || {
            read_keys(&bind, keyboard, DELAY, user_);
        });
    }

    std::thread::park();
}
