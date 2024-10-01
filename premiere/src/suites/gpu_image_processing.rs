use crate::*;

define_suite!(
    /// Access to common GPU image processing algorithms
    GPUImageProcessingSuite,
    PrSDKGPUImageProcessingSuite,
    kPrSDKGPUImageProcessingSuite,
    kPrSDKGPUImageProcessingSuiteVersion
);

impl GPUImageProcessingSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Convert between formats on the GPU
    /// One of src_format or dst_format must be a host format,
    /// the other must be either [`PixelFormat::GpuBgra4444_16f`] or [`PixelFormat::GpuBgra4444_32f`]
    pub fn pixel_format_convert(&self,
        device_index: u32,
        src: *const std::ffi::c_void, src_stride: i32, src_format: PixelFormat,
        dst: *mut   std::ffi::c_void, dst_stride: i32, dst_format: PixelFormat,
        width: u32,
        height: u32,
        quality: RenderQuality
    ) -> Result<(), Error> {
        call_suite_fn!(self, PixelFormatConvert, device_index, src, src_stride, src_format.into(), dst, dst_stride, dst_format.into(), width, height, quality.into())
    }

    /// Scale a frame on the GPU
    /// `format` must be [`PixelFormat::GpuBgra4444_16f`] or [`PixelFormat::GpuBgra4444_32f`]
    pub fn scale(&self,
        device_index: u32,
        src: *const std::ffi::c_void, src_stride: i32, src_width: u32, src_height: u32,
        dst: *mut   std::ffi::c_void, dst_stride: i32, dst_width: u32, dst_height: u32,
        format: PixelFormat,
        scale_x: f32,
        scale_y: f32,
        quality: RenderQuality
    ) -> Result<(), Error> {
        call_suite_fn!(self, Scale, device_index, src, src_stride, src_width, src_height, dst, dst_stride, dst_width, dst_height, format.into(), scale_x, scale_y, quality.into())
    }

    /// Gaussian blur on the GPU
    /// `format` must be [`PixelFormat::GpuBgra4444_16f`] or [`PixelFormat::GpuBgra4444_32f`]
    pub fn gaussian_blur(&self,
        device_index: u32,
        src: *const std::ffi::c_void, src_stride: i32, src_width: u32, src_height: u32,
        dst: *mut   std::ffi::c_void, dst_stride: i32, dst_width: u32, dst_height: u32,
        format: PixelFormat,
        sigma_x: f32,
        sigma_y: f32,
        repeat_edge_pixels: bool,
        blur_horizontally: bool,
        blur_vertically: bool,
        quality: RenderQuality
    ) -> Result<(), Error> {
        call_suite_fn!(self, GaussianBlur, device_index, src, src_stride, src_width, src_height, dst, dst_stride, dst_width, dst_height, format.into(), sigma_x, sigma_y, if repeat_edge_pixels { 1 } else { 0 }, if blur_horizontally { 1 } else { 0 }, if blur_vertically { 1 } else { 0 }, quality.into())
    }
}
