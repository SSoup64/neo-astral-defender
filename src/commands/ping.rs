use serenity::all::{
    Context,
    CommandInteraction,
    CreateCommand,
    CreateInteractionResponseFollowup
};

pub fn register() -> CreateCommand {
    CreateCommand::new("ping")
    .description("make sure the bot is alive")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    if let Err(e) = interaction.defer_ephemeral(&ctx.http).await {
        println!("Failed to defer response: {}", e);
    }

    let response = CreateInteractionResponseFollowup::new().content("Pong!");

    let msg = interaction.create_followup(&ctx.http, response).await;

    if let Err(_) = msg {
        println!("Failed to respond tp ping command");
    }
}
