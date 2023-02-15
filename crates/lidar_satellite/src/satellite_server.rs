use std::hash::Hash;
use std::io::Read;
use std::process::Command;
use std::net;
use std::fs::File;
use std::collections::HashMap;

use lidar_common::lidar::*;
use lidar_common::geography::*;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct LidarSatelliteServerConfig {
    hostname : String,
    address : std::net::Ipv4Addr,
    controller_address : std::net::Ipv4Addr,
    sensors : Vec<LidarSensorConfig>,
}

impl LidarSatelliteServerConfig {
    pub fn new() -> Self {
        Self {
            hostname : String::from(""),
            address : std::net::Ipv4Addr::new(0,0,0,0),
            controller_address : net::Ipv4Addr::new(0,0,0,0),
            sensors : vec![],
        }
    }
}

pub struct LidarSatelliteServer {
    config : LidarSatelliteServerConfig,
    pub lidars : HashMap<u16, LidarSensor>,
    pub child_processes : HashMap<u16, std::process::Child>,
    pub bagfile_process : Option<std::process::Child>,
}

impl LidarSatelliteServer {
    pub fn new() -> Self {
        LidarSatelliteServer { 
            config : LidarSatelliteServerConfig::new(),
            lidars : HashMap::new(),
            child_processes : HashMap::new(),
            bagfile_process : None,
        }
    }

    pub fn load_config(&mut self, path_str : &str) {
        let mut file = File::open(path_str).unwrap();
        let mut strbuf = String::new();
        file.read_to_string(&mut strbuf).unwrap();

        let config = serde_yaml::from_str::<LidarSatelliteServerConfig>(&strbuf).unwrap();

        self.config = config;

        let mut id : u16 = 0;
        for lidar in self.config.sensors.iter() {
            let lidar_sensor = 
                LidarSensor::builder()
                    .id(id)
                    .frame_id(lidar.frame_id.clone())
                    .address(lidar.address)
                    .port(lidar.port)
                    .namespace(lidar.namespace.clone())
                    .build();
            
            self.lidars.insert(id, lidar_sensor);
            id += 1;
        }
    }

    // Private helper methods
    // Execute a Command to start a lidar
    pub fn start_lidar(&mut self, lidar_id : u16) {
        if self.lidars.contains_key(&lidar_id) {
            let lidar = self.lidars.get(&lidar_id).unwrap();

            let launch_command = Command::new("roslaunch")
                .arg("velodyne_pointcloud")
                .arg("VLP16_points.launch")
                .arg(format!("device_ip:={:?}", lidar.address))
                .arg(format!("port:={}", lidar.port))
                .arg(format!("frame_id:={}", lidar.frame_id))
                .arg(format!("__ns:={}", lidar.namespace))
                .spawn()
                .expect("Failed to launch");

            self.child_processes.insert(lidar_id, launch_command);

        }
    }

    pub fn stop_lidar(&mut self, lidar_id : u16) {
        if self.lidars.contains_key(&lidar_id) {
            let mut child = self.child_processes.get_mut(&lidar_id).unwrap();
            child.kill().expect("Failed to kill.");
            child.wait();
        }
    }

    pub fn start_record(&mut self) {
        let mut lidar_topics : Vec<String> = vec![];
        for (_, lidar) in self.lidars.iter() {
            lidar_topics.push(format!("/{}/velodyne_points", lidar.namespace));
        }

        println!("{:?}", lidar_topics);

        let mut record_command = Command::new("rosbag")
                .arg("record")
                .args(lidar_topics)
                .arg("__name:=rosbag_recording")
                .spawn()
                .expect("Failed to record");

        self.bagfile_process = Some(record_command);
    }

    pub fn stop_record(&mut self) {
        if let Some(child) = &mut self.bagfile_process {
            Command::new("rosnode")
                .arg("kill")
                .arg("/rosbag_recording")
                .spawn()
                .expect("Failed to kill recorder");
        }
    }

    pub fn send_data(&self) {
        todo!();
    }


}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_config_load() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("config/satellite.yaml");

        let mut server = LidarSatelliteServer::new();
        server.load_config(d.to_str().unwrap());
    }

    #[test]
    fn test_lidars_hashmap_build() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("config/satellite.yaml");

        let mut server = LidarSatelliteServer::new();
        server.load_config(d.to_str().unwrap());

        println!("{:?}", server.lidars.get(&0));
        println!("{:?}", server.lidars.get(&1));
        
        assert_eq!(server.lidars.len(), 2);
        
    }

}