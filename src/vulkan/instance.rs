use std::{
    ffi::{c_char, c_void, CStr},
    sync::Arc,
};

use ash::{ext::debug_utils, khr::surface, prelude::VkResult, vk, Entry};
use log::{log, warn, Level};

use crate::{vulkan::{VulkanError, VulkanPhysicalDevice}, Error, InstanceDesc, InstanceFlags, PhysicalDevice, Device};
use crate::vulkan::VulkanDevice;

pub struct InstanceLayers {
    supported: Vec<vk::LayerProperties>,
    enabled: Vec<*const c_char>,

    khronos_validation: bool,
}

impl InstanceLayers {
    unsafe fn new(entry: &Entry) -> VkResult<Self> {
        let supported = entry.enumerate_instance_layer_properties()?;

        Ok(Self {
            supported,
            enabled: Vec::new(),

            khronos_validation: false,
        })
    }

    #[inline]
    unsafe fn push(&mut self, name: *const c_char) -> Result<(), VulkanError> {
        if self
            .supported
            .iter()
            .any(|e| libc::strcmp(e.layer_name.as_ptr(), name) == 0)
        {
            self.enabled.push(name);
            Ok(())
        } else {
            Err(VulkanError::Custom(format!(
                "Failed to push layer: {}",
                CStr::from_ptr(name).to_str()?
            )))
        }
    }

    fn push_khronos_validation(&mut self) -> Result<(), VulkanError> {
        let result = unsafe { self.push(b"VK_LAYER_KHRONOS_validation\0".as_ptr().cast()) };

        if result.is_ok() {
            self.khronos_validation = true;
        }

        result
    }

    #[inline]
    pub fn khronos_validation(&self) -> bool {
        self.khronos_validation
    }
}

unsafe impl Send for InstanceLayers {}
unsafe impl Sync for InstanceLayers {}

pub struct InstanceExtensions {
    supported: Vec<vk::ExtensionProperties>,
    supported_khronos_validation: Vec<vk::ExtensionProperties>,
    enabled: Vec<*const c_char>,

    ext_debug_utils: bool,
    ext_validation_features: bool,
    khr_portability_enumeration: bool,
    khr_surface: bool,
}

impl InstanceExtensions {
    unsafe fn new(entry: &Entry, layers: &InstanceLayers) -> VkResult<Self> {
        let supported = entry.enumerate_instance_extension_properties(None)?;

        let supported_khronos_validation = if layers.khronos_validation() {
            entry.enumerate_instance_extension_properties(Some(
                CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0"),
            ))?
        } else {
            Vec::new()
        };

        Ok(Self {
            supported,
            supported_khronos_validation,
            enabled: Vec::new(),

            ext_debug_utils: false,
            ext_validation_features: false,
            khr_portability_enumeration: false,
            khr_surface: false,
        })
    }

    #[inline]
    unsafe fn push(&mut self, name: *const c_char) -> Result<(), VulkanError> {
        if self
            .supported
            .iter()
            .any(|e| libc::strcmp(e.extension_name.as_ptr(), name) == 0)
            || self
                .supported_khronos_validation
                .iter()
                .any(|e| libc::strcmp(e.extension_name.as_ptr(), name) == 0)
        {
            self.enabled.push(name);
            Ok(())
        } else {
            Err(VulkanError::Custom(format!(
                "Failed to push extension: {}",
                CStr::from_ptr(name).to_str()?
            )))
        }
    }

    #[inline]
    pub fn push_ext_debug_utils(&mut self) -> Result<(), VulkanError> {
        let result = unsafe { self.push(debug_utils::NAME.as_ptr()) };

        if result.is_ok() {
            self.ext_debug_utils = true;
        }

        result
    }

    #[inline]
    pub fn push_khr_surface(&mut self) -> Result<(), VulkanError> {
        let result = unsafe { self.push(surface::NAME.as_ptr()) };

        if result.is_ok() {
            self.khr_surface = true;
        }

        result
    }
}

unsafe impl Send for InstanceExtensions {}
unsafe impl Sync for InstanceExtensions {}

struct Inner {
    entry: Entry,
    instance: ash::Instance,

    debug_utils_instance: debug_utils::Instance,
    surface_instance: surface::Instance,

    debug_utils_messenger: vk::DebugUtilsMessengerEXT,

    physical_devices: Vec<PhysicalDevice>,
}

#[derive(Clone)]
pub struct VulkanInstance(Arc<Inner>);

impl VulkanInstance {
    pub unsafe fn new(desc: &InstanceDesc) -> Result<Self, VulkanError> {
        let entry = Entry::load().unwrap();

        let mut layers = InstanceLayers::new(&entry)?;
        let mut extensions = InstanceExtensions::new(&entry, &layers)?;

        let application_info = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_3);

        let validation_enabled =
            (desc.flags & InstanceFlags::ENABLE_VALIDATION) == InstanceFlags::ENABLE_VALIDATION;

        if validation_enabled {
            layers.push_khronos_validation()?;

            extensions.push_ext_debug_utils()?;
        }

        let enabled_validation_features = [
            vk::ValidationFeatureEnableEXT::BEST_PRACTICES,
            vk::ValidationFeatureEnableEXT::DEBUG_PRINTF,
            vk::ValidationFeatureEnableEXT::SYNCHRONIZATION_VALIDATION,
        ];
        let disabled_validation_features = [];

        let mut validation_features = vk::ValidationFeaturesEXT::default()
            .enabled_validation_features(&enabled_validation_features)
            .disabled_validation_features(&disabled_validation_features);

        let mut instance_create_info = vk::InstanceCreateInfo::default()
            .application_info(&application_info)
            .enabled_extension_names(&extensions.enabled)
            .enabled_layer_names(&layers.enabled);

        if validation_enabled {
            instance_create_info = instance_create_info.push_next(&mut validation_features);
        }

        let instance = entry.create_instance(&instance_create_info, None)?;
        let debug_utils_instance = debug_utils::Instance::new(&entry, &instance);
        let surface_instance = surface::Instance::new(&entry, &instance);

        let debug_utils_messenger = if extensions.ext_debug_utils {
            let debug_utils_messenger_create_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
                .message_severity(
                    vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                        | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                        | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
                )
                .message_type(
                    vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                )
                .pfn_user_callback(Some(debug_callback));

            debug_utils_instance
                .create_debug_utils_messenger(&debug_utils_messenger_create_info, None)?
        } else {
            vk::DebugUtilsMessengerEXT::null()
        };

        let physical_devices = instance
            .enumerate_physical_devices()?
            .into_iter()
            .map(|physical_device| {
                PhysicalDevice::Vulkan(VulkanPhysicalDevice::new(&instance, physical_device))
            })
            .collect::<Vec<_>>();

        Ok(Self(Arc::new(Inner {
            entry,
            instance,

            debug_utils_instance,
            surface_instance,

            debug_utils_messenger,

            physical_devices,
        })))
    }

    #[inline]
    pub fn instance(&self) -> &ash::Instance {
        &self.0.instance
    }

    #[inline]
    pub fn get_physical_devices(&self) -> &[PhysicalDevice] {
        &self.0.physical_devices
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            if self.debug_utils_messenger != vk::DebugUtilsMessengerEXT::null() {
                self.debug_utils_instance
                    .destroy_debug_utils_messenger(self.debug_utils_messenger, None);
            }

            self.instance.destroy_instance(None);
        }
    }
}

unsafe extern "system" fn debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_types: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut c_void,
) -> vk::Bool32 {
    log!(
        match message_severity {
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => Level::Trace,
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO => Level::Info,
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => Level::Warn,
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => Level::Error,
            _ => {
                warn!(
                    "Unknown {}: {}",
                    "vk::DebugUtilsMessageSeverityFlagsEXT",
                    message_severity.as_raw()
                );
                Level::Warn
            }
        },
        "[{:?}] {}",
        message_types,
        CStr::from_ptr((*callback_data).p_message).to_str().unwrap()
    );

    vk::FALSE
}
