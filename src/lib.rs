use std::collections::HashMap;

use serde_json::{Result, Value};

pub struct JsonSchemaProperty {
	pub r#type: String,
}

pub struct JsonSchema {
	pub schema: String,
	pub id: String,
	pub title: Option<String>,
	pub description: Option<String>,
	pub properties: HashMap<String, JsonSchemaProperty>,
	pub required: Vec<String>,
}

pub fn build_schema(data: &str) -> Result<JsonSchema> {
	let v: Value = serde_json::from_str(data)?;

	let properties: HashMap<String, JsonSchemaProperty> = match v {
		Value::Object(x) => HashMap::from_iter(x.into_iter().map(|(key, value)| {
			(
				key,
				JsonSchemaProperty {
					r#type: value_to_type(value),
				},
			)
		})),
		_ => HashMap::new(),
	};

	Ok(JsonSchema {
		schema: String::from("https://json-schema.org/draft/2020-12/schema"),
		id: String::from(""),
		title: None,
		description: None,
		properties,
		required: vec![],
	})
}

fn value_to_type(value: Value) -> String {
	if value.is_i64() || value.is_u64() {
		String::from("integer")
	} else if value.is_f64() {
		String::from("float")
	} else if value.is_number() {
		String::from("number")
	} else if value.is_array() {
		String::from("array")
	} else if value.is_string() {
		String::from("string")
	} else if value.is_boolean() {
		String::from("boolean")
	} else if value.is_null() {
		String::from("null")
	} else if value.is_object() {
		String::from("object")
	} else {
		String::from("unknown")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn builds_simple_schema() {
		let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

		let output = build_schema(data).expect("Could not parse json");

		assert_eq!(output.properties["name"].r#type, "string");
		assert_eq!(output.properties["phones"].r#type, "array");
		assert_eq!(output.properties["age"].r#type, "integer")
	}

	#[test]
	fn builds_required_fields() {
		let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": null
        }"#;

		let output = build_schema(data).expect("Could not parse json");
		let required = output.required;

		assert_eq!(required.len(), 2)
	}
}
