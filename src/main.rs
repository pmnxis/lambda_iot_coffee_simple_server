use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use std::sync::Mutex;

struct CoffeeState{
    shot_uid: Mutex<u32>,
    long_uid: Mutex<u32>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome To Lambda Coffee!")
}

#[get("/reset")]
async fn get_reset_order(data: web::Data< CoffeeState>) -> impl Responder {
    let mut shot_uid = data.shot_uid.lock().unwrap();
    let mut long_uid = data.long_uid.lock().unwrap();
    
    *shot_uid = 0;
    *long_uid = 0;

    // return response with the body "Reset Complete!"
    // Should reset when start sub-server or iot node.
    HttpResponse::Ok().body("Reset Complete!")
}

#[get("/plz_shot")]
async fn get_shot(data: web::Data< CoffeeState>) -> impl Responder {
    let mut rng = rand::thread_rng();
    let mut shot_uid = data.shot_uid.lock().unwrap();
    let prev_shot_uid = shot_uid.clone();
    
    loop {
        *shot_uid = rng.gen();
        if *shot_uid != prev_shot_uid { break; }
    }

    let contents = format!("Shot : {} -> {}", prev_shot_uid, shot_uid);

    // return response with the body "Shot:19038 -> 634522";
    HttpResponse::Ok().body(contents)
}

#[get("/plz_long")]
async fn get_long(data: web::Data< CoffeeState>) -> impl Responder {
    let mut rng = rand::thread_rng();
    let mut long_uid = data.long_uid.lock().unwrap();
    let prev_long_uid = long_uid.clone();
    
    loop {
        *long_uid = rng.gen();
        if *long_uid != prev_long_uid { break; }
    }

    let contents = format!("Long : {} -> {}", prev_long_uid, long_uid);

    // return response with the body "Long:7392 -> 23523";
    HttpResponse::Ok().body(contents)
}

#[get("/lambda-coffee")]
async fn lambda_coffee(data: web::Data<CoffeeState>) -> impl Responder {
    let shot_uid = data.shot_uid.lock().unwrap();
    let long_uid = data.long_uid.lock().unwrap();
    let contents = format!("Shot:{} , Long:{}", shot_uid, long_uid);
    
    // return response with the body "Shot:634522 , Long:23523";
    HttpResponse::Ok().body(contents)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let states = web::Data::new(CoffeeState{
        shot_uid: Mutex::new(0), long_uid: Mutex::new(0)
    });

    HttpServer::new(move || {
        App::new()
            .app_data(states.clone())
            .service(get_reset_order)
            .service(hello)
            .service(get_shot)
            .service(get_long)
            .service(lambda_coffee)
    })
    .bind("0.0.0.0:7979")?
    .run()
    .await
}

