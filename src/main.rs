extern crate vulkano;

use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::ApplicationInfo;
use vulkano::instance::PhysicalDevice;


fn main() {
    let info = &ApplicationInfo::from_cargo_toml();
    let instance = Instance::new(Some(info), &InstanceExtensions::none(), None)
        .expect("failed to create instance");


    // let dev = {
    let devices = PhysicalDevice::enumerate(&instance).find()

    // let phys PhysicalDevice;

    let mut phys = Option<

    for dev in devices {
        if dev.name().eq("GTX 1060") {
            println!("found device")
        }
    }
        


    // PhysicalDevice::enumerate(&instance).map(|device| device );
    // };


}
