use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{thread, time};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f64,
    y: f64,
}

struct Boid {
    position: Point,
    velo: Point,
}

// Handles a single client
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let boids_vec: Vec<Boid> = Vec::new();

    println!("Incoming connection from: {}", stream.peer_addr()?);
    //let mut buf = [0; 512];
    let ten_millis = time::Duration::from_millis(20);

    let mut point = Point { x: 0., y: 0. };
    let mut v = Point { x: 1., y: 1. };

    let win_size_w = 400.;
    let win_size_h = 400.;

    let BALL_RADIUS = 20.;
    let COL_PADDING = 20.;

    // Convert the Point to a JSON string.

    loop {
        // let bytes_read = stream.read(&mut buf)?;
        // if bytes_read == 0 {
        //     return Ok(());
        // }

        point.x += v.x;
        point.y += v.y;

        let win_w_half = win_size_w / 2.;
        let win_h_half = win_size_h / 2.;

        if point.x <= (-win_w_half + BALL_RADIUS) {
            v.x *= -1.;
            point.x += COL_PADDING;
        } else if point.x >= (win_w_half - BALL_RADIUS) {
            v.x *= -1.;
            point.x -= COL_PADDING;
        }

        if point.y <= (-win_h_half + BALL_RADIUS) {
            v.y *= -1.;
            point.y += COL_PADDING;
        } else if point.y >= (win_h_half - BALL_RADIUS) {
            v.y *= -1.;
            point.y -= COL_PADDING;
        }

        {
            // println!("{:?}", point);
            let serialized = serde_json::to_string(&point).unwrap();
            println!("{:?}", serialized);
            thread::sleep(ten_millis);
            stream.write(serialized.as_bytes())?;
            // stream.write("Y".as_bytes())?;
            stream.write("\n".as_bytes())?;
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").expect("Could not bind");

    for stream in listener.incoming() {
        print!("New connection");
        match stream {
            Err(e) => {
                eprintln!("failed: {}", e)
            }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
