use {
    curseforge::models::*,
    
    colored::*,
};

#[inline]
pub fn format_mod(
    entry: &ModEntry
) -> String {
    let size = termsize::get().unwrap_or(
        termsize::Size {
            rows: 20,
            cols: 20
        }
    );

    let cols = size.cols as usize;
    let separator = format!("{}", "-".cyan()).repeat(cols - 1usize);
    let mut summary = entry.summary.clone();

    if summary.len() > cols + 13 {
        summary = format!("{}...", &summary[..(cols - 17)]);
    }

    format!(
        concat!(
            "+{}\n",
            "|    Name: {} (id: {})\n",
            "|    Summary: {}\n",
            "|    CurseForge page: {}"
        ),

        separator,
        entry.name.green(),
        entry.id.to_string().red(),  // Important to display
        summary,
        entry.curseforge_url.bright_blue(),
    )
}
