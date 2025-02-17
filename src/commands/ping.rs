// Filename: commands/ping.rs
// A simple ping command
/// Responds with "Pong!"
#[poise::command(slash_command)]
pub async fn ping(ctx: poise::Context<'_, crate::Data, crate::Error>) -> Result<(), crate::Error> {
    println!("Ping command used by {}", ctx.author().name);
    ctx.say("Pong!").await?;
    Ok(())
}