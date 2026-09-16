#[cfg(test)]
mod tests {
    use windows::{
        Foundation::Uri,
        Web::Syndication::SyndicationClient,
        core::HSTRING,
    };

    #[test]
    fn can_create_uri() {
        let uri = Uri::CreateUri(&HSTRING::from("https://blogs.windows.com/feed/")).unwrap();
        let value = uri.AbsoluteUri().unwrap();
        assert!(value.to_string().contains("blogs.windows.com"));
    }

    #[test]
    fn can_create_client() {
        let client = SyndicationClient::new().unwrap();
        let _ = client;
    }
}
