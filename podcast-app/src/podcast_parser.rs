use anyhow::Result;

enum ParseState {
    Start,
    InTitle,
    InDescription,
}

async fn read_podcasts_from_xml(url: &str) -> Result<Vec<Podcast>> {
    let mut results = Vec::new();

    let data = reqwest::get(url).await?.text().await?;
    let parser = EventReader::new(BufReader::new(data.as_bytes()));

    let mut podcast = Podcast::new();
    let mut state = ParseState::Start;
    
    Ok(results)
}