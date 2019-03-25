extern crate lib_dot_installer;

use std::fmt::Error;
use std::path::PathBuf;
use structopt::StructOpt;

use lib_dot_installer::{find_config_map, DIResult};

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(name = "", parse(from_os_str))]
    paths: Vec<PathBuf>,
}

fn main() -> DIResult<()> {
    let config_map_paths: Vec<PathBuf> = Opt::from_args()
        .paths
        .into_iter()
        .flat_map(|path: PathBuf| find_config_map(path.as_ref()))
        .collect();

    config_map_paths
        .iter()
        .for_each(|path| println!("Path: {:?}", path));

    Ok(())
}
