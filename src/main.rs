use scraper::{Html, Selector};
use std::collections::{HashSet, HashMap};
use url::Url;
use hello_rust::{extract_id_from_url,do_get};
use tokio::time;
use tokio::time::Instant;

use clap::Parser;








#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    let start_url ="https://m.jcdf99.com/html/18312_39/#all";
    let mut current_url = start_url.to_string();
    let base_url ="https://m.jcdf99.com/";


    let mut pages: Vec<Page> = Vec::new();
    loop {


        if let Ok(base) = Url::parse(base_url){
            if  let Ok(url) = base.join(current_url.as_str()){

                current_url = url.to_string();
            }
       }
        let links = extract_links_from_page(&current_url).await;

        // wait 
        time::sleep(time::Duration::from_millis(500)).await;
        if let Ok(links) =links{
                current_url = links.url.clone();
    
                pages.push(links.clone());
                if links.is_last{
                    break;
                }
        }
        else if let Err(e) = links{
            println!(" err:{}",e);
            break;
        }
        else{
            println!("break else");
            break;
        }
       
    }
    // todo: iter all page and iter page's charpter
    for i in pages.iter(){
     
        // if !i.is_last{
        //     println!("[iter] {}, {:?}",i.url,i.charpters);
        // }


        println!("{:?}",i);
    }
    Ok(())
}



#[derive(PartialEq, Eq,Clone,Debug)]
struct Page{
    url: String,
    is_last: bool,
    charpters: Vec<Charpter>
}
impl Page {

    fn default()-> Self{

        Self { url: String::new(), is_last: false, charpters: vec![] }
    }
    fn print(&self) {
        let line_number = line!();
        println!("[line:{}]Page url: {} is_last:{}",line_number, self.url,self.is_last);
    }

  
}

#[derive(PartialEq, Eq,Clone,Debug)]
struct Charpter{
    url: String,
    text: String,
    title: String
}
impl Charpter {
    fn default() ->Self{
        Self { url: String::new(), text: String::new(), title: String::new() }
    }
}


async fn extract_links_from_page(url: &str) -> Result<Page,Box::<dyn std::error::Error>> {

    let base_url ="https://m.jcdf99.com/";

    let base =Url::parse(base_url).unwrap();
    let next_page_selector = Selector::parse(".right>.onclick").unwrap();

    let start =Instant::now();
    // let res = reqwest::get(url).await?.text().await?;

    let res =do_get(url).await?;
    println!("visiting :{} eplased: {:?}",url,start.elapsed());
    let html = Html::parse_document(&res);

    let charpt_selector = Selector::parse(".p2>li>a").unwrap();

    if let Some(e) =html.select(&next_page_selector).next() {
        if let Some(href) = e.value().attr("href") {


            let mut p =Page::default();

            p.is_last =true;
            if  let Ok(url) = base.join(href){
                p.url = url.to_string();
            }

            let charpters: Vec<Charpter> = html
                .select(&charpt_selector)
                .rev()
                .map( |e| {
                    if let Some(charp_href) = e.value().attr("href") {
                            if  let Ok(charp_url) = base.join(charp_href){
                                let inner_html = e.inner_html();
                                let url_string = charp_url.to_string();
                                return Charpter{
                                    url: url_string,
                                    title: inner_html,
                                    text: String::new()
                                };
                            }
                    } 
                    Charpter::default()
                })
                .collect();

            p.charpters =charpters;
      
            return  Ok(p);
        }
    }

    Ok(Page::default())
}


// 获取下一页的内容
async fn extract_charpter_perpage() ->Result<(),Box::<dyn std::error::Error>>{



    Ok(())
}


/// 获取每章详情页的文本
async fn extract_novel_from_charpter(url: &str) ->Result<(),Box::<dyn std::error::Error>>{

    let start =Instant::now();
    // let res = reqwest::get(url).await?.text().await?;

    let res =do_get(url).await?;

    let html = Html::parse_document(&res);


    let novel_selector = Selector::parse(".novelcontent").unwrap();
    let result: String = html
        .select(&novel_selector)
        .rev()
        .map( |e| {
            // let inner_html = e.inner_html();

            let text = e.text().collect::<Vec<&str>>().join("\r\n");
            return text;
        })
        .collect
        ::<Vec<String>>()
        .join("\r\n")
        ;


        println!("body: {}\r\n elapsed: {:?}",
            result,
            start.elapsed());
     
    Ok(())
}