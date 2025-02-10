// Filename: commands/town.rs
// Retrieves town information from the database

use crate::services::town::TownService;

#[poise::command(slash_command, description_localized("en-US", "Retrieve information about a town"))]
pub async fn town(
    ctx: poise::Context<'_, crate::Data, crate::Error>, 
    #[description = "Name of the town"] town_name: String
) -> Result<(), crate::Error> {
    println!("Town command used by {}", ctx.author().name);
    let client = &ctx.data().db_client;
    let service = TownService::new(client);
    
    match service.get_town_info(&town_name).await? {
        Some(town) => {
            ctx.say(format!("Town info: {:?}", town)).await?;
        }
        None => {
            ctx.say("Town not found").await?;
        }
    }
    Ok(())
}
