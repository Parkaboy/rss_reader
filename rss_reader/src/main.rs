// src\main.rs
use windows::{
    Foundation::Uri, Web::Syndication::SyndicationClient, Win32::UI::WindowsAndMessaging::{MB_OK, MessageBoxW}, core::*
};



// Observe que el tipo de valor devuelto de la función main es un resultado, de windows::core::. Esto hará que las cosas sean más fáciles, ya que es habitual tratar los errores de las API del sistema operativo (SO). windows::core::Result nos ayuda con la propagación de errores y con una administración concisa de los mismos.
fn main() -> windows::core::Result<()> {

    // Puede ver el operador de signo de interrogación al final de la línea de código. Para ahorrar en escritura, hacemos esto para usar la propagación de errores y la lógica de cortocircuito de Rust. Esto significa que no es necesario realizar un montón de control manual de errores para este ejemplo sencillo.
    // Observe también la macro h! del crate windows. Se usa para construir una referencia a HSTRING a partir de un literal de cadena de Rust. La API de WinRT utiliza extensamente HSTRING para los valores de cadena.


    let uri = Uri::CreateUri(h!("https://blogs.windows.com/feed/"))?;
    let client = SyndicationClient::new()?;

    let feed = getFeed(&client, &uri)?;

    print_feed(&feed)?;

    Ok(())

}


fn getFeed(client: &SyndicationClient, uri: &Uri) -> windows::core::Result<windows::Web::Syndication::SyndicationFeed> {

    client.SetRequestHeader(
        h!("User-Agent"),
        h!("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/140.0 Safari/537.36"),
    )?;

    let feed = client.RetrieveFeedAsync(uri)?.join()?;

    Ok(feed)
}


fn print_feed(
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