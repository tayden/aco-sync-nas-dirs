# AIMS Seed ACO NAS

This utility looks at projects existing in the AIMS for a certain year, and seeds folders on the NAS.

## Installation

You can use the precompiled Linux binaries on the Release page or install using cargo install.

```shell
# Install Rust and Cargo
curl https://sh.rustup.rs -sSf | sh

# Install this package
cargo install --git https://github.com/tayden/aims-seed-aco-nas.git --branch main
```

## Usage

This program expects the following environment variables to be defined in order to query the database.

```dotenv
DB_HOST=db.hakai.org
DB_NAME=hakai
DB_USER=your_username
DB_PASS=secret!
DB_PORT=5432
```

Then call the program as follows

```shell
./aims-seed-aco-nas --help

#  aims-seed-aco-nas 0.2.0
#  Create ACO directory seeds on the NAS using the AIMS database projects
#  
#  USAGE:
#      aims-seed-aco-nas <year> <min-status> <root-dir> <seed-dir> --db-host <db-host> --db-name <db-name> --db-pass <db-pass> --db-port <db-port> --db-user <db-user>
#  
#  FLAGS:
#          --help       Prints help information
#      -V, --version    Prints version information
#  
#  OPTIONS:
#      -h, --db-host <db-host>    The AIMS database host [env: DB_HOST=db.hakai.org]
#      -d, --db-name <db-name>    The AIMS database name [env: DB_NAME=hakai]
#      -w, --db-pass <db-pass>    Password for the AIMS database USER [env: DB_PASS]
#      -p, --db-port <db-port>    The AIMS database port [env: DB_PORT=5432]
#      -U, --db-user <db-user>    The AIMS database user [env: DB_USER=your_username]
#  
#  ARGS:
#      <year>          The year in the AIMS to search for projects
#      <min-status>    The minimum project status to seed [possible values: proposed, work order done, approved, flight
#                      plan done, flown, data checked, processed, delivered & complete]
#      <root-dir>      The root directory where project directories are located
#      <seed-dir>      The seed directory to copy to produce a new empty project directory under the root_dir
```

By default, the program logs to stdout.

This allows for easily redirecting logs to some file (e.g. when running as a cronjob, etc.)

## Development

The program is written in Rust and compiled for target `x86_64_unknown_linux_musl`. Binaries are statically linked for
maximum portability.