use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() -> Result<(), Error> {
   

    // declare matrices as nested vectors
    let matrix_a: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    let matrix_b: Vec<Vec<i32>> = vec![
        vec![9, 8, 7],
        vec![6, 5, 4],
        vec![3, 2, 1],
    ];

    // lemme see them
    fn print_matrix(matrix: &Vec<Vec<i32>>, name: &str) {
        println!("{}:", name);
        for row in matrix {
            println!("{:?}", row);
        }
    }

    print_matrix(&matrix_a, "matrix_a");
    print_matrix(&matrix_b, "matrix_b");
    // multiply em up
    fn multiply_matrices(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = a.len();
        let mut result = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }

        result
    
    }
    print_matrix(&multiply_matrices(&matrix_a, &matrix_b), "result");

    //check if two matricies are multiplyable (a columns must equal b rows)
    fn check_dimensional_equivalence(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> bool {
        if a.is_empty() || b.is_empty() {
            return false;
        }

        let a_columns = a[0].len();
        let b_rows = b.len();
        

        a_columns == b_rows
    }

    fn scale_matrix(matrix: &mut Vec<Vec<i32>>, scalar: i32) {
        for row in matrix {
            for val in row {
                *val *= scalar;
            }
        }
    }

    // take two points in coordinate space and create the vec of the movement from a to b
    fn point_to_point_to_matrix(point_a: (i32, i32), point_b: (i32, i32)) -> (i32, i32) {
        return (point_b.0 - point_a.0, point_b.1 - point_a.1)
    }


    // add em up
    fn sum_matrices(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = a.len();
        let mut result = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                result[i][j] = a[i][j] + b[i][j];
            }
        }

        result
    }

    // how long is the vector
    fn magnitude(matrix: &Vec<Vec<i32>>) -> f64 {
        let mut sum = 0;
        for row in matrix {
            for val in row {
                sum += val * val;
            }
        }
        (sum as f64).sqrt()
    }

    //how much do they align 
    fn dot_product(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> i32 {
        let n = a.len();
        let mut result = 0;

        for i in 0..n {
            for j in 0..n {
                result += a[i][j] * b[i][j];
            }
        }

        result
    }
    
    fn angle_between(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> f64 {
        let dot = dot_product(a, b) as f64;
        let mag_a = magnitude(a);
        let mag_b = magnitude(b);
        (dot / (mag_a * mag_b)).acos()
    }

    //yaw
    fn x_axis_rotation_matrix(angle: f64) -> Vec<Vec<f64>> {
        vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, angle.cos(), -angle.sin()],
            vec![0.0, angle.sin(), angle.cos()],
        ]
    }
    // pitch
    fn y_axis_rotation_matrix(angle: f64) -> Vec<Vec<f64>> {
        vec![
            vec![angle.cos(), 0.0, angle.sin()],
            vec![0.0, 1.0, 0.0],
            vec![-angle.sin(), 0.0, angle.cos()],
        ]
    }
    // roll
    fn z_axis_rotation_matrix(angle: f64) -> Vec<Vec<f64>> {
        vec![
            vec![angle.cos(), -angle.sin(), 0.0],
            vec![angle.sin(), angle.cos(), 0.0],
            vec![0.0, 0.0, 1.0],
        ]
    }

    // init winit innit
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Renderer")
        .with_inner_size(LogicalSize::new(800u32, 800u32))
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(800, 600, surface_texture)?
    };
    //event handler / main loop (non blocking)
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawRequested(_) => {
                // Clear to black
                let frame = pixels.frame_mut();
                for pixel in frame.chunks_exact_mut(4) {
                    pixel[0] = 0x00; // R
                    pixel[1] = 0x00; // G  
                    pixel[2] = 0x00; // B
                    pixel[3] = 0xff; // A
                }

                if let Err(err) = pixels.render() {
                    eprintln!("pixels.render() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}   