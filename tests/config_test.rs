use termcineua::{
    browser::{close, start, Browser},
    config::{Cartoon, Category},
};
use thirtyfour::prelude::*;

async fn setup() -> Browser {
    let browser = start().await.expect("Failed to launch webdriver");
    browser
}

#[tokio::test]
async fn config() {
    let case = Cartoon { category: Category::Cartoon, title: "Розчарування".to_string(), seria_title: Some("КЛОПІТ НА ГОЛОВУ".to_string()), season: Some(5), seria: Some(1), description: Some("Королева Даґмар допитує жителів Дрімленду, щоб дізнатися, хто ховає тіло Бін. Тим часом Люцик відновлює стосунки з батьком.".to_string()), stream: "https://jk19ocmjeoyql3tj.ashdi.vip/video15/serials/disenchantment_s3/disenchantment_s03e01_heads_or_tails_webdl_1080p_hurtom_101050/hls/BKiBlHaKmPtengbhBI0=/index.m3u8".to_string() };
    let mut buf: Vec<Cartoon> = Vec::new();
    let browser = setup().await;
    let driver = &browser.driver;
    driver
        .goto("https://simpsonsua.tv/")
        .await
        .expect("Failed to reach site");
    multfilm(&driver, &mut buf).await;
    println!("BUFFER:\n{:?}", buf);
    // Tests
    assert_eq!(buf[0], case);

    close(browser).await.expect("Failed to close webdriver");
}

async fn multfilm(driver: &WebDriver, buf: &mut Vec<Cartoon>) {
    let hrefs = vec![
        "https://simpsonsua.tv/rozcharuvannya/".to_string(),
        "https://simpsonsua.tv/tuca-and-bertie/".to_string(),
    ];
    get_to_root(driver, hrefs, buf).await;
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

use async_recursion::async_recursion;

#[async_recursion]
async fn get_to_root(driver: &WebDriver, hrefs: Vec<String>, buf: &mut Vec<Cartoon>) {
    for href in hrefs {
        driver.goto(href).await.expect("Failed to reach href");
        let iframe = driver.find(By::Id("Player1")).await;
        match iframe {
            Ok(_) => {
                match get_player(&driver).await {
                    Ok((t, st, d, s)) => {
                        let (season, seria, title) = get_season_and_seria_and_title(t);
                        driver.goto(s).await.expect("Failed to reach stream page");
                        let video = driver
                            .find(By::Tag("video"))
                            .await
                            .expect("Failed to find 'video' tag");
                        let stream = video
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
                let hrefs = get_figures_href(driver).await;
                get_to_root(driver, hrefs, buf).await
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
    let title = info[0].text().await.expect("Failed to get info text");
    let deskription = info[1].text().await.expect("Failed to get info text");
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

    Ok((title, seria_title, deskription, _iframe))
}

fn get_season_and_seria_and_title(title: String) -> (u8, u8, String) {
    let numbers: Vec<String> = title
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_string())
        .collect();

    let season = numbers[0].parse::<u8>().unwrap();
    let seria = numbers[1].parse::<u8>().unwrap();
    let words: Vec<String> = title
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .map(|c| c.to_string())
        .collect();

    let new_title = words
        .join("")
        .replace(" сезон ", "")
        .replace("серія", "")
        .replace("українською", "")
        .trim()
        .to_string();

    (season, seria, new_title)
}
