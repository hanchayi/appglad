use yew::{Component, Context, html, Html, Properties, NodeRef};
use web_sys::{HtmlCanvasElement, WebGlRenderingContext as GL};
use yew::html::Scope;
use gloo_render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::{JsCast, prelude::Closure};

use crate::console_log;

#[derive(PartialEq, Properties)]
pub struct Props;

pub enum Msg {
    Click,
    Resize,
    Render(f64),
    MouseMove,
}

pub struct Canvas {
    gl: Option<GL>,
    node_ref: NodeRef,
    _render_loop: Option<AnimationFrame>,
    width: i32,
    height: i32,
    listener: Option<Closure<dyn FnMut(web_sys::Event)>>,
}

impl Component for Canvas {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            gl: None,
            node_ref: NodeRef::default(),
            _render_loop: None,
            width: 0,
            height: 0,
            listener: None,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let link = _ctx.link();
        html! {
            <canvas class="w-full h-full block" onclick={link.callback(|_| Msg::Click)} onmousemove={link.callback(|_| Msg::MouseMove)} ref={self.node_ref.clone()} />
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render(timestamp) => {
                // Render functions are likely to get quite large, so it is good practice to split
                // it into it's own function rather than keeping it inline in the update match
                // case. This also allows for updating other UI elements that may be rendered in
                // the DOM like a framerate counter, or other overlaid textual elements.
                self.render_gl(timestamp, ctx.link());
                false
            },
            Msg::Click => {
                console_log!("clicked");
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            },
            Msg::MouseMove => {
                // console_log!("mouse move");
                true
            },
            Msg::Resize => {
                self.resize();
                true
            },
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // Once rendered, store references for the canvas and GL context. These can be used for
        // resizing the rendering area when the window or canvas element are resized, as well as
        // for making GL calls.

        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.gl = Some(gl);

        // In a more complex use-case, there will be additional WebGL initialization that should be
        // done here, such as enabling or disabling depth testing, depth functions, face
        // culling etc.

        if first_render {
            console_log!("first_render");
            // The callback to request animation frame is passed a time value which can be used for
            // rendering motion independent of the framerate which may vary.
            let handle = {
                let link = ctx.link().clone();
                request_animation_frame(move |time| link.send_message(Msg::Render(time)))
            };

            // A reference to the handle must be stored, otherwise it is dropped and the render won't
            // occur.
            self._render_loop = Some(handle);
            self.resize();
            self.add_event_listener(ctx.link());
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        console_log!("destroy");
        self.remove_event_listener()
    }
}

impl Canvas {
    fn add_event_listener(&mut self, link: &Scope<Self>) {
        let window = web_sys::window().expect("no global `window` exists");
        let closure = {
            let link = link.clone();
            Closure::<dyn FnMut(web_sys::Event)>::wrap(Box::new(move |_event| {
                link.send_message(Msg::Resize)
            }))
        };
        let res = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
        self.listener = Some(closure);

        if res.is_err() {
            panic!("add event listener error");
        }
        // closure.forget();
    }

    fn remove_event_listener(&mut self) {
        let window = web_sys::window().expect("no global `window` exists");

        if let Some(listener) = self.listener.take()  {
            console_log!("remove resize");
            window.remove_event_listener_with_callback("resize", listener.as_ref().unchecked_ref()).unwrap();
        }
    }

    fn resize(&mut self) {
        console_log!("resize");
        let window = web_sys::window().expect("no global `window` exists");
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

        let offset_width = canvas.offset_width();
        let offset_height = canvas.offset_height();

        window.device_pixel_ratio();

        let display_width = (offset_width * 2) as u32;
        let display_height = (offset_height * 2) as u32;
        self.width = display_width as i32;
        self.height = display_height as i32;

        if canvas.width() != display_width || canvas.height() != display_height {
            canvas.set_width(display_width);
            canvas.set_height(display_height);
        }
    }

    fn render_gl(&mut self, timestamp: f64, link: &Scope<Self>) {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");
        gl.viewport(0, 0, self.width, self.height);

        let vert_code = include_str!("../assets/basic.vert");
        let frag_code = include_str!("../assets/basic.frag");

        // This list of vertices will draw two triangles to cover the entire canvas.
        let vertices: Vec<f32> = vec![
            -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];
        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        // Attach the position vector as an attribute for the GL context.
        let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        // Attach the time as a uniform for the GL context.
        let time = gl.get_uniform_location(&shader_program, "u_time");
        gl.uniform1f(time.as_ref(), timestamp as f32);

        gl.draw_arrays(GL::TRIANGLES, 0, 6);

        let handle = {
            let link = link.clone();
            request_animation_frame(move |time| link.send_message(Msg::Render(time)))
        };

        // A reference to the new handle must be retained for the next render to run.
        self._render_loop = Some(handle);
    }
}