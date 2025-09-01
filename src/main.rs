// pind - A simple keyboard shortcut runner in Rust
// 
// This program reads key events from all connected keyboards and executes
// configured shell commands when a matching key combination is pressed.
//
// Features:
// - Supports modifier keys (Ctrl, Shift, Alt, Meta) and common keys.
// - Loads key bindings from a configuration file (~/.config/pind/pindrc).
// - Runs commands as a specified user using `runuser`.
// - Detects multiple keyboards and avoids duplicates.
// - Non-blocking key polling with a configurable delay (25ms by default).
//
// Usage:
//   sudo ./pindc   # automatically uses current user to run commands
// Config file format (~/.config/pind/pindrc):
//   # Lines starting with '#' are comments
//   <key_combination>:<command>
//   Example:
//     C+E:echo "Ctrl+Enter pressed"
//
// Dependencies:
// - evdev crate for reading keyboard input
// - std for filesystem, environment, threading, and process handling



use evdev::{AttributeSet, Device, KeyCode, enumerate};
use std::{env::{var,args},fs::read_to_string, path::PathBuf, process::{exit,Command, Stdio}, sync::{Arc, OnceLock}, thread};

const DELAY  :u64  =  25; // 25ms
const CONFIG :&str = "~/.config/pind/pindrc"; // path cfg

static BINDINGS: OnceLock<Arc<Vec<(AttributeSet<KeyCode>, String)>>> = OnceLock::new();

fn get_bindings() -> Arc<Vec<(AttributeSet<KeyCode>, String)>>
{
    BINDINGS.get_or_init(|| Arc::new(load_config(CONFIG))).clone()
}

fn error(title: &str, message: &str) -> !
{
    eprintln!("[\x1b[33mE\x1b[0m] \x1b[31m{title}:\x1b[0m {message}.");
    exit(1)
}

fn run(command: &str, user: &str)
{
    let shell = var("SHELL").unwrap_or_else(|_| "sh".into());
    
    Command::new("runuser")
        .args(["-u", user, "--", &shell, "-c", command])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|e| error("Command execution failed", &e.to_string()));
}

fn key_to_keycode(input: &str) -> AttributeSet<KeyCode> 
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

fn keyboards() -> Vec<PathBuf>
{
    enumerate().filter_map(|(path, dev)| {
        dev.supported_keys()
            .filter(|keys| keys.contains(KeyCode::KEY_A) && keys.contains(KeyCode::KEY_ENTER))
            .map(|_| path)
    })
    .collect()
}

fn load_config(config: &str) -> Vec<(AttributeSet<KeyCode>, String)>
{
    let config_path = config.replace("~", &var("HOME").unwrap_or_default());
    
    let content = read_to_string(PathBuf::from(&config_path))
        .unwrap_or_else(|e| error("CONFIG", &e.to_string()));
    
    content.lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .filter_map(|line| {
            let mut parts = line.splitn(2, ':');
            let key = parts.next()?.trim();
            let cmd = parts.next()?.trim();
            Some((key_to_keycode(key), cmd.to_string()))
        })
        .collect()
}

fn read_keys(kc: &[(AttributeSet<KeyCode>, String)], kbs: PathBuf, delay: u64, user: String)
{
    let mut state: Vec<(u64, u64)> = vec![(0, 0); kc.len()];
    let mut tick = 1;
    let init = (320 + delay - 1) / delay;

    loop {
        if let Ok(keys) = Device::open(&kbs).and_then(|d| d.get_key_state()) {
            for (i, (combo, cmd)) in kc.iter().enumerate() {
                let pressed = combo.iter().all(|k| keys.contains(k));
                let (since, last) = &mut state[i];
                if pressed {
                    if *since == 0 {
                        run(cmd, &user);
                        *since = tick;
                        *last = tick;
                    } else if tick - *since >= init && tick > *last {
                        run(cmd, &user);
                        *last = tick;
                    }
                } else {
                    *since = 0;
                }
            }
        }
        thread::sleep(std::time::Duration::from_millis(delay));
        tick = tick.wrapping_add(1);
    }
}

fn main()
{
    let user = args().nth(1).unwrap_or_else(|| error("USER", "Username argument required"));
    let bindings = get_bindings();
    if bindings.is_empty() { error("binding", "No key bindings detected"); }
    let keyboards = keyboards();
    if keyboards.is_empty() { error("Hardware", "No keyboards detected"); }

    let handles: Vec<_> = keyboards.into_iter().map(|keyboard| {
        let bindings_ref = Arc::clone(&bindings);
        let user_ref = user.clone();
        thread::spawn(move || read_keys(&bindings_ref, keyboard, DELAY, user_ref))
    }).collect();
    
    for handle in handles {
        handle.join().unwrap_or_else(|_| error("Thread", "Keyboard thread panicked"));
    }
}
