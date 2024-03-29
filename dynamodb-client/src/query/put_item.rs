use aws_sdk_dynamodb::model::{AttributeValue, ReturnValue};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

/*
#[derive(Deserialize, Debug)]
pub struct PutItemQuery {
    #[serde(rename = "TableName")]
    table_name: String,
    #[serde(rename = "Items")]
    items: Items
}*/

#[derive(Debug)]
pub struct PutItemQuery {
    table_name: String,
    items: Items,
    return_values: Option<ReturnValue>,
    condition_expression: Option<String>,
}

impl PutItemQuery {
    pub fn new(
        table_name: impl Into<String>,
        items: Items,
        return_values: Option<ReturnValue>,
        condition_expression: Option<impl Into<String>>,
    ) -> Self {
        Self {
            table_name: table_name.into(),
            items,
            return_values,
            condition_expression: condition_expression.map(|value| value.into()),
        }
    }

    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    pub fn items(&self) -> Items {
        let reference_items = &self.items;

        let items: HashMap<String, AttributeValue> = reference_items
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();

        items
    }

    pub fn return_values(&self) -> ReturnValue {
        match &self.return_values {
            Some(value) => value.clone(),
            None => ReturnValue::None,
        }
    }

    pub fn condition_expression(&self) -> &Option<String> {
        &self.condition_expression
    }
}

pub type Items = HashMap<String, AttributeValue>;

/*
 * @Todo Fix deserialize.
 */
impl<'de> Deserialize<'de> for PutItemQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct PutItemQueryHelper {
            #[serde(rename = "TableName")]
            table_name: String,
        }

        let helper = PutItemQueryHelper::deserialize(deserializer)?;

        let mut hash: HashMap<String, AttributeValue> = HashMap::new();

        hash.insert("temp".to_string(), AttributeValue::S("test!!!".to_string()));

        Ok(PutItemQuery {
            table_name: helper.table_name,
            items: hash,
            return_values: None,
            condition_expression: None,
        })
    }
}

/*
#[derive(Deserialize, Debug)]
pub enum AttributeValue {
    S(String),
    Unknown
}

impl From<AwsAttributeValue> for AttributeValue {
    fn from(aws_value: AwsAttributeValue) -> Self {
        if let AwsAttributeValue::S(value) = aws_value {
            return AttributeValue::S(value)
        }

        AttributeValue::Unknown
    }
}*/
