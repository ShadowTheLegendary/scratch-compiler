use zip::ZipArchive;

use bytes::Bytes;

use std::fs;
use std::io::{Cursor, Write};

use reqwest::StatusCode;

use serde_json::Value;

use crate::BUILD_DIRECTORY;

pub fn download(project_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("- Fetching project.");
    println!("-- Fetching project metadata.");

    // https://api.scratch.mit.edu/projects/<PROJECT_ID>/
    let link = "https://api.scratch.mit.edu/projects/".to_owned() + &project_id + "/";
    let response = reqwest::blocking::get(link.as_str())?;
    match response.status() {
        StatusCode::OK => {
            println!("-- Successfully fetched project metadata.")
        }
        _ => {
            println!("-- Error fetching project metadata.");
            return Ok(());
        }
    }

    let metadata = response.text()?;
    let output: Value = serde_json::from_str(&metadata)?;

    fs::create_dir_all(BUILD_DIRECTORY.to_owned() + &project_id + "/meta/")?;
    fs::write(BUILD_DIRECTORY.to_owned() + &project_id + "/meta/data.json", serde_json::to_string_pretty(&output)?)?;

    let project_token = output["project_token"]
        .as_str()
        .expect("- - Error fetching project token");

    // https://projects.scratch.mit.edu/<PROJECT_ID>?token=<PROJECT_TOKEN>
    let link =
        "https://projects.scratch.mit.edu/".to_owned() + &project_id + "?token=" + project_token;

    let response = reqwest::blocking::get(link.as_str())?;
    match response.status() {
        StatusCode::OK => {
            println!("- Successfully fetched project.\n")
        }
        _ => {
            println!("- Issue fetching project: {}.", response.status());
            return Ok(());
        }
    }

    println!("- Writing files.");

    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("")
        .to_string();
    let project_dir = BUILD_DIRECTORY.to_owned() + &project_id;

    if content_type.contains("zip") || content_type.contains("sb3") {
        let bytes = response.bytes()?;
        handle_sb3(&project_dir, bytes)?;
    } else {
        handle_json(&project_dir, &response.text()?)?;
    }

    println!("- Successfully wrote files.\n");

    Ok(())
}

fn handle_sb3(directory: &str, bytes: Bytes) -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Extracting .sb3 file.");
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = std::path::Path::new(directory).join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(p) = out_path.parent() {
                fs::create_dir_all(p)?;
            }
            let mut out_file = fs::File::create(&out_path)?;
            std::io::copy(&mut file, &mut out_file)?;
        }
    }

    println!("-- Successfully extracted .sb3 file to {}.", directory);
    Ok(())
}

fn handle_json(directory: &str, json: &str) -> Result<(), Box<dyn std::error::Error>> {
    // https://assets.scratch.mit.edu/internalapi/asset/<MD5EXT>/get/
    // this just tries to find and download all the assets for the json

    println!("-- Downloading project assets.");

    fs::write(directory.to_owned() + "/project.json", json)?;

    let json: Value = serde_json::from_str(json)?;

    let targets = json["targets"].as_array().expect("Error parsing json.").clone();
    let total_targets = targets.len();

    for (target_index, target) in targets.iter().enumerate() {
        let costumes = target["costumes"].as_array().expect("Error parsing json.").clone();
        let sounds = target["sounds"].as_array().expect("Error parsing json.").clone();

        let assets: Vec<Value> = costumes.into_iter().chain(sounds).collect();
        let total_assets = assets.len();

        for (asset_index, asset) in assets.iter().enumerate() {
            let asset_id = asset["assetId"].as_str().expect("Error parsing json.");
            let data_format = asset["dataFormat"].as_str().expect("Error parsing json.");

            let md5ext = asset_id.to_owned() + "." + data_format;

            print!("\r--- Fetching {} - asset: {}/{}, target: {}/{}", md5ext, asset_index + 1, total_assets, target_index + 1, total_targets);
            std::io::stdout().flush()?;

            let link = "https://assets.scratch.mit.edu/internalapi/asset/".to_owned() + md5ext.as_str() + "/get/";

            let response = reqwest::blocking::get(link.as_str())?;
            match response.status() {
                StatusCode::OK => {}
                _ => {
                    println!("\n--- Issue fetching {}, {}.", md5ext, response.status());
                    println!("{}", link);
                    return Ok(());
                }
            }

            let bytes = response.bytes()?;
            fs::write(directory.to_owned() + "/" + md5ext.as_str(), bytes)?;
        }
    }

    println!("\n-- Successfully downloaded project assets.");

    Ok(())
}
