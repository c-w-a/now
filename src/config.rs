// tunable parameters

pub struct Config {
    // probe settings
    pub probe_timeout_ms: u64,
    pub probe_interval_secs: u64,

    // user location
    pub user_lat: f64,
    pub user_lon: f64,

    // latency color thresholds (absolute, in ms)
    pub green_below_ms: f64,
    pub yellow_below_ms: f64,
    // anything >= yellow_below_ms is red

    // map viewport (lon/lat bounds)
    pub map_lon_min: f64,
    pub map_lon_max: f64,
    pub map_lat_min: f64,
    pub map_lat_max: f64,
}

const WORLD_LON_MIN: f64 = -180.0;
const WORLD_LON_MAX: f64 = 180.0;
const WORLD_LAT_MIN: f64 = -60.0;
const WORLD_LAT_MAX: f64 = 75.0;

// Toronto
const USER_LAT: f64 = 43.65;
const USER_LON: f64 = -79.38;

impl Config {
    pub fn default_canada() -> Self {
        Self {
            probe_timeout_ms: 500,
            probe_interval_secs: 3,
            user_lat: USER_LAT,
            user_lon: USER_LON,
            green_below_ms: 30.0,
            yellow_below_ms: 80.0,
            map_lon_min: -145.0,
            map_lon_max: -45.0,
            map_lat_min: 38.0,
            map_lat_max: 65.0,
        }
    }

    pub fn default_root_dns() -> Self {
        Self {
            probe_timeout_ms: 2000,
            probe_interval_secs: 5,
            user_lat: USER_LAT,
            user_lon: USER_LON,
            green_below_ms: 30.0,
            yellow_below_ms: 100.0,
            map_lon_min: WORLD_LON_MIN,
            map_lon_max: WORLD_LON_MAX,
            map_lat_min: WORLD_LAT_MIN,
            map_lat_max: WORLD_LAT_MAX,
        }
    }

    pub fn default_cctld() -> Self {
        Self {
            probe_timeout_ms: 3000,
            probe_interval_secs: 5,
            user_lat: USER_LAT,
            user_lon: USER_LON,
            green_below_ms: 50.0,
            yellow_below_ms: 150.0,
            map_lon_min: WORLD_LON_MIN,
            map_lon_max: WORLD_LON_MAX,
            map_lat_min: WORLD_LAT_MIN,
            map_lat_max: WORLD_LAT_MAX,
        }
    }

    pub fn default_global_ntp() -> Self {
        Self {
            probe_timeout_ms: 2000,
            probe_interval_secs: 4,
            user_lat: USER_LAT,
            user_lon: USER_LON,
            green_below_ms: 50.0,
            yellow_below_ms: 150.0,
            map_lon_min: WORLD_LON_MIN,
            map_lon_max: WORLD_LON_MAX,
            map_lat_min: WORLD_LAT_MIN,
            map_lat_max: WORLD_LAT_MAX,
        }
    }

    pub fn default_ixp() -> Self {
        Self {
            probe_timeout_ms: 3000,
            probe_interval_secs: 5,
            user_lat: USER_LAT,
            user_lon: USER_LON,
            green_below_ms: 100.0,
            yellow_below_ms: 250.0,
            map_lon_min: WORLD_LON_MIN,
            map_lon_max: WORLD_LON_MAX,
            map_lat_min: WORLD_LAT_MIN,
            map_lat_max: WORLD_LAT_MAX,
        }
    }
}
