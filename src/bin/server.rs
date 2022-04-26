#![allow(unused_variables)]
#![allow(unused_imports)]
extern crate hyper;
extern crate pretty_env_logger;

use hyper::{Body, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};
use std::{env, mem};
use std::io::Cursor;
use hyper::http::HeaderValue;
use raytracer::render::renderer::Renderer;
use raytracer::SceneBuilder;

fn render(time: usize) -> Response<Body> {
    let builder = SceneBuilder { time };
    let mut renderer = Renderer::new(builder.scene());
    renderer.render();
    let mut content = vec![];
    let mut tar = tar::Builder::new(&mut content);
    for (name, image) in renderer.images() {
        let mut header = tar::Header::new_gnu();
        header.set_size(image.len() as u64);
        header.set_path(name).unwrap();
        header.set_mode(0b000111000000);
        header.set_cksum();
        tar.append(&header, Cursor::new(&image)).unwrap();
    }
    tar.finish().unwrap();
    mem::drop(tar);
    let mut response = Response::new(Body::from(content));
    response.headers_mut().insert("Content-Type", HeaderValue::from_str("application/x-tar").unwrap());
    response
}

fn main() {
    pretty_env_logger::init();

    let mut port: u16 = 8080;
    match env::var("PORT") {
        Ok(p) => {
            match p.parse::<u16>() {
                Ok(n) => { port = n; }
                Err(_e) => {}
            };
        }
        Err(_e) => {}
    };
    let addr = ([0, 0, 0, 0], port).into();

    let new_service = || {
        service_fn_ok(|req| {
            if req.uri().path() == "/render" {
                let mut time = None;
                for var in req.uri().path_and_query().unwrap().query().unwrap().split("&") {
                    if let Some((key, value)) = var.split_once("=") {
                        if key == "time" {
                            time = Some(value.parse().unwrap());
                        }
                    }
                }
                render(time.unwrap())
            } else {
                Response::new(Body::from(format!("hello")))
            }
        })
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
