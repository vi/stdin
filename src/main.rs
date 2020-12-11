
use futures::{stream::StreamExt};

/// Simple Telegram bot for posting each line read from stdin to a Telegram group
#[derive(argh::FromArgs)]
struct Opts {
    /// a file where Telegram bot token is stored
    #[argh(positional)]
    token_file: std::path::PathBuf,

    /// telegram group ID to send all stdin lines to
    #[argh(option,short='g')]
    group: i64,

    /// do not poll for updates for the bot, start only the message sending task
    #[argh(switch, short='n')]
    inhibit_updates: bool,
}

async fn run(opts: Opts) -> anyhow::Result<()> {
    let token = std::fs::read_to_string(opts.token_file)?;
    let token = token.trim();
    let api = telegram_bot::Api::new(token);

    let api2 = api.clone();
    let telegram_group_id = opts.group;

    let jh = tokio::spawn(async move {
        let mut stream = tokio_util::codec::FramedRead::new(tokio::io::stdin(), tokio_util::codec::LinesCodec::new());
        while let Some(line) = stream.next().await {
            if let Ok(line) = line {
                println!("Incoming line: {}", line);

                let chat = telegram_bot::types::Chat::Group(telegram_bot::types::Group{
                    id: telegram_bot::types::GroupId::new(telegram_group_id),
                    title: "".to_string(),
                    all_members_are_administrators: false,
                    invite_link: None,
                });
                let msg = telegram_bot::types::requests::send_message::SendMessage::new(chat, line);
                if let Err(e) = api2.send(msg).await {
                    eprintln!("Error sending: {}", e);
                }
            } else {
                std::process::exit(0);
            }
        }
    });
    

    if opts.inhibit_updates {
        jh.await?;
        Ok(())
    } else {
        let mut stream = api.stream();
        while let Some(update) = stream.next().await {
            let update  = update?;
            println!("{:?}", update);        
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let opts : Opts = argh::from_env();

    tokio::runtime::Runtime::new()?.block_on(run(opts))
}
