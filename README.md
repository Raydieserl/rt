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
rt testcommand /
```

## Testing with source code
```
# e.g. RUST_BACKTRACE=1 cargo run -- [COMMAND] [VARIABLES]

# shows the help menu
cargo run -- help

# runs ls on /
cargo run -- testcommand /
```

## How to add a command
### 1. Edit the original file
```
open ~/.rt/commands.json
# Add command see "Command format"
```

### 2. Create backup edit and reimport
```
rt export commands.backup.json
open commands.backup.json
rt import commands.backup.json
# Add command see "Command format"
```

## Command format
```
{
    "triggers": ["pyproj"],
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
    ],
    "groups": ["Fav Commands"]
}
```
### To run the above example after it was added to the commands.json
```
rt pyproj my_python_project
```
### Format explained
**triggers:**  
can contain multiple triggers to run the program  
e.g. rt _pyproj_  

**description:**  
gets shown in help menu to remember your functionality  

**commands:**  
list of commands that get executed  

**variables:**  
_target_ gets searched for and replaced by and input arg  
e.g. rt pyproj &lt;PATH&gt;  
_description_ gets shown in help menu

**groups:**
Group names shown in help menu

<hr>

<br><br>
<h2>TODO:</h2>
<pre>
Features:
  - split system commands files
  - export as .sh script
Future:
  - change shell
  - windows support
  - http, or mcp support
Revisit:
  - Add tests
  - error handling
  - cloning
</pre>