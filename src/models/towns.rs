use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;
use aws_sdk_dynamodb::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Town {
    pub town_name: String,
    pub town_name_lower: String,
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
        item.insert("town_name_lower".to_string(), AttributeValue::S(self.town_name_lower.clone()));
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

    pub fn from_dynamodb_item(item: &HashMap<String, AttributeValue>) -> Result<Self, Error> {
        let coords_str = item.get("coords").unwrap().as_s().unwrap();
        let coords: Vec<&str> = coords_str.split(',').collect();
        
        Ok(Town {
            town_name: item.get("town_name").unwrap().as_s().unwrap().to_string(),
            town_name_lower: item.get("town_name_lower").unwrap().as_s().unwrap().to_string(),
            nation: item.get("nation").and_then(|v| if v.is_null() {
                None
            } else {
                Some(v.as_s().unwrap().to_string())
            }),
            mayor: item.get("mayor").unwrap().as_s().unwrap().to_string(),
            peaceful: *item.get("peaceful").unwrap().as_bool().unwrap(),
            culture: item.get("culture").unwrap().as_s().unwrap().to_string(),
            board: item.get("board").unwrap().as_s().unwrap().to_string(),
            bank: item.get("bank").unwrap().as_n().unwrap().parse().unwrap(),
            upkeep: item.get("upkeep").unwrap().as_n().unwrap().parse().unwrap(),
            founded: item.get("founded").unwrap().as_n().unwrap().parse().unwrap(),
            resources: item.get("resources").map_or(Vec::new(), |v| {
                v.as_ss().unwrap_or(&Vec::new()).iter().map(|s| s.to_string()).collect()
            }),
            residents: item.get("residents").map_or(Vec::new(), |v| {
                v.as_ss().unwrap_or(&Vec::new()).iter().map(|s| s.to_string()).collect()
            }),
            trusted_players: item.get("trusted_players").map_or(Vec::new(), |v| {
                v.as_ss().unwrap_or(&Vec::new()).iter().map(|s| s.to_string()).collect()
            }),
            area: item.get("area").unwrap().as_n().unwrap().parse().unwrap(),
            coords: (
                coords[0].parse().unwrap(),
                coords[1].parse().unwrap()
            ),
            last_updated: item.get("timestamp").unwrap().as_n().unwrap().parse().unwrap(),
        })
    }
}
