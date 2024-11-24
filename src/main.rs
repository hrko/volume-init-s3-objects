use aws_config;
use aws_sdk_s3;
use log::{error, info};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::time::Duration;
use tokio::time::timeout;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Config {
    time_out_sec: Option<u64>,
    retry_count_per_file: Option<u32>,
    files: HashMap<String, S3File>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct S3File {
    bucket: String,
    key: String,
    version_id: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let config: Config = load_config();
    let time_out_sec = config.time_out_sec.unwrap_or(15);
    let retry_count = config.retry_count_per_file.unwrap_or(3);

    let mut handles = vec![];
    let files = config.files.clone();

    // Create a task for each file to download
    for (path, s3_file) in files.iter() {
        let path = path.clone();
        let s3_file = s3_file.clone();

        let handle = tokio::spawn(timeout(Duration::from_secs(time_out_sec), async move {
            for attempt in 1..=retry_count {
                match download_file(&path, &s3_file).await {
                    Ok(_) => {
                        info!("Successfully downloaded {} on attempt {}", path, attempt);
                        break;
                    }
                    Err(e) => {
                        error!(
                            "Failed to download {} on attempt {}: {:?}",
                            path, attempt, e
                        );
                        if attempt == retry_count {
                            error!("Exceeded retry limit for {}", path);
                        } else {
                            tokio::time::sleep(Duration::from_secs(2)).await;
                        }
                    }
                }
            }
        }));

        handles.push(handle);
    }

    // Wait for all downloads to complete
    for handle in handles {
        if let Err(e) = handle.await {
            error!("Failed to download file: {:?}", e);
        }
    }
    info!("All downloads completed");
}

fn load_config() -> Config {
    let config_str = env::var("VOLUME_INIT_S3_OBJECTS_CONFIG")
        .expect("VOLUME_INIT_S3_OBJECTS_CONFIG environment variable not set");
    serde_json::from_str(&config_str).expect("Failed to parse config")
}

async fn download_file(
    path: &str,
    s3_file: &S3File,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Load the default AWS SDK configuration
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = aws_sdk_s3::Client::new(&config);

    // Download the file
    let mut request = client
        .get_object()
        .bucket(s3_file.bucket.clone())
        .key(s3_file.key.clone());
    if let Some(version_id) = &s3_file.version_id {
        request = request.version_id(version_id.clone());
    }
    let mut object = request.send().await?;

    // Write the file
    let mut file = std::fs::File::create(path)?;
    while let Some(bytes) = object.body.try_next().await? {
        file.write_all(&bytes)?;
    }

    Ok(())
}
