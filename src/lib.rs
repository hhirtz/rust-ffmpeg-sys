#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::approx_constant)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::useless_transmute)]
#![allow(unpredictable_function_pointer_comparisons)]
#![allow(unnecessary_transmutes)]

extern crate ash;
extern crate libc;

use ash::vk::{
    AccessFlags as VkAccessFlagBits, Device as VkDevice, DeviceMemory as VkDeviceMemory,
    Format as VkFormat, Image as VkImage, ImageCreateFlags as VkImageCreateFlags,
    ImageLayout as VkImageLayout, ImageTiling as VkImageTiling,
    ImageUsageFlags as VkImageUsageFlagBits, Instance as VkInstance,
    MemoryPropertyFlags as VkMemoryPropertyFlagBits, PFN_vkGetInstanceProcAddr,
    PhysicalDevice as VkPhysicalDevice, QueueFlags as VkQueueFlagBits, Semaphore as VkSemaphore,
    VideoCodecOperationFlagsKHR as VkVideoCodecOperationFlagBitsKHR,
};
type VkAllocationCallbacks = ash::vk::AllocationCallbacks<'static>;
type VkPhysicalDeviceFeatures2 = ash::vk::PhysicalDeviceFeatures2<'static>;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_use]
mod avutil;
pub use avutil::*;
