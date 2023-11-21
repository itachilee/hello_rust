use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use scraper::{Html,Selector};
use url::Url;
#[tokio::main]
async fn main() ->Result<(),Box<dyn std::error::Error>>{
    println!("Hello, world!");
    let url ="https://m.jcdf99.com/html/18312/";

    scrape_url(url).await
}

async fn scrape_url(root_url: &str)->Result<(),Box<dyn std::error::Error>> {
    let res =reqwest::get(root_url).await?
    .text().await?;

    // res.status();
    
    let html = Html::parse_document(&res);


    let root_ref = html.root_element();
    let title = &root_ref
        .select(&Selector::parse(".nav_name>h1").unwrap())
        .next()
        .unwrap();
    assert_eq!(title.inner_html(), "将夜");
 
    let charpt_selector = Selector::parse(".p2>li>a").unwrap();
    let result: Vec<(String,&str)> = html
        .select(&charpt_selector)
        .rev()
        .map(|e| {
           
            if let Some(href) = e.value().attr("href") {
                (e.inner_html() , href)
            }else{

                (e.inner_html() , "")
            }
        })
        .collect();


    for item in result{
        println!("{} {} ",item.0,item.1);
    }

    let next_page_selector  = Selector::parse(".right>.onclick").unwrap();

    let next_page_url= html.select(&next_page_selector)
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap()
        ;


        scrape_charpter(&next_page_url).await?;
    println!("next page url is {}",next_page_url);


    Ok(())
}



async fn scrape_charpter(charpter_url: &str)->Result<(),Box<dyn std::error::Error>>{
    let base_url ="https://m.jcdf99.com/";
    let base = Url::parse(base_url)?;
    let url = base.join(charpter_url)?;
    println!("visiting page url: {}",url);
    // let chartper_content =reqwest::get(url).await?
    //     .text().await?;


        Ok(())

}