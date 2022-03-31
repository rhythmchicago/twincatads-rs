#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// pull in the bindings that were generated into the target build directory.
// this will be something like target/build/bind-test@#$%^%^##/out/bindings.rs
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl AmsNetId_ {
    pub fn new() -> Self {
        let addr :  [::std::os::raw::c_uchar; 6usize] =  [0,0,0,0,0,0];
        Self { b :addr }
    }
}

impl AmsAddr {
    pub fn new() -> Self {
        Self{  netId : AmsNetId_::new(), port:851}
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::{time};

    
    #[test]
    fn hello_ads() {
        println!("Hello, ADS!");
    
        unsafe {
            let val = AdsGetDllVersion();
    
            println!("The ADS Dll version is {}", val);
    
            let client_port = AdsPortOpen();
    
            println!("The ADS client port is {}", client_port);
    
    
            let mut local_addr = AmsAddr::new();
    
            AdsGetLocalAddress(&mut local_addr);
    
            println!("local addr is {}.{}.{}.{}.{}.{}", 
                local_addr.netId.b[0], 
                local_addr.netId.b[1], 
                local_addr.netId.b[2], 
                local_addr.netId.b[3], 
                local_addr.netId.b[4], 
                local_addr.netId.b[5]
            );
    
            std::thread::sleep(time::Duration::from_millis(1000));
    
    
            AdsPortClose();
    
            println!("Goodbye...");
    
        }
    }
    
}
