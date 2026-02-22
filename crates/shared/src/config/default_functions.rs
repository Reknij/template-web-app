pub fn default_server_host() -> String {
    "0.0.0.0".to_string()
}

pub fn default_server_port() -> u16 {
    8888
}

pub fn default_security_auth_key() -> String {
    "unsafe-default-auth-key".to_string()
}

pub fn default_db_url() -> String {
    "sqlite:./data.sqlite".to_string() 
}