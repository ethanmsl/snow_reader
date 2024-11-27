# Justfile (Convenience Command Runner)

# rust vars
RUST_LOG:= 'debug'
RUST_BACKTRACE:= '1'
RUSTFLAGS:='--cfg tokio_unstable'
TOML_VERSION:=`rg '^version = ".*"' Cargo.toml | sd '.*"(.*)".*' '$1'`
# just path vars
HOME_DIR := env_var('HOME')
LOCAL_ROOT := justfile_directory()
INVOCD_FROM := invocation_directory()
INVOC_IS_ROOT := if INVOCD_FROM == LOCAL_ROOT { "true" } else { "false" }
# custom vars
FROZE_SHA_REGEX := 'FROZE_[a-fA-F0-9]{64}_FROZE-'
# ANSI Color Codes for use with echo command
NC := '\033[0m'     # No Color
CYN := '\033[0;36m' # Cyan
BLU := '\033[0;34m' # Blue
GRN := '\033[0;32m' # Green
PRP := '\033[0;35m' # Purple
BRN := '\033[0;33m' # Brown

# Default, lists commands.
_default:
        @just --list --unsorted

# Initialize repository.
[confirm]
init: && deps-ext _gen-env _gen_git_hooks
    cargo clean
    cargo build    
    cargo doc
    
# Clean up cargo build artifacts.
[confirm]
teardown:
    cargo clean

# Linting, formatting, typo checking, etc.
check:
    cargo clippy
    cargo fmt
    typos
    committed
    
# Auto-fix some errors picked up by check. (Manual exclusion of data folder as additional safeguard.)
[confirm]
fix:
     typos --exclude '*/data/*' --write-changes

# Show docs.
docs:
    rustup doc
    rustup doc --std
    cargo doc --all-features --document-private-items --open

# Update Rust-crates even through breaking changes
update_breaking:
    cargo update --breaking -Z unstable-options
    
# Add a package to workspace // update-comment: the heck am I doing adding, removing, then using cargo-generate?
packadd name:
    cargo new --bin {{name}}
    rm -rf {{name}}
    cargo generate --path ./.support_data/templates/template__cli_bin --name {{name}}

# Run git hook.
git_hook hook='pre-commit':
    git hook run {{hook}} 

# Tests, docs and general.
test:
    cargo test --doc --quiet
    cargo nextest run --status-level=leak

# Run a specific test with output visible. (Use '' for test_name to see all tests and set log_level)
test-view test_name="" log_level="error":
    @echo "'Fun' Fact; the '--test' flag only allows integration test selection and will just fail on unit tests."
    RUST_LOG={{log_level}} cargo test {{test_name}} -- --nocapture 
    
# Run a specific test with NEXTEST with output visible. (Use '' for test_name to see all tests and set log_level)
testnx-view test_name="" log_level="error":
    @echo "'Fun' Fact; the '--test' flag only allows integration test selection and will just fail on unit tests."
    RUST_LOG={{log_level}} cargo nextest run {{test_name}} --no-capture 

# Watch a file: compile & run on changes.
watch file_to_run:
    cargo watch --quiet --clear --exec 'run --quiet --example {{file_to_run}}'

# List dependencies. (This command has dependencies.)
deps-ext:
    @echo "{{CYN}}List of external dependencies for this command runner and repo:"
    xsv table ext_deps.csv
    
# ######################################################################## #

# Print reminder: how to set env vars that propagate to child shells.
_remind_setenv:
    @ echo '{{GRN}}set -a{{NC}}; {{GRN}}source {{BLU}}.env{{NC}}; {{GRN}}set +a{{NC}}'
    
# ######################################################################## #
    
# Generate .env file from template, if .env file not present.
_gen-env:
    @ if [ -f '.env' ]; then echo '`{{BRN}}.env{{NC}}` exists, {{PRP}}skipping creation{{NC}}...' && exit 0; else cp -n .support_data/template.env .env; echo "{{BLU}}.env{{NC}} created from template. {{GRN}}Please fill in the necessary values.{{NC}}"; echo "e.g. via 'nvim .env'"; fi
    
# Attempt to add all git-hooks. (no overwrite)
_gen_git_hooks: _gen-precommit-hook _gen-commitmsg-hook

# Attempt to add `pre-commit` git-hook. (no overwrite)
_gen-precommit-hook:
    @ if [ -f '.git/hooks/pre-commit' ]; then echo '`.git/hooks/{{BRN}}pre-commit{{NC}}` exists, {{PRP}}skipping creation{{NC}}...' && exit 0; else cp -n .support_data/pre-commit .git/hooks/pre-commit; chmod u+x .git/hooks/pre-commit; echo live "{{BLU}}pre-commit{{NC}} hook added to {{GRN}}.git/hooks{{NC}} and set as executable"; fi
    
# Attempt to add `commit-msg` git-hook. (no overwrite)
_gen-commitmsg-hook:
    @ if [ -f '.git/hooks/commit-msg' ]; then echo '`.git/hooks/{{BRN}}commit-msg{{NC}}` exists, {{PRP}}skipping creation{{NC}}...' && exit 0; else cp -n .support_data/commit-msg .git/hooks/commit-msg; chmod u+x .git/hooks/commit-msg; echo live "{{BLU}}commit-msg{{NC}} hook added to {{GRN}}.git/hooks{{NC}} and set as executable"; fi

# ######################################################################## #

# Freeze! For your safety.
_freeze file:
	mv -iv {{file}} FROZE_{{sha256(file)}}_FROZE-{{file}} | rg {{file}}

# Unfreeze a file. (removes 'FROZE...FROZE-' tag from filename)
_thaw file:
	echo {{file}} | sd '{{FROZE_SHA_REGEX}}' '' | xargs mv -iv {{file}}

# Search local files through ice.
_arctic_recon iceless_name:
	fd --max-depth 1 '{{FROZE_SHA_REGEX}}{{iceless_name}}' | rg {{iceless_name}}


# ######################################################################## #

# Speak Funny to Me!
_uu:
	echo {{uuid()}}

# Say my name.
_sha file:
	echo {{sha256_file(file)}}

# Example function for syntax reference
_example_file_exists_test file:
    echo {{ if path_exists(file) == "true" { "hello" } else { "goodbye" } }}

# ######################################################################## #

# # Ad hoc hyperfine tests for the release version of the cli app.
# bench-hyperf regex='ho' :
#     @echo "{{GRN}}Release{{NC}}, search-only:"
#     hyperfine --warmup 3 "target/release/rename_files '{{regex}}' --recurse"
#     @echo "{{GRN}}Release{{NC}}, search & replace, no file write:"
#     hyperfine --warmup 3 "target/release/rename_files '{{regex}}' --rep 'ohhoho' --recurse --test-run"
#     @echo "{{PRP}}Comparison{{NC}}: fd --unrestricted, search-only:"
#     hyperfine --warmup 3 "fd --unrestricted '{{regex}}'"

# ######################################################################## #

# # # Needs updating to work with workspace.
# # Clean, release build, deploy file to `/user/local/bin/`
# [confirm]
# deploy binary_name version: check
#     @ echo "TOML_VERSION: {{TOML_VERSION}}"
#     @ echo "input version: {{version}}"
#     echo {{ if TOML_VERSION == version  {"TOML version declaration matches input version."} else  {`error("version_mismatch")`} }}
#     cargo clean
#     cargo build --release
#     cargo doc --release
#     sudo cp target/release/{{binary_name}} /usr/local/bin/{{binary_name}}
    
# # # Needs updating to work with workspace.
# # Push version x.y.z; deploy if used with `dist`
# [confirm]
# deploy-remote version: check
#     @ echo "TOML_VERSION: {{TOML_VERSION}}"
#     @ echo "input version: {{version}}"
#     echo {{ if TOML_VERSION == version  {"TOML version declaration matches input version."} else  {`error("version_mismatch")`} }}
#     cargo clean
#     cargo build --release
#     cargo doc --release
#     - git add .
#     - git commit -m "release: {{version}}"
#     git tag "v{{version}}"
#     - git push
#     git push --tags
