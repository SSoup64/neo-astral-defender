use serenity::{ 
    all::{ 
        ClientBuilder,
        EventHandler,
        GatewayIntents,
        Ready,
        Context,
        Interaction,
        GuildId
    }, 
    async_trait
};

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Defender has been activated!");

        let guild_id = GuildId::new(
            dotenv::var("GUILD_ID")
            .unwrap()
            .parse::<u64>()
            .expect("Guild ID is not an integer")
        );

        let commands = guild_id.set_commands(
            &ctx.http,
            vec![
                commands::ping::register()
            ]
        ).await
        .unwrap();

        println!(
            "The following commands have been registed:\n{}",
            commands
                .into_iter()
                .map(|cmd| format!("- {}", cmd.name))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            match cmd.data.name.as_str() {
                "ping" => {
                    commands::ping::run(&ctx, &cmd).await
                }
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = dotenv::var("TOKEN").unwrap();
    let intents = GatewayIntents::GUILDS;

    let mut client = ClientBuilder::new(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client");

    if let Err(e) = client.start().await {
        println!("Failed to start client: {}", e);
    }
}
