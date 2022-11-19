use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::{ContentArrangement, Table};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string(std::env::args().nth(1).unwrap_or(String::from("input.txt")))?; //read from a file in CLI arguments, falling back to "input.txt"
    let mut hm: HashMap<String, u64> = HashMap::new(); //Make a new HashMap for storing the entries and their counts
    input.lines().map(|line| line.trim().into()).for_each(|line| *hm.entry(line).or_default() += 1); //For each line, all trimmed to remove extra spaces, and converted back to Strings to avoid ownership faffery. Then, in each iteration get a reference to an entry for the current kind, or the default and add one.

    let total = hm.values().sum::<u64>() as f32; //get the total number of entries of each element as an f32
    let no_rows = hm.keys().len() as f32; //same but for the number of string kinds
    let mut rows: Vec<(String, u64, String)> = hm.into_iter().map(|(key, no)| (key, no, format!("{:.1}%", (no as f32) / total * 100.0))).collect(); //make all of the rows
    rows.sort_by(|a, b| b.1.cmp(&a.1)); //sort based on the key
    let rows: Vec<Vec<String>> = rows.into_iter().map(|(k, n, p)| vec![k, n.to_string(), p]).collect(); //make it all good for the table

    let mut table = Table::new(); //new table
    table //set all table prefs
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic); 
    table //add content
        .set_header(vec![format!("Val / {no_rows}"), format!("# / {total} / {:.1}", total / no_rows), format!("% / {:.1}%", no_rows.powf(-1.0) * 100.0)])
        .add_rows(rows);

    println!("{table}");

    Ok(())
}
