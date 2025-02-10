use crate::repositories::towns::TownRepository;
use crate::models::towns::Town;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::Error;

pub struct TownService<'a> {
    repository: TownRepository<'a>,
}

impl<'a> TownService<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            repository: TownRepository::new(client),
        }
    }

    pub async fn get_town_info(&self, town_name: &str) -> Result<Option<Town>, Error> {
        self.repository.get_town(town_name).await
    }
}
