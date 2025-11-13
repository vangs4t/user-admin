use crate::client;
// use crate untuk penggunaan modul selain di main
pub struct Admin{
    pub username: String,
    pub password: String,
    pub pin: u32,
}

impl Admin{
    pub fn say_admin(){
        println!("Selamat datang admin");
        client::Client::login();
    }
}