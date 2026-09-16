use windows::{
    Foundation::Uri, Web::Syndication::SyndicationClient, Win32::UI::WindowsAndMessaging::{MB_OK, MessageBoxW}, core::*
};

pub fn get_feed(client: &SyndicationClient, uri: &Uri) -> windows::core::Result<windows::Web::Syndication::SyndicationFeed> {

    client.SetRequestHeader(
        h!("User-Agent"),
        h!("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/140.0 Safari/537.36"),
    )?;

    let feed = client.RetrieveFeedAsync(uri)?.join()?;

    Ok(feed)
}


pub fn print_feed(
    feed: &windows::Web::Syndication::SyndicationFeed,
) -> windows::core::Result<()> {
    unsafe {
        for item in feed.Items()? {
            let title = item.Title()?.Text()?;

            MessageBoxW(
                None,
                PCWSTR(title.as_ptr()),
                h!("RSS Reader"),
                MB_OK,
            );
        }
    }

    Ok(())
}