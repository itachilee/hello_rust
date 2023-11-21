use scraper::{Html, Selector};
use std::collections::HashSet;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use url::Url;
use image::io::Reader as ImageReader;
use reqwest::header::CONTENT_TYPE;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

  
    // let url ="https://m.jcdf99.com/html/18312/";
    let mut visited_urls = HashSet::new(); // 用于存储已访问过的 URL
                                           // manga url
    let url = "http://imgapi.xl0408.top";

    let res = reqwest::get(url).await;

    match res {
        Ok(resp) => {
            if resp.status().is_success() {
                // let body = &resp.text().await.unwrap();

                // 检查响应的 Content-Type 是否为图片类型
                if let Some(content_type) = &resp.headers().get(CONTENT_TYPE) {
                    if let Ok(content_type_str) = content_type.to_str() {
                        if content_type_str.starts_with("image/") {
                            println!("Downloading image...");

                            // 使用 image 库读取图片数据
                            let image_data = &resp.bytes().await?;
                         
                            // 将图片保存到本地文件
                            let mut file = File::create("downloaded_image.jpg").await?;
                            // img.write_to(&mut file, image::ImageOutputFormat::Jpeg(100))?;


                            file.write_all(&image_data).await?;
                            // file.write_all(&image_data).await?;

                            println!("Image downloaded successfully.");
                        } else {
                            println!("The URL does not point to an image.");
                        }
                    }
                }

                // 使用 Scraper 解析页面内容，提取所需数据等操作

                // 记录已访问过的页面 URL
                visited_urls.insert(url.to_string());

                // 假设这里有一些爬取逻辑，处理页面内容

                // 这里可以添加延时，避免频繁请求
                // tokio::time::sleep(Duration::from_secs(5)).await;
            } else {
                println!("Failed to fetch the page: {}", url);
            }
        }
        Err(e) => {
            println!("err: {}", e);
        }
    }

    Ok(())
    // scrape_url(url).await
}

async fn scrape_url(root_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get(root_url).await?.text().await?;

    // res.status();

    let html = Html::parse_document(&res);

    let root_ref = html.root_element();
    let title = &root_ref
        .select(&Selector::parse(".nav_name>h1").unwrap())
        .next()
        .unwrap();
    assert_eq!(title.inner_html(), "将夜");

    let charpt_selector = Selector::parse(".p2>li>a").unwrap();
    let result: Vec<(String, &str)> = html
        .select(&charpt_selector)
        .rev()
        .map(|e| {
            if let Some(href) = e.value().attr("href") {
                (e.inner_html(), href)
            } else {
                (e.inner_html(), "")
            }
        })
        .collect();

    for item in result {
        println!("{} {} ", item.0, item.1);
    }

    let next_page_selector = Selector::parse(".right>.onclick").unwrap();

    let next_page_url = html
        .select(&next_page_selector)
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();

    scrape_charpter(&next_page_url).await?;
    println!("next page url is {}", next_page_url);

    Ok(())
}

async fn scrape_charpter(charpter_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://m.jcdf99.com/";
    let base = Url::parse(base_url)?;
    let url = base.join(charpter_url)?;
    println!("visiting page url: {}", url);
    // let chartper_content =reqwest::get(url).await?
    //     .text().await?;

    Ok(())
}
