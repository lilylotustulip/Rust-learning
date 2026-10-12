enum Media {
    Song {title: String, duration: u32},
    Podcast {episode_id: u32},
    Stream,
}

impl Media {
    fn play_info(&self) -> String {
        match self {
            Media::Song {title, duration} => format!("playing song: {title} ({duration}s)"),
            Media::Podcast {episode_id} => format!("playing podcast episode #{episode_id}"),
            Media::Stream => format!("playing stream"),
        }
    }
}

fn main(){
    let song1 = Media::Song{
        title: String::from("cookie"),
        duration: 240,
    };
    println!("{}", song1.play_info());

    let podcast1 = Media::Podcast{
        episode_id: 3,
    };
    println!("{}", podcast1.play_info());

    let stream1 = Media::Stream;
    println!("{}", stream1.play_info())
}