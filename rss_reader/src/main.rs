use windows::{
    Foundation::Uri, Web::Syndication::SyndicationClient, core::*
};
mod helper;
use crate::helper::get_feed;
use crate::helper::print_feed;



// Observe que el tipo de valor devuelto de la función main es un resultado, de windows::core::. Esto hará que las cosas sean más fáciles, ya que es habitual tratar los errores de las API del sistema operativo (SO). windows::core::Result nos ayuda con la propagación de errores y con una administración concisa de los mismos.
fn main() -> windows::core::Result<()> {

    // Puede ver el operador de signo de interrogación al final de la línea de código. Para ahorrar en escritura, hacemos esto para usar la propagación de errores y la lógica de cortocircuito de Rust. Esto significa que no es necesario realizar un montón de control manual de errores para este ejemplo sencillo.
    // Observe también la macro h! del crate windows. Se usa para construir una referencia a HSTRING a partir de un literal de cadena de Rust. La API de WinRT utiliza extensamente HSTRING para los valores de cadena.


    let uri = Uri::CreateUri(h!("https://blogs.windows.com/feed/"))?;
    let client = SyndicationClient::new()?;

    print_feed(&get_feed(&client, &uri)?)?;

    Ok(())

}