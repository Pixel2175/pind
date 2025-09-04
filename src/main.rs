use evdev::{AttributeSet, Device, KeyCode, enumerate, EventType, uinput::VirtualDevice};
use std::{env::{var,args},fs::{read_to_string,canonicalize}, path::PathBuf, process::{exit,Command, Stdio}, sync::{Arc, OnceLock}, thread};

const DELAY  :u64  =  25; // 25ms
const CONFIG :&str =  "~/.config/pind/pindrc";

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

    for key in input.replace(' ',"").split('+')  {
        match key.to_lowercase().as_str(){
            "meta_left"  | "meta"  => attribute_set.insert(KeyCode::KEY_LEFTMETA),
            "ctrl_left"  | "ctrl"  => attribute_set.insert(KeyCode::KEY_LEFTCTRL),
            "shift_left" | "shift" => attribute_set.insert(KeyCode::KEY_LEFTSHIFT),
            "alt_left"   | "alt"   => attribute_set.insert(KeyCode::KEY_LEFTALT),
            "back_slash" | "\\"    => attribute_set.insert(KeyCode::KEY_BACKSLASH),
            "slash"      | "/"     => attribute_set.insert(KeyCode::KEY_SLASH),
            "enter"     | "return" => attribute_set.insert(KeyCode::KEY_ENTER),
            "ctrl_right"  => attribute_set.insert(KeyCode::KEY_RIGHTCTRL),
            "shift_right" => attribute_set.insert(KeyCode::KEY_RIGHTSHIFT),
            "meta_right"  => attribute_set.insert(KeyCode::KEY_RIGHTMETA),
            "alt_right"   => attribute_set.insert(KeyCode::KEY_RIGHTALT),
            "capslock"    => attribute_set.insert(KeyCode::KEY_CAPSLOCK),
            "tab" => attribute_set.insert(KeyCode::KEY_TAB),
            "0" => attribute_set.insert(KeyCode::KEY_0),
            "1" => attribute_set.insert(KeyCode::KEY_1),
            "2" => attribute_set.insert(KeyCode::KEY_2),
            "3" => attribute_set.insert(KeyCode::KEY_3),
            "4" => attribute_set.insert(KeyCode::KEY_4),
            "5" => attribute_set.insert(KeyCode::KEY_5),
            "6" => attribute_set.insert(KeyCode::KEY_6),
            "7" => attribute_set.insert(KeyCode::KEY_7),
            "8" => attribute_set.insert(KeyCode::KEY_8),
            "9" => attribute_set.insert(KeyCode::KEY_9),
            "a" => attribute_set.insert(KeyCode::KEY_A),
            "b" => attribute_set.insert(KeyCode::KEY_B),
            "c" => attribute_set.insert(KeyCode::KEY_C),
            "d" => attribute_set.insert(KeyCode::KEY_D),
            "e" => attribute_set.insert(KeyCode::KEY_E),
            "f" => attribute_set.insert(KeyCode::KEY_F),
            "g" => attribute_set.insert(KeyCode::KEY_G),
            "h" => attribute_set.insert(KeyCode::KEY_H),
            "i" => attribute_set.insert(KeyCode::KEY_I),
            "j" => attribute_set.insert(KeyCode::KEY_J),
            "k" => attribute_set.insert(KeyCode::KEY_K),
            "l" => attribute_set.insert(KeyCode::KEY_L),
            "m" => attribute_set.insert(KeyCode::KEY_M),
            "n" => attribute_set.insert(KeyCode::KEY_N),
            "o" => attribute_set.insert(KeyCode::KEY_O),
            "p" => attribute_set.insert(KeyCode::KEY_P),
            "q" => attribute_set.insert(KeyCode::KEY_Q),
            "r" => attribute_set.insert(KeyCode::KEY_R),
            "s" => attribute_set.insert(KeyCode::KEY_S),
            "t" => attribute_set.insert(KeyCode::KEY_T),
            "u" => attribute_set.insert(KeyCode::KEY_U),
            "v" => attribute_set.insert(KeyCode::KEY_V),
            "w" => attribute_set.insert(KeyCode::KEY_W),
            "x" => attribute_set.insert(KeyCode::KEY_X),
            "y" => attribute_set.insert(KeyCode::KEY_Y),
            "z" => attribute_set.insert(KeyCode::KEY_Z),
            "up"    => attribute_set.insert(KeyCode::KEY_UP),
            "down"  => attribute_set.insert(KeyCode::KEY_DOWN),
            "right" => attribute_set.insert(KeyCode::KEY_RIGHT),
            "left"  => attribute_set.insert(KeyCode::KEY_LEFT),
            "dot" | "." => attribute_set.insert(KeyCode::KEY_DOT),
            "comma" | "," => attribute_set.insert(KeyCode::KEY_COMMA),
            "semicolon"  | ";"  => attribute_set.insert(KeyCode::KEY_SEMICOLON),
            "apostrophe" | "\"" => attribute_set.insert(KeyCode::KEY_APOSTROPHE),
            "leftbrace"  | "["  => attribute_set.insert(KeyCode::KEY_LEFTBRACE),
            "rightbrace" | "]"  => attribute_set.insert(KeyCode::KEY_RIGHTBRACE),
            "numlock"     => attribute_set.insert(KeyCode::KEY_NUMLOCK),
            "scroll_lock"  => attribute_set.insert(KeyCode::KEY_SCROLLLOCK),
            "minus" | "-" => attribute_set.insert(KeyCode::KEY_MINUS),
            "equal" | "=" => attribute_set.insert(KeyCode::KEY_EQUAL),
            "plus" => attribute_set.insert(KeyCode::KEY_KPPLUS),
            "grave" | "`" => attribute_set.insert(KeyCode::KEY_GRAVE),
            "space" => attribute_set.insert(KeyCode::KEY_SPACE),
            "esc" => attribute_set.insert(KeyCode::KEY_ESC),
            "f1"  => attribute_set.insert(KeyCode::KEY_F1),
            "f2"  => attribute_set.insert(KeyCode::KEY_F2),
            "f3"  => attribute_set.insert(KeyCode::KEY_F3),
            "f4"  => attribute_set.insert(KeyCode::KEY_F4),
            "f5"  => attribute_set.insert(KeyCode::KEY_F5),
            "f6"  => attribute_set.insert(KeyCode::KEY_F6),
            "f7"  => attribute_set.insert(KeyCode::KEY_F7),
            "f8"  => attribute_set.insert(KeyCode::KEY_F8),
            "f9"  => attribute_set.insert(KeyCode::KEY_F9),
            "f10" => attribute_set.insert(KeyCode::KEY_F10),
            "f11" => attribute_set.insert(KeyCode::KEY_F11),
            "f12" => attribute_set.insert(KeyCode::KEY_F12),
            "kp0"     | "keypad_0" => attribute_set.insert(KeyCode::KEY_KP0),
            "kp1"     | "keypad_1" => attribute_set.insert(KeyCode::KEY_KP1),
            "kp2"     | "keypad_2" => attribute_set.insert(KeyCode::KEY_KP2),
            "kp3"     | "keypad_3" => attribute_set.insert(KeyCode::KEY_KP3),
            "kp4"     | "keypad_4" => attribute_set.insert(KeyCode::KEY_KP4),
            "kp5"     | "keypad_5" => attribute_set.insert(KeyCode::KEY_KP5),
            "kp6"     | "keypad_6" => attribute_set.insert(KeyCode::KEY_KP6),
            "kp7"     | "keypad_7" => attribute_set.insert(KeyCode::KEY_KP7),
            "kp8"     | "keypad_8" => attribute_set.insert(KeyCode::KEY_KP8),
            "kp9"     | "keypad_9" => attribute_set.insert(KeyCode::KEY_KP9),
            "kpdot"   | "keypad_dot" => attribute_set.insert(KeyCode::KEY_KPDOT),
            "kpplus"  | "keypad_plus" => attribute_set.insert(KeyCode::KEY_KPPLUS),
            "kpminus" | "keypad_minus" => attribute_set.insert(KeyCode::KEY_KPMINUS),
            _ => {error("Key", &format!("key {} is not exists",key))},
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
    let content = read_to_string(canonicalize(PathBuf::from(config.replace("~",&var("HOME").unwrap_or_default()))).unwrap_or_else(|e|{error("config",&e.to_string())}))
        .unwrap_or_else(|e| error("Config file read failed", &e.to_string()));
    
    content.lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .filter_map(|line| {
            let mut parts = line.splitn(2, "=>");
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
    
    let mut device = match Device::open(&kbs) {
        Ok(dev) => dev,
        Err(_) => return, // Device no longer exists, exit thread
    };
    
    // Create a virtual device to forward non-shortcut keys
    // First, get all supported keys from the physical device to set up the virtual device properly
    let supported_keys = match device.supported_keys() {
        Some(keys) => keys,
        _ => {
            eprintln!("Failed to get supported keys from device");
            return;
        }
    };
    
    let mut virtual_device = match VirtualDevice::builder() {
        Ok(builder) => builder,
        Err(e) => {
            eprintln!("Failed to create virtual device builder: {}", e);
            return;
        }
    };
    
    virtual_device = match virtual_device
        .name("pind-virtual-keyboard")
        .with_keys(&supported_keys)
    {
        Ok(builder) => builder,
        Err(e) => {
            eprintln!("Failed to set up virtual device with keys: {}", e);
            return;
        }
    };
    
    let mut virtual_device = match virtual_device.build() {
        Ok(dev) => dev,
        Err(e) => {
            eprintln!("Failed to build virtual device: {}", e);
            return;
        }
    };
    
    // Grab the physical device to capture all events
    if let Err(e) = device.grab() {
        eprintln!("Failed to grab device: {}", e);
        return;
    }

    loop {
        // Collect events
        let events: Vec<_> = match device.fetch_events() {
            Ok(events_iter) => events_iter.collect(),
            Err(_) => break, // Device no longer exists
        };
        
        let mut events_to_forward = Vec::new();
        let current_state = match device.get_key_state() {
            Ok(state) => state,
            Err(_) => break, // Device no longer exists
        };
        
        // Check if any shortcut is currently active
        let mut active_shortcut = false;
        for (combo, _) in kc {
            // Check if all keys in combo are pressed
            if combo.iter().all(|k| current_state.contains(k)) {
                active_shortcut = true;
                break;
            }
        }
        
        for event in events {
            if event.event_type() == EventType::KEY {
                let key_code = KeyCode::new(event.code());
                
                // Check if this key is part of any shortcut
                let mut is_shortcut_key = false;
                for (combo, _) in kc {
                    if combo.contains(key_code) {
                        is_shortcut_key = true;
                        break;
                    }
                }
                
                if is_shortcut_key && active_shortcut {
                    for (i, (combo, cmd)) in kc.iter().enumerate() {
                        // Check if all keys in combo are pressed
                        let pressed = combo.iter().all(|k| current_state.contains(k));
                        let (since, last) = &mut state[i];
                        if pressed {
                            if *since == 0 {
                                *since = tick;
                                *last = tick;
                            } else if tick - *since >= init && tick > *last {
                                *last = tick;
                            }
                            run(cmd, &user);
                        } else {
                            *since = 0;
                        }
                    }
                } else {
                    events_to_forward.push(event);
                }
            } else {
                events_to_forward.push(event);
            }
        }
        if !events_to_forward.is_empty() {
            if let Err(e) = virtual_device.emit(&events_to_forward) {
                eprintln!("Failed to emit events: {}", e);
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
