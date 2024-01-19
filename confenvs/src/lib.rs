use std::path::{Path, PathBuf};

const SYS_PATH_STR : &'static str = std::env!("CFCOMMON_SYSTEM_PATH", "You should set CFCOMMON_SYSTEM_PATH with the path of system-wide configuration directory path");
const PER_USR_STR : &'static str = std::env!("CFCOMMON_USER_REL_PATH", "You should set CFCOMMON_USER_REL_PATH with the path of user configuration directory relative path");


pub fn get_system_dir(pkgname : &str) -> Result<PathBuf, String> {
    let mut sys_path = Path::new(SYS_PATH_STR).canonicalize().map_err(|x|  x.to_string())?;

    let awpath = Path::new(pkgname);
    if awpath.has_root() {
        return Err("Package name can't contain roots".to_string());
    }
    sys_path.push(awpath);
    Ok(sys_path)
}

pub fn get_user_dir(pkgname : &str) -> Result<PathBuf, String> {
    let mut us_dir = home::home_dir().ok_or("Unable to get the home directory".to_string())?;
    us_dir.push(PER_USR_STR);

    let awpath = Path::new(pkgname);
    if awpath.has_root() {
        return Err("Package name can't contain roots".to_string());
    }

    us_dir.push(awpath);
    Ok(us_dir)
}
/*
#[cfg(test)]
mod tests{
    #[test]
    fn path_test() {
        use crate::{get_system_dir, get_user_dir};

        println!("{}", get_system_dir("man").unwrap().display());
        println!("{}", get_user_dir("man").unwrap().display());
    }
}
*/
