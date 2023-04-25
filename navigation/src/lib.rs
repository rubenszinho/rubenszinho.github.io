use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::XmlHttpRequest;

#[wasm_bindgen]
pub fn navigate_to_page(page_name: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let content_element = document
        .get_element_by_id("content")
        .expect("element not found");

    let url = format!("./pages/{}.html", page_name);

    let request = XmlHttpRequest::new().unwrap();
    request.open("GET", &url).unwrap();

    let content_element_clone = content_element.clone();
    let request_clone = request.clone();
    let onload_callback = Closure::wrap(Box::new(move || {
        let response = request_clone.response_text().unwrap().unwrap();
        content_element_clone.set_inner_html(&response);
    }) as Box<dyn FnMut()>);

    request.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
    onload_callback.forget();

    request.send().unwrap();
}
