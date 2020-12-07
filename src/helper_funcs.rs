use std::{fs, path::Path};

pub const SKYLINE_PLUGIN_DIR: &str = "sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins";

const FOLDERS_TO_CHECK_FOR_CLEANING: [&str; 3] = [
    "sd:/atmosphere/contents/0000000000534C56",
    "sd:/SaltySD",
    "sd:/atmosphere/contents/01006A800016E000/romfs/data.arc",
];

/* Master function for calling other cleaning tasks */
pub fn clean() {
    /* If we're in the middle of installing, don't clean */
    if Path::new("sd:/installing.tmpfile").exists() {
        return;
    }

    let needs_cleaning = /*should_disable_other_plugins() ||*/ FOLDERS_TO_CHECK_FOR_CLEANING.iter().any(|&x| Path::new(x).exists());

    if needs_cleaning {
        if skyline_web::Dialog::yes_no(
            "HDR-Installer detected problematic files. These could be data.arc mods, or SaltySD mods.
            These may cause unintended behavior for HDR. Would you like to clean up potentially conflicting mods?
            Please note this will DELETE SaltySD directories, and DISABLE your data.arc file.
            If you aren't sure, select Yes."
        ) {
            remove_saltysd();
            //disable_other_plugins();
            disable_data_arc();
        }
    }
}


/* Plugin names that we *shouldn't* disable */
const HDR_PLUGIN_NAMES: [&str; 5] = [
    "libHDR.nro",
    "libarcropolis.nro",
    "libacmd_hook.nro",
    "libnro_hook.nro",
    "libHDR_Installer.nro"
];
/* Goes through the plugin directory and if any plugins exist that aren't the ones we specify above, 
    move them to the "disabled_plugins" folder (<- used by arcadia for enabling/disabling plugins) 
*/
fn disable_other_plugins() {
    println!("[HDR Installer] Disabling unnecessary skyline plugins...");
    for readdir in fs::read_dir(SKYLINE_PLUGIN_DIR) {
        for entry in readdir.map(|x| x.unwrap()) {
            let entry_path = Path::new(SKYLINE_PLUGIN_DIR).join(entry.path());
            let dst = entry_path.to_str().unwrap().replace("plugins", "disabled_plugins");
            let dst = Path::new(&dst);
            if !HDR_PLUGIN_NAMES.contains(&entry_path.file_name().unwrap().to_str().unwrap()) {
                if !dst.parent().unwrap().exists() {
                    let _ = std::fs::create_dir_all(dst.parent().unwrap());
                }
                let _ = fs::rename(entry_path.to_str().unwrap(), dst);
            }
            else {
                if dst.exists() {
                    let _ = std::fs::remove_file(dst);
                }
            }
        }
    }
}
/* If there are any plugins that aren't the ones we allow... i kinda forgot why I wrote this instead of just.. cleaning them up..... lol */
fn should_disable_other_plugins() -> bool {
    for readdir in fs::read_dir(SKYLINE_PLUGIN_DIR) {
        for entry in readdir.map(|x| x.unwrap()) {
            let entry_path = entry.path();
            if !HDR_PLUGIN_NAMES.contains(&entry_path.file_name().unwrap().to_str().unwrap()) {
                return true;
            }
        }
    }
    false
}
/* Renames the "data.arc" to ".data.arc" which effectively disables it. */
fn disable_data_arc() {
    println!("[HDR Installer] Disabling data.arc file...");
    let arc_path = Path::new("sd:/atmosphere/contents/01006A800016E000/romfs/data.arc");
    if arc_path.exists() {
        let dst = arc_path.to_str().unwrap().replace("data.arc", ".data.arc");
        let _ = fs::rename(arc_path, dst);
    }
    else {
        println!("[HDR Installer] data.arc file not found... continuing...");
    }
}
/* Removes folder at sd:/SaltySD and at sd:/atmosphere/contents/0000000000534C56 (Salty user folder and sysmodule folder) */
fn remove_saltysd() {
    /* Check if SaltySD/SaltySD mods exist, and if so, prompt to remove them */
    println!("[HDR Installer] Removing SaltySD...");
    let saltysd_sysmodule = Path::new("sd:/atmosphere/contents/0000000000534C56");
    let saltysd_root = Path::new("sd:/SaltySD");

    if saltysd_sysmodule.exists() || saltysd_root.exists() {
            
        match fs::remove_dir_all(saltysd_sysmodule) {
            Ok(()) => println!("[HDR Installer] Removed SaltySD sysmodule"),
            Err(e) => println!("[HDR Installer] Failed to remove SaltySD sysmodule: {}", e)
        };
        match fs::remove_dir_all(saltysd_root) {
            Ok(()) => println!("[HDR Installer] Removed SaltySD root folder"),
            Err(e) => println!("[HDR Installer] Failed to remove SaltySD root folder: {}", e)
        };

    }
    else {
        println!("[HDR Installer] SaltySD installation not found... continuing...");
    }
}
/* Seppuku */
pub fn suicide() {
    /* Skyline plugin suicide */
    match fs::remove_file(SKYLINE_PLUGIN_DIR.to_owned() + "/libHDR_Installer.nro") {
        Ok(_) => println!("[HDR Installer] Sucessfully cleaned up"),
        Err(e) => println!("[HDR Installer] Failed to remove HDR Installer: {}", e),
    }
}