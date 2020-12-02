use std::fs::File;
use std::io::{Read, Write};

const ARCROP_CONFIG_PATH: &str = "sd:/atmosphere/contents/01006A800016E000/romfs/arcropolis.toml";

/* 
Since arcrop uses its config file to determine the current workspace we need to edit that if we want the HDR romfs to be in a workspace.
Since the config file is only created after arcrop runs, I have no guarentee that it will exist when this code runs.   :/
Going to wait for the rewrite/API before making our romfs a workspace. 
This is dead code for now
*/
#[allow(dead_code)]
pub fn set_default_arcrop_umm_path() -> Result<(), std::io::Error> {
    println!("Setting arcropolis default workspace directory to 'ultimate/mods/HDR'");
        
    /* Read arcrop config file */
    let mut src = File::open(&ARCROP_CONFIG_PATH)?;
    let mut data = String::new();
    src.read_to_string(&mut data)?;
    drop(src);

    /* Edit data in arcrop config */
    let mut new_data: String = String::new();
    for mut line in data.lines() {
        if line.contains("umm") {
            line = "umm = 'sd:/ultimate/HDR'";
        }
        new_data.push_str(&(line.to_owned() + "\n"));
    }

    /* Write edited data to arcrop config file */
    let mut dst = File::create(&ARCROP_CONFIG_PATH)?;
    dst.write(new_data.as_bytes())?;

    Ok(())
}