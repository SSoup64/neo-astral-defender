use serenity::{
    all::{ClientBuilder, Context, EventHandler, GatewayIntents, Guild, Interaction, Ready},
    async_trait,
};

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _ready: Ready) {
        println!("Defender has been activated!");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            // TODO: Add cmd_run_branch! macro that automatically expands to an arm like in here
            match cmd.data.name.as_str() {
                "ping" => {
                    commands::ping::run(&ctx, &cmd).await;
                }
                _ => {}
            }
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: Option<bool>) {
        println!("'{}': {}", guild.name, guild.id);

        let commands = guild
            .id
            .set_commands(&ctx.http, vec![commands::ping::register()])
            .await
            .expect("Couldn't register commands into guild."); // TODO: Maybe not panic?

        println!(
            "The following commands have been registed:\n{}",
            commands
                .into_iter()
                .map(|cmd| format!("- {}", cmd.name))
                .collect::<Vec<String>>()
                .join("\n")
        );
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
