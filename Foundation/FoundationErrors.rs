//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum __anonymous__ {
        NSFileNoSuchFileError = 4,
        NSFileLockingError = 255,
        NSFileReadUnknownError = 256,
        NSFileReadNoPermissionError = 257,
        NSFileReadInvalidFileNameError = 258,
        NSFileReadCorruptFileError = 259,
        NSFileReadNoSuchFileError = 260,
        NSFileReadInapplicableStringEncodingError = 261,
        NSFileReadUnsupportedSchemeError = 262,
        NSFileReadTooLargeError = 263,
        NSFileReadUnknownStringEncodingError = 264,
        NSFileWriteUnknownError = 512,
        NSFileWriteNoPermissionError = 513,
        NSFileWriteInvalidFileNameError = 514,
        NSFileWriteFileExistsError = 516,
        NSFileWriteInapplicableStringEncodingError = 517,
        NSFileWriteUnsupportedSchemeError = 518,
        NSFileWriteOutOfSpaceError = 640,
        NSFileWriteVolumeReadOnlyError = 642,
        NSFileManagerUnmountUnknownError = 768,
        NSFileManagerUnmountBusyError = 769,
        NSKeyValueValidationError = 1024,
        NSFormattingError = 2048,
        NSUserCancelledError = 3072,
        NSFeatureUnsupportedError = 3328,
        NSExecutableNotLoadableError = 3584,
        NSExecutableArchitectureMismatchError = 3585,
        NSExecutableRuntimeMismatchError = 3586,
        NSExecutableLoadError = 3587,
        NSExecutableLinkError = 3588,
        NSFileErrorMinimum = 0,
        NSFileErrorMaximum = 1023,
        NSValidationErrorMinimum = 1024,
        NSValidationErrorMaximum = 2047,
        NSExecutableErrorMinimum = 3584,
        NSExecutableErrorMaximum = 3839,
        NSFormattingErrorMinimum = 2048,
        NSFormattingErrorMaximum = 2559,
        NSPropertyListReadCorruptError = 3840,
        NSPropertyListReadUnknownVersionError = 3841,
        NSPropertyListReadStreamError = 3842,
        NSPropertyListWriteStreamError = 3851,
        NSPropertyListWriteInvalidError = 3852,
        NSPropertyListErrorMinimum = 3840,
        NSPropertyListErrorMaximum = 4095,
        NSXPCConnectionInterrupted = 4097,
        NSXPCConnectionInvalid = 4099,
        NSXPCConnectionReplyInvalid = 4101,
        NSXPCConnectionCodeSigningRequirementFailure = 4102,
        NSXPCConnectionErrorMinimum = 4096,
        NSXPCConnectionErrorMaximum = 4224,
        NSUbiquitousFileUnavailableError = 4353,
        NSUbiquitousFileNotUploadedDueToQuotaError = 4354,
        NSUbiquitousFileUbiquityServerNotAvailable = 4355,
        NSUbiquitousFileErrorMinimum = 4352,
        NSUbiquitousFileErrorMaximum = 4607,
        NSUserActivityHandoffFailedError = 4608,
        NSUserActivityConnectionUnavailableError = 4609,
        NSUserActivityRemoteApplicationTimedOutError = 4610,
        NSUserActivityHandoffUserInfoTooLargeError = 4611,
        NSUserActivityErrorMinimum = 4608,
        NSUserActivityErrorMaximum = 4863,
        NSCoderReadCorruptError = 4864,
        NSCoderValueNotFoundError = 4865,
        NSCoderInvalidValueError = 4866,
        NSCoderErrorMinimum = 4864,
        NSCoderErrorMaximum = 4991,
        NSBundleErrorMinimum = 4992,
        NSBundleErrorMaximum = 5119,
        NSBundleOnDemandResourceOutOfSpaceError = 4992,
        NSBundleOnDemandResourceExceededMaximumSizeError = 4993,
        NSBundleOnDemandResourceInvalidTagError = 4994,
        NSCloudSharingNetworkFailureError = 5120,
        NSCloudSharingQuotaExceededError = 5121,
        NSCloudSharingTooManyParticipantsError = 5122,
        NSCloudSharingConflictError = 5123,
        NSCloudSharingNoPermissionError = 5124,
        NSCloudSharingOtherError = 5375,
        NSCloudSharingErrorMinimum = 5120,
        NSCloudSharingErrorMaximum = 5375,
        NSCompressionFailedError = 5376,
        NSDecompressionFailedError = 5377,
        NSCompressionErrorMinimum = 5376,
        NSCompressionErrorMaximum = 5503,
    }
);
