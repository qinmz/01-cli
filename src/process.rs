use csv::Reader;
//use serde::{Deserialize, Serialize};
use crate::OutputFormat;

// #[derive(Debug, Deserialize, Serialize)]
// /// PascalCase 就是自己写的名称的首字母大写形式
// #[serde(rename_all = "PascalCase")]
// pub struct Player {
//     pub name: String,
//     pub position: String,
//     #[serde(rename = "DOB")]
//     pub dob: String,
//     pub nationality: String,
//     #[serde(rename = "Kit Number")]
//     pub kit_number: u8,
// }

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    //println!("{:?}", headers);
    for result in reader.records() {
        let record = result?;
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value)
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    std::fs::write(output, content)?;
    Ok(())
}
