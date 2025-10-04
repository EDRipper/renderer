// graphics imports
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

//file reading imports
use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn read_file_into_array() -> io::Result<Vec<String>> {//as of now i cba to parse binary stl. only ascii stl files.
    let file_path = "model.stl"; //keep in same dir
    let file = File::open(file_path)?;// open read only
    let reader = BufReader::new(file);// buffer for efficiency

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?; // Handle potential errors reading each line
        //println!("{}", line);
        lines.push(line);
    }
    Ok(lines)
}

fn main() {
    let model_arr = read_file_into_array().expect("Failed to read file");

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
        Pixels::new(800, 800, surface_texture).unwrap()
    };

    // coords for triangles
    let base_points = vec![
        vec![0.0, 0.0, 0.0],// Triangle 1 
        vec![100.0, 0.0, 0.0],   
        vec![0.0, 100.0, 0.0],   
        
        vec![100.0, 0.0, 0.0],// Triangle 2 
        vec![0.0, 100.0, 0.0],    
        vec![100.0, 100.0, 0.0]
    ];

    let mut theta = 0.0;

    // Helper function to apply 3D matrix multiplication
    fn matrix_multiply_3d(matrix: &Vec<Vec<f64>>, point: &Vec<f64>) -> Vec<f64> {
        vec![
            matrix[0][0] * point[0] + matrix[0][1] * point[1] + matrix[0][2] * point[2],
            matrix[1][0] * point[0] + matrix[1][1] * point[1] + matrix[1][2] * point[2],
            matrix[2][0] * point[0] + matrix[2][1] * point[1] + matrix[2][2] * point[2],
        ]
    }

    // Bresenham's line drawing algorithm 
    fn draw_line(frame: &mut [u8], x0: i32, y0: i32, x1: i32, y1: i32, width: u32) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        let mut x = x0;
        let mut y = y0;

        loop {
            // Draw pixel if within bounds
            if x >= 0 && x < width as i32 && y >= 0 && y < width as i32 {
                let idx = (y as usize * width as usize + x as usize) * 4;
                if idx + 3 < frame.len() {
                    frame[idx] = 255;     // R
                    frame[idx + 1] = 255; // G
                    frame[idx + 2] = 255; // B
                    frame[idx + 3] = 255; // A
                }
            }

            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn draw_triangle(frame: &mut [u8], p1: (i32, i32), p2: (i32, i32), p3: (i32, i32), width: u32) {
        draw_line(frame, p1.0, p1.1, p2.0, p2.1, width);
        draw_line(frame, p2.0, p2.1, p3.0, p3.1, width);
        draw_line(frame, p3.0, p3.1, p1.0, p1.1, width);
    }
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

                // Update rotation angle
                theta += 0.01; // Faster than Python version for visibility

                // Transform and render points (like the Python version)
                let mut transformed_points = Vec::new();
                
                for point in &base_points {
                    // Apply X-axis rotation
                    let x_rotated = matrix_multiply_3d(&x_axis_rotation_matrix(theta), point);
                    // Apply Z-axis rotation  
                    let z_rotated = matrix_multiply_3d(&z_axis_rotation_matrix(theta), &x_rotated);
                    
                    // Offset to center screen (like adding 200 in Python)
                    let screen_x = (z_rotated[0] + 400.0) as i32;
                    let screen_y = (z_rotated[1] + 400.0) as i32;
                    
                    transformed_points.push((screen_x, screen_y));
                }

                // Draw the two triangles (like in Python)
                if transformed_points.len() >= 6 {
                    draw_triangle(
                        frame, 
                        transformed_points[0], 
                        transformed_points[1], 
                        transformed_points[2], 
                        800
                    );
                    draw_triangle(
                        frame, 
                        transformed_points[3], 
                        transformed_points[4], 
                        transformed_points[5], 
                        800
                    );
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