use std::{
	time::{Duration, Instant},
};

use glium::{
	glutin::{event, event_loop, window, ContextBuilder},
	texture::{TextureHandle},
};
use mtge_core::{
	render::{self, sprite::Sprite2d},
	utils::tex,
};
use nalgebra_glm::*;


fn main() {
	#[allow(unused_imports)]
	use glium::{glutin, Surface};

	let event_loop = event_loop::EventLoop::new();
	let wb = window::WindowBuilder::new();
	let cb = ContextBuilder::new().with_depth_buffer(16);
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();

	let program = render::generate_program(&display).unwrap();
	let (_, texture) = tex::rgba8_srgb2d(
		"C:/Users/Gebruiker/Pictures/Saved Pictures/botw glitch.png",
		// "C:/Users/Gebruiker/Pictures/Saved Pictures/NNN.png",
		&display,
	)
	.unwrap();

	let sprite = Sprite2d::new(0, translation2d(&vec2(0.0, 0.0)), 0.0);

	let (vertex_buffer, index_buffer) = render::generate_buffers(&display, &[sprite]).unwrap();

	let mut t = -0.5;
	event_loop.run(move |event, _, control_flow| {
		match event {
			event::Event::WindowEvent { event, .. } => match event {
				event::WindowEvent::CloseRequested => {
					*control_flow = event_loop::ControlFlow::Exit;
					return;
				}
				_ => return,
			},
			event::Event::NewEvents(cause) => match cause {
				event::StartCause::ResumeTimeReached { .. } => (),
				event::StartCause::Init => (),
				_ => return,
			},
			_ => return,
		}

		let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
		*control_flow = event_loop::ControlFlow::WaitUntil(next_frame_time);

		// we update `t`
		t += 0.002;
		if t > 0.5 {
			t = -0.5;
		}

		let mut target = display.draw();
		target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

		let camera = translation(&vec3(t as f32, 0.0, 0.0));
		let camera = rotate(&camera, std::f32::consts::TAU * t, &vec3(0.0, 0.0, 1.0));

		let handle = TextureHandle::new(&texture, &Default::default());
		let texture_buffer = render::generate_textures(&display, handle).unwrap();

		render::render(
			&program,
			&texture_buffer,
			&mut target,
			camera,
			&vertex_buffer,
			&index_buffer,
		)
		.unwrap();

		// let uniforms = uniform! {
		// 	camera: camera.data.0,
		// 	Textures: &renderer.texture_buffer
		// };

		target.finish().unwrap();
	});
}

mod tests {
	#[test]
	fn format_string_test() {
		let str = "hello there {}!";
		let str2 = str.replace("{}", "programmer");
		assert_eq!(str2, "hello there programmer!");
	}
}
