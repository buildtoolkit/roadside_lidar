use std::net::Ipv4Addr;
use serde::{Serialize, Deserialize};

use crate::geography::*;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum LidarType {
    VelodyneVLP16,
    VelodyneVLP32C,
    VelodyneVLS128,
    Ouster64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LidarSensorConfig {
    pub lidar_type : LidarType,
    pub gps : DecimalGpsPoint,
    pub address : std::net::Ipv4Addr,
    pub port : u16,
    pub frame_id : String,
    pub namespace : String,
}

impl LidarSensorConfig {

    pub fn new() -> Self {
        Self {
            lidar_type : LidarType::VelodyneVLP16,
            gps : DecimalGpsPoint::new(),
            address : Ipv4Addr::new(0,0,0,0),
            port : 0,
            frame_id : String::from("velodyne"),
            namespace : String::from("")
        }
    }
}


#[derive(Debug)]
pub struct LidarSensor {
    pub id : u16,
    pub address : Ipv4Addr,
    pub port : u16,
    pub frame_id : String,
    pub namespace : String,
}

impl LidarSensor {
    pub fn builder() -> LidarSensorBuilder {
        LidarSensorBuilder::new()
    }
}

pub struct LidarSensorBuilder {
    pub id : u16,
    pub address : Ipv4Addr,
    pub port : u16,
    pub frame_id : String,
    pub namespace : String,
}

impl LidarSensorBuilder {
    pub fn new() -> Self {
        Self {
            id : 0,
            address : Ipv4Addr::new(0,0,0,0),
            port : 2368,
            frame_id : String::from(""),
            namespace : String::from(""),
        }
    }

    pub fn id(mut self, id : u16) -> Self {
        self.id = id;
        self
    }

    pub fn address(mut self, ip_address : Ipv4Addr) -> Self {
        self.address = ip_address;
        self
    }

    pub fn port(mut self, port : u16) -> Self {
        self.port = port;
        self
    }

    pub fn frame_id(mut self, frame : String) -> Self {
        self.frame_id = frame;
        self
    }

    pub fn namespace (mut self, namespace : String) -> Self {
        self.namespace = namespace;
        self
    }

    pub fn build(self) -> LidarSensor {
        LidarSensor {
            id : self.id,
            address : self.address,
            port : self.port,
            frame_id : self.frame_id,
            namespace : self.namespace,
        }
    }

}

