use std::collections::HashMap;

fn req() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testReq() {
        req();
    }
}