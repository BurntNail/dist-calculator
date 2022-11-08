use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::{ContentArrangement, Table};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;
    let mut hm: HashMap<String, u64> = HashMap::new();
    for line in input.lines().map(|line| line.trim().into()) {
        let e = hm.entry(line).or_default();
        *e += 1;
    }

    let total = hm.values().sum::<u64>() as f32;
    let no_rows = hm.keys().len() as f32;
    let mut rows: Vec<(String, u64, String)> = hm.into_iter().map(|(key, no)| (key, no, format!("{:.1}%", (no as f32) / total * 100.0))).collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1));
    let rows: Vec<Vec<String>> = rows.into_iter().map(|(k, n, p)| vec![k, n.to_string(), p]).collect();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![format!("Val / {no_rows}"), format!("# / {total} / {:.1}", total / no_rows), format!("% / {:.1}%", no_rows.powf(-1.0) * 100.0)]);
    table.add_rows(rows);

    println!("{table}");

    Ok(())
}
