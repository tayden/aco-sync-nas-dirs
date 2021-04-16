use dotenv::dotenv;
use fs_extra::dir;
use log::{debug, error, info};
use simple_logger::SimpleLogger;
use structopt::StructOpt;

use aco_sync_nas_dirs::*;

fn main() {
    // Get .env values
    dotenv().ok();

    // Initialize a logger
    SimpleLogger::new().init().unwrap();

    // Parse CLI args
    let args = Cli::from_args();
    debug!("Args: {:?}", args);

    // Get project paths that should exist, using ACO database
    let db_projects = match get_db_projects(&args.root_dir, &args.year, &args.min_status) {
        Ok(paths) => paths,
        Err(e) => {
            error!("Error fetching projects from AIMS: {}", e);
            panic!("{}", e);
        }
    };
    debug!("DB Projects: {:?}", &db_projects);

    // Get project paths that actually exist on the NAS
    let fs_projects = match get_fs_projects(&args.root_dir) {
        Ok(paths) => paths,
        Err(e) => {
            error!("Error fetching projects in root_dir: {}", e);
            panic!("{}", e);
        }
    };
    debug!("FS Projects: {:?}", &fs_projects);

    // Get a list of the missing directories on the NAS, based on the above directory sets
    let missing = db_projects.difference(&fs_projects);
    debug!("Missing: {:?}", &missing);

    // Copy the seed directory to produce the missing ACO directories
    let mut copy_options = dir::CopyOptions::new();
    copy_options.copy_inside = true;
    for path in missing {
        match dir::copy(&args.seed_dir, &path, &copy_options) {
            Ok(_) => info!("Created: {:?}", &path),
            Err(e) => {
                error!("Could not create directory {:?}: {}", &path, e);
                panic!("{}", e);
            }
        };
    }
}
