use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::{ffi, fs, io};

use log::{debug, error};
use postgres::types::Type;
use postgres::{Client, NoTls};
use structopt::StructOpt;

const PHASE_STATUS: [&str; 8] = [
    "proposed",
    "work order done",
    "approved",
    "flight plan done",
    "flown",
    "data checked",
    "processed",
    "delivered & complete",
];

#[derive(StructOpt, Debug)]
/// Create ACO directory seeds on the NAS using the AIMS database projects
pub struct Cli {
    /// The year in the AIMS to search for projects
    pub year: i32,

    /// The minimum project status to seed
    #[structopt(possible_values = & PHASE_STATUS, case_insensitive = true)]
    pub min_status: String,

    /// The root directory where project directories are located
    #[structopt(parse(from_os_str = Self::parse_canonical_path))]
    pub root_dir: PathBuf,

    /// The seed directory to copy to produce a new empty project directory under the root_dir
    #[structopt(parse(from_os_str = Self::parse_canonical_path))]
    pub seed_dir: PathBuf,

    /// The AIMS database host
    #[structopt(short = "h", long, env = "DB_HOST")]
    pub db_host: String,

    /// The AIMS database port
    #[structopt(short = "p", long, env = "DB_PORT")]
    pub db_port: String,

    /// The AIMS database name
    #[structopt(short = "d", long, env = "DB_NAME")]
    pub db_name: String,

    /// The AIMS database user
    #[structopt(short = "U", long, env = "DB_USER")]
    pub db_user: String,

    /// Password for the AIMS database USER
    #[structopt(short = "w", long, env = "DB_PASS")]
    pub db_pass: String,
}

impl Cli {
    fn parse_canonical_path(path: &ffi::OsStr) -> PathBuf {
        let buf = PathBuf::from(path);
        match fs::canonicalize(&buf) {
            Ok(path) => path,
            Err(e) => {
                error!("Could not parse system path {:?}: {}", &buf, &e);
                panic!("{}", e)
            }
        }
    }
}

fn get_db_client(args: &Cli) -> Result<Client, postgres::Error> {
    let connection = format!(
        "host={} port={} dbname={} user={} password={}",
        &args.db_host, &args.db_port, &args.db_name, &args.db_user, &args.db_pass
    );
    debug!("Postgres connection: {}", &connection);

    Client::connect(&connection, NoTls)
}

pub fn get_db_projects(args: &Cli) -> Result<HashSet<PathBuf>, postgres::Error> {
    let mut client = get_db_client(&args)?;

    let stmt = client.prepare_typed(
        "
        SELECT dirname 
            FROM aco.output_project_phases
            WHERE project_year = $1
            AND status_project >= $2::aco.enum_status_phase
        ",
        &[Type::INT4, Type::TEXT],
    )?;

    let paths: Vec<PathBuf> = client
        .query(&stmt, &[&args.year, &args.min_status])?
        .into_iter()
        .map(|row| row.get(0))
        .map(|r: String| args.root_dir.join(r))
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
