#![feature(proc_macro_hygiene)]
#![allow(non_snake_case)]

use std::{fs, path::Path};
use skyline::nn::oe::*;

#[skyline::main(name = "HDR_Installer")]
pub fn main() {

    // SPEEEEEEEEEEEED BOOOOST
    unsafe { SetCpuBoostMode(CpuBoostMode::Boost); }

    remove_saltysd();

    if !Path::new("sd:/is_restart_installing.txt").exists() {
        skyline_web::DialogOk::ok(format!("HDR will now be installed. This is a first-time setup and will take some time. The screen will be blank for a while, but don't worry, this is normal. Please be patient. On the next screen you'll be prompted for an update, please press 'Yes'."));
    }

    let mut should_install = false;

    for mut f in fs::read_dir("sd:/ultimate/mods") {
        if !f.any(|x| x.unwrap().file_name().to_str().unwrap().contains("HDR-Base")) {
            should_install = true;
        }
    }
    for mut f in fs::read_dir("sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins") {
        if !f.any(|x| x.unwrap().file_name().to_str().unwrap().contains("HDR.nro")) {
            should_install = true;
        }
    }

    if should_install {
        // Check if an update is available
        println!("[HDR_Installer] Checking update server...");
        if skyline_update::check_update("3.17.96.120".parse().unwrap(), "HDR", env!("CARGO_PKG_VERSION"), false) {
            println!("[HDR_Installer] Installed HDR!");
            suicide();
            unsafe { SetCpuBoostMode(CpuBoostMode::Disabled); }
            RestartProgramNoArgs();
        }
    }

    unsafe { SetCpuBoostMode(CpuBoostMode::Disabled); }
}






fn suicide() {
    /* Skyline plugin suicide */
    
    match fs::remove_file("sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libHDR_Installer.nro") {
        Ok(_) => println!("[HDR Installer] Sucessfully cleaned up"),
        Err(e) => println!("[HDR Installer] Failed to remove HDR Installer: {}", e)
    }
}

fn remove_saltysd() {
    /* Check if SaltySD/SaltySD mods exist, and if so, prompt to remove them */

    let saltysd_sysmodule = Path::new("sd:/atmosphere/contents/0000000000534C56");
    let saltysd_root = Path::new("sd:/SaltySD");

    if saltysd_sysmodule.exists() || saltysd_root.exists() {

        if skyline_web::Dialog::yes_no(format!("SaltySD detected on SD card. This might have unintended side-effects on HDR and other skyline mods. Would you like to delete them now? (Highly recommended)")) {
            
            match fs::remove_dir_all(saltysd_sysmodule) {
                Ok(()) => println!("[HDR Installer] Removed SaltySD sysmodule"),
                Err(e) => println!("[HDR Installer] Failed to remove SaltySD sysmodule: {}", e)
            };
            match fs::remove_dir_all(saltysd_root) {
                Ok(()) => println!("[HDR Installer] Removed SaltySD root folder"),
                Err(e) => println!("[HDR Installer] Failed to remove SaltySD root folder: {}", e)
            };

        }

    }
}