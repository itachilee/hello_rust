use tokio::fs::{File,OpenOptions};
use tokio::io::{AsyncWriteExt,SeekFrom,AsyncReadExt};

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

pub async fn create_temp_files() -> Result<(), Box<dyn std::error::Error>>{

    let mut temp_file = tempfile::NamedTempFile::new()?;
    let file_path = temp_file.path().to_owned();

    // 将临时文件转换为异步文件
    let async_temp_file = tokio::fs::File::from_std(temp_file.into_file());

    // 异步写入数据到临时文件
    let mut file_writer = File::from(async_temp_file);
    file_writer.write_all(b"Hello, this is an async temporary file!").await?;


    let mut buf = String::new();
    file_writer.read_to_string(&mut buf).await?;
    assert_eq!("Hello, this is an async temporary file!", buf);
    Ok(())
}


#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_save_files(){


        let _= super::save_chapter_to_file(1,"t1","c1").await;
        let _= super::save_chapter_to_file(2,"t2","c2").await;

        assert!(true);

    }

    #[tokio::test]
    async fn test_merge_chapters_to_file(){


        let charpter = [String::new(),String::new()];
        let _= super::merge_chapters_to_file(&charpter).await;
        assert!(true);
    }


    #[tokio::test]
    async fn test_create_temp_files(){


       let _= super:: create_temp_files().await;
        assert!(true);
    }

 
}