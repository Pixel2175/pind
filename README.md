````markdown
# Pind

## About
When I switched from X11 to Wayland, I ran into problems with hotkey binding apps like `sxhkd`. I tried `swhkd`, but it wasn’t good enough.  
After testing [Waybind](https://github.com/postman721/Waybind/issues), I decided to build my own minimal hotkey daemon in **Rust**: **Pind**.

## Features
- Written in Rust
- Supports almost all keyboard keys
- Minimal as possible (~230 lines of code)
- Works with Wayland and X11 environments

## Installation
> **⚠️ Warning:** Run without `sudo`.

```bash
make install
````

Uninstall:

```bash
make uninstall
```

## Usage

Run:

```bash
pindc <doas|sudo|etc>
```

To restart, just run the command again (it kills the old process).

## Config

The config file is located at:

```
~/.config/pind/pindrc
```

### Config Example

```bash
# Ctrl + Shift + Up
ctrl + shift + up => light -A 10 ; notify-send "Light" "Light Up %$(light)"

# Ctrl + Shift + Down
ctrl_left + shift + down => doas light -U 10 ; notify-send "Light" "Light Down %$(light)"

# Meta + a
meta_right + a => set file (ls ~/learn/book | wmenu); and test -n "$file"; and zathura ~/learn/book/"$file"

# Volume Up
ctrl_left + shift_right + right => pactl set-sink-volume @DEFAULT_SINK@ +10% ; notify-send "Sound" "Volume Up %$(pamixer --get-volume)"

# Volume Down
ctrl + shift + left => pactl set-sink-volume @DEFAULT_SINK@ -10% ; notify-send "Sound" "Volume Down %$(pamixer --get-volume)"
```

## Key Symbol Table

| **Key**     | **Symbol(s)**              |
| ----------- | -------------------------- |
| Ctrl Left   | ctrl, ctrl_left           |
| Ctrl Right  | ctrl_right                |
| Shift Left  | shift, shift_left         |
| Shift Right | shift_right               |
| Alt Left    | alt, alt_left             |
| Alt Right   | alt_right                 |
| Meta Left   | meta, meta_left           |
| Meta Right  | meta_right                |
| CapsLock    | capslock                   |
| Tab         | tab                        |
| Enter       | enter, return              |
| Esc         | esc                        |
| Space       | space                      |
| Up          | up                         |
| Down        | down                       |
| Left        | left                       |
| Right       | right                      |
| Backslash   | , back_slash              |
| Slash       | /, slash                   |
| Dot         | ., dot                     |
| Comma       | ,, comma                   |
| Semicolon   | ;, semicolon               |
| Apostrophe  | ', apostrophe              |
| Left Brace  | [, leftbrace              |
| Right Brace | ], rightbrace              |
| Minus       | -, minus                   |
| Equal       | =, equal                   |
| Grave       | \`, grave                  |
| NumLock     | numlock                    |
| ScrollLock  | scroll_lock               |
| 0–9         | 0,1,2,...,9                |
| A–Z         | a–z                        |
| F1–F12      | f1–f12                     |
| KP0–KP9     | kp0–kp9, keypad_0–9       |
| KP Dot      | kpdot, keypad_dot         |
| KP Minus    | kpminus, keypad_minus     |

## How it Works

There are **two executables**:

* **pindd**: Pind Daemon
  Runs as root, opens `/dev/input/eventX`, and listens for key presses.
  If the pressed keys match the config, it runs the mapped command.
  If the pressed keys don't match the config, it uses `uinput` to pass the key event to the focused client.

* **pindc**: Pind Client
  Captures the user’s environment variables and passes them to `pindd`.
  This allows root daemon processes to still access the user environment (paths, `XDG_RUNTIME_DIR`, etc).

```
```
