use reqwest::blocking::{Client, multipart};
use reqwest::StatusCode;
use crate::storage::{Storage, StorageError};
use std::path::Path;
use std::collections::HashMap;

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
        
        match _res.status() {
            StatusCode::OK => Ok(()),
            _              => Err(Box::new(StorageError::AccessError)),
        }
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

mod test
{
    use super::*;
    #[test]
    fn image_upload_test() {
        
        let config  = ini!("./etc/img_sync");
        let token   = config["slack"]["token"].clone().unwrap();
        let channel_id = config["slack"]["channel_id"].clone().unwrap();
        let slack = Slack::new(&token, &channel_id);
        let test_image_path = Path::new("./Debug/test.png");
        assert!(slack.upload(test_image_path).is_ok());
    }
}