use js_sys::Uint8ClampedArray;
use dominator::{html, with_node};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlCanvasElement, ImageData};

mod mandelbrot;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const MAX_ITERATIONS: u32 = 1000;

#[wasm_bindgen(main)]
pub async fn main() {
    /* initialize the thread pool with two workers */
    JsFuture::from(wasm_bindgen_rayon::init_thread_pool(2)).await
        .expect("could not initialize thread pool");

    /* create the html for our page */
    dominator::append_dom(&dominator::body(), html!("div", {
        .child(html!("h1", { .text("Mandelbrot fractal") }))
        .child(html!("canvas" => HtmlCanvasElement, {
            .with_node!(canvas => {
                /* future polls a impl Future<Output = T> while the node is added to the
                   DOM. Under the hood, it is using wasm_bindgen_futures::spawn_local */
                .future(draw(canvas))
            })
        }))
    }));
}

async fn draw(canvas: HtmlCanvasElement) {
    /* generate data off the main thread, keeping the user interface responsive.
       async_rayon allows us to await the CPU-bound work via a future */
    let data = async_rayon::spawn(|| {
        mandelbrot::Generator::new(WIDTH, HEIGHT, MAX_ITERATIONS)
            .iter_bytes()
            .collect::<Vec<u8>>()
    }).await;
    
    /* convert the generated data to a ImageData via a Uint8ClampedArray */
    let data = Uint8ClampedArray::from(data.as_slice());
    let image_data = ImageData::new_with_js_u8_clamped_array(&data, WIDTH).unwrap();
    
    /* set the width and height and draw the pixels */
    canvas.set_width(WIDTH);
    canvas.set_height(HEIGHT);
    canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .unchecked_into::<web_sys::CanvasRenderingContext2d>()
        .put_image_data(&image_data, 0.0, 0.0)
        .unwrap();
}
