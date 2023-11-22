

use reqwest::header::CONTENT_TYPE;
use std::collections::HashSet;
use tokio::fs::{File,OpenOptions};
use tokio::io::AsyncWriteExt;
use std::collections::HashMap;
use reqwest::header::HeaderMap;


pub fn extract_id_from_url(url: &str,tail_str: &str) -> Option<u32> {
    // 找到最后一个 '/' 字符的位置
    if let Some(index) = url.rfind('/') {
        // 从最后一个 '/' 字符位置开始提取子字符串
        if let Some(substring) = url.get(index + 1..url.len() - tail_str.len()) { // 去除末尾的 ".html" 部分
            if let Ok(id) = substring.parse::<u32>() {
                return Some(id);
            }
        }
    }
    None
}



pub async fn downloaded_image(url: &str)-> Result<(),Box::<dyn std::error::Error>>{

        let mut visited_urls = HashSet::new(); // 用于存储已访问过的 URL
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
}

pub  async fn do_get(url:&str) -> Result<String, reqwest::Error>{
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let mut data = HashMap::new();
    data.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36");
    data.insert("X-Requested-With", "XMLHttpRequest");
    data.insert("Sec-Ch-Ua-Platform", "\"Windows\"");
    data.insert("Accept", "text/javascript, application/javascript, application/ecmascript, application/x-ecmascript, */*; q=0.01");



    Ok(client.get(url)
    .headers(headers)
    .send()
    .await?
    .text()
    .await?)
}


pub async  fn save_chapter_to_file(index: usize, title: &str, content: &str) -> Result<(),Box::<dyn std::error::Error>> {

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("chapter_{}.txt", index))
        .await?;

        file.write_all(title.as_bytes()).await?;
        file.write_all(content.as_bytes()).await?;

    Ok(())
}

pub async fn merge_chapters_to_file(chapter_links: &[String]) -> Result<(),Box::<dyn std::error::Error>> {



    
    let mut merged_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("merged_book.txt")
        .await?;
    for (index, chapter_link) in chapter_links.iter().enumerate() {
        let chapter_file_name = format!("chapter_{}.txt", index + 1);
        let chapter_content = tokio::fs::read_to_string(chapter_file_name).await?;
        merged_file.write_all(chapter_content.as_bytes()).await?;
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn test_extract_id_from_url(){
        assert_eq!( super::extract_id_from_url("/html/18312/9950547.html",".html").unwrap(),9950547);

    }

    #[tokio::test]
    async fn test_download_image(){

        let url= "http://imgapi.xl0408.top";

        let res =super::downloaded_image(url).await.unwrap();
        assert_eq!( res,());
    }



    #[test]
    fn test_parse_html(){

        let html =r#"
        <html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    
</body>
</html>
        
        "#;

        let html = scraper::Html::parse_document(html);

        let next_page_selector = scraper::Selector::parse(".right>.onclick").unwrap();
        let res= html.select(&next_page_selector).next();

        assert_eq!(res,None);
    }

    #[test]
    fn test_is_empty(){


        let r1 =String::new();
        assert_eq!( true, r1.is_empty());

    }
}
