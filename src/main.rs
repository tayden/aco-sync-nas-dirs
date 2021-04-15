use std::fs;

use dotenv::dotenv;
use fs_extra::dir;
use log::{debug, info};
use simple_logger::SimpleLogger;
use structopt::StructOpt;

use aco_sync_nas_dirs::*;

fn main() {
    // Get .env values
    dotenv().ok();

    // Initialize a logger
    SimpleLogger::new().init().unwrap();

    let mut args = Cli::from_args();
    args.root_dir = fs::canonicalize(args.root_dir).unwrap();
    args.seed_dir = fs::canonicalize(args.seed_dir).unwrap();
    debug!("Args: {:?}", args);

    // Get project paths that should exist, using ACO database
    let db_projects = get_db_projects(&args.root_dir, &args.year).expect("Error fetching projects from database.");
    debug!("DB Projects: {:?}", &db_projects);

    // Get project paths that actually exist on the NAS
    let fs_projects = get_fs_projects(&args.root_dir).expect("Could not get filesystem project directories.");
    debug!("FS Projects: {:?}", &fs_projects);

    // Get a list of the missing directories on the NAS, based on the above directory sets
    let missing = db_projects.difference(&fs_projects);
    debug!("Missing: {:?}", &missing);

    // Copy the seed directory to produce the missing ACO directories
    let mut copy_options = dir::CopyOptions::new();
    copy_options.copy_inside = true;
    for path in missing {
        info!("Created: {}", path.clone().into_os_string().into_string().unwrap());
        dir::copy(&args.seed_dir, path, &copy_options).expect("Could not copy seed directory");
    }
}