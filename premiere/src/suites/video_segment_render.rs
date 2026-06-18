use crate::*;

define_suite!(
    /// Render frames from video-segment nodes off the playback path, at a
    /// resolution you control.
    ///
    /// The Video Segment Render Suite lets a plugin ask the host to render *any
    /// node* in a timeline's segment tree — clip, media, compositor, transition,
    /// multicam, solid-color, adjustment — and hand back the resulting frame via
    /// an async completion proc. It is the off-playback counterpart of the
    /// Video Segment Suite (which only *reads* the tree topology).
    ///
    /// # Why this suite matters for clip extraction
    ///
    /// The headline call is [`produce_frame_async`](Self::produce_frame_async).
    /// Given a `PrTimelineID` + a node ID, it renders *just that node* at a
    /// resolution **the caller specifies** (`in_sequence_width/height`), over
    /// only the node's in→out range, asynchronously, and returns a `PPixHand` the
    /// caller must `Dispose`. Crucially:
    ///
    /// - **The monitor's preview downsample cannot shrink the frame** — you pass
    ///   explicit `in_sequence_width/height`, so a 1/2 / 1/4 preview does not
    ///   affect extraction. This is what makes the suite downsample-independent.
    /// - **Effect nodes are not suitable** — pass a clip/media/compositor node.
    ///   Rendering the clip node yields source + intrinsic transforms *up to but
    ///   not including* your effect/operators, which is exactly "the clip's
    ///   pixels". To re-apply a subset of operators, use
    ///   [`apply_operators_to_frame_async`](Self::apply_operators_to_frame_async).
    /// - **The returned frame may not match your requested pixel formats** —
    ///   the host picks the closest match; always check via the PPix Suite.
    ///
    /// # Async model
    ///
    /// Every `*_async` call takes an
    /// [`AsyncRenderCompletionProc`](pr_sys::PrSDKVideoSegmentAsyncRenderCompletionProc)
    /// + an opaque `csSDK_int64` cookie. The completion proc receives the
    /// `PPixHand`, the cookie, and a `prSuiteError` result. **The caller must
    /// `Dispose` the PPixHand** via the PPix Suite — failure leaks VRAM. The
    /// `out_request_id` can be passed to
    /// [`cancel_async_request`](Self::cancel_async_request); note the docs warn
    /// the completion proc may fire *before* `produce_frame_async` returns, so
    /// treat the request id as best-effort.
    ///
    /// # Cache probe (identifier) calls
    ///
    /// Each async render has a matching `get_identifier_for_*` call that returns
    /// a host-computed `prPluginID` cache key for the same arguments. Probe the
    /// identifier first; on a hit you skip the render and read the cached frame
    /// instead. This is the cheap clip-scoped identity the host already
    /// maintains — useful as a *hint* with a content hash as the source of truth.
    ///
    /// # Versions
    ///
    /// - **v1** (`kPrSDKVideoSegmentRenderSuiteVersion5`, CS5): the original
    ///   `ProduceFrameAsync` / `ApplyOperatorsToFrameAsync` /
    ///   `ApplyTransitionToFrameAsync` + their `GetIdentifierFor*` counterparts,
    ///   plus clip-prefetch (`InitiateClipPrefetch`, `SelectClipFrameDescriptor`,
    ///   `CancelAsyncRequest`, `SupportsInitiateClipPrefetch`).
    /// - **v2/v3** add an `imRenderContext` (playback intent/rate) and, for v3,
    ///   an `inBypassEffects` flag to skip non-intrinsic effects while rendering.
    /// - **v4+** add color-managed variants taking `SequenceRender_ParamsRecExt`
    ///   + a captioning-stream-format arg.
    ///
    /// This wrapper binds all of them; pick the lowest version that has the
    /// features you need. For Phase-0 clip extraction the v1
    /// `produce_frame_async` is sufficient.
    VideoSegmentRenderSuite,
    PrSDKVideoSegmentRenderSuite,
    kPrSDKVideoSegmentRenderSuite,
    kPrSDKVideoSegmentRenderSuiteVersion
);

/// Render-context intent passed to the v2+ variants.
///
/// Describes why the host is asking for the frame (export, scrubbing, playing,
/// prefetch…) plus the playback ratio/rate, so the importer/renderer can pick
/// a cheaper path when appropriate. Pass
/// [`RenderIntent::Unknown`](Self::Unknown) with ratio/rate `0.0` when you
/// have no playback context (the typical clip-extraction case).
#[derive(Clone, Copy, Debug)]
pub struct RenderContext {
    pub intent:      RenderIntent,
    pub play_ratio:  f64,
    pub play_rate:   f64,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self { intent: RenderIntent::Unknown, play_ratio: 0.0, play_rate: 0.0 }
    }
}

impl From<RenderContext> for pr_sys::imRenderContext {
    fn from(c: RenderContext) -> Self {
        // SAFETY: imRenderContext is #[repr(C, packed)] POD (c_int + 2×f64).
        // Constructing it via zeroed-then-fill avoids the unaligned-field-write
        // UB that direct field assignment on a packed struct can trigger.
        unsafe {
            let mut raw: pr_sys::imRenderContext = std::mem::zeroed();
            raw.inIntent       = c.intent as pr_sys::imRenderIntent;
            raw.inPlaybackRatio = c.play_ratio;
            raw.inPlaybackRate  = c.play_rate;
            raw
        }
    }
}

/// Why a frame is being requested. See the `imRenderIntent_*` constants in
/// [`pr_sys`]. `Unknown` (-1) is the safe default when you have no playback
/// context (e.g. background clip extraction).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum RenderIntent {
    Unknown            = -1,
    Export             = 0,
    Stopped            = 1,
    Scrubbing          = 2,
    Preroll            = 3,
    Playing            = 4,
    SpeculativePrefetch = 5,
    Thumbnail          = 6,
    Analysis           = 7,
    ExportPreview      = 8,
    ExportProxies       = 9,
    DistantPrefetch    = 10,
}

/// Bypass-non-intrinsic-effects flag for the v3+ `*_3` variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum BypassEffects {
    No  = 0,
    Yes = 1,
}

/// Result of [`VideoSegmentRenderSuite::produce_frame_async`] and friends —
/// the host hands back a `request_id` you can pass to
/// [`cancel_async_request`](VideoSegmentRenderSuite::cancel_async_request).
/// Keep in mind the completion proc can fire *before* this returns, so the id
/// is best-effort.
#[derive(Clone, Copy, Debug)]
pub struct AsyncRequest {
    pub request_id: i32,
}

/// Describes a frame format being requested from a clip source. Any member can
/// be 0 / `None`, meaning "any value is acceptable". Returned by
/// [`VideoSegmentRenderSuite::select_clip_frame_descriptor`].
#[derive(Clone, Copy, Debug, Default)]
pub struct ClipFrameDescriptor {
    pub pixel_format:   Option<PixelFormat>,
    pub width:           i32,
    pub height:          i32,
    pub par_numerator:   i32,
    pub par_denominator: i32,
    pub field_type:      Option<pr_sys::prFieldType>,
    pub quality:         Option<RenderQuality>,
}

impl From<ClipFrameDescriptor> for pr_sys::ClipFrameDescriptor {
    fn from(d: ClipFrameDescriptor) -> Self {
        unsafe {
            let mut raw: pr_sys::ClipFrameDescriptor = std::mem::zeroed();
            raw.inPixelFormat                     = d.pixel_format.map(|f| f.into()).unwrap_or(0);
            raw.inWidth                          = d.width;
            raw.inHeight                         = d.height;
            raw.inPixelAspectRatioNumerator       = d.par_numerator;
            raw.inPixelAspectRatioDenominator     = d.par_denominator;
            raw.inFieldType                      = d.field_type.unwrap_or(0);
            raw.inQuality                        = d.quality.map(|q| q.into()).unwrap_or(pr_sys::PrRenderQuality_kPrRenderQuality_Max);
            raw
        }
    }
}

impl VideoSegmentRenderSuite {
    /// Acquire this suite from the host. Returns error if the suite is not
    /// available (e.g. running outside a Premiere GPU-filter/transmit/exporter
    /// context). Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Render the frame a node would normally produce, at a resolution you
    /// control, off the playback path.
    ///
    /// This is the clip-extraction primitive: pass the **clip node** ID (not an
    /// effect node — effect nodes are not suitable; inputs work, operators do
    /// not) and explicit `sequence_width/height`, and the host renders *just
    /// that node's frame* asynchronously, ignoring the monitor's preview
    /// downsample. The completion proc receives the `PPixHand`; **you must
    /// `Dispose` it** via the PPix Suite.
    ///
    /// * `timeline_id`  - host-provided timeline identifier
    /// * `node_id`      - the node to render (clip/media/compositor/…; **not**
    ///   an effect node)
    /// * `sequence_time` / `segment_time` - time in the containing timeline /
    ///   in the node's own time
    /// * `ticks_per_frame` - sequence frame rate in ticks (1 frame)
    /// * `width` / `height` - the **overridden** output resolution; pass 0 for
    ///   "no override" (use the node's native size)
    /// * `par_num` / `par_den` - overridden pixel aspect ratio; 0/0 = no override
    /// * `render_params` - the same render-params struct used by the
    ///   Sequence Render Suite; `composite_on_black` is ignored
    /// * `completion_proc` - async callback receiving the rendered `PPixHand`
    /// * `completion_data` - opaque cookie passed back to the completion proc
    ///
    /// Returns an [`AsyncRequest`] whose `request_id` can cancel the render.
    /// The completion proc may fire before this returns — treat the id as
    /// best-effort.
    pub fn produce_frame_async(
        &self,
        timeline_id: pr_sys::PrTimelineID,
        node_id: i32,
        sequence_time: i64,
        segment_time: i64,
        ticks_per_frame: i64,
        width: i32,
        height: i32,
        par_num: i32,
        par_den: i32,
        render_params: &pr_sys::SequenceRender_ParamsRec,
        completion_proc: pr_sys::PrSDKVideoSegmentAsyncRenderCompletionProc,
        completion_data: i64,
    ) -> Result<AsyncRequest, Error> {
        let mut request_id: i32 = 0;
        call_suite_fn!(
            self, ProduceFrameAsync,
            timeline_id, node_id, sequence_time, segment_time, ticks_per_frame,
            width, height, par_num, par_den,
            render_params as *const _,
            completion_proc, completion_data,
            &mut request_id
        )?;
        Ok(AsyncRequest { request_id })
    }

    /// Cache probe matching [`produce_frame_async`](Self::produce_frame_async).
    ///
    /// Returns a host-computed `prPluginID` cache key for the same arguments.
    /// Probe it first; on a hit, read the cached frame instead of re-rendering.
    /// Use as a *hint* — a content hash is the source of truth.
    pub fn identifier_for_produce_frame_async(
        &self,
        timeline_id: pr_sys::PrTimelineID,
        node_id: i32,
        sequence_time: i64,
        segment_time: i64,
        ticks_per_frame: i64,
        width: i32,
        height: i32,
        par_num: i32,
        par_den: i32,
        render_params: &pr_sys::SequenceRender_ParamsRec,
    ) -> Result<pr_sys::prPluginID, Error> {
        let mut id: pr_sys::prPluginID = unsafe { std::mem::zeroed() };
        call_suite_fn!(
            self, GetIdentifierForProduceFrameAsync,
            timeline_id, node_id, sequence_time, segment_time, ticks_per_frame,
            width, height, par_num, par_den,
            render_params as *const _,
            &mut id
        )?;
        Ok(id)
    }

    /// Render-context variant of
    /// [`produce_frame_async`](Self::produce_frame_async) (suite v2+, CC).
    ///
    /// Same as `produce_frame_async` but takes a [`RenderContext`] so the
    /// importer/renderer knows the playback intent (export/scrub/preview…)
    /// and can pick a cheaper path when appropriate. Pass
    /// [`RenderContext::default()`] when you have no playback context.
    pub fn produce_frame_async2(
        &self,
        timeline_id: pr_sys::PrTimelineID,
        node_id: i32,
        sequence_time: i64,
        segment_time: i64,
        ticks_per_frame: i64,
        width: i32,
        height: i32,
        par_num: i32,
        par_den: i32,
        render_params: &pr_sys::SequenceRender_ParamsRec,
        render_context: RenderContext,
        completion_proc: pr_sys::PrSDKVideoSegmentAsyncRenderCompletionProc,
        completion_data: i64,
    ) -> Result<AsyncRequest, Error> {
        let mut request_id: i32 = 0;
        call_suite_fn!(
            self, ProduceFrameAsync2,
            timeline_id, node_id, sequence_time, segment_time, ticks_per_frame,
            width, height, par_num, par_den,
            render_params as *const _,
            render_context.into(),
            completion_proc, completion_data,
            &mut request_id
        )?;
        Ok(AsyncRequest { request_id })
    }

    /// v3+ variant of [`produce_frame_async`](Self::produce_frame_async) with
    /// an explicit `bypass_effects` flag.
    ///
    /// When `bypass_effects` is `Yes`, the render skips non-intrinsic video
    /// effects — useful when you want the clip's raw source + intrinsic
    /// transforms only, with effects layered back on separately.
    pub fn produce_frame_async3(
        &self,
        timeline_id: pr_sys::PrTimelineID,
        node_id: i32,
        sequence_time: i64,
        segment_time: i64,
        ticks_per_frame: i64,
        width: i32,
        height: i32,
        par_num: i32,
        par_den: i32,
        render_params: &pr_sys::SequenceRender_ParamsRec,
        render_context: RenderContext,
        completion_proc: pr_sys::PrSDKVideoSegmentAsyncRenderCompletionProc,
        completion_data: i64,
        bypass_effects: BypassEffects,
    ) -> Result<AsyncRequest, Error> {
        let mut request_id: i32 = 0;
        call_suite_fn!(
            self, ProduceFrameAsync3,
            timeline_id, node_id, sequence_time, segment_time, ticks_per_frame,
            width, height, par_num, par_den,
            render_params as *const _,
            render_context.into(),
            completion_proc, completion_data,
            bypass_effects as pr_sys::prBool,
            &mut request_id
        )?;
        Ok(AsyncRequest { request_id })
    }

    /// Cache probe matching
    /// [`produce_frame_async3`](Self::produce_frame_async3) (includes the
    /// `bypass_effects` flag in the key).
    pub fn identifier_for_produce_frame_async2(
        &self,
        timeline_id: pr_sys::PrTimelineID,
        node_id: i32,
        sequence_time: i64,
        segment_time: i64,
        ticks_per_frame: i64,
        width: i32,
        height: i32,
        par_num: i32,
        par_den: i32,
        render_params: &pr_sys::SequenceRender_ParamsRec,
        bypass_effects: BypassEffects,
    ) -> Result<pr_sys::prPluginID, Error> {
        let mut id: pr_sys::prPluginID = unsafe { std::mem::zeroed() };
        call_suite_fn!(
            self, GetIdentifierForProduceFrameAsync2,
            timeline_id, node_id, sequence_time, segment_time, ticks_per_frame,
            width, height, par_num, par_den,
            render_params as *const _,
            bypass_effects as pr_sys::prBool,
            &mut id
        )?;
        Ok(id)
    }

    /// Apply a range of operators (effects) to a clip node asynchronously,
    /// given an optional input frame.
    ///
    /// * `clip_node_id` - the clip that owns the operators
    /// * `operator_start_index` / `operator_count` - zero-based range of
    ///   operators to apply
    /// * `input_frame` - may be null when `operator_start_index == 0`, meaning
    ///   "use the clip node's own input frame"
    ///
    /// Use this when you want to re-apply a *subset* of a clip's effects to a
    /// frame you already have (e.g. after extracting the raw clip via
    /// `produce_frame_async`).
    pub fn apply_operators_to_frame_async(
        &self,
        timeline_id: pr_sys::PrTimelineID,
        clip_node_id: i32,
        operator_start_index: i32,
        operator_count: i32,
        sequence_time: i64,
        segment_time: i64,
        ticks_per_frame: i64,
        width: i32,
        height: i32,
        par_num: i32,
        par_den: i32,
        input_frame: pr_sys::PPixHand,
        render_params: &pr_sys::SequenceRender_ParamsRec,
        completion_proc: pr_sys::PrSDKVideoSegmentAsyncRenderCompletionProc,
        completion_data: i64,
    ) -> Result<AsyncRequest, Error> {
        let mut request_id: i32 = 0;
        call_suite_fn!(
            self, ApplyOperatorsToFrameAsync,
            timeline_id, clip_node_id, operator_start_index, operator_count,
            sequence_time, segment_time, ticks_per_frame,
            width, height, par_num, par_den,
            input_frame,
            render_params as *const _,
            completion_proc, completion_data,
            &mut request_id
        )?;
        Ok(AsyncRequest { request_id })
    }

    /// Cache probe matching
    /// [`apply_operators_to_frame_async`](Self::apply_operators_to_frame_async).
    pub fn identifier_for_apply_operators_to_frame_async(
        &self,
        timeline_id: pr_sys::PrTimelineID,
        clip_node_id: i32,
        operator_start_index: i32,
        operator_count: i32,
        sequence_time: i64,
        segment_time: i64,
        ticks_per_frame: i64,
        width: i32,
        height: i32,
        par_num: i32,
        par_den: i32,
        input_frame: pr_sys::PPixHand,
        render_params: &pr_sys::SequenceRender_ParamsRec,
    ) -> Result<pr_sys::prPluginID, Error> {
        let mut id: pr_sys::prPluginID = unsafe { std::mem::zeroed() };
        call_suite_fn!(
            self, GetIdentifierForApplyOperatorsToFrameAsync,
            timeline_id, clip_node_id, operator_start_index, operator_count,
            sequence_time, segment_time, ticks_per_frame,
            width, height, par_num, par_den,
            input_frame,
            render_params as *const _,
            &mut id
        )?;
        Ok(id)
    }

    /// Render a transition between two (optional) input frames asynchronously.
    ///
    /// * `transition_node_id` - **must** be a transition node
    /// * `outgoing` / `incoming` - null PPixHand ⇒ transparent black
    pub fn apply_transition_to_frame_async(
        &self,
        timeline_id: pr_sys::PrTimelineID,
        transition_node_id: i32,
        sequence_time: i64,
        segment_time: i64,
        ticks_per_frame: i64,
        outgoing: pr_sys::PPixHand,
        incoming: pr_sys::PPixHand,
        render_params: &pr_sys::SequenceRender_ParamsRec,
        completion_proc: pr_sys::PrSDKVideoSegmentAsyncRenderCompletionProc,
        completion_data: i64,
    ) -> Result<AsyncRequest, Error> {
        let mut request_id: i32 = 0;
        call_suite_fn!(
            self, ApplyTransitionToFrameAsync,
            timeline_id, transition_node_id,
            sequence_time, segment_time, ticks_per_frame,
            outgoing, incoming,
            render_params as *const _,
            completion_proc, completion_data,
            &mut request_id
        )?;
        Ok(AsyncRequest { request_id })
    }

    /// Cancel an outstanding async render/prefetch request.
    ///
    /// Best-effort: the completion proc may have already fired.
    pub fn cancel_async_request(&self, request_id: i32) -> Result<(), Error> {
        call_suite_fn!(self, CancelAsyncRequest, request_id)
    }

    /// For a clip ID (a property of a media node), find the closest available
    /// match to a desired frame descriptor.
    ///
    /// * `clip_id` - from a media node's `MediaNode::ClipID` property
    /// * `clip_time` - the source may vary its answer over time
    /// * `desired` - size/par/quality/pixel format you want; leave size or par
    ///   at 0 for "native size"
    ///
    /// Returns the closest match the importer/source can actually provide.
    pub fn select_clip_frame_descriptor(
        &self,
        clip_id: pr_sys::PrClipID,
        clip_time: i64,
        desired: &ClipFrameDescriptor,
    ) -> Result<ClipFrameDescriptor, Error> {
        let desired_raw: pr_sys::ClipFrameDescriptor = (*desired).into();
        let mut best: pr_sys::ClipFrameDescriptor = unsafe { std::mem::zeroed() };
        call_suite_fn!(
            self, SelectClipFrameDescriptor,
            clip_id, clip_time,
            &desired_raw as *const _,
            &mut best
        )?;
        Ok(ClipFrameDescriptor {
            pixel_format:   if best.inPixelFormat == 0 { None } else { Some(best.inPixelFormat.into()) },
            width:           best.inWidth,
            height:          best.inHeight,
            par_numerator:   best.inPixelAspectRatioNumerator,
            par_denominator: best.inPixelAspectRatioDenominator,
            field_type:      if best.inFieldType == 0 { None } else { Some(best.inFieldType) },
            quality:         Some(best.inQuality.into()),
        })
    }

    /// Start a prefetch for a clip at a media time.
    ///
    /// * `clip_id` - from a media node's `MediaNode::ClipID` property
    /// * `descriptor` - a frame descriptor previously returned by
    ///   [`select_clip_frame_descriptor`](Self::select_clip_frame_descriptor)
    /// * `media_time` - time in the media's own space
    pub fn initiate_clip_prefetch(
        &self,
        clip_id: pr_sys::PrClipID,
        descriptor: &ClipFrameDescriptor,
        media_time: i64,
        completion_proc: pr_sys::PrSDKVideoSegmentAsyncRenderCompletionProc,
        completion_data: i64,
    ) -> Result<AsyncRequest, Error> {
        let mut request_id: i32 = 0;
        let desc: pr_sys::ClipFrameDescriptor = (*descriptor).into();
        call_suite_fn!(
            self, InitiateClipPrefetch,
            clip_id, &desc as *const _, media_time,
            completion_proc, completion_data,
            &mut request_id
        )?;
        Ok(AsyncRequest { request_id })
    }

    /// Whether a clip's source supports
    /// [`initiate_clip_prefetch`](Self::initiate_clip_prefetch).
    pub fn supports_initiate_clip_prefetch(&self, clip_id: pr_sys::PrClipID) -> Result<bool, Error> {
        let mut supported: pr_sys::prBool = 0;
        call_suite_fn!(self, SupportsInitiateClipPrefetch, clip_id, &mut supported)?;
        Ok(supported != 0)
    }
}
