#[macro_use]
extern crate dotenv_codegen;

use std::{fs, io};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::PathBuf;

use log::debug;
use postgres::{Client, NoTls};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// Create ACO directory seeds on the NAS using the AIMS database projects
pub struct Cli {
    /// The year of files to sync
    pub year: i32,

    /// The root directory where project directories are stored
    #[structopt(parse(from_os_str))]
    pub root_dir: PathBuf,

    /// The seed directory used to create a new directory under root_dir
    #[structopt(parse(from_os_str))]
    pub seed_dir: PathBuf,
}

pub fn get_db_projects(root_dir: &PathBuf, year: &i32) -> Result<HashSet<PathBuf>, postgres::error::Error> {
    // TODO: Get connection params from env variables/.env file
    let connection = format!("host={DB_HOST} port={DB_PORT} dbname={DB_NAME} user={DB_USER} password={DB_PASS}",
                             DB_HOST = dotenv!("DB_HOST"),
                             DB_PORT = dotenv!("DB_PORT"),
                             DB_NAME = dotenv!("DB_NAME"),
                             DB_USER = dotenv!("DB_USER"),
                             DB_PASS = dotenv!("DB_PASS")
    );

    debug!("Postgres connection: {}", &connection);
    let mut client = Client::connect(&connection, NoTls).unwrap();

    let mut projects: Vec<String> = Vec::new();
    for row in client.query(
        "SELECT projectphase_num, project_name FROM aco.output_project_phases WHERE project_year = $1",
        &[&year],
    )? {
        let mut num: String = row.get(0);
        let mut name: String = row.get(1);

        num = num.replace("-", "_");
        name = name.replace(" ", "_").replace("-", "_");

        let filename = format!("{}_{}", num, name);
        projects.push(filename);
    };

    let paths: Vec<PathBuf> = projects.iter().map(|r| root_dir.join(r)).collect();

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
