mod satellite_server;
use rocket::State;
use satellite_server::LidarSatelliteServer;

use std::sync::Mutex;

#[macro_use] extern crate rocket;

#[get("/num_lidars")]
fn num_lidars(satellite : &State<Mutex<LidarSatelliteServer>>) -> String {
    let server = satellite.lock().expect("Lock mutex");
    let server = server.lidars.len();
    format!("{}", server)
}

#[post("/run_lidar/<lidar_id>")]
fn run_lidar(lidar_id : u16, satellite : &State<Mutex<LidarSatelliteServer>>) {
    let mut sat = satellite.lock().expect("Lock mutex");
    sat.start_lidar(lidar_id);
}

#[post("/run_all_lidars")]
fn run_all_lidars(satellite : &State<Mutex<LidarSatelliteServer>>) {
    let mut sat = satellite.lock().expect("Lock mutex");
    sat.start_all_lidars()
}

#[post("/stop_lidar/<lidar_id>")]
fn stop_lidar(lidar_id : u16, satellite: &State<Mutex<LidarSatelliteServer>>) {
    let mut sat = satellite.lock().expect("Lock mutex");
    sat.stop_lidar(lidar_id);
}

#[post("/start_recording")]
fn start_recording(satellite: &State<Mutex<LidarSatelliteServer>>) {
    let mut sat = satellite.lock().expect("Lock mutex");
    sat.start_record();
}

#[post("/stop_recording")]
fn stop_recording(satellite : &State<Mutex<LidarSatelliteServer>>) {
    let mut sat = satellite.lock().expect("Lock mutex");
    sat.stop_record()
}

#[launch]
fn rocket() -> _ {
    println!("Creating satellite server instance.");
    let mut server = satellite_server::LidarSatelliteServer::new();

    println!("Loading YAML file.");
    server.load_config("./crates/lidar_satellite/config/satellite.yaml");

    println!("Running satellite server.");
    rocket::build()
        .manage(Mutex::new(server))
        .mount("/", routes![num_lidars, run_lidar, run_all_lidars, stop_lidar, start_recording, stop_recording])
}