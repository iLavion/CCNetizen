// Filename: commands/town.rs
// Retrieves town information from the database

use crate::services::town::TownService;
use poise::serenity_prelude::{CreateEmbed, Colour, CreateEmbedFooter};
use chrono::{DateTime, Utc, Duration};

#[poise::command(slash_command, description_localized("en-US", "Retrieve information about a town"))]
pub async fn town(
    ctx: poise::Context<'_, crate::Data, crate::Error>, 
    #[description = "Name of the town"] town_name: String
) -> Result<(), crate::Error> {
    println!("Town command used by {}", ctx.author().name);
    let client = &ctx.data().db_client;
    let service = TownService::new(client);
    
    fn get_tier_title(residents_count: usize) -> &'static str {
        match residents_count {
            1 => "Homestead",
            2..=5 => "Hamlet",
            6..=9 => "Village",
            10..=15 => "Town",
            16..=21 => "Large Town",
            22..=27 => "City",
            28..=35 => "Large City",
            36..=53 => "Metropolis",
            54..=77 => "Conurbation",
            78..=99 => "Megalopolis",
            _ => "Eperopolis",
        }
    }

    match service.get_town_info(&town_name).await? {
        Some(town) => {
            let culture = if town.culture.is_empty() { "None".to_string() } else { town.culture };
            let peaceful = if town.peaceful { "✅".to_string() } else { "❌".to_string() };
            let base_upkeep = 20.0;
            let total_claims = (town.upkeep - base_upkeep).max(0.0);
            let last_updated = DateTime::<Utc>::from_timestamp(town.last_updated, 0)
                .expect("Invalid timestamp")
                .format("%Y-%m-%d")
                .to_string();
            let founded = DateTime::<Utc>::from_timestamp(town.founded, 0)
                .expect("Invalid timestamp")
                .format("%Y-%m-%d")
                .to_string();
            
            // Calculate the exact time when the bank balance will fall below zero
            let now = Utc::now();
            let upkeep_time = now.date_naive().and_hms_opt(19, 0, 0).expect("Invalid time").and_local_timezone(Utc).unwrap();
            let mut remaining_balance = town.bank;
            let mut next_upkeep_time = if now < upkeep_time {
                upkeep_time
            } else {
                upkeep_time + Duration::days(1)
            };

            while remaining_balance > 0.0 {
                remaining_balance -= town.upkeep;
                next_upkeep_time += Duration::days(1);
            }

            let tier_title = get_tier_title(town.residents.len());
            let embed = CreateEmbed::default()
                .title(format!("The {} of {}", tier_title, town.town_name))
                .description(format!("*The {} of {} was founded on {}*", tier_title, town.town_name, founded))
                .colour(Colour::BLITZ_BLUE)
                .thumbnail(format!("https://mc-heads.net/avatar/{}", town.mayor))
                .field("Mayor", town.mayor.clone(), true)
                .field("Nation", town.nation.unwrap_or_else(|| "None".to_string()), true)
                .field("Peaceful", peaceful, true)
                .field("Location", format!("{}, {}", town.coords.0, town.coords.1), true)
                .field("Balance", format!("${:.2}", town.bank), true)
                .field("Culture", culture, true)
                .field("Chunks", format!("{:.2}", total_claims), true)
                .field("Upkeep", format!("${:.2}", town.upkeep), true)
                .field("Falls", format!("<t:{}:R>", next_upkeep_time.timestamp()), true)
                .field(format!("Residents [{}]", town.residents.len()), format!("```{}```", town.residents.join(", ")), false)
                .field(format!("Trusted [{}]", town.trusted_players.len()), format!("```{}```", town.trusted_players.join(", ")), false)
                .footer(CreateEmbedFooter::new(format!("Last updated: {}", last_updated)))
                .to_owned();
            ctx.send(poise::CreateReply::default()
                .embed(embed)
            ).await?;
        }
        None => {
            ctx.say("Town not found").await?;
        }
    }
    Ok(())
}