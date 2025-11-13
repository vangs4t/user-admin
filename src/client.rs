pub struct Client{
    pub username: String,
    pub password: String,
    pub pin: u32,
}

impl Client{
    pub fn login(){
        println!("Selamat datang di rusting");
    }
}
