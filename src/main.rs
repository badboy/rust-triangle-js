#[macro_use]
extern crate glium;

use std::cell::RefCell;
use std::ptr::null_mut;
use std::os::raw::{c_int, c_void};

use glium::{DisplayBuild, Surface};

fn main() {
    let display = glium:: glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium().unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = program!(&display,
        100 => {
            vertex: "
                #version 100

                attribute lowp vec2 position;
                varying lowp vec2 my_attr;      // our new attribute

                uniform lowp mat4 matrix;

                void main() {
                    my_attr = position;     // we need to set the value of each `out` variable.
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                }
            ",

            fragment: "
                #version 100

                varying lowp vec2 my_attr;

                void main() {
                    gl_FragColor = vec4(my_attr, 0.0, 1.0);   // we build a vec4 from a vec2 and two floats
                }
            ",
        },
    ).expect("Can't compile shader program");

    let mut t = -0.5;
    let mut v = 0.002;

    set_main_loop_callback(|| {
        // we update `t`
        t += v;
        if t > 0.5 {
            v = -v;
        }
        if t < -0.5 {
            v = -v;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ]
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).expect("Can't draw");
        target.finish().expect("Can't finish");
    });
}

#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern fn();
extern {
    fn emscripten_set_main_loop(func : em_callback_func, fps : c_int, simulate_infinite_loop : c_int);
}

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

pub fn set_main_loop_callback<F>(callback : F) where F : FnMut() {
    MAIN_LOOP_CALLBACK.with(|log| {
            *log.borrow_mut() = &callback as *const _ as *mut c_void;
            });

    unsafe { emscripten_set_main_loop(wrapper::<F>, 0, 1); }

    unsafe extern "C" fn wrapper<F>() where F : FnMut() {
        MAIN_LOOP_CALLBACK.with(|z| {
            let closure = *z.borrow_mut() as *mut F;
            (*closure)();
        });
    }
}
