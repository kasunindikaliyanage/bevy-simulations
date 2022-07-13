use std::io::{Error, Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, RwLock};

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
async fn handle_client(mut stream: TcpStream, point_arc: Arc<RwLock<Point>>) -> Result<(), Error> {
    let mut serialized: String;
    loop {
        {
            let w = point_arc.read().unwrap();
            let point = &*w;
            //let point = point_arc.lock().unwrap();
            serialized = serde_json::to_string(&(*point)).unwrap();
        }
        stream.write(serialized.as_bytes()).await;
        stream.write("\n".as_bytes()).await;
        //println!("This is test");
        sleep(Duration::from_millis(15)).await;
    }

    Ok(())
}

async fn update_movement(point_arc: Arc<RwLock<Point>>) {
    let mut v = Point { x: 1., y: 1. };

    let win_size_w = 400.;
    let win_size_h = 400.;

    let BALL_RADIUS = 20.;
    let COL_PADDING = 20.;

    // Convert the Point to a JSON string.
    //let mut point;

    loop {
        // let bytes_read = stream.read(&mut buf)?;
        // if bytes_read == 0 {
        //     return Ok(());
        // }
        {
            //let mut point = point_arc.lock().unwrap();
            let mut w = point_arc.write().unwrap();
            let mut point = &mut *w;

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
        }
        //println!("This is also a test");
        sleep(Duration::from_millis(10)).await;
        //sleep(Duration::from_millis(1000)).await;
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let mut point_arc = Arc::new(RwLock::new(Point { x: 0., y: 0. }));

    let mut point = point_arc.clone();
    tokio::spawn(async move {
        update_movement(point).await;
    });

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // Clone the handle to the hash map.
        let point = point_arc.clone();

        tokio::spawn(async move {
            handle_client(socket, point).await;
        });
    }
}
