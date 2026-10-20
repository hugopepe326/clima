use futures_channel::oneshot;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::PositionOptions;

/// Pide la posición actual al navegador usando `web_sys::Geolocation`.
/// Devuelve `(latitud, longitud, precisión en metros)`.
pub async fn request_position() -> Result<(f64, f64, f64), String> {
    let window = web_sys::window()
        .ok_or_else(|| "No se pudo acceder a la ventana del navegador".to_string())?;
    let geolocation = window
        .navigator()
        .geolocation()
        .map_err(|_| "Este navegador no expone la API de geolocalización".to_string())?;

    let (sender, receiver) = oneshot::channel::<Result<(f64, f64, f64), String>>();
    let sender = Rc::new(RefCell::new(Some(sender)));

    let success_sender = Rc::clone(&sender);
    let success = Closure::once(move |position: web_sys::Position| {
        let coords = position.coords();
        if let Some(tx) = success_sender.borrow_mut().take() {
            let _ = tx.send(Ok((
                coords.latitude(),
                coords.longitude(),
                coords.accuracy(),
            )));
        }
    });

    let error_sender = Rc::clone(&sender);
    let error = Closure::once(move |error: web_sys::PositionError| {
        let message = match error.code() {
            1 => "Permiso de ubicación denegado. Podés habilitarlo desde el candado de la barra de direcciones.".to_string(),
            2 => "La ubicación no está disponible en este momento.".to_string(),
            3 => "Se agotó el tiempo para obtener la ubicación. Intentá de nuevo.".to_string(),
            _ => {
                let detail = error.message();
                if detail.is_empty() {
                    "No se pudo obtener la ubicación.".to_string()
                } else {
                    detail
                }
            }
        };
        if let Some(tx) = error_sender.borrow_mut().take() {
            let _ = tx.send(Err(message));
        }
    });

    let options = PositionOptions::new();
    options.set_enable_high_accuracy(true);
    options.set_timeout(15_000);
    options.set_maximum_age(30_000);

    geolocation
        .get_current_position_with_error_callback_and_options(
            success.as_ref().unchecked_ref(),
            Some(error.as_ref().unchecked_ref()),
            &options,
        )
        .map_err(|_| {
            "El navegador bloqueó la geolocalización. Se requiere HTTPS o localhost.".to_string()
        })?;

    // Las closures deben seguir vivas hasta que el navegador responda.
    success.forget();
    error.forget();

    receiver
        .await
        .map_err(|_| "La solicitud de ubicación se canceló".to_string())?
}
