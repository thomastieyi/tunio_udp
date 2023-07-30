use tunio::{DefaultTokioInterface, DefaultDriver, traits::{DriverT, InterfaceT}};
use tokio::io::AsyncWrite; 
use tokio::io::AsyncRead; 
pub fn create_tun(name: &str, ip4: &str) -> DefaultTokioInterface {
    let mut driver = DefaultDriver::new().unwrap();
    let interface_config = DefaultDriver::if_config_builder()
        .name(name.into())
        .build()
        .unwrap();
    let interface = DefaultTokioInterface::new_up(&mut driver, interface_config).unwrap();
    let iff = interface.handle();
    
    iff.add_ip(format!("{}/24", ip4).parse().unwrap());
    interface
}
#[tokio::main]
async fn main() {
    let mut tun = create_tun("tun0", "10.24.3.1") ;
    // tun.handle().    // println!("tun name: {:#?}", tun.);
    // tun.
    loop {
        
    }
}