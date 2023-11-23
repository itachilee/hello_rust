use scraper::{Html, Selector};
use std::collections::{HashSet, HashMap};
use url::Url;
use hello_rust::{extract_id_from_url,do_get};
use tokio::time;
use tokio::time::Instant;

use clap::Parser;
use regex::Regex;



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



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name)
    // }

    let base_url ="https://m.jcdf99.com/";
    let start_url ="https://m.jcdf99.com/html/18312/9950547.html";

    let base_url = Url::parse(base_url).unwrap();
    let mut current_url = start_url.to_string();

    let mut texts: Vec<String> =vec![];
    let mut count =1;
    loop {

        let start =Instant::now();
        // let res = reqwest::get(url).await?.text().await?;

        let res =do_get(current_url.as_str()).await?;
        println!("visiting :{} eplased: {:?}",current_url,start.elapsed());

        let html = Html::parse_document(&res);
        let novel_selector = Selector::parse(".novelcontent").unwrap();
        if let Some(text) = extract_html_text(&html, &novel_selector){
            texts.push(text);
        }else{
            break;
        }

        let next =is_has_next_charpter_page(&html);
        if next.0{
            current_url = base_url.join( &next.1).unwrap().to_string();
        }else{
            break;
        }
        count +=1;
        if count >2{
            break;
        }
    }
    for i in texts.iter(){
        println!("{:?}\r\n\r\n\r\n\r\n",i);
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
            // .replace("\t", "  ")
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

            if "下一页" == e.text().collect::<Vec<&str>>().join("\r\n"){
                return (true,next_cpage.to_string());
            }
        }
    }

    (false,String::new())
}

