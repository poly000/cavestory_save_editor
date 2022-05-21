use std::process::exit;

use cavestory_save_editor::{Profile, INVENTORY, WEAPON};
use inquire::{Select, Text};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path_in = Text::new("Path of the profile#.dat");
    path_in.default = Some("profile.dat");
    let path = &path_in.prompt()?;

    let mut profile = Profile::new(path)?;

    loop {
        let option = Select::new(
            ": ",
            vec!["Health", "Weapon", "Inventory", "* Save", "* Exit"],
        )
        .prompt()?;
        match option {
            "Health" => {
                profile.set_current_health(type_int_value("current: ")?);
                profile.set_max_health(type_int_value("max: ")?)
            }
            "Weapon" => {
                let slot = type_int_value("slot to edit:")? as isize;

                println!("Weapon Type");
                profile.set_weapon_type(select_id(&WEAPON)?, slot);

                let level = type_int_value("level: ")?;
                profile.set_weapon_level(level, slot);

                let exp_level = type_int_value("exp level: ")?;
                profile.set_weapon_exp(exp_level, slot);

                let ammo = type_int_value("ammo: ")?;
                profile.set_weapon_ammo(ammo, slot);

                let max_ammo = type_int_value("max ammo: ")?;
                profile.set_weapon_max_ammo(max_ammo, slot);
            }
            "* Save" => profile.write_to(path)?,
            "* Exit" => exit(0),
            _ => println!("Unimplemented choice!"),
        }
    }
}

fn type_int_value(tip: &str) -> Result<i32, Box<dyn std::error::Error>> {
    loop {
        if let Ok(n) = Text::new(tip).prompt()?.trim().parse::<i32>() {
            return Ok(n);
        }
    }
}

fn select_id(list: &Vec<&str>) -> Result<i32, Box<dyn std::error::Error>> {
    let opt = Select::new("select the new value: ", list.clone()).prompt()?;
    Ok(list
        .iter()
        .enumerate()
        .find_map(|(i, &op)| if op == opt { Some(i) } else { None })
        .ok_or_else(|| "cannot find this option!")? as i32)
}
