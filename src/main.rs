use scraper::{Html, Selector};
use std::collections::{HashSet, HashMap};
use url::Url;
use hello_rust::{extract_id_from_url,do_get};
use tokio::time;
use tokio::time::Instant;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

  
    let url ="https://m.jcdf99.com/html/18312/";


    // scrape_url(url).await?;
   
    get_all().await?;
    Ok(())
}



#[derive(PartialEq, Eq,Clone,Debug)]
struct Page{
    url: String,
    is_last: bool,
    charpter_links: Vec<(String,String)>
}

impl Page {
  
    fn print(&self) {
        let line_number = line!();
        println!("[line:{}]Page url: {} is_last:{}",line_number, self.url,self.is_last);
    }
}

async fn get_all() -> Result<(),Box::<dyn std::error::Error>> {
    // let start_url = "https://m.jcdf99.com/html/18312/"; // 初始页面 URL


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
    for i in pages.iter(){
     
        if !i.is_last{
            println!("[iter] {}, {:?}",i.url,i.charpter_links);
        }
    }
    Ok(())
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


    if let Some(e) =html.select(&next_page_selector).next() {
        if let Some(url) = e.value().attr("href") {

            let charpt_selector = Selector::parse(".p2>li>a").unwrap();
            let result: Vec<(String, String)> = html
                .select(&charpt_selector)
                .rev()
                .map( |e| {
                    if let Some(href) = e.value().attr("href") {
        
                            if  let Ok(url) = base.join(href){
        
                                let inner_html = e.inner_html();
                                let url_string = url.to_string();
                                return (inner_html, url_string);
                            }
                    } 
                    (e.inner_html(), String::new())
                })
                .collect();

                if  let Ok(url) = base.join(url){
                    let p =Page { url: url.to_string(), is_last: false, charpter_links: result};
                    return Ok(p);
                }
      
        }
    }

    Ok(Page { url: "".to_string(), 
        is_last: true,
        charpter_links: vec![]
     })
}
