// Print information on every native and non-native pstoedit driver.

use pstoedit::driver_info::DriverDescription;
use pstoedit::{DriverInfo, Result};
use std::collections::HashSet;

// Concatenate all the stuff this backend supports
fn support_string(driver: DriverDescription) -> String {
    let mut support = Vec::new();
    if driver.subpath_support() {
        support.push("subpaths");
    }
    if driver.curveto_support() {
        support.push("curveto");
    }
    if driver.merging_support() {
        support.push("merging");
    }
    if driver.text_support() {
        support.push("text");
    }
    if driver.image_support() {
        support.push("images");
    }
    if driver.multipage_support() {
        support.push("multiple pages");
    }
    support.join(", ")
}

// Print information on the driver, indented by four spaces
fn print_driver(driver: DriverDescription) -> Result<()> {
    println!("    Symbolic name:   {}", driver.symbolic_name()?);
    println!("    Extension:       {}", driver.extension()?);
    println!("    Explanation:     {}", driver.explanation()?);
    let info = driver.additional_info()?;
    if info.len() > 0 {
        println!("    Additional info: {}", driver.additional_info()?);
    }
    let support = support_string(driver);
    if support.len() > 0 {
        println!("    Support for:     {}", support);
    }
    Ok(())
}

fn main() -> Result<()> {
    pstoedit::init()?;

    // Print all native drivers
    let native_drivers = DriverInfo::get_native()?;
    let mut native_formats = HashSet::new();
    println!("Native drivers:");
    for driver in &native_drivers {
        native_formats.insert(driver.symbolic_name()?);
        print_driver(driver)?;
        println!("");
    }

    // Print all non-native drivers
    let drivers = DriverInfo::get()?;
    println!("Non-native drivers:");
    for driver in &drivers {
        if native_formats.contains(driver.symbolic_name()?) {
            continue;
        }
        print_driver(driver)?;
        println!("");
    }

    Ok(())
}
