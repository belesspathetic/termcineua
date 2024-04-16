use crossterm::terminal::{self, ClearType};
use crossterm::Command;
use std::collections::HashSet;
use std::{fs, io};

use termcineua::config::{Cartoon, Category};
use termcineua::mode::{download, main_prompt, watch};

fn read_config() -> anyhow::Result<Vec<Cartoon>> {
    let contents = std::fs::read_to_string("config.json")?;
    let cartoons: Vec<Cartoon> = serde_json::from_str(&contents)?;
    Ok(cartoons)
}

fn unique_titles(cartoons: &Vec<Cartoon>) -> Vec<(usize, String)> {
    let unique_titles: HashSet<_> = cartoons
        .iter()
        .filter(|&c| c.category == Category::Cartoon)
        .map(|c| c.title.clone())
        .collect();
    let mut sorted_titles: Vec<_> = unique_titles.into_iter().collect();
    sorted_titles.sort();
    let indexed: Vec<_> = sorted_titles.into_iter().enumerate().collect();
    indexed
}

fn print_unique_titles(indexed: &Vec<(usize, String)>) {
    clear_terminal();
    for (i, t) in indexed {
        println!("{}. {}", i + 1, t);
    }
}

fn get_user_index() -> usize {
    let mut chosen_index = String::new();
    io::stdin()
        .read_line(&mut chosen_index)
        .expect("Failed to read input");
    let parsed_index = chosen_index.trim().parse::<usize>().unwrap();
    parsed_index
}

fn seasons(
    parsed_index: usize,
    indexed: &Vec<(usize, String)>,
    cartoons: &Vec<Cartoon>,
) -> Vec<u8> {
    if let Some((_, chosen_title)) = indexed.get(parsed_index - 1) {
        let seasons: HashSet<u8> = cartoons
            .iter()
            .filter(|c| c.title == *chosen_title)
            .filter_map(|c| c.season)
            .collect();
        let mut sorted: Vec<_> = seasons.into_iter().collect();
        sorted.sort();
        return sorted;
    } else {
        return vec![];
    }
}

fn print_seasons(seasons: &Vec<u8>) {
    clear_terminal();
    for (i, season) in seasons.into_iter().enumerate() {
        println!("{}. Сезон {}", i + 1, season)
    }
}

fn serias(
    parsed_index: usize,
    parsed_season_index: usize,
    indexed: &Vec<(usize, String)>,
    cartoons: &Vec<Cartoon>,
) -> Vec<(u8, std::string::String, std::string::String)> {
    if let Some((_, chosen_title)) = indexed.get(parsed_index - 1) {
        let chosen_season = seasons(parsed_index, indexed, cartoons)[parsed_season_index - 1];

        let mut serias: Vec<(u8, String, String)> = cartoons
            .iter()
            .filter(|c| c.title == *chosen_title && c.season == Some(chosen_season))
            .map(|c| {
                (
                    c.seria.unwrap(),
                    c.seria_title.clone().unwrap_or_else(|| "".to_string()),
                    c.stream.clone(),
                )
            })
            .collect();

        serias.sort_by_key(|&(num, _, _)| num);

        return serias;
    } else {
        return vec![];
    }
}

fn print_serias(serias: &Vec<(u8, String, String)>) {
    clear_terminal();
    for (i, &(num, ref name, _)) in serias.iter().enumerate() {
        println!("{}. Серія {}: {}", i + 1, num, name);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = main_prompt();

    match input.as_str() {
        "1" => {
            let cartoons = read_config()?;
            let indexed = unique_titles(&cartoons);
            print_unique_titles(&indexed);

            println!("Please enter the number of the cartoon:");

            let parsed_index = get_user_index();
            let seasons = seasons(parsed_index, &indexed, &cartoons);
            print_seasons(&seasons);

            println!("Please enter the number of the season:");

            let parsed_season_index = get_user_index();

            let serias = serias(parsed_index, parsed_season_index, &indexed, &cartoons);
            print_serias(&serias);

            let choice = get_user_index() - 1;
            let mut _stream = String::new();

            if let Some((_, _, s)) = &serias.get(choice) {
                _stream = s.clone()
            }

            println!("Будь ласка виберіть опцію:\n1. Дивитися зараз\n2. Завантажити кіно (потрібен ffmpeg)");
            let option = get_user_index();

            match option {
                1 => {
                    watch(_stream).await?;
                }
                2 => {
                    download(_stream).await?;
                }
                _ => {
                    println!("Ведіть число")
                }
            }
        }
        "0" => {
            let cartoons = Cartoon::scrape_cartoons().await?;
            let json = serde_json::to_string_pretty(&cartoons)?;
            fs::write("config.json", json)?;
        }
        _ => {}
    }
    Ok(())
}

fn clear_terminal() {
    terminal::Clear(ClearType::All)
        .execute_winapi()
        .expect("Failed to clean terminal");
}
