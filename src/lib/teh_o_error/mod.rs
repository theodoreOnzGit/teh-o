
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

}


