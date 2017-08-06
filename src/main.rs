// TODO get rid of these, eventually.
#![allow(unused_variables)]  
#![allow(unused_imports)]    
#![allow(dead_code)] 

#[macro_use]
extern crate vulkano;

use std::sync::Arc;

use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::ApplicationInfo;
use vulkano::instance::PhysicalDevice;


fn main() {
    let info = &ApplicationInfo::from_cargo_toml();
    let instance = Instance::new(Some(info), &InstanceExtensions::none(), None)
        .expect("failed to create instance");


    
    let devices = PhysicalDevice::enumerate(&instance);

    let mut found = Vec::new();

    for dev in devices {
        if dev.name().eq("GTX 1060") {
            println!("found device {:?}", dev.name());
            found.push(dev.name());
        }
    }
}

fn select_device <'a>(instance: &'a Arc<Instance>) -> Option<PhysicalDevice<'a>> {
    // preferred ranks our choices of supported devices. 
    let preferred = vec!["GTX 1060"];
    let devices = PhysicalDevice::enumerate(&instance);

    let mut found = Vec::new();

    for dev in devices {
        if dev.name().eq("GTX 1060") {
            println!("found device {:?}", dev.name());
            found.push(dev.name());
        }
    }

    for p in preferred {
        for f in found {
            if p == f {
                let i = found.into_iter().position(|t| t == f ).unwrap();
                return PhysicalDevice::from_index(&instance, i)
            }
        }
    }


    // for dev in devices {
    //     if dev.name().eq("GTX 1060") {
    //         println!("found device {:?}", dev.name());
    //         found.push(dev.name());
    //     }
    // }
    None
}