# Justfile (Convenience Command Runner)

# local vars
LOCAL_VAR_EXAMPLE:='yes, I am!'

# rust vars
RUST_LOG:= 'debug'
RUST_BACKTRACE:= '1'
RUSTFLAGS:='--cfg tokio_unstable'

# home_dir := env_var('HOME')
local_root := justfile_directory()
invocd_from := invocation_directory()
invoc_is_root := if invocd_from == local_root { "true" } else { "false" }
## ANSI Color Codes for use with echo command
GRN := '\033[0;32m' # Green
BLU := '\033[0;34m' # Blue
PRP := '\033[0;35m' # Purple
BRN := '\033[0;33m' # Brown
CYN := '\033[0;36m' # Cyan
NC := '\033[0m'     # No Color

# Default, lists commands.
_default:
        @ just --list --unsorted

# Initialize repository.
init: && deps-ext
    cargo build    
    cargo doc

# Linting, formatting, typo checking, etc.
check:
    -cargo clippy
    -cargo fmt
    -typos
    -committed

# Auto-fix errors picked up by check. (Manual exclusion of data folder as additional safeguard.)
[confirm]
fix:
     typos --exclude 'data/*' --write-changes

# Clean up cargo build artifacts.
[confirm]
teardown:
    cargo clean

# Watch a file: compile & run on changes.
watch file_to_run:
    cargo watch --quiet --clear --exec 'run --quiet --example {{file_to_run}}'

# List dependencies. (This command has dependencies.)
deps-ext:
    @echo "{{CYN}}List of external dependencies for this command runner and repo:"
    xsv table ext_dependencies.csv


// Common 

// add `gen-env` to `init`
// # Generate a `.env` file from `template.env`.
// gen-env:
//     @echo "{{CYN}}The {{GRN}}.env DATABASE_URL value{{CYN}}will populate your database path when needed.  Please edit the file to manually specify."
//     @echo {{ if path_exists(".env") == "true" { `echo "\(.env file already exists\)"` } else { `cp 'template.env' '.env'; echo "\(.env file created\)"`} }}

