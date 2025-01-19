use crate::*;

mod field_position;
mod game_context;
mod quadratic_bezier;

pub use field_position::*;
pub use game_context::*;
pub use quadratic_bezier::*;

pub struct Game {
    track_spline: Vec<FieldPosition>,
}

// the play field is 20% to the right of the screen.
pub const FIELD_OFFSET_X: f32 = 0.0; 

impl Game {

    pub fn new() -> Self {
        Self {
            track_spline: Vec::with_capacity(512),
        }
    }

	pub fn track_spline_uniform_buffer(
            &self, 
            aspect_x: f32, aspect_y: f32) -> Vec<f32> {

		self.track_spline
            .chunks_exact(3).map(|t| QuadraticBezier::from((t[0], t[1], t[2])))
            .collect::<Vec<_>>()
            .iter().map(
            |bezier| bezier
                .to_padded_buffer(
			         (aspect_x, aspect_y), 
                     (FIELD_OFFSET_X, 0.0)))
    			.collect::<Vec<_>>()
                .into_iter()
                .flatten()
                .collect::<Vec<f32>>()
    }

    pub fn track_spline_vertex_buffer(
            &self, 
            aspect_x: f32, aspect_y: f32) -> Vec<f32> {

        self.track_spline.iter().map(
            |vertex| {
                let v = vertex
                    .clipspace(
                         (aspect_x, aspect_y), 
                         (FIELD_OFFSET_X, 0.0));
                [v.0, v.1]
            })
            .flatten()
            .collect()
    }



    pub fn load_stage(&mut self, stage: &dyn Stage) {
        self.track_spline = stage.track_spline();
    }
}


pub fn start_game_loop(mut game_context: GameContext, mut game: Game) {
	// https://github.com/anlumo/webgl_rust_demo/blob/master/src/renderer.rs
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let outer_f = f.clone();
    let wgl_context = game_context.wp.context.clone();
    let window = game_context.wp.window.clone();
	let wgl_context = wgl_context.borrow();
	let window = window.borrow();
    let window = window.clone();

	let attr = wgl_context.get_context_attributes().unwrap();
   	attr.set_antialias(true);

    *outer_f.borrow_mut() = Some(Closure::wrap(Box::new(move || {
   	 	do_loop_iter(&mut game_context, &mut game);
        window.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("failed requesting animation frame");
    }) as Box<dyn FnMut()>));

    let window = web_sys::window().unwrap();
    window.request_animation_frame(outer_f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("failed requesting animation frame");
}


fn do_loop_iter(game_context: &mut GameContext, game: &mut Game) {
    let (pixels_x, pixels_y) = game_context.display_resolution();
    let (canvas_x, canvas_y) = game_context.mouse_internal_resolution();
    //alert(&format!("vert {:?}", (pixels_x, pixels_y) ));
    let (aspect_x, aspect_y) = game_context.aspect_ratio;
    let (mouse_x, mouse_y) = game_context.input_handler.mouse_clipspace_coords(canvas_x, canvas_y);
	let wgl_context = game_context.wp.context.clone();
	let wgl_context = wgl_context.borrow();
    
    wgl_context.clear_color(0.0, 0.0, 0.0, 1.0);
    wgl_context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    // render the track
    wgl_context.use_program(Some(&game_context.track_shader));

    renderer::load_buffer(
        &[pixels_x, pixels_y, 0.0, 0.0], 
        renderer::BufferArg::Uniform(
            BufferDataType::Float, "u_resolution".to_string(), renderer::UNIFORM_RESOLUTION_IDX), 
        &wgl_context, &game_context.track_shader);

    renderer::load_buffer(
        &game_context.camera.to_buffer((aspect_x, aspect_y), (FIELD_OFFSET_X, 0.0)), 
        renderer::BufferArg::Uniform(BufferDataType::Float, "u_camera".to_string(), renderer::UNIFORM_CAMERA_IDX), 
        &wgl_context, &game_context.track_shader);
    let track_spline_vertex_buffer = game.track_spline_vertex_buffer(aspect_x, aspect_y);
    renderer::load_buffer(
    	&track_spline_vertex_buffer, 
    	renderer::BufferArg::Vertexes(2), 
    	&wgl_context, 
    	&game_context.track_shader);
    // pad vec2 to vec4 cause webgl is dumb 
	let track_nodes_uniform_buf: Vec<f32> = game.track_spline_uniform_buffer(aspect_x, aspect_y);
    renderer::load_buffer(&track_nodes_uniform_buf, 
    	renderer::BufferArg::Uniform(BufferDataType::Float, "u_bezier_nodes".to_string(), renderer::UNIFORM_NODES_IDX), 
    	&wgl_context, &game_context.track_shader);
    wgl_context.draw_arrays(
        WebGl2RenderingContext::TRIANGLES, 0,
        (track_spline_vertex_buffer.len() / 2) as i32);

    //
    //alert(&format!("vert {:?}", track_spline_vertex_buffer));
    //alert(&format!("uni {:?}", track_nodes_uniform_buf));
    // render the UI
    let mouse_draw = mouse_pos_bar_clipspace_vert(mouse_x, mouse_y);
    wgl_context.use_program(Some(&game_context.interface_shader));
    //alert(&format!("mouse {:?}", &mouse_draw.1));
    renderer::load_buffer(&mouse_draw.1, renderer::BufferArg::Vertexes(2), &wgl_context, &game_context.interface_shader);
	wgl_context.draw_arrays(mouse_draw.0, 0, (mouse_draw.1.len() / 2) as i32);
}

