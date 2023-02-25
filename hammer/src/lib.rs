//! # Hammer
//! 
//! Hammer is a no-config cli tool for running concurrent tasks with monorepo support.
//! Monorepo tooling *should* be as simple as it can. Unfortunately, the current tooling is overly 
//! complex and can lead to a lot of time spent configuring and debugging, when you should be 
//! worried about writing your code.
//! 
//! You can think of Hammer as a lightweight alternative for turborepo. Currently, it only
//! supports JS/TS projects. In the future it could expand and have a larger number of languages
//! supported.
//! 
//! ### The `hammer` bin
//! 
//! The `hammer` binary is hammer's CLI. It aims to be as simple as it can get.
//! 
//! Syntax:
//! ```bash
//! hammer <SCRIPT> [OPTS]
//! ```
//! 
//! Example:
//! 
//! ```bash
//! hammer dev
//! ```
//! *Runs all the workspaces projects "hammer:dev" scripts.*
//! 
//! #### Disabling the prefix
//! 
//! By default, hammer will look up for all the scripts containing the prefix `hammer:` in package.json,
//! thus making it easy to gradually adopt the tool and run only the scripts you want using it.
//! If you don't want this, just use the `--no-prefix` flag and hammer will look for the `dev` script directly:
//! 
//! ```bash
//! hammer dev --no-prefix
//! ``` 
//! *Runs all workspace projects "dev" scripts.*
//! 
//! **alias**: -n
//! 
//! #### Filtering
//! 
//! You can filter which projects should be targeted by hammer:
//! 
//! ```bash
//! hammer dev --filter web
//! ```
//! *Will only run the dev script of the project that contains a package.json with the "name" being "web"* 
//! 
//! **alias**: -f

pub mod fs_checks;
pub mod npm_process;
pub mod package_json;
pub mod tasks;
pub mod errors;
pub mod args;