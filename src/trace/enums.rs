use crate::types::types::*;

#[derive(Debug, Clone)]
pub enum TraceEntryParamValue {
    Byte(GLbyte),
    Short(GLshort),
    UShort(GLushort),
    Int(GLint),
    UInt(GLuint),
    Int64(GLint64),
    UInt64(GLuint64),
    IntPtr(GLintptr),
    Sync(GLsync),
    Float(GLfloat),
    Double(GLdouble),
    UByte(GLubyte),
    
    ConstIntPtr(*const GLint),
    ConstBytePtr(*const GLbyte),
    ConstShortPtr(*const GLshort),
    ConstUShortPtr(*const GLushort),
    ConstUIntPtr(*const GLuint),
    ConstInt64Ptr(*const GLint64),
    ConstUInt64Ptr(*const GLuint64),
    ConstIntPtrPtr(*const GLintptr),
    ConstSyncPtr(*const GLsync),
    ConstFloatPtr(*const GLfloat),
    ConstDoublePtr(*const GLdouble),
    ConstUBytePtr(*const GLubyte),

    MutIntPtr(*mut GLint),
    MutBytePtr(*mut GLbyte),
    MutShortPtr(*mut GLshort),
    MutUShortPtr(*mut GLushort),
    MutUIntPtr(*mut GLuint),
    MutInt64Ptr(*mut GLint64),
    MutUInt64Ptr(*mut GLuint64),
    MutIntPtrPtr(*mut GLintptr),
    MutSyncPtr(*mut GLsync),
    MutFloatPtr(*mut GLfloat),
    MutDoublePtr(*mut GLdouble),
    MutUBytePtr(*mut GLubyte),

    ConstConstBytePtr(  *const *const GLbyte),
    ConstMutVoidPtr(    *const *mut std::ffi::c_void),
    ConstConstVoidPtr(  *const *const std::ffi::c_void),

    
    Callback(*mut std::ffi::c_void),
    VoidPtr(*const std::ffi::c_void),
    DebugProc(GLDEBUGPROC)

}

macro_rules! trace_entry_param_from {
    ($from_type:ty, $to_type:expr) => {
        impl From<$from_type> for TraceEntryParamValue {
            fn from(value: $from_type) -> Self {
                ($to_type)(value)
            }
        }
    };
}

trace_entry_param_from!(GLint, TraceEntryParamValue::Int);
trace_entry_param_from!(GLbyte, TraceEntryParamValue::Byte);
trace_entry_param_from!(GLshort, TraceEntryParamValue::Short);
trace_entry_param_from!(GLushort,TraceEntryParamValue::UShort);
trace_entry_param_from!(GLuint,  TraceEntryParamValue::UInt);
trace_entry_param_from!(GLint64, TraceEntryParamValue::Int64);
trace_entry_param_from!(GLuint64,TraceEntryParamValue::UInt64);
trace_entry_param_from!(GLintptr,TraceEntryParamValue::IntPtr);
trace_entry_param_from!(GLsync,  TraceEntryParamValue::Sync);
trace_entry_param_from!(GLfloat, TraceEntryParamValue::Float);
trace_entry_param_from!(GLdouble,TraceEntryParamValue::Double);
trace_entry_param_from!(GLubyte, TraceEntryParamValue::UByte);

trace_entry_param_from!(*mut std::ffi::c_void, TraceEntryParamValue::Callback);
trace_entry_param_from!(*const std::ffi::c_void, TraceEntryParamValue::VoidPtr);
trace_entry_param_from!(GLDEBUGPROC, TraceEntryParamValue::DebugProc);
trace_entry_param_from!(*const GLint,TraceEntryParamValue::ConstIntPtr);
trace_entry_param_from!(*const GLbyte,TraceEntryParamValue::ConstBytePtr);
trace_entry_param_from!(*const GLshort,TraceEntryParamValue::ConstShortPtr);
trace_entry_param_from!(*const GLushort,TraceEntryParamValue::ConstUShortPtr);
trace_entry_param_from!(*const GLuint,TraceEntryParamValue::ConstUIntPtr);
trace_entry_param_from!(*const GLint64,TraceEntryParamValue::ConstInt64Ptr);
trace_entry_param_from!(*const GLuint64,TraceEntryParamValue::ConstUInt64Ptr);
trace_entry_param_from!(*const GLintptr,TraceEntryParamValue::ConstIntPtrPtr);
trace_entry_param_from!(*const GLsync,TraceEntryParamValue::ConstSyncPtr);
trace_entry_param_from!(*const GLfloat,TraceEntryParamValue::ConstFloatPtr);
trace_entry_param_from!(*const GLdouble,TraceEntryParamValue::ConstDoublePtr);
trace_entry_param_from!(*const GLubyte, TraceEntryParamValue::ConstUBytePtr);

trace_entry_param_from!(*mut GLint,TraceEntryParamValue::MutIntPtr);
trace_entry_param_from!(*mut GLbyte,TraceEntryParamValue::MutBytePtr);
trace_entry_param_from!(*mut GLshort,TraceEntryParamValue::MutShortPtr);
trace_entry_param_from!(*mut GLushort,TraceEntryParamValue::MutUShortPtr);
trace_entry_param_from!(*mut GLuint,TraceEntryParamValue::MutUIntPtr);
trace_entry_param_from!(*mut GLint64,TraceEntryParamValue::MutInt64Ptr);
trace_entry_param_from!(*mut GLuint64,TraceEntryParamValue::MutUInt64Ptr);
trace_entry_param_from!(*mut GLintptr,TraceEntryParamValue::MutIntPtrPtr);
trace_entry_param_from!(*mut GLsync,TraceEntryParamValue::MutSyncPtr);
trace_entry_param_from!(*mut GLfloat,TraceEntryParamValue::MutFloatPtr);
trace_entry_param_from!(*mut GLdouble,TraceEntryParamValue::MutDoublePtr);
trace_entry_param_from!(*mut GLubyte, TraceEntryParamValue::MutUBytePtr);

trace_entry_param_from!(*const *const GLbyte,TraceEntryParamValue::ConstConstBytePtr);
trace_entry_param_from!(*const *mut std::ffi::c_void, TraceEntryParamValue::ConstMutVoidPtr);
trace_entry_param_from!(*const *const std::ffi::c_void, TraceEntryParamValue::ConstConstVoidPtr);