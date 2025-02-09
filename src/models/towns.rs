use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Town {
    pub town_name: String,
    pub nation: Option<String>,
    pub mayor: String,
    pub peaceful: bool,
    pub culture: String,
    pub board: String,
    pub bank: f64,
    pub upkeep: f64,
    pub founded: i64,
    pub resources: Vec<String>,
    pub residents: Vec<String>,
    pub trusted_players: Vec<String>,
    pub area: f64,
    pub coords: (f64, f64),
    pub last_updated: i64,
}

impl Town {
    pub fn to_dynamodb_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        item.insert("town_name".to_string(), AttributeValue::S(self.town_name.clone()));
        item.insert("timestamp".to_string(), AttributeValue::N(self.last_updated.to_string()));
        item.insert("nation".to_string(), match &self.nation {
            Some(n) => AttributeValue::S(n.clone()),
            None => AttributeValue::Null(true),
        });
        item.insert("mayor".to_string(), AttributeValue::S(self.mayor.clone()));
        item.insert("peaceful".to_string(), AttributeValue::Bool(self.peaceful));
        item.insert("culture".to_string(), AttributeValue::S(self.culture.clone()));
        item.insert("board".to_string(), AttributeValue::S(self.board.clone()));
        item.insert("bank".to_string(), AttributeValue::N(self.bank.to_string()));
        item.insert("upkeep".to_string(), AttributeValue::N(self.upkeep.to_string()));
        item.insert("founded".to_string(), AttributeValue::N(self.founded.to_string()));
        if !self.resources.is_empty() {
            item.insert("resources".to_string(), AttributeValue::Ss(self.resources.clone()));
        } else {
            item.insert("resources".to_string(), AttributeValue::Null(true));
        }
        if !self.residents.is_empty() {
            item.insert("residents".to_string(), AttributeValue::Ss(self.residents.clone()));
        } else {
            item.insert("residents".to_string(), AttributeValue::Null(true));
        }
        if !self.trusted_players.is_empty() {
            item.insert("trusted_players".to_string(), AttributeValue::Ss(self.trusted_players.clone()));
        } else {
            item.insert("trusted_players".to_string(), AttributeValue::Null(true));
        }
        item.insert("area".to_string(), AttributeValue::N(self.area.to_string()));
        item.insert("coords".to_string(), AttributeValue::S(format!("{},{}", self.coords.0, self.coords.1)));
        item
    }
}
