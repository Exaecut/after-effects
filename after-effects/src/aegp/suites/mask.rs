use crate::*;
use ae_sys::{ A_long, AEGP_LayerH, AEGP_MaskRefH, AEGP_MaskOutlineValH };

define_suite!(
    /// Access, manipulate, and delete a layer's masks.
    MaskSuite,
    AEGP_MaskSuite6,
    kAEGPMaskSuite,
    kAEGPMaskSuiteVersion6
);

impl MaskSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Counts the masks applied to a layer.
    pub fn layer_num_masks(&self, layer: impl AsPtr<AEGP_LayerH>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerNumMasks -> A_long, layer.as_ptr())? as i32)
    }

    /// Given a layer handle and mask index, returns a pointer to the mask handle.
    pub fn layer_mask_by_index(&self, layer: impl AsPtr<AEGP_LayerH>, mask_index: i32) -> Result<MaskRefHandle, Error> {
        Ok(MaskRefHandle::from_raw_owned(
            call_suite_fn_single!(self, AEGP_GetLayerMaskByIndex -> ae_sys::AEGP_MaskRefH, layer.as_ptr(), mask_index as _)?
        ))
    }

    /// Dispose of a mask handle.
    pub fn dispose_mask(&self, mask: &mut MaskRefHandle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DisposeMask, mask.as_ptr())
    }

    /// Given a mask handle, determines if the mask is inverted or not.
    pub fn mask_invert(&self, mask: impl AsPtr<AEGP_MaskRefH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetMaskInvert -> ae_sys::A_Boolean, mask.as_ptr())? != 0)
    }

    /// Sets the inversion state of a mask.
    pub fn set_mask_invert(&self, mask: impl AsPtr<AEGP_MaskRefH>, invert: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetMaskInvert, mask.as_ptr(), invert as _)
    }

    /// Given a mask handle, returns the current mode of the mask.
    ///
    /// * [`MaskMode::None`] does nothing.
    /// * [`MaskMode::Add`] is the default behavior.
    pub fn mask_mode(&self, mask: impl AsPtr<AEGP_MaskRefH>) -> Result<MaskMode, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetMaskMode -> ae_sys::PF_MaskMode, mask.as_ptr())?.into())
    }

    /// Sets the mode of the given mask.
    pub fn set_mask_mode(&self, mask: impl AsPtr<AEGP_MaskRefH>, mode: MaskMode) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetMaskMode, mask.as_ptr(), mode.into())
    }

    /// Retrieves the motion blur setting for the given mask.
    pub fn mask_motion_blur_state(&self, mask: impl AsPtr<AEGP_MaskRefH>) -> Result<MaskMBlur, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetMaskMotionBlurState -> ae_sys::AEGP_MaskMBlur, mask.as_ptr())?.into())
    }

    /// New in CS6. Sets the motion blur setting for the given mask.
    pub fn set_mask_motion_blur_state(&self, mask: impl AsPtr<AEGP_MaskRefH>, blur_state: MaskMBlur) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetMaskMotionBlurState, mask.as_ptr(), blur_state.into())
    }

    /// New in CS6. Gets the type of feather falloff for the given mask, either
    /// [`MaskFeatherFalloff::Smooth`] or [`MaskFeatherFalloff::Linear`].
    pub fn mask_feather_falloff(&self, mask: impl AsPtr<AEGP_MaskRefH>) -> Result<MaskFeatherFalloff, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetMaskFeatherFalloff -> ae_sys::AEGP_MaskFeatherFalloff, mask.as_ptr())?.into())
    }

    /// Sets the type of feather falloff for the given mask.
    pub fn set_mask_feather_falloff(&self, mask: impl AsPtr<AEGP_MaskRefH>, falloff: MaskFeatherFalloff) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetMaskFeatherFalloff, mask.as_ptr(), falloff.into())
    }

    /// Retrieves the color of the specified mask.
    pub fn mask_color(&self, mask: impl AsPtr<AEGP_MaskRefH>) -> Result<pf::PixelF64, Error> {
        let color_val = call_suite_fn_single!(self, AEGP_GetMaskColor -> ae_sys::AEGP_ColorVal, mask.as_ptr())?;
        Ok(unsafe { std::mem::transmute(color_val) })
    }

    /// Sets the color of the specified mask.
    pub fn set_mask_color(&self, mask: impl AsPtr<AEGP_MaskRefH>, color: pf::PixelF64) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetMaskColor, mask.as_ptr(), std::mem::transmute(&color))
    }

    /// Retrieves the lock state of the specified mask.
    pub fn mask_lock_state(&self, mask: impl AsPtr<AEGP_MaskRefH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetMaskLockState -> ae_sys::A_Boolean, mask.as_ptr())? != 0)
    }

    /// Sets the lock state of the specified mask.
    pub fn set_mask_lock_state(&self, mask: impl AsPtr<AEGP_MaskRefH>, lock: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetMaskLockState, mask.as_ptr(), lock as _)
    }

    /// Returns whether or not the given mask is used as a rotobezier.
    pub fn mask_is_roto_bezier(&self, mask: impl AsPtr<AEGP_MaskRefH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetMaskIsRotoBezier -> ae_sys::A_Boolean, mask.as_ptr())? != 0)
    }

    /// Sets whether a given mask is to be used as a rotobezier.
    pub fn set_mask_is_roto_bezier(&self, mask: impl AsPtr<AEGP_MaskRefH>, is_roto_bezier: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetMaskIsRotoBezier, mask.as_ptr(), is_roto_bezier as _)
    }

    /// Duplicates a given mask.
    pub fn duplicate_mask(&self, mask: impl AsPtr<AEGP_MaskRefH>) -> Result<MaskRefHandle, Error> {
        Ok(MaskRefHandle::from_raw_owned(
            call_suite_fn_single!(self, AEGP_DuplicateMask -> ae_sys::AEGP_MaskRefH, mask.as_ptr())?
        ))
    }

    /// Creates a new mask on the referenced layer, with zero nodes. Returns new mask and its index.
    pub fn create_new_mask(&self, layer: impl AsPtr<AEGP_LayerH>) -> Result<(MaskRefHandle, i32), Error> {
        let (mask, index) = call_suite_fn_double!(self, AEGP_CreateNewMask -> ae_sys::AEGP_MaskRefH, A_long, layer.as_ptr())?;
        Ok((MaskRefHandle::from_raw_owned(mask), index))
    }

    /// NOTE: As of 6.5, if you delete a mask and it or a child stream is selected, the current selection within the composition will become NULL.
    pub fn delete_mask_from_layer(&self, mask: impl AsPtr<AEGP_MaskRefH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DeleteMaskFromLayer, mask.as_ptr())
    }

    /// Retrieves the ``AEGP_MaskIDVal`` for the given [`MaskRefHandle`], for use in uniquely identifying the mask.
    pub fn mask_id(&self, mask: impl AsPtr<AEGP_MaskRefH>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetMaskID -> ae_sys::AEGP_MaskIDVal, mask.as_ptr())? as i32)
    }
}

define_suite!(
    /// The Mask Suite above tells plug-ins about the masks on a layer, but not about the details of those masks.
    ///
    /// This is because processing is required on After Effects' part to access the information; the information isn't just lying around.
    ///
    /// Plug-ins access that information using this Mask Outline Suite.
    ///
    /// ## Mask Feathering
    /// New for CS6, masks can be feathered.
    ///
    /// `AEGP_MaskFeather` is defined as follows:
    /// ```ignore
    /// pub struct AEGP_MaskFeather {
    ///     pub segment:          A_long,     // mask segment where feather is
    ///     pub segment_sF:       PF_FpLong,  // 0-1: feather location on segment
    ///     pub radiusF:          PF_FpLong,  // negative value allowed if type == AEGP_MaskFeatherType_INNER
    ///     pub ui_corner_angleF: PF_FpShort, // 0-1: angle of UI handle on corners
    ///     pub tensionF:         PF_FpShort, // 0-1: tension of boundary at feather pt
    ///     pub interp:           AEGP_MaskFeatherInterp,
    ///     pub type_:            AEGP_MaskFeatherType,
    /// }
    /// ```
    /// `AEGP_MaskFeatherInterp` is either `AEGP_MaskFeatherInterp_NORMAL` or `AEGP_MaskFeatherInterp_HOLD_CW`.
    ///
    /// `AEGP_MaskFeatherType` is either `AEGP_MaskFeatherType_OUTER` or `AEGP_MaskFeatherType_INNER`.
    ///
    /// This suite enables AEGPs to get and set the text associated with text layers.
    ///
    /// Note: to get started, retrieve an [`aegp::TextDocumentHandle`] by calling [`suites::Stream::layer_stream_value()`](aegp::suites::Stream::layer_stream_value), above, and passing [`aegp::StreamType::TextDocument`] as the `StreamType`.
    MaskOutlineSuite,
    AEGP_MaskOutlineSuite3,
    kAEGPMaskOutlineSuite,
    kAEGPMaskOutlineSuiteVersion3
);

impl MaskOutlineSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Given an mask outline pointer (obtainable through the [`suites::Stream`](aegp::suites::Stream), determines if the mask path is open or closed.
    pub fn is_mask_outline_open(&self, mask_outline: impl AsPtr<AEGP_MaskOutlineValH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsMaskOutlineOpen -> ae_sys::A_Boolean, mask_outline.as_ptr())? != 0)
    }

    /// Sets the open state of the given mask outline.
    pub fn set_mask_outline_open(&self, mask_outline: impl AsPtr<AEGP_MaskOutlineValH>, open: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetMaskOutlineOpen, mask_outline.as_ptr(), open as _)
    }

    /// Given a mask outline pointer, returns the number of segments in the path.
    pub fn mask_outline_num_segments(&self, mask_outline: impl AsPtr<AEGP_MaskOutlineValH>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetMaskOutlineNumSegments -> A_long, mask_outline.as_ptr())? as i32)
    }

    /// Given a mask outline pointer and a point between 0 and the total number of segments.
    pub fn mask_outline_vertex_info(&self, mask_outline: impl AsPtr<AEGP_MaskOutlineValH>, point: i32) -> Result<ae_sys::AEGP_MaskVertex, Error> {
        call_suite_fn_single!(self, AEGP_GetMaskOutlineVertexInfo -> ae_sys::AEGP_MaskVertex, mask_outline.as_ptr(), point as _)
    }

    /// Sets the vertex information for a given index.
    pub fn set_mask_outline_vertex_info(&self, mask_outline: impl AsPtr<AEGP_MaskOutlineValH>, point: i32, vertex: &ae_sys::AEGP_MaskVertex) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetMaskOutlineVertexInfo, mask_outline.as_ptr(), point as _, vertex)
    }

    /// Creates a vertex at index position.
    pub fn create_vertex(&self, mask_outline: impl AsPtr<AEGP_MaskOutlineValH>, position: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_CreateVertex, mask_outline.as_ptr(), position as _)
    }

    /// Removes a vertex from a mask.
    pub fn delete_vertex(&self, mask_outline: impl AsPtr<AEGP_MaskOutlineValH>, index: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DeleteVertex, mask_outline.as_ptr(), index as _)
    }

    /// Given a mask outline pointer, returns the number of feathers in the path.
    pub fn mask_outline_num_feathers(&self, mask_outline: impl AsPtr<AEGP_MaskOutlineValH>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetMaskOutlineNumFeathers -> A_long, mask_outline.as_ptr())? as i32)
    }

    /// Given a mask outline pointer and a feather index, returns the feather information.
    pub fn mask_outline_feather_info(&self, mask_outline: impl AsPtr<AEGP_MaskOutlineValH>, feather_index: i32) -> Result<ae_sys::AEGP_MaskFeather, Error> {
        call_suite_fn_single!(self, AEGP_GetMaskOutlineFeatherInfo -> ae_sys::AEGP_MaskFeather, mask_outline.as_ptr(), feather_index as _)
    }

    /// Sets the feather information for a given index. Feather must already exist; use [`create_mask_outline_feather()`](Self::create_mask_outline_feather) first, if needed.
    pub fn set_mask_outline_feather_info(&self, mask_outline: impl AsPtr<AEGP_MaskOutlineValH>, feather_index: i32, feather: &ae_sys::AEGP_MaskFeather) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetMaskOutlineFeatherInfo, mask_outline.as_ptr(), feather_index as _, feather)
    }

    /// Creates a feather at the given index. Returns the index of new feather.
    pub fn create_mask_outline_feather(&self, mask_outline: impl AsPtr<AEGP_MaskOutlineValH>, feather: Option<ae_sys::AEGP_MaskFeather>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_CreateMaskOutlineFeather -> ae_sys::AEGP_FeatherIndex, mask_outline.as_ptr(), feather.as_ref().map_or(std::ptr::null_mut(), |t| t))? as i32)
    }

    /// Deletes a feather from the mask.
    pub fn delete_mask_outline_feather(&self, mask_outline: impl AsPtr<AEGP_MaskOutlineValH>, index: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DeleteMaskOutlineFeather, mask_outline.as_ptr(), index as _)
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

register_handle!(AEGP_MaskOutlineValH);
define_handle_wrapper!(MaskOutlineHandle, AEGP_MaskOutlineValH);

register_handle!(AEGP_MaskRefH);
define_owned_handle_wrapper!(MaskRefHandle, AEGP_MaskRefH);
impl Drop for MaskRefHandle {
    fn drop(&mut self) {
        if self.is_owned() {
            MaskSuite::new().unwrap().dispose_mask(self).unwrap();
        }
    }
}

define_enum! {
    ae_sys::PF_MaskMode,
    MaskMode {
        None       = ae_sys::PF_MaskMode_NONE,
        Add        = ae_sys::PF_MaskMode_ADD,
        Subtract   = ae_sys::PF_MaskMode_SUBTRACT,
        Intersect  = ae_sys::PF_MaskMode_INTERSECT,
        Lighten    = ae_sys::PF_MaskMode_LIGHTEN,
        Darken     = ae_sys::PF_MaskMode_DARKEN,
        Difference = ae_sys::PF_MaskMode_DIFFERENCE,
        Accum      = ae_sys::PF_MaskMode_ACCUM,
    }
}

define_enum! {
    ae_sys::AEGP_MaskMBlur,
    MaskMBlur {
        SameAsLayer = ae_sys::AEGP_MaskMBlur_SAME_AS_LAYER,
        Off         = ae_sys::AEGP_MaskMBlur_OFF,
        On          = ae_sys::AEGP_MaskMBlur_ON,
    }
}

define_enum! {
    ae_sys::AEGP_MaskFeatherFalloff,
    MaskFeatherFalloff {
        Smooth = ae_sys::AEGP_MaskFeatherFalloff_SMOOTH,
        Linear = ae_sys::AEGP_MaskFeatherFalloff_LINEAR,
    }
}
define_enum! {
    ae_sys::AEGP_MaskFeatherInterp,
    MaskFeatherInterp {
        Normal  = ae_sys::AEGP_MaskFeatherInterp_NORMAL,
        HoldCW  = ae_sys::AEGP_MaskFeatherInterp_HOLD_CW,
    }
}
define_enum! {
    ae_sys::AEGP_MaskFeatherType,
    MaskFeatherType {
        Outer = ae_sys::AEGP_MaskFeatherType_OUTER,
        Inner = ae_sys::AEGP_MaskFeatherType_INNER,
    }
}

define_suite_item_wrapper!(
    ae_sys::AEGP_MaskRefH, MaskRefHandle,
    suite: MaskSuite,
    stream: aegp::suites::Stream,
    /// Access, manipulate, and delete a layer's masks.
    Mask {
        dispose: ;

        /// Given a mask handle, determines if the mask is inverted or not.
        is_inverted() -> bool => suite.mask_invert,

        /// Sets the inversion state of a mask.
        set_inverted(invert: bool) -> () => suite.set_mask_invert,

        /// Given a mask handle, returns the current mode of the mask.
        ///
        /// * [`MaskMode::None`] does nothing.
        /// * [`MaskMode::Add`] is the default behavior.
        mode() -> MaskMode => suite.mask_mode,

        /// Sets the mode of the given mask.
        set_mode(mode: MaskMode) -> () => suite.set_mask_mode,

        /// Retrieves the motion blur setting for the given mask.
        motion_blur_state() -> MaskMBlur => suite.mask_motion_blur_state,

        /// New in CS6. Sets the motion blur setting for the given mask.
        set_motion_blur_state(blur_state: MaskMBlur) -> () => suite.set_mask_motion_blur_state,

        /// New in CS6. Gets the type of feather falloff for the given mask, either
        /// [`MaskFeatherFalloff::Smooth`] or [`MaskFeatherFalloff::Linear`].
        feather_falloff() -> MaskFeatherFalloff => suite.mask_feather_falloff,

        /// Sets the type of feather falloff for the given mask.
        set_feather_falloff(falloff: MaskFeatherFalloff) -> () => suite.set_mask_feather_falloff,

        /// Retrieves the color of the specified mask.
        color() -> pf::PixelF64 => suite.mask_color,

        /// Sets the color of the specified mask.
        set_color(color: pf::PixelF64) -> () => suite.set_mask_color,

        /// Retrieves the lock state of the specified mask.
        lock_state() -> bool => suite.mask_lock_state,

        /// Sets the lock state of the specified mask.
        set_lock_state(lock: bool) -> () => suite.set_mask_lock_state,

        /// Returns whether or not the given mask is used as a rotobezier.
        is_roto_bezier() -> bool => suite.mask_is_roto_bezier,

        /// Sets whether a given mask is to be used as a rotobezier.
        set_is_roto_bezier(is_roto_bezier: bool) -> () => suite.set_mask_is_roto_bezier,

        /// Duplicates a given mask.
        duplicate() -> Mask => suite.duplicate_mask,

        /// NOTE: As of 6.5, if you delete a mask and it or a child stream is selected, the current selection within the composition will become NULL.
        delete_from_layer() -> () => suite.delete_mask_from_layer,

        /// Retrieves the ``AEGP_MaskIDVal`` for the given [`MaskRefHandle`], for use in uniquely identifying the mask.
        id() -> i32 => suite.mask_id,

        /// Get a mask's stream.
        stream(plugin_id: aegp::PluginId, stream: aegp::MaskStream) -> aegp::Stream => stream.new_mask_stream,
    }
);

define_suite_item_wrapper!(
    ae_sys::AEGP_MaskOutlineValH, MaskOutlineHandle,
    suite: MaskOutlineSuite,
    /// The [`Mask`] type tells plug-ins about the masks on a layer, but not about the details of those masks.
    ///
    /// This is because processing is required on After Effects' part to access the information; the information isn't just lying around.
    ///
    /// Plug-ins access that information using this Mask Outline Suite.
    ///
    /// ## Mask Feathering
    /// New for CS6, masks can be feathered.
    ///
    /// `AEGP_MaskFeather` is defined as follows:
    /// ```ignore
    /// pub struct AEGP_MaskFeather {
    ///     pub segment:          A_long,     // mask segment where feather is
    ///     pub segment_sF:       PF_FpLong,  // 0-1: feather location on segment
    ///     pub radiusF:          PF_FpLong,  // negative value allowed if type == AEGP_MaskFeatherType_INNER
    ///     pub ui_corner_angleF: PF_FpShort, // 0-1: angle of UI handle on corners
    ///     pub tensionF:         PF_FpShort, // 0-1: tension of boundary at feather pt
    ///     pub interp:           AEGP_MaskFeatherInterp,
    ///     pub type_:            AEGP_MaskFeatherType,
    /// }
    /// ```
    /// `AEGP_MaskFeatherInterp` is either `AEGP_MaskFeatherInterp_NORMAL` or `AEGP_MaskFeatherInterp_HOLD_CW`.
    ///
    /// `AEGP_MaskFeatherType` is either `AEGP_MaskFeatherType_OUTER` or `AEGP_MaskFeatherType_INNER`.
    ///
    /// This suite enables AEGPs to get and set the text associated with text layers.
    ///
    /// Note: to get started, retrieve an [`aegp::TextDocumentHandle`] by calling [`suites::Stream::layer_stream_value()`](aegp::suites::Stream::layer_stream_value), above, and passing [`aegp::StreamType::TextDocument`] as the `StreamType`.
    MaskOutline {
        dispose: ;

        /// Given an mask outline pointer (obtainable through the [`suites::Stream`](aegp::suites::Stream), determines if the mask path is open or closed.
        is_open() -> bool => suite.is_mask_outline_open,

        /// Sets the open state of the given mask outline.
        set_open(open: bool) -> () => suite.set_mask_outline_open,

        /// Given a mask outline pointer, returns the number of segments in the path.
        num_segments() -> i32 => suite.mask_outline_num_segments,

        /// Given a mask outline pointer and a point between 0 and the total number of segments.
        vertex_info(point: i32) -> ae_sys::AEGP_MaskVertex => suite.mask_outline_vertex_info,

        /// Sets the vertex information for a given index.
        set_vertex_info(point: i32, vertex: &ae_sys::AEGP_MaskVertex) -> () => suite.set_mask_outline_vertex_info,

        /// Creates a vertex at index position.
        create_vertex(position: i32) -> () => suite.create_vertex,

        /// Removes a vertex from a mask.
        delete_vertex(index: i32) -> () => suite.delete_vertex,

        /// Given a mask outline pointer, returns the number of feathers in the path.
        num_feathers() -> i32 => suite.mask_outline_num_feathers,

        /// Given a mask outline pointer and a feather index, returns the feather information.
        feather_info(feather_index: i32) -> ae_sys::AEGP_MaskFeather => suite.mask_outline_feather_info,

        /// Sets the feather information for a given index. Feather must already exist; use [`create_feather()`](Self::create_feather) first, if needed.
        set_feather_info(feather_index: i32, feather: &ae_sys::AEGP_MaskFeather) -> () => suite.set_mask_outline_feather_info,

        /// Creates a feather at the given index. Returns the index of new feather.
        create_feather(feather: Option<ae_sys::AEGP_MaskFeather>) -> i32 => suite.create_mask_outline_feather,

        /// Deletes a feather from the mask.
        delete_feather(index: i32) -> () => suite.delete_mask_outline_feather,
    }
);
