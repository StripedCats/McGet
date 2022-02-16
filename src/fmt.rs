use {
    curseforge::models::*,
    
    std::{
        path::Path,
        io::{self, BufRead}
    },
    colored::*,
};

#[inline]
pub fn ask_for(choices: &[&'static str]) -> usize {
    for (index, choice) in choices.iter().enumerate() {
        println!("{}) {}", index+1, choice.green());
    }

    let selected;
    loop {
        let stdin = io::stdin();
        let line = stdin.lock()
                        .lines()
                        .next()
                        .unwrap()
                        .unwrap();
        let opt = match line.parse::<i32>() {
            Ok(r) => r,
            Err(_) => { continue; }
        } - 1;

        if (opt < 0) || (opt >= choices.len() as i32) {
            println!("Try again");
            continue;
        }

        selected = opt as usize;
        break;
    }

    selected
}

#[inline]
pub fn to_yaml_ext(src: &str) -> String {
    let ext = Path::new(src).extension();

    if ext.is_none() {
        format!("{}.yaml", src)
    } else {
        src.to_owned()
    }
}

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
