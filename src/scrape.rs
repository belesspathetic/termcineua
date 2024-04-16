use async_recursion::async_recursion;
use thirtyfour::prelude::*;

use crate::{
    browser::{close, start},
    config::{Cartoon, Category},
};

impl Cartoon {
    pub async fn scrape_cartoons() -> anyhow::Result<Vec<Cartoon>> {
        let mut cartoons: Vec<Cartoon> = Vec::new();
        let browser = start().await?;
        browser.driver.goto("https://simpsonsua.tv/").await?;

        let hrefs = Self::get_figures_href(&browser.driver).await;
        Self::get_to_root(&browser.driver, hrefs, &mut cartoons).await;
        close(browser).await?;
        Ok(cartoons)
    }
    async fn get_figures_href(driver: &WebDriver) -> Vec<String> {
        let mut hrefs: Vec<String> = Vec::new();
        let figures = driver
            .find_all(By::Tag("figure"))
            .await
            .expect("Failed to find figure");

        for figure in figures {
            let a = figure
                .find(By::Tag("a"))
                .await
                .expect("Failed to find 'a' tag");
            let href = a
                .attr("href")
                .await
                .expect("Failed to get href attr")
                .unwrap();
            hrefs.push(href)
        }
        hrefs
    }
    #[async_recursion]
    async fn get_to_root(driver: &WebDriver, hrefs: Vec<String>, buf: &mut Vec<Cartoon>) {
        for href in hrefs {
            driver.goto(href).await.expect("Failed to reach href");
            let iframe = driver.find(By::Id("Player1")).await;
            match iframe {
                Ok(_) => {
                    match Self::get_player(&driver).await {
                        Ok((t, st, d, s)) => {
                            let (season, seria, title) = Self::get_season_and_seria_and_title(t);
                            driver.goto(s).await.expect("Failed to reach stream page");
                            let video = driver.find(By::Tag("video")).await;

                            let v: WebElement = match video {
                                Ok(v) => v,
                                Err(_) => continue,
                            };

                            let stream = v
                                .attr("src")
                                .await
                                .unwrap()
                                .expect("Failed to get 'src' attr");

                            buf.push(Cartoon {
                                category: Category::Cartoon,
                                title: title,
                                seria_title: Some(st),
                                season: Some(season),
                                seria: Some(seria),
                                description: Some(d),
                                stream: stream,
                            });
                        }
                        Err(_) => continue,
                    };
                }
                Err(_) => {
                    let hrefs = Self::get_figures_href(driver).await;
                    Self::get_to_root(driver, hrefs, buf).await
                }
            }
        }
    }

    async fn get_player(driver: &WebDriver) -> anyhow::Result<(String, String, String, String)> {
        let seria_title = driver
            .find(By::Tag("h2"))
            .await
            .expect("Failed to find 'h2' tag")
            .text()
            .await
            .expect("Failed to convert 'h2' to text");
        let info = driver
            .find_all(By::ClassName("fullstory"))
            .await
            .expect("Failed to find 'fullstory' class");
        let title = match info.get(0) {
            Some(t) => t.text().await?,
            None => "title".to_string(),
        };
        let description = match info.get(1) {
            Some(d) => d.text().await?,
            None => "description".to_string(),
        };
        let player = driver
            .find(By::Id("Player1"))
            .await
            .expect("Failed to find 'Player1' id");
        let over = player.find(By::Id("overroll1")).await;
        let mut _iframe = String::new();

        match over {
            Ok(o) => {
                _iframe = o
                    .find(By::Tag("iframe"))
                    .await
                    .expect("Failed to find 'iframe' tag")
                    .attr("src")
                    .await
                    .expect("Failed to get 'src' attr")
                    .unwrap();
            }
            Err(_) => {
                let player = driver
                    .find(By::Id("Player2"))
                    .await
                    .expect("Failed to find 'Player2' id");
                let over = player.find(By::Id("overroll2")).await?;
                _iframe = over
                    .find(By::Tag("iframe"))
                    .await
                    .expect("Failed to find 'iframe' tag")
                    .attr("src")
                    .await
                    .expect("Failed to get 'src' attr")
                    .unwrap();
            }
        }

        Ok((title, seria_title, description, _iframe))
    }

    fn get_season_and_seria_and_title(title: String) -> (u8, u8, String) {
        let numbers: String = title
            .replace(|c: char| !c.is_numeric(), " ")
            .trim()
            .to_string();
        let parts: Vec<&str> = numbers.split_whitespace().collect();
        println!("parts {:?}", parts);
        let mut _season = 0_u8;
        let mut _seria = 0_u8;

        if parts.len() >= 2 {
            _season = parts[0].parse::<u8>().unwrap();
            _seria = parts[1].parse::<u8>().unwrap();
        } else {
            _season = 99_u8;
            _seria = 99_u8;
        }

        let mut _new_title = String::new();
        if title.contains("Губка Боб") || title.contains("ГУБКА БОБ") {
            _new_title = "Губка Боб".to_string();
        } else if title.contains("Сімпсони") || title.contains("Сімпсон") {
            _new_title = "Сімпсони".to_string();
        } else if title.contains("Гріфіни") {
            _new_title = "Гріфіни".to_string()
        } else if title.contains("Готель Хазбін") {
            _new_title = "Готель Хазбін".to_string()
        } else if title.contains("Дунканвілл") || title.contains("Дунканвілль")
        {
            _new_title = "Дунканвілль".to_string()
        } else if title.contains("БоДжек") {
            _new_title = "Кінь БоДжек".to_string()
        } else if title.contains("Ґравіті Фолз") {
            _new_title = "Ґравіті Фолз".to_string()
        } else if title.contains("Південний Парк")
            || title.contains("Картманові")
            || title.contains("Саус Парку")
        {
            _new_title = "Південний Парк".to_string();
        } else if title.contains("Зім") {
            _new_title = "Завойовник Зім".to_string()
        } else if title.contains("Роко")
            || title.contains("Сучасне Рокове Життя")
            || title.contains("Сучасне рокове життя")
        {
            _new_title = "Сучасне Рокове Життя".to_string();
        } else if title.contains("Футурама") {
            _new_title = "Футурама".to_string();
        } else {
            let words: Vec<String> = title
                .chars()
                .filter(|c| c.is_alphabetic() || c.is_whitespace())
                .map(|c| c.to_string())
                .collect();
            _new_title = words
                .join("")
                .replace(" сезон ", "")
                .replace("серія", "")
                .replace("українською", "")
                .trim()
                .to_string();
        }

        (_season, _seria, _new_title)
    }
}
