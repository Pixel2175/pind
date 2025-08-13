## Pixel HotKey Binding (Pind)
when i switch from x11 to wayland i ran into some problems. one of them is Hotkey binding apps like sxhkd,i tried swhkd, but its shit (sorry) 

## Waybind
so i some research and found a project called [Waybind](https://github.com/postman721/Waybind/issues)
this is a masterpiece but there are some problems:
- written in python (not a big problem but i prefer one written in low-level lang :)
- only support CTRL ,Alt and Suport Keys.
- can't detect the user ENV.


# Pind 
so now im going to talk more about Pind and how to use it

## Pind Features 
- written in Rust 
- support almost all keyboard Keys (i will add more in the future).
- minimal as posible (~230 line of code )

## Installation
to install it you just need to run this commands
> **⚠️ Warning:** You need to run it without sudo.

- install:
```bash
make install
```

- uninstall:
```bash
make uninstall
```

## How to use
- Run it 
```bash 
pindc <doas|sudo|etc>
```


## Config
the default config path is in ```/etc/pind/pindrc```.
check the pindrc file for more info.

**Config example:**
```
# Ctrl + Shift + a
CSa : doas light -A 10 ; pino -t "Light" -m "Light Up %$(light)"

# Ctrl + Shift + Down Key
CSd : doas light -U 10 ; pino -t "Light" -m "Light Down %$(light)"

# its ok if you want to use it like this
C + S +w :   walrs -i ~/.config/wallpaper

```

## Key Symbol Table

| **Key**    | **Symbol** |
|------------|------------|
| Ctrl       | C          |
| Alt        | A          |
| Shift      | S          |
| Meta       | M          |
| CapsLock   | K          |
| Tab        | T          |
| Enter      | E          |
| Up         | U          |
| Down       | D          |
| Left       | L          |
| Right      | R          |
| Slash      | /          |
| Backslash  | \          |
| 0          | 0          |
| 1          | 1          |
| 2          | 2          |
| 3          | 3          |
| 4          | 4          |
| 5          | 5          |
| 6          | 6          |
| 7          | 7          |
| 8          | 8          |
| 9          | 9          |
| A          | a          |
| B          | b          |
| C          | c          |
| D          | d          |
| E          | e          |
| F          | f          |
| G          | g          |
| H          | h          |
| I          | i          |
| J          | j          |
| K          | k          |
| L          | l          |
| M          | m          |
| N          | n          |
| O          | o          |
| P          | p          |
| Q          | q          |
| R          | r          |
| S          | s          |
| T          | t          |
| U          | u          |
| V          | v          |
| W          | w          |
| X          | x          |
| Y          | y          |
| Z          | z          |
| Dot        | .          |
| Comma      | ,          |
| Semicolon  | ;          |
| Apostrophe | '          |
| LeftBrace  | [          |
| RightBrace | ]          |
| Minus      | -          |
| Equal      | =          |
| Grave      | `          |


## How does this app work 
there is 2 executable files 
- **pindd**: Pind-Daemon
This is the daemon where all the main functionality happens.
It opens ```/dev/input/eventX``` devices and filters for keyboards.
Then, it creates threads for each keyboard device to start listening for key presses. If the keys pressed match those in the config, it will run the corresponding command.

- **pindc**: Pind-Client
Since pindd needs root privileges to run, it cannot access the user environment directly.
That’s where pindc comes in — it captures the user’s environment variables and passes them to pindd running as root.
This means all user environment variables, such as XDG_RUNTIME_DIR, paths, and other information, are sent to the daemon.
