// src\main.rs
use windows::{
    core::*,
    Foundation::Uri,
    Web::Syndication::SyndicationClient
};

fn main() -> Result<()> {

    // Observe que el tipo de valor devuelto de la función main es un resultado, de windows::core::. Esto hará que las cosas sean más fáciles, ya que es habitual tratar los errores de las API del sistema operativo (SO). windows::core::Result nos ayuda con la propagación de errores y con una administración concisa de los mismos.
    let uri = Uri::CreateUri(h!("https://blogs.windows.com/feed"))?;

    Ok(())
}