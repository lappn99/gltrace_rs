use crate::hooks::gl::enums::{SAMPLES_PASSED, ANY_SAMPLES_PASSED, ANY_SAMPLES_PASSED_CONSERVATIVE, PRIMITIVES_GENERATED, TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN, TIME_ELAPSED};
use crate::types::types::GLenum;

pub enum QueryTarget {
    SamplesPassed,
    AnySamplesPassed,
    AnySamplesPassedConservative,
    PrimitivesGenerated,
    TransformFeedbackPrimitivesWritten,
    TimeElapsed,
    Unknown

}

impl From<GLenum> for QueryTarget {
    fn from(value: GLenum) -> Self {
        match value {
            SAMPLES_PASSED => QueryTarget::SamplesPassed,
            ANY_SAMPLES_PASSED => QueryTarget::AnySamplesPassed,
            ANY_SAMPLES_PASSED_CONSERVATIVE => QueryTarget::AnySamplesPassedConservative,
            PRIMITIVES_GENERATED => QueryTarget::PrimitivesGenerated,
            TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN => QueryTarget::TransformFeedbackPrimitivesWritten,
            TIME_ELAPSED => QueryTarget::TimeElapsed,
            _ => QueryTarget::Unknown
        }
    }
}

impl From<QueryTarget> for GLenum {
    fn from(value: QueryTarget) -> Self {
        match value {
            QueryTarget::SamplesPassed => SAMPLES_PASSED,
            QueryTarget::AnySamplesPassed => ANY_SAMPLES_PASSED,
            QueryTarget::AnySamplesPassedConservative => ANY_SAMPLES_PASSED_CONSERVATIVE,
            QueryTarget::PrimitivesGenerated => PRIMITIVES_GENERATED,
            QueryTarget::TransformFeedbackPrimitivesWritten => TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN,
            QueryTarget::TimeElapsed => TIME_ELAPSED,
            _ => 0
        }
    }
}
