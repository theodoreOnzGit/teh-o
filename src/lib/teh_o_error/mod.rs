
use std::time::SystemTimeError;

use thiserror::Error;

/// Master Error type of this crate
#[derive(Debug, Error)]
pub enum TehOError {
    /// 
    #[error("OpenMC_Err_Unassigned")]
    OpenMcErrUnassigned,

    /// 
    #[error("OpenMC_Err_Allocate")]
    OpenMcErrAllocate,
    
    /// 
    #[error("OpenMC_Err_Out_of_Bounds")]
    OpenMcErrOutOfBounds,

    /// 
    #[error("OpenMC Err Invalid Size")]
    OpenMcErrInvalidSize,

    /// 
    #[error("OpenMC Err Invalid Argument")]
    OpenMcErrInvalidArgument,

    /// 
    #[error("OpenMC Err Invalid Type")]
    OpenMcErrInvalidType,

    /// 
    #[error("OpenMC Err Invalid ID")]
    OpenMcErrInvalidID,

    /// 
    #[error("OpenMC Err Geometry")]
    OpenMcErrGeometry,

    /// 
    #[error("OpenMC Err Data")]
    OpenMcErrData,

    /// 
    #[error("OpenMC Err Physics")]
    OpenMcErrPhysics,

    /// 
    #[error("OpenMC Err Warning")]
    OpenMcErrWarning,

    /// 
    #[error("SystemTimeError")]
    SystemTimeError(SystemTimeError),

    /// IO error 
    #[error("IOError")]
    IOError(std::io::Error),

    /// hdf5 error 
    #[allow(non_camel_case_types)]
    #[error("hdf5 error")]
    hdf5Error(hdf5::Error),
}

impl From<SystemTimeError> for TehOError {
    fn from(value: SystemTimeError) -> Self {
        TehOError::SystemTimeError(value)
    }
}

impl From<std::io::Error> for TehOError {
    fn from(value: std::io::Error) -> Self {
        TehOError::IOError(value)
    }
}

impl From <hdf5::Error> for TehOError {
    fn from(value: hdf5::Error) -> Self {
        TehOError::hdf5Error(value)
    }
}
