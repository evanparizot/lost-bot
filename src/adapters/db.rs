use std::collections::HashMap;

use aws_sdk_dynamodb::types::{AttributeValue, ReturnValue, Select};
use tracing::{error, info};
pub struct ShotSaver {
    client: aws_sdk_dynamodb::Client,
}
static TABLE_NAME: &'static str = "shots";
impl ShotSaver {
    pub fn new(client: aws_sdk_dynamodb::Client) -> Self {
        Self { client: client }
    }

    pub async fn add(&self, name: &str, amount: i32) -> Result<i32, ()> {
        match self
            .client
            .update_item()
            .table_name(TABLE_NAME)
            .key("name", AttributeValue::S(name.to_string()))
            .update_expression("ADD shots :amount")
            .expression_attribute_values(":amount", AttributeValue::N(amount.to_string()))
            .return_values(ReturnValue::UpdatedNew)
            .send()
            .await
        {
            Ok(output) => {
                let map = output.attributes.unwrap();
                let attribute_value = map.get("shots").unwrap();
                let value = match attribute_value.as_n() {
                    Ok(v) => v,
                    Err(_) => "",
                };
                let f = value.parse::<i32>().unwrap();

                Ok(f)
            }
            Err(_e) => {
                error!("{}", _e);
                Ok(0)
            }
        }
    }

    pub async fn subtract(&self, name: &str, amount: i32) -> Result<i32, ()> {
        match self
            .client
            .update_item()
            .table_name(TABLE_NAME)
            .key("name", AttributeValue::S(name.to_string()))
            .update_expression("SET shots = shots - :amount")
            .expression_attribute_values(":floor", AttributeValue::N("1".to_string()))
            .expression_attribute_values(":amount", AttributeValue::N(amount.to_string()))
            .condition_expression("shots >= :floor")
            .return_values(ReturnValue::UpdatedNew)
            .send()
            .await
        {
            Ok(output) => {
                let map = output.attributes.unwrap();
                let attribute_value = map.get("shots").unwrap();
                let value = match attribute_value.as_n() {
                    Ok(v) => v,
                    Err(_) => "",
                };
                Ok(value.parse::<i32>().unwrap())
            }
            Err(_e) => {
                error!("{}", _e);
                Ok(0)
            }
        }
    }

    pub async fn list(&self) -> HashMap<String, i32> {
        match self
            .client
            .scan()
            .table_name(TABLE_NAME)
            .select(Select::AllAttributes)
            .send()
            .await
        {
            Ok(output) => {
                let items = output.items.unwrap();
                let mut to_return: HashMap<String, i32> = HashMap::new();
                for n in items {
                    let name = n.get("name").unwrap().as_s().unwrap();
                    let shots = n
                        .get("shots")
                        .unwrap()
                        .as_n()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();
                    to_return.insert(name.to_string(), shots);
                }
                return to_return;
            }
            Err(_e) => {
                error!("{}", _e);
                HashMap::new()
            }
        }
    }

    pub async fn exists(&self, name: &str) -> bool {
        match self
            .client
            .get_item()
            .table_name(TABLE_NAME)
            .key("name", AttributeValue::S(name.to_string()))
            .send()
            .await
        {
            Ok(output) => {
                let item = output.item;
                info!("{:?}", item);
                !item.is_none()
            }
            Err(_e) => {
                error!("{}", _e);
                false
            }
        }
    }

    pub async fn reset(&self, name: &str) -> Result<(), ()> {
        match self
            .client
            .update_item()
            .table_name(TABLE_NAME)
            .key("name", AttributeValue::S(name.to_string()))
            .expression_attribute_values(":amount", AttributeValue::N("0".to_string()))
            .update_expression("SET shots = :amount")
            .send()
            .await
        {
            Ok(_output) => Ok(()),
            Err(_e) => {
                error!("{}", _e);
                Ok(())
            }
        }
    }

    pub async fn remove(&self, name: &str) -> Result<(), ()> {
        match self
            .client
            .delete_item()
            .table_name(TABLE_NAME)
            .key("name", AttributeValue::S(name.to_string()))
            .send()
            .await
        {
            Ok(_output) => Ok(()),
            Err(_e) => {
                error!("{}", _e);
                Ok(())
            }
        }
    }
}
