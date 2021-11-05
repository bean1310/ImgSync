use reqwest::blocking::{Client, multipart};
use reqwest::StatusCode;
use crate::storage::{Storage, StorageError};
use std::path::Path;
use serde_json::Value;

pub struct Slack
{
    token: String,
    channel_id: String
}

impl Storage for Slack
{
    fn upload(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>>
    {
        let _client = Client::new();
        let _form = multipart::Form::new()
                        .text("channels", (self.channel_id).clone())
                        .file("file", path)?;
        let _res = _client.post("https://slack.com/api/files.upload")
                        .header("Authorization", "Bearer ".to_string() + &(self.token).clone())
                        .multipart(_form)
                        .send()?;


        if let StatusCode::OK = _res.status()
        {
            let response_body: Value = _res.json()?;
            // TODO: Error handling
            let _is_failed = !response_body["ok"].as_bool().unwrap();
            if _is_failed
            {
                let slack_api_error_message = response_body["error"].as_str().unwrap();
                Err(StorageError::ApiError(slack_api_error_message.to_string()))?
            }
        } 
        else
        {
            Err(StorageError::HttpError(_res.status().as_u16()))?
        }

        Ok(())
    }
    
    fn storage_name(&self) -> String {
        "Slack".to_string()
    }
}

impl Slack {
    pub fn new(token: &str, channel_id: &str) -> Self
    {
        Self {
            token: token.to_string(),
            channel_id: channel_id.to_string()
        }
    }
}

// mod test
// {
//     use super::*;
//     #[test]
//     fn image_upload_test() {
        
//         let config  = ini!("./etc/img_sync");
//         let token   = config["slack"]["token"].clone().unwrap();
//         let channel_id = config["slack"]["channel_id"].clone().unwrap();
//         let slack = Slack::new(&token, &channel_id);
//         let test_image_path = Path::new("./Debug/test.png");
//         assert!(slack.upload(test_image_path).is_ok());
//     }
// }