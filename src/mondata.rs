use std::collections::HashMap;

pub type MonInfo = HashMap<String, String>;

pub fn parse_mon_info(input: &str) -> Option<MonInfo> {
    let mut info: MonInfo = HashMap::new();

    let infobox = {
        let start = input.find("{{Pokémon Infobox")?;
        let end = start + input[start..].find("}}")?;
        &input[start..end]
    };

    for entry in infobox.split("|").skip(1) {
        let pair: Vec<&str> = entry.split("=").collect();
        info.insert(pair[0].trim().to_string(), pair[1].trim().to_string());
    }

    Some(info)
}

#[derive(Default)]
pub struct MonStats {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub spa: u8,
    pub spd: u8,
    pub spe: u8,
}

pub fn parse_mon_stats(input: &str) -> Option<MonStats> {
    let mut stats: MonStats = Default::default();

    let statsbox = {
       let start = input.find("{{Stats")?;
       let end = start + input[start..].find("}}")?;
       &input[start..end]
    };
    
    for entry in statsbox.split("|").skip(1) {
        let pair: Vec<&str> = entry.split('=').collect();
        match pair[0] {
            "HP" => { stats.hp = str::parse::<u8>(pair[1].trim()).unwrap(); },
            "Attack" => { stats.atk = str::parse::<u8>(pair[1].trim()).unwrap(); },
            "Defense" => { stats.def = str::parse::<u8>(pair[1].trim()).unwrap(); },
            "SpAtk" => { stats.spa = str::parse::<u8>(pair[1].trim()).unwrap(); },
            "SpDef" => { stats.spd = str::parse::<u8>(pair[1].trim()).unwrap(); },
            "Speed" => { stats.spe = str::parse::<u8>(pair[1].trim()).unwrap(); },
            _ => {},
        }
    }

    Some(stats)
}
