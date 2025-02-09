// Filename: commands/help.rs
// Help command

use poise::serenity_prelude::{CreateEmbed, Colour, CreateEmbedFooter};

#[poise::command(slash_command)]
pub async fn help(ctx: poise::Context<'_, crate::Data, crate::Error>) -> Result<(), crate::Error> {
    println!("Help command used by {}", ctx.author().name);

    let embed = CreateEmbed::default()
        .title("Help Command")
        .description("This is the help command response.")
        .colour(Colour::BLITZ_BLUE)
        .field("Field 1", "This is the first field", false)
        .field("Field 2", "This is the second field", false)
        .footer(CreateEmbedFooter::new("Footer text"))
        .to_owned();

    ctx.send(poise::CreateReply::default()
        .content("Here is the help command response.")
        .embed(embed)
).await?;

    Ok(())
}