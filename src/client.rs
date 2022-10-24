use std::{collections::HashMap, path::Path, fs::File, io::Write};

fn req() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}

async fn getResource(url: &str, parent: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let dir = format!("{}/{}", parent, "o.png");
    let path = Path::new(dir.as_str());
    let mut file = File::create(path)?;
    let content = response.bytes().await?;
    file.write_all(&content)?;
    Ok(dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testReq() {
        req();
    }

    #[tokio::test]
    async fn test_get_resource() {
        println!("current_exe: ${:?}", std::env::current_exe());
        println!("current_dir: ${:?}", std::env::current_dir());
        let parent = std::env::current_dir().unwrap();
        let url = "https://oss.bm001.com/jzhomeland_resource/appimage/makeMoney/icon-shareToday.png";
        let path = getResource(url, parent.to_str().unwrap()).await;
        println!("donwload file: {:?}", path)
    }
}