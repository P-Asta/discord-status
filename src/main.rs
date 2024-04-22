use discord_rich_presence::{
    activity::{self, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new("871348411356545057")?;

    client.connect()?;
    client.set_activity(
        activity::Activity::new()
            .timestamps(Timestamps::new().start(1))
            .state("foo")
            .details("bar")
            .assets(Assets::new().large_image("stone").large_text("돌돌이"))
            .buttons(vec![activity::Button::new(
                "test",
                "https://github.com/5-23",
            )]),
    )?;
    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
    // client.close()?;

    Ok(())
}
