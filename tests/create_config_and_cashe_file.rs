extern crate jamdocs;

use std::{path::Path};


const TEST_CONFIG_FILE_PATH: &str = "/files/create_config_and_cashe_file/jamdocs.toml";
const TEST_CACHE_FILE_PATH: &str = "/files/create_config_and_cashe_file/jamdocs_cache.toml";

#[test]
fn test_create_config_file() {
    jamdocs::init::create_config();
    let file_flag = Path::new(&TEST_CONFIG_FILE_PATH).exists();
    assert_eq!(file_flag, true);
}
    // test the contents of the file

#[test]
fn test_create_cache_file() {
    jamdocs::init::create_cache();
    let file_flag = Path::new(&TEST_CACHE_FILE_PATH).exists();
    assert_eq!(file_flag, true);
}
    // test the contents of the file