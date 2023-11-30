use scraper::{Html, Selector};
use url::Url;
use hello_rust::httpops::do_get;
use tokio::time::{Instant,sleep, Duration};
use clap::Parser;
use regex::Regex;
use std::io::Write;
use tempfile::NamedTempFile;
use rand::Rng;
use tokio_retry::Retry;
use tokio_retry::strategy::{ExponentialBackoff, jitter};
use std::fs::OpenOptions;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}


const MAX_RETRIES: u64 = 3; // 设置最大重试次数
const WAIT_TIME: u64 = 1000; // 设置等待时间（毫秒）


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name)
    // }

    let base_url = Url::parse("https://m.jcdf99.com/").unwrap();
    // let mut current_url = "https://m.jcdf99.com/html/18312/9949372.html".to_string();
    let mut current_url = "https://m.jcdf99.com/html/18312/9949733_2.html".to_string();

    
    // let mut texts: Vec<String> =vec![];

  
    let mut combined_content = Vec::new(); // 用于存储所有页面的内容
    loop {

        let start =Instant::now();

        let retry_strategy = ExponentialBackoff::from_millis(10)
            .map(jitter) // add jitter to delays
            .take(3);    // limit to 3 retries


          if  let Ok(res) = Retry::spawn(retry_strategy, || async {
                let url =current_url.as_str();
                do_get(url).await
            }).await{

                println!("visiting :{} eplased: {:?}",current_url,start.elapsed());

                let html = Html::parse_document(&res);
                let novel_selector = Selector::parse(".novelcontent").unwrap();
                if let Some(text) = extract_html_text(&html, &novel_selector){
        
                    // 创建临时文件并写入页面内容
                    let mut temp_file = NamedTempFile::new().expect("创建临时文件出错");
                    write!(temp_file, "{}", text).expect("写临时文件出错");
        
                    // 读取临时文件内容并将其存储在 `combined_content` 中
                    let mut file = std::fs::File::open(temp_file.path()).expect("获取临时文件地址");
                    std::io::copy(&mut file, &mut combined_content).expect("保存临时文件地址时出错");
                }else{
                    break;
                }
           
                let next = is_has_next_charpter_page(&html);
                if next.0{
                    current_url = base_url.join( &next.1).unwrap().to_string();
                }else{
                    break;
                }
            }else{
                break;
            }

        let mut rng = rand::thread_rng();
        let r: u64= rng.gen_range(1..3);
       
        sleep(Duration::from_secs(r)).await;
    }
    // for i in texts.iter(){
    //     println!("{:?}\r\n\r\n\r\n\r\n",i);
    // }

    let file_path ="full_novel.txt";

    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true) // 如果文件不存在则创建
    .open(file_path)
    .expect("Unable to open file");

    // let mut output_file = std::fs::File::create()?;
    // file.write_all(&combined_content)?;

    if let Err(e) = file.write_all(&combined_content) {
        eprintln!("Error writing to file: {}", e);
    } else {
        println!("Content has been written/appended to the file successfully.");
    }
    Ok(())
}










/// 获取页面中文章正文

fn extract_html_text(html: &Html,selector: &Selector) ->Option<String>{
   Some(html
        .select(&selector)
        .rev()
        .map( |e| {
            // let inner_html = e.inner_html();

            let text = e.text().collect::<Vec<&str>>().join("\r\n");
            let re = Regex::new(r"[\u{00A0}\r\n\t]").unwrap();
            let replaced_content = re.replace_all(&text, " ")
            // .replace("\r", " ")
            // .replace("\n", " ")
            // .replace("\t", "  ")
            .replace("本章未完，请点击【下一页】继续阅读》》", "  ")
            .to_string();
            return replaced_content;
        })
        .collect
        ::<Vec<String>>()
        .join("\r\n")
    )
}


/// 判断当前章是否存在下一页
/// has_nex | next_page_url
 fn is_has_next_charpter_page(html: &Html) ->(bool,String){


    let next_cpage_seletor = Selector::parse(".page_chapter>ul>li>.p4").unwrap();

    if let Some(e) = html.select(&next_cpage_seletor).last(){

        if let Some(next_cpage) =e.attr("href")  {
            return (true,next_cpage.to_string());
            // if "下一页" == e.text().collect::<Vec<&str>>().join("\r\n"){
            //     return (true,next_cpage.to_string());
            // }
        }
    }

    (false,String::new())
}

