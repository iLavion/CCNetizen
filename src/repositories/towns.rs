use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::Error;
use crate::models::towns::Town;

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
            .send()
            .await?;
        Ok(())
    }
}
