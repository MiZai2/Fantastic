use usdpl_back::api::files::*;
use std::process::Command;

pub fn read_fan() -> Option<u64> {
    let mut path = std::env::current_exe().ok()?;
    path.pop();
    path.push("sh_tools.sh");
    let str_path = path.to_str().unwrap();
    //log::info!("dir: {}",str_path);

    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("{} --read_fan",str_path))
        .output()
        .unwrap();
    let out = String::from_utf8(output.stdout).unwrap();
    log::debug!("fan: {}",out);
    out.trim().parse::<u64>().ok()

    //read_single("/sys/class/hwmon/hwmon4/fan1_input").ok()
}

pub fn read_thermal_zone(index: u8) -> Option<u64> {
    let mut path = std::env::current_exe().ok()?;
    path.pop();
    path.push("sh_tools.sh");
    let str_path = path.to_str().unwrap();

    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("{} --read_thermal_zone",str_path))
        .output()
        .unwrap();
    let out = String::from_utf8(output.stdout).unwrap();
    log::debug!("temp: {}",out);
    out.trim().parse::<u64>().ok()

    //read_single(format!("/sys/class/thermal/thermal_zone{}/temp", index)).ok()
    //read_single("/sys/class/drm/card0/device/hwmon/hwmon5/temp1_input").ok()
}

/*pub fn write_fan_recalc(enabled: bool) -> Result<(), std::io::Error> {
    write_single("/sys/class/hwmon/hwmon4/pwm1_enable", enabled as u8)
}

pub fn write_fan_target(rpm: u64) -> Result<(), std::io::Error> {
    write_single("/sys/class/hwmon/hwmon4/pwm1", rpm)
}*/

pub fn write_fan_recalc2(enabled: bool) {
    let mut path = std::env::current_exe().ok().expect("REASON");
    path.pop();
    path.push("sh_tools.sh");
    let str_path = path.to_str().unwrap();

    Command::new("bash")
        .arg("-c")
        .arg(format!("{} --write_fan_recalc {}",str_path,if(enabled){1}else{0}))
        .output()
        .unwrap();
}

pub fn write_fan_target2(rpm: u64) {
    let mut path = std::env::current_exe().ok().expect("REASON");
    path.pop();
    path.push("sh_tools.sh");
    let str_path = path.to_str().unwrap();

    Command::new("bash")
        .arg("-c")
        .arg(format!("{} --write_fan_target {}",str_path,rpm))
        .output()
        .unwrap();
}
