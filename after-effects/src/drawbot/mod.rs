use crate::*;

register_handle!(DRAWBOT_DrawRef);
register_handle!(DRAWBOT_SupplierRef);
register_handle!(DRAWBOT_SurfaceRef);
define_handle_wrapper!(SupplierRef, DRAWBOT_SupplierRef);
define_handle_wrapper!(SurfaceRef, DRAWBOT_SurfaceRef);

pub mod suites {
    pub(crate) mod supplier; pub use supplier::SupplierSuite as Supplier;
    pub(crate) mod surface;  pub use surface::SurfaceSuite as Surface;
}

pub use suites::supplier::{
    PixelLayout,
    Supplier,
};
pub use suites::surface::{
    FillType,
    TextAlignment,
    TextTruncation,
    InterpolationPolicy,
    AntiAliasPolicy,
    Surface,
};

pub type PointF32  = ae_sys::DRAWBOT_PointF32;
pub type ColorRgba = ae_sys::DRAWBOT_ColorRGBA;
pub type RectF32   = ae_sys::DRAWBOT_RectF32;
pub type MatrixF32 = ae_sys::DRAWBOT_MatrixF32;
pub type Rect32    = ae_sys::DRAWBOT_Rect32;

// ――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

define_suite!(
    DrawbotSuite,
    DRAWBOT_DrawbotSuite1,
    kDRAWBOT_DrawSuite,
    kDRAWBOT_DrawSuite_Version1
);

pub struct Drawbot {
    pub(crate) handle: ae_sys::DRAWBOT_DrawRef,
    pub(crate) suite: DrawbotSuite,
}
impl Drawbot {
    /// Get the supplier reference.
    pub fn supplier(&self) -> Result<Supplier, Error> {
        Ok(Supplier::from_raw(
            call_suite_fn_single!(self.suite, GetSupplier -> ae_sys::DRAWBOT_SupplierRef, self.handle)?
        ))
    }

    /// Get the surface reference.
    pub fn surface(&self) -> Result<Surface, Error> {
        Ok(Surface::from_raw(
            call_suite_fn_single!(self.suite, GetSurface -> ae_sys::DRAWBOT_SurfaceRef, self.handle)?
        ))
    }

    /// Fills the path with overlay theme foreground color.
    ///
    /// Optionally draw the shadow using the overlay theme shadow color.
    pub fn fill_theme_path(&self, path: impl AsPtr<ae_sys::DRAWBOT_PathRef>, draw_shadow: bool) -> Result<(), Error> {
        let suite = pf::suites::EffectCustomUIOverlayTheme::new()?;
        suite.fill_path(self.handle, path, draw_shadow)
    }

    /// Fills a square vertex around the center point using the overlay theme foreground color and vertex size.
    pub fn fill_theme_vertex(&self, center_point: FloatPoint, draw_shadow: bool) -> Result<(), Error> {
        let suite = pf::suites::EffectCustomUIOverlayTheme::new()?;
        suite.fill_vertex(self.handle, center_point, draw_shadow)
    }
}
impl AsPtr<ae_sys::DRAWBOT_DrawRef> for Drawbot {
    fn as_ptr(&self) -> ae_sys::DRAWBOT_DrawRef {
        self.handle
    }
}
impl AsPtr<ae_sys::DRAWBOT_DrawRef> for &Drawbot {
    fn as_ptr(&self) -> ae_sys::DRAWBOT_DrawRef {
        self.handle
    }
}

// ――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

define_suite!(
    PenSuite,
    DRAWBOT_PenSuite1,
    kDRAWBOT_PenSuite,
    kDRAWBOT_PenSuite_Version1
);
pub struct Pen {
    pub(crate) handle: ae_sys::DRAWBOT_PenRef,
    pub(crate) suite: PenSuite,
    pub(crate) supplier_suite: suites::Supplier,
}
impl Pen {
    /// Set pen dash pattern.
    pub fn set_dash_pattern(&mut self, dashes: Vec<f32>) -> Result<(), Error> {
        call_suite_fn!(self.suite, SetDashPattern, self.handle, dashes.as_ptr(), dashes.len() as _)
    }
}
impl Drop for Pen {
    fn drop(&mut self) {
        self.supplier_suite.release_object(self.handle as *mut _).unwrap();
    }
}

// ――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

pub struct Brush {
    pub(crate) handle: ae_sys::DRAWBOT_BrushRef,
    pub(crate) supplier_suite: suites::Supplier,
}
impl Drop for Brush {
    fn drop(&mut self) {
        self.supplier_suite.release_object(self.handle as *mut _).unwrap();
    }
}

// ――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

pub struct Font {
    pub(crate) handle: ae_sys::DRAWBOT_FontRef,
    pub(crate) supplier_suite: suites::Supplier,
}
impl Drop for Font {
    fn drop(&mut self) {
        self.supplier_suite.release_object(self.handle as *mut _).unwrap();
    }
}

// ――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

define_suite!(
    ImageSuite,
    DRAWBOT_ImageSuite1,
    kDRAWBOT_ImageSuite,
    kDRAWBOT_ImageSuite_Version1
);
pub struct Image {
    pub(crate) handle: ae_sys::DRAWBOT_ImageRef,
    pub(crate) suite: ImageSuite,
    pub(crate) supplier_suite: suites::Supplier,
}
impl Image {
    /// Set image scale factor.
    pub fn set_scale_factor(&self, scale_factor: f32) -> Result<(), Error> {
        call_suite_fn!(self.suite, SetScaleFactor, self.handle, scale_factor)
    }
}
impl Drop for Image {
    fn drop(&mut self) {
        self.supplier_suite.release_object(self.handle as *mut _).unwrap();
    }
}

// ――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

define_suite!(
    /// Calls to draw paths.
    PathSuite,
    DRAWBOT_PathSuite1,
    kDRAWBOT_PathSuite,
    kDRAWBOT_PathSuite_Version1
);
pub struct Path {
    pub(crate) handle: ae_sys::DRAWBOT_PathRef,
    pub(crate) suite: PathSuite,
    pub(crate) supplier_suite: suites::Supplier,
}
impl Path {
    /// Move to a point.
    pub fn move_to(&mut self, x: f32, y: f32) -> Result<(), Error> {
        call_suite_fn!(self.suite, MoveTo, self.handle, x, y)
    }

    /// Add a line to the path.
    pub fn line_to(&mut self, x: f32, y: f32) -> Result<(), Error> {
        call_suite_fn!(self.suite, LineTo, self.handle, x, y)
    }

    /// Add a cubic bezier to the path.
    pub fn bezier_to(&mut self, pt1: &PointF32, pt2: &PointF32, pt3: &PointF32) -> Result<(), Error> {
        call_suite_fn!(self.suite, BezierTo, self.handle, pt1, pt2, pt3)
    }

    /// Add a rect to the path.
    pub fn add_rect(&mut self, rect: &RectF32) -> Result<(), Error> {
        call_suite_fn!(self.suite, AddRect, self.handle, rect)
    }

    /// Add a arc to the path. Zero start degrees == 3 o'clock. Sweep is clockwise. Units for angle are in degrees.
    pub fn add_arc(&mut self, center: &PointF32, radius: f32, start_angle: f32, sweep: f32) -> Result<(), Error> {
        call_suite_fn!(self.suite, AddArc, self.handle, center, radius, start_angle, sweep)
    }

    /// Add a rounded rect to the path.
    /// * `rect` - specifies the bounds of the rectangle.
    /// * `radius` - specifies the radius of the rounding circle at each corner.
    pub fn add_rounded_rect(&mut self, rect: &RectF32, radius: f32) -> Result<(), Error> {
        let (x, y, w, h) = (rect.left, rect.top, rect.width, rect.height);
        let r = radius.min(w / 2.0).min(h / 2.0);

        self.move_to(x + r, y)?;
        self.line_to(x + w - r, y)?;
        self.add_arc(&PointF32 { x: x + w - r, y: y + r }, r, -90.0, 90.0)?;
        self.line_to(x + w, y + h - r)?;
        self.add_arc(&PointF32 { x: x + w - r, y: y + h - r }, r, 0.0, 90.0)?;
        self.line_to(x + r, y + h)?;
        self.add_arc(&PointF32 { x: x + r, y: y + h - r }, r, 90.0, 90.0)?;
        self.line_to(x, y + r)?;
        self.add_arc(&PointF32 { x: x + r, y: y + r }, r, 180.0, 90.0)?;

        self.close()
    }

    /// Close the path.
    pub fn close(&mut self) -> Result<(), Error> {
        call_suite_fn!(self.suite, Close, self.handle)
    }
}
impl AsPtr<ae_sys::DRAWBOT_PathRef> for Path {
    fn as_ptr(&self) -> ae_sys::DRAWBOT_PathRef {
        self.handle
    }
}
impl AsPtr<ae_sys::DRAWBOT_PathRef> for &Path {
    fn as_ptr(&self) -> ae_sys::DRAWBOT_PathRef {
        self.handle
    }
}
impl Drop for Path {
    fn drop(&mut self) {
        self.supplier_suite.release_object(self.handle as *mut _).unwrap();
    }
}

// ――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
