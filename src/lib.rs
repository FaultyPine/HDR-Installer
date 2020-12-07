#![feature(proc_macro_hygiene)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod helper_funcs;
mod update_arcrop_config;

use skyline::nn::oe::*;
use std::{fs, path::Path};
use crate::helper_funcs::*;

const HDR_ROMFS_PATH: &str = "sd:/ultimate/mods";

#[skyline::main(name = "HDR_Installer")]
pub fn main() {
    // SPEEEEEEEEEEEED BOOOOST
    unsafe { SetCpuBoostMode(CpuBoostMode::Boost); }

    let mut should_install = false;

    /* Create arcrop UMM folder if it doesn't already exist */
    let HDR_workspace_folder_path = Path::new(HDR_ROMFS_PATH);
    if !HDR_workspace_folder_path.exists() {
        let _ = fs::create_dir_all(HDR_workspace_folder_path);
    }

    /* iterate through our UMM folder and our plugin folder */
    /* if either the HDR romfs OR the HDR plugin doesn't exist, we want to update */
    for mut f in fs::read_dir(HDR_workspace_folder_path) {
        if !f.any(|x| x.unwrap().file_name().to_str().unwrap().contains("HDR-Base")) {
            should_install = true;
        }
    }
    for mut f in fs::read_dir(SKYLINE_PLUGIN_DIR) {
        if !f.any(|x| x.unwrap().file_name().to_str().unwrap().contains("HDR.nro")) {
            should_install = true;
        }
    }

    let mut is_force_reinstall = false;
    /* If both the plugin and romfs are present, this plugin shouldn't really exist since it deletes itself. This means that the user probably wants to re-install so prompt for that here */
    if !should_install && !Path::new("sd:/installing.tmpfile").exists() {
        if skyline_web::Dialog::yes_no("HDR Installer present but a previous installation of HDR was detected. Would you like to force-reinstall?") {
            /* Tbh don't even really need to remove these files... since the update will overwrite the plugins and delete the romfs anyway ¯\_(ツ)_/¯ */
            let _ = fs::remove_file(SKYLINE_PLUGIN_DIR.to_owned() + "/libHDR.nro");
            let _ = fs::remove_dir_all(HDR_workspace_folder_path.join("HDR-Base"));
            should_install = true;
            is_force_reinstall = true;
        }
    }

    if should_install {

        /* Handle stuff like SaltySD deletion, data.arc disabling, and skyline plugin disabling */
        if !is_force_reinstall {
            clean();
        }

        /* If the "marker" for being in the middle of an install doesn't exist, prompt for the update */
        if !Path::new("sd:/installing.tmpfile").exists() && !is_force_reinstall {
            skyline_web::DialogOk::ok(
                "ATTENTION: HDR will now be installed. This is a first-time setup and will take some time.
                THE SCREEN WILL BE BLANK FOR A WHILE, but don't worry, THIS IS NORMAL. Please be patient. 
                On the next screen you'll be prompted for an update, please select 'Yes'."
            );
        }

        /* Check if an update is available */
        println!("[HDR Installer] Checking update server...");
        if skyline_update::check_update("157.230.67.115".parse().unwrap(), "HDR", env!("CARGO_PKG_VERSION"), false,) {
            println!("[HDR Installer] Installed HDR!");

            /* Update arcrop config file to point to workspace path... disabled until we can actually edit the config file from this plugin */
            /*match update_arcrop_config::set_default_arcrop_umm_path() {
                Ok(()) => println!("[HDR Installer] Set Arcropolis active workspace to 'ultimate/mods/HDR'"),
                Err(e) => println!("[HDR Installer] Failed to set arcropolis active workspace: {}", e)
            };*/

            /* Delete this plugin. Since plugins are loaded into memory, its not like we're killing this process immediately */
            suicide();
            unsafe { SetCpuBoostMode(CpuBoostMode::Disabled); }
            RestartProgramNoArgs();
        }
    }

    unsafe { SetCpuBoostMode(CpuBoostMode::Disabled); }
}