use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::Error;
use aws_sdk_dynamodb::types::AttributeValue;
use crate::models::towns::Town;

#[derive(Debug)]
pub struct TownRepository<'a> {
    db_client: &'a Client,
}

impl<'a> TownRepository<'a> {
    pub fn new(db_client: &'a Client) -> Self {
        Self { db_client }
    }

    pub async fn save_town(&self, town: &Town) -> Result<(), Error> {
        let item = town.to_dynamodb_item();
        self.db_client
            .put_item()
            .table_name("towns")
            .set_item(Some(item))
            .condition_expression("attribute_not_exists(id)")
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_town(&self, town_name: &str) -> Result<Option<Town>, Error> {
        let town_name_lower = town_name.to_lowercase();
        let result = self.db_client
            .query()
            .table_name("towns") // Use the correct table name
            .index_name("town_name_lower-index") // Use the secondary index
            .key_condition_expression("town_name_lower = :town_name_lower")
            .expression_attribute_values(":town_name_lower", AttributeValue::S(town_name_lower))
            .limit(1)
            .scan_index_forward(false)
            .send()
            .await?;

        if let Some(items) = result.items {
            if let Some(item) = items.first() {
                // Convert DynamoDB item to Town struct
                return Ok(Some(Town::from_dynamodb_item(item)?));
            }
        }
        Ok(None)
    } 
}
