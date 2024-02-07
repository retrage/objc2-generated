//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_static!(HKErrorDomain: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKErrorCode {
        HKUnknownError = 0,
        #[deprecated]
        HKNoError = HKUnknownError.0,
        HKErrorHealthDataUnavailable = 1,
        HKErrorHealthDataRestricted = 2,
        HKErrorInvalidArgument = 3,
        HKErrorAuthorizationDenied = 4,
        HKErrorAuthorizationNotDetermined = 5,
        HKErrorDatabaseInaccessible = 6,
        HKErrorUserCanceled = 7,
        HKErrorAnotherWorkoutSessionStarted = 8,
        HKErrorUserExitedWorkoutSession = 9,
        HKErrorRequiredAuthorizationDenied = 10,
        HKErrorNoData = 11,
        HKErrorWorkoutActivityNotAllowed = 12,
        HKErrorDataSizeExceeded = 13,
        HKErrorBackgroundWorkoutSessionNotAllowed = 14,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKUpdateFrequency {
        HKUpdateFrequencyImmediate = 1,
        HKUpdateFrequencyHourly = 2,
        HKUpdateFrequencyDaily = 3,
        HKUpdateFrequencyWeekly = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKAuthorizationStatus {
        HKAuthorizationStatusNotDetermined = 0,
        HKAuthorizationStatusSharingDenied = 1,
        HKAuthorizationStatusSharingAuthorized = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKAuthorizationRequestStatus {
        HKAuthorizationRequestStatusUnknown = 0,
        HKAuthorizationRequestStatusShouldRequest = 1,
        HKAuthorizationRequestStatusUnnecessary = 2,
    }
);

extern_fn!(
    #[cfg(all(feature = "Foundation_NSNumber", feature = "Foundation_NSSet"))]
    pub unsafe fn HKCategoryValueSleepAnalysisAsleepValues() -> NonNull<NSSet<NSNumber>>;
);
