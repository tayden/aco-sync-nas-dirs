#[macro_use]
extern crate dotenv_codegen;

use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::{ffi, fs, io};

use log::{debug, error};
use postgres::{Client, NoTls};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// Create ACO directory seeds on the NAS using the AIMS database projects
pub struct Cli {
    /// The year of files to sync
    pub year: i32,

    /// The root directory where project directories are stored
    #[structopt(parse(from_os_str = Self::parse_canonical_path))]
    pub root_dir: PathBuf,

    /// The seed directory used to create a new directory under root_dir
    #[structopt(parse(from_os_str = Self::parse_canonical_path))]
    pub seed_dir: PathBuf,
}

impl Cli {
    fn parse_canonical_path(path: &ffi::OsStr) -> PathBuf {
        let buf = PathBuf::from(path);
        fs::canonicalize(buf).unwrap()
    }
}

fn get_db_client() -> Client {
    let connection = format!(
        "host={} port={} dbname={} user={} password={}",
        dotenv!("DB_HOST"),
        dotenv!("DB_PORT"),
        dotenv!("DB_NAME"),
        dotenv!("DB_USER"),
        dotenv!("DB_PASS")
    );
    debug!("Postgres connection: {}", &connection);

    match Client::connect(&connection, NoTls) {
        Ok(client) => client,
        Err(e) => {
            error!("{:?}", e);
            std::process::exit(1);
        }
    }
}

pub fn get_db_projects(
    root_dir: &PathBuf,
    year: &i32,
) -> Result<HashSet<PathBuf>, postgres::Error> {
    let mut client = get_db_client();

    let paths: Vec<PathBuf> = client
        .query(
            "SELECT \
            replace(projectphase_num, '-', '_') || '_' || \
            regexp_replace(project_name, '\\W+', '_', 'g') \
            FROM aco.output_project_phases WHERE project_year = $1",
            &[&year],
        )?
        .into_iter()
        .map(|row| row.get(0))
        .map(|r: String| root_dir.join(r))
        .collect();

    Ok(HashSet::<PathBuf>::from_iter(paths))
}

pub fn get_fs_projects(root_dir: &PathBuf) -> Result<HashSet<PathBuf>, io::Error> {
    let directories: Vec<PathBuf> = fs::read_dir(&root_dir)?
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| r.is_dir())
        .collect();

    Ok(HashSet::<PathBuf>::from_iter(directories))
}
