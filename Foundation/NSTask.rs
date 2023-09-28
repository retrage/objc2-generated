//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTaskTerminationReason {
        NSTaskTerminationReasonExit = 1,
        NSTaskTerminationReasonUncaughtSignal = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSTask")]
    pub struct NSTask;

    #[cfg(feature = "Foundation_NSTask")]
    unsafe impl ClassType for NSTask {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSTask")]
unsafe impl NSObjectProtocol for NSTask {}

extern_methods!(
    #[cfg(feature = "Foundation_NSTask")]
    unsafe impl NSTask {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other executableURL)]
        pub unsafe fn executableURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setExecutableURL:)]
        pub unsafe fn setExecutableURL(&self, executable_url: Option<&NSURL>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setArguments:)]
        pub unsafe fn setArguments(&self, arguments: Option<&NSArray<NSString>>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other environment)]
        pub unsafe fn environment(&self) -> Option<Id<NSDictionary<NSString, NSString>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setEnvironment:)]
        pub unsafe fn setEnvironment(&self, environment: Option<&NSDictionary<NSString, NSString>>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other currentDirectoryURL)]
        pub unsafe fn currentDirectoryURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setCurrentDirectoryURL:)]
        pub unsafe fn setCurrentDirectoryURL(&self, current_directory_url: Option<&NSURL>);

        #[method_id(@__retain_semantics Other standardInput)]
        pub unsafe fn standardInput(&self) -> Option<Id<AnyObject>>;

        #[method(setStandardInput:)]
        pub unsafe fn setStandardInput(&self, standard_input: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other standardOutput)]
        pub unsafe fn standardOutput(&self) -> Option<Id<AnyObject>>;

        #[method(setStandardOutput:)]
        pub unsafe fn setStandardOutput(&self, standard_output: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other standardError)]
        pub unsafe fn standardError(&self) -> Option<Id<AnyObject>>;

        #[method(setStandardError:)]
        pub unsafe fn setStandardError(&self, standard_error: Option<&AnyObject>);

        #[cfg(feature = "Foundation_NSError")]
        #[method(launchAndReturnError:_)]
        pub unsafe fn launchAndReturnError(&self) -> Result<(), Id<NSError>>;

        #[method(interrupt)]
        pub unsafe fn interrupt(&self);

        #[method(terminate)]
        pub unsafe fn terminate(&self);

        #[method(suspend)]
        pub unsafe fn suspend(&self) -> bool;

        #[method(resume)]
        pub unsafe fn resume(&self) -> bool;

        #[method(processIdentifier)]
        pub unsafe fn processIdentifier(&self) -> c_int;

        #[method(isRunning)]
        pub unsafe fn isRunning(&self) -> bool;

        #[method(terminationStatus)]
        pub unsafe fn terminationStatus(&self) -> c_int;

        #[method(terminationReason)]
        pub unsafe fn terminationReason(&self) -> NSTaskTerminationReason;

        #[method(terminationHandler)]
        pub unsafe fn terminationHandler(&self) -> *mut Block<(NonNull<NSTask>,), ()>;

        #[method(setTerminationHandler:)]
        pub unsafe fn setTerminationHandler(
            &self,
            termination_handler: Option<&Block<(NonNull<NSTask>,), ()>>,
        );

        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;

        #[method(setQualityOfService:)]
        pub unsafe fn setQualityOfService(&self, quality_of_service: NSQualityOfService);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSTask")]
    unsafe impl NSTask {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSTaskConveniences
    #[cfg(feature = "Foundation_NSTask")]
    unsafe impl NSTask {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other launchedTaskWithExecutableURL:arguments:error:terminationHandler:)]
        pub unsafe fn launchedTaskWithExecutableURL_arguments_error_terminationHandler(
            url: &NSURL,
            arguments: &NSArray<NSString>,
            error: Option<&mut Option<Id<NSError>>>,
            termination_handler: Option<&Block<(NonNull<NSTask>,), ()>>,
        ) -> Option<Id<NSTask>>;

        #[method(waitUntilExit)]
        pub unsafe fn waitUntilExit(&self);
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSTask")]
    unsafe impl NSTask {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other launchPath)]
        pub unsafe fn launchPath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setLaunchPath:)]
        pub unsafe fn setLaunchPath(&self, launch_path: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other currentDirectoryPath)]
        pub unsafe fn currentDirectoryPath(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setCurrentDirectoryPath:)]
        pub unsafe fn setCurrentDirectoryPath(&self, current_directory_path: &NSString);

        #[deprecated]
        #[method(launch)]
        pub unsafe fn launch(&self);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other launchedTaskWithLaunchPath:arguments:)]
        pub unsafe fn launchedTaskWithLaunchPath_arguments(
            path: &NSString,
            arguments: &NSArray<NSString>,
        ) -> Id<NSTask>;
    }
);

extern_static!(NSTaskDidTerminateNotification: &'static NSNotificationName);
