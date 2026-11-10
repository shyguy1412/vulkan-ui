use std::sync::Arc;

use crate::*;

use jafw::{Error::OtherError, Event::XEvent, Result, Window, open_window};
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::allocator::{
        StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
    },
    device::{Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo, QueueFlags},
    image::ImageUsage,
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    swapchain::{Swapchain, SwapchainCreateInfo},
};

#[test]
fn main() -> Result<()> {
    let window = open_window()?;
    window.title(&"Hello World")?;

    let lib = vulkano::VulkanLibrary::new().expect("Vulkan lib must be available");
    let mut create_info = vulkano::instance::InstanceCreateInfo::application_from_cargo_toml();
    let extensions = vulkano::swapchain::Surface::required_extensions(&window)
        .expect("Must be able to get required extensions");

    create_info.enabled_extensions = extensions;

    let instance = vulkano::instance::Instance::new(lib, create_info)
        .expect("Must be able to instance vulkan");

    let physical_device = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .next()
        .expect("no devices available");

    let surface = vulkano::swapchain::Surface::from_window(instance, window.clone())
        .ok()
        .ok_or(OtherError)?;

    dbg!(&surface);
    dbg!(&physical_device.properties().device_name);

    for family in physical_device.queue_family_properties() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queue_count,
        );
    }

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .position(|queue_family_properties| {
            queue_family_properties
                .queue_flags
                .contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;

    let (device, mut queues) = Device::new(
        physical_device.clone(),
        DeviceCreateInfo {
            // here we pass the desired queue family to use by index
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            enabled_extensions: DeviceExtensions {
                khr_swapchain: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .expect("failed to create device");

    let queue = queues.next().unwrap();

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let data: i32 = 12;
    //can contain arbitrary data
    let buffer = Buffer::from_data(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::UNIFORM_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        data,
    )
    .expect("failed to create buffer");

    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );

    let caps = physical_device
        .surface_capabilities(&surface, Default::default())
        .expect("failed to get surface capabilities");

    let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
    let image_format = physical_device
        .surface_formats(&surface, Default::default())
        .unwrap()[0]
        .0;

    let (mut swapchain, images) = Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: caps.min_image_count + 1, // How many buffers to use in the swapchain
            image_format,
            image_extent: [150, 150],
            image_usage: ImageUsage::COLOR_ATTACHMENT, // What the images are going to be used for
            composite_alpha,
            ..Default::default()
        },
    )
    .unwrap();

    while let Some(XEvent(event)) = window.next_event() {
        // dbg!(event);
    }

    Ok(())
}
