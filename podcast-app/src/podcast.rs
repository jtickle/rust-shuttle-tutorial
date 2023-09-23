struct Podcast {
    title: String,
    description: String,
    audio_file: Option<String>,
}

impl Podcast {
    fn new() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            audio_file: None,
        }
    }
}