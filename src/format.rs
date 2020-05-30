use crate::mondata::{MonInfo, MonStats};

pub fn mon_default(mon: &MonInfo, stats: Option<MonStats>) {
    print!("{}", mon.get("name").unwrap_or(&"???".to_string()));
    if let Some(tmname) = mon.get("tmname") {
        print!(" ({})", tmname);
    }
    if let Some(category) = mon.get("category") {
        print!(": {} Pokemon", category);
    }
    print!("\n");

    print!("\tType: {}", mon.get("type1").unwrap_or(&"None".to_string()));
    if let Some(type2) = mon.get("type2") {
        print!(" / {}", type2);
    }
    print!("\n");

    print!("\tAbility: {}", mon.get("ability1").unwrap_or(&"None".to_string()));
    if let Some(ability2) = mon.get("ability2") {
        print!(" / {}", ability2);
    }
    if let Some(abilityd) = mon.get("abilityd") { 
        print!(" / {} (Hidden)", abilityd);
    }
    print!("\n");

    println!("\tHeight: {} ({} m)",
        mon.get("height-ftin").unwrap_or(&"0'00\"".to_string()),
        mon.get("height-m").unwrap_or(&"0 m".to_string())
    );
    
    println!("\tWeight: {} lbs ({} kg)",
        mon.get("weight-lbs").unwrap_or(&"0 lbs".to_string()),
        mon.get("weight-kg").unwrap_or(&"0 kg".to_string()),
    );
    
    if let Some(stats) = stats {
        println!("\t{} HP / {} ATK / {} DEF / {} SPA / {} SPD / {} SPE",
            stats.hp, stats.atk, stats.def, stats.spa, stats.spd, stats.spe
        );
    }
}
