use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};
use std::{
    collections::HashMap,
    env::{args, set_var, var},
    fs::read_to_string,
};

fn main() -> color_eyre::Result<()> {
    for (k, v) in [("RUST_BACKTRACE", "full"), ("RUST_LIB_BACKTRACE", "full")] {
        if var(k).is_err() {
            set_var(k, v);
        }
    }
    color_eyre::install()?;

    let mut args = args().skip(1);

    let input: Vec<String> = {
        if let Some(path) = args.next() {
            read_to_string(path)? //If a path is passed in, read from that
        } else {
            let mut s = String::new(); //Else make a buffer string
            std::io::stdin().read_line(&mut s)?; //Read from stdin
            s
        }
        .lines()
        .flat_map(|line| line.split(' '))
        .filter(|el| !el.contains(')') && !el.is_empty()) //remove numbering
        .map(|el| el.trim().into())
        .collect()
    };

    let mut hm: HashMap<String, u64> = HashMap::new(); //Make a new HashMap for storing the entries and their counts
    input.into_iter()
        .for_each(|line| *hm.entry(line).or_default() += 1); //For each bit, get a reference to an entry for the current kind, or the default and add one.

    let total = args.next().and_then(|st| st.parse::<u64>().ok()).unwrap_or_else(|| hm.values().sum::<u64>()) as f32; //get the total number of entries of each element as an f32, either from the args or from the list
    let no_rows = hm.keys().len() as f32; //same but for the number of string kinds
    let mut rows: Vec<(String, u64, String)> = hm
        .into_iter()
        .map(|(key, no)| {
            (
                format!("\"{key}\""),
                no,
                format!("{:.1}%", (no as f32) / total * 100.0),
            )
        })
        .collect(); //make all of the rows
    rows.sort_by(|(_, a, _), (_, b, _)| b.cmp(a)); //sort based on the key
    let rows: Vec<Vec<String>> = rows
        .into_iter()
        .map(|(k, n, p)| vec![k, n.to_string(), p])
        .collect(); //make it all good for the table

    let mut table = Table::new(); //new table
    table //set all table prefs
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);
    table //add content
        .set_header(vec![
            format!("Val / {no_rows}"),
            format!("# / {total} / {:.1}", total / no_rows),
            format!("% / {:.1}%", no_rows.powf(-1.0) * 100.0),
        ])
        .add_rows(rows);

    println!("{table}");

    Ok(())
}
