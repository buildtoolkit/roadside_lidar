# Roadside Lidar Control System

This repository defines a set of packages for remotely controlling a
collection of networked lidar sensors. There are three components:

- *The lidar controller* is responsible for sending commands to a set
  of edge computers, each controlling one or more lidar sensors.
- *The lidar satellite* is a server that runs on an edge computer
  connected to one or more lidar sensors.
- *Common code* related to lidar sensor representation and
  calculations on geographical data.

Example configurations for a small deployment (7 satellite servers, 13
lidars) can be found in the `configs/` directory.

## Prerequisites

The machine running the satellite server should have a working ROS
installation.


## Running

Make sure that your configurations are all in order: you like the
values in `Rocket.toml` for the satellite server, and `satellite.yaml`
in `lidar_satellite/config` has correct IP addresses and ports
specified. Then you can `cargo run` the satellite server, which will
present a REST-like interface with the following endpoints:

- `/num_lidars` - the number of lidars being managed by the server.
- `/run_lidar/<lidar_id>` - start the lidar with given id. Runs
  `roslaunch` for the `velodyne_pointcloud` package.
- `/run_all_lidars` - start all available lidars.
- `/stop_lidar/<lidar_id>` - stop the lidar with given id. But see
  TODO items below.
- `/start_recording` - run `rosbag record` with all of the lidar
  topics that have been started on the satellite.
- `/stop_recording` - run `rosnode kill` to stop an existing
  recording.

## TODO

A wishlist of sorts:

- Graceful shutdown of lidars - this requires tracking more processes
  than we do at the moment. Currently all launches are killed when the
  server is killed.
- Mutual TLS for client authentication.
- Easy movement of recorded bagfiles from satellites to the controller.
- Automated or semiautomated calibration of mutually-visible sensors.

