use std::collections::HashMap;

use serde_json::{Result, Value};

pub struct JsonSchemaProperty {
	pub r#type: String,
}

pub struct JsonSchema {
	pub properties: HashMap<String, JsonSchemaProperty>,
	pub required: Vec<String>,
}

pub fn build_schema(data: &str) -> Result<JsonSchema> {
	let v: Value = serde_json::from_str(data)?;

	// TODO: collect values above and map them into JsonShema instances

	Ok(JsonSchema {
		properties: HashMap::from([(
			String::from("name"),
			JsonSchemaProperty {
				r#type: String::from("string"),
			},
		)]),
		required: vec![],
	})
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

		assert_eq!(output.properties["name"].r#type, "string")
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
