use clap::{App, load_yaml};

mod mondata;
mod net;
mod format;

fn main() {
    let yaml = load_yaml!("args.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let wikitext = if let Some(text) = net::get_page_contents(matches.value_of("target").unwrap()) {
        text
    } else {
        println!("Invalid pokemon");
        return;
    };

    let moninfo = if let Some(text) = mondata::parse_mon_info(&wikitext){
        text
    } else {
        println!("Invalid pokemon");
        return;
    };
    let monstats = mondata::parse_mon_stats(&wikitext);

    format::mon_default(&moninfo, monstats)
}
