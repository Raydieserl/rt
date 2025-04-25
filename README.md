# RT
rt is a small commandline tool which helps managing your daily commands. That includes combining tasks sharing functionality on different machines or backing them up and share them with other people.

## Install
```
chmod +x install.sh
./install.sh
```

## Examples
```
# shows the help menu
rt help

# runs ls on /
rt testcmd /
```

## Testing with source code
```
# e.g. RUST_BACKTRACE=1 cargo run -- [COMMAND] [VARIABLES]

# shows the help menu
cargo run -- help

# runs ls on /
cargo run -- testcmd /
```

## How to add a command
### 1. Edit the original file
```
open ~/.rt/commands.json
# Add command see "Command format"
```

### 2. Create backup edit and reimport
```
rt export
open commands.backup.json
rt import
# Add command see "Command format"
```

### Command format
```
{
    "names": ["pyproj"],
    "description": "Create python project with venv and git",
    "commands": [
        "mkdir <PATH>",
        "cd <PATH>",
        "python3 -m venv .venv",
        "git init",
        "touch .gitignore",
        "touch main.py"
    ],
    "variables": [
        {
            "target": "<PATH>",
            "description": "Path for project e.g. ~/Desktop/test_pyproj"
        }
    ]
}
```