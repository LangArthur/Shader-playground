use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::VulkanLibrary;

fn main() {
    let library = VulkanLibrary::new().expect("No local Vulkan library/DLL");
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    )
    .expect("Failed to create Vulkan instance");

    let physical_device = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .find(|device| device.properties().driver_name == Some("NVIDIA".to_string())) // This line is specific to my machine ;)
        .expect("no devices available");

    let queue_family = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties
                .queue_flags
                .contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index: queue_family,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("Failed to create the device");

    let queue = queues.next().unwrap();

    // for family in physical_device.queue_family_properties() {
    //     println!(
    //         "Found a queue family with {:?} queue(s)",
    //         family.queue_count
    //     );
    // }

    println!("Hello, world!");
}
