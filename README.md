# pomod
A pomodoro written in rust

# Installation

Install [Rust](https://www.rust-lang.org/tools/install) if you don't have. 

```shell
cargo build --release
# copy pomod and pomod-fetch from target/release to desired path you want
```

# Usage 

Copy `.config/pomod/*` under the project directory to  `~/.config/pomod/`

Start pomod

```shell
pomod
```

# Configuration

``````toml
# pomod.toml

display = "None" # Txt, Tui, None.
cache = true  # true if you want to use pomod-fetch 

[time]
hours = 0
minutes = 0
seconds = 10

[audio]
play = true # need music to wake you up?
path = "/home/charles/.config/pomod/notify1.mp3"
time = 3 # Music duration (in seconds)

[notify]
notify = true # need desktop notifications?
message = "it is time to take a break"
``````

![Txt](https://raw.githubusercontent.com/charleschetty/pomod/master/shots/Txt.png)

![Tui](https://raw.githubusercontent.com/charleschetty/pomod/master/shots/Tui.png)

# Polybar

```
[module/pomod]
type = custom/script
exec = pomod-fetch
label =     %output%
tail = true
```

![Tui](https://raw.githubusercontent.com/charleschetty/pomod/master/shots/polybar.png)
