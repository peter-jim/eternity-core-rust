#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_endpoint: String,

    pub futures_rest_api_endpoint: String,
    pub futures_ws_endpoint: String,

    pub recv_window: u64,
}

impl Config {
    pub fn default() -> Config {
        Config {
            rest_api_endpoint: "https://api2.binance.com".into(),
            ws_endpoint: "wss://stream.binance.com:9443/ws/".into(),

            futures_rest_api_endpoint: "https://fapi.binance.com".into(),
            futures_ws_endpoint: "wss://fstream.binance.com/ws".into(),

            recv_window: 5000,
        }
    }
}