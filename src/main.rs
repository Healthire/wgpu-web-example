use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{closure::Closure, JsCast};

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info).unwrap();

    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);

    let document = web_sys::window()
        .and_then(|win| win.document())
        .expect("Cannot get document");
    let canvas = document
        .create_element("canvas")
        .expect("Cannot create canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Cannot get canvas element");
    document
        .body()
        .expect("Cannot get document body")
        .append_child(&canvas)
        .expect("Cannot insert canvas into document body");

    let size = (800, 600);
    canvas
        .set_attribute("width", &format!("{}", size.0))
        .expect("cannot set width");
    canvas
        .set_attribute("height", &format!("{}", size.1))
        .expect("cannot set height");

    log::info!("Init!");

    let surface = wgpu::Surface::create_with_canvas(&canvas);
    wasm_bindgen_futures::spawn_local(async move {
        let adapter = wgpu::Adapter::request_async(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
        })
        .await;
        let (device, queue) = adapter
            .request_device_async(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: wgpu::Limits::default(),
            })
            .await;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Vsync,
        };
        let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |_time: f64| {
            let frame = swap_chain
                .get_next_texture()
                .expect("Timeout when acquiring next swap chain texture");
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

            let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                        a: 1.,
                    },
                }],
                depth_stencil_attachment: None,
            });

            std::mem::drop(rpass);

            queue.submit(&[encoder.finish()]);

            web_sys::window()
                .expect("no global window")
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .expect("could not request animation frame");
        }) as Box<dyn FnMut(f64)>));

        web_sys::window()
            .expect("no global window")
            .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("could not request animation frame");
    });
}
