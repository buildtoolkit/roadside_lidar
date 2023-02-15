use serde::{Serialize, Deserialize};

// A spherical coordinate point in degrees, minutes, seconds.
#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq)]
pub struct DMS {
    pub degrees : i32,
    pub minutes : i32,
    pub seconds : i32,
}

impl DMS {
    pub fn new(d : i32, m : i32, s : i32) -> Self {
        Self {
            degrees: d,
            minutes : m,
            seconds : s,
        }
    }
}

// A point on the surface of the earth, specified using degrees, minutes, seconds.
// For west and south, make all terms negative.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct DmsGpsPoint {
    pub latitude : DMS,
    pub longitude : DMS,
}

impl DmsGpsPoint {
    pub fn new(lat : DMS, lon : DMS) -> Self {
        Self {
            latitude : lat,
            longitude : lon,
        }
    }
}

// conversion from decimal gps point to DMS. We don't currently need this. The GPS coordinates at the intersection 
// are currently specified in DMS, and we need to convert to decimal to construct and ENU frame for the frame
// optimization.
/*
impl From<DecimalGpsPoint> for DmsGpsPoint {
    fn from(ll_pt : DecimalGpsPoint) -> Self {
        todo!();
    }
}
*/

// A point on the surface of the earth, specified using 64-bit floating point numbers.
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct DecimalGpsPoint {
    pub latitude : f64,
    pub longitude : f64,
}

impl DecimalGpsPoint {
    pub fn new() -> Self {
        Self {
            latitude : 0.0,
            longitude : 0.0,
        }
    }
}

// Conversion from DMS to decimal.
impl From<DmsGpsPoint> for DecimalGpsPoint {
    fn from(dms_pt : DmsGpsPoint) -> Self {
        let lat_dms = dms_pt.latitude;
        let lon_dms = dms_pt.longitude;

        let lat_decimal = (lat_dms.degrees as f64) + (lat_dms.minutes as f64)/60.0 + (lat_dms.seconds as f64)/3600.0;
        let lon_decimal = (lon_dms.degrees as f64) + (lon_dms.minutes as f64)/60.0 + (lon_dms.seconds as f64)/3600.0;

        Self {
            latitude : lat_decimal,
            longitude : lon_decimal,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dms_to_decimal() {
        // point taken from Wikipedia.
        let dms_pt = DmsGpsPoint::new(
            DMS::new(38, 53, 23),
            DMS::new(-77, 0, -32),
        );

        let ll_pt = DecimalGpsPoint::from(dms_pt);
        assert!((ll_pt.latitude - 38.8897).abs() < 1e-3);
        assert!((ll_pt.longitude - -77.0089) < 1e-3);
    }
}
