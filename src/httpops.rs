

use std::collections::{HashMap,HashSet};
use reqwest::ClientBuilder;
use reqwest::header::{HeaderMap,HeaderValue};
use reqwest::header::CONTENT_TYPE;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

const DEFAULT_IMG_URL: &'static str ="http://imgapi.xl0408.top";

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

    // let client = reqwest::Client::new();


    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"));
    headers.insert("X-Requested-With", HeaderValue::from_static("XMLHttpRequest"));
    headers.insert("Sec-Ch-Ua-Platform", HeaderValue::from_static("\"Windows\""));
    headers.insert("Accept", HeaderValue::from_static("text/javascript, application/javascript, application/ecmascript, application/x-ecmascript, */*; q=0.01"));


    let client= ClientBuilder::new()
    .default_headers(headers)
    .build()
    .unwrap();


   Ok(client.get(url)
   .send()
   .await?
   .text()
   .await?)

}

#[cfg(test)]
mod tests {

    


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

}