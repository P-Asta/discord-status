use std::{net::TcpListener, vec};

use discord_rich_presence::{
    activity::{self, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};

pub fn main() {
    let mut client: DiscordIpcClient = DiscordIpcClient::new("871348411356545057").unwrap();
    client.connect().unwrap();
    let status = Status {
        title: "5-23",
        detail: "5-23",
        // large_img: "stone",
        // large_text: "stone",
        ..Default::default()
    };
    let client = status.start(&mut client).unwrap();
}
pub struct Button {
    label: String,
    url: String,
}

#[allow(non_snake_case)]
fn Button(label: &str, url: &str) -> Button {
    Button {
        label: label.to_string(),
        url: url.to_string(),
    }
}

pub struct Status {
    title: &'static str,
    detail: &'static str,
    large_img: &'static str,
    large_text: &'static str,
    small_img: &'static str,
    small_text: &'static str,
    buttons: Vec<Button>,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            title: "1",
            detail: "1",
            large_img: "1",
            large_text: "1",
            small_img: "1",
            small_text: "1",
            buttons: vec![],
        }
    }
}

impl Status {
    fn start(&self, client: &mut DiscordIpcClient) -> Result<(), Box<dyn std::error::Error>> {
        let mut btns = vec![];
        for btn in &self.buttons {
            btns.push(activity::Button::new(&btn.label, &btn.url))
        }

        client.set_activity(
            activity::Activity::new()
                .timestamps(Timestamps::new().start(1))
                .state(&self.title)
                .details(&self.detail)
                .assets(
                    Assets::new()
                        .large_image(&self.large_img)
                        .large_text(&self.large_text)
                        .small_image(&self.small_img)
                        .small_text(&self.small_text),
                ), // .buttons(btns),
        )?;
        // client.close()?;
        loop {}
        Ok(())
    }
}
