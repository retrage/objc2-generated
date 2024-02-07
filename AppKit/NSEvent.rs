//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSEventType {
        NSEventTypeLeftMouseDown = 1,
        NSEventTypeLeftMouseUp = 2,
        NSEventTypeRightMouseDown = 3,
        NSEventTypeRightMouseUp = 4,
        NSEventTypeMouseMoved = 5,
        NSEventTypeLeftMouseDragged = 6,
        NSEventTypeRightMouseDragged = 7,
        NSEventTypeMouseEntered = 8,
        NSEventTypeMouseExited = 9,
        NSEventTypeKeyDown = 10,
        NSEventTypeKeyUp = 11,
        NSEventTypeFlagsChanged = 12,
        NSEventTypeAppKitDefined = 13,
        NSEventTypeSystemDefined = 14,
        NSEventTypeApplicationDefined = 15,
        NSEventTypePeriodic = 16,
        NSEventTypeCursorUpdate = 17,
        NSEventTypeScrollWheel = 22,
        NSEventTypeTabletPoint = 23,
        NSEventTypeTabletProximity = 24,
        NSEventTypeOtherMouseDown = 25,
        NSEventTypeOtherMouseUp = 26,
        NSEventTypeOtherMouseDragged = 27,
        NSEventTypeGesture = 29,
        NSEventTypeMagnify = 30,
        NSEventTypeSwipe = 31,
        NSEventTypeRotate = 18,
        NSEventTypeBeginGesture = 19,
        NSEventTypeEndGesture = 20,
        NSEventTypeSmartMagnify = 32,
        NSEventTypeQuickLook = 33,
        NSEventTypePressure = 34,
        NSEventTypeDirectTouch = 37,
        NSEventTypeChangeMode = 38,
    }
);

extern_static!(NSLeftMouseDown: NSEventType = NSEventType(NSEventTypeLeftMouseDown.0));

extern_static!(NSLeftMouseUp: NSEventType = NSEventType(NSEventTypeLeftMouseUp.0));

extern_static!(NSRightMouseDown: NSEventType = NSEventType(NSEventTypeRightMouseDown.0));

extern_static!(NSRightMouseUp: NSEventType = NSEventType(NSEventTypeRightMouseUp.0));

extern_static!(NSMouseMoved: NSEventType = NSEventType(NSEventTypeMouseMoved.0));

extern_static!(NSLeftMouseDragged: NSEventType = NSEventType(NSEventTypeLeftMouseDragged.0));

extern_static!(NSRightMouseDragged: NSEventType = NSEventType(NSEventTypeRightMouseDragged.0));

extern_static!(NSMouseEntered: NSEventType = NSEventType(NSEventTypeMouseEntered.0));

extern_static!(NSMouseExited: NSEventType = NSEventType(NSEventTypeMouseExited.0));

extern_static!(NSKeyDown: NSEventType = NSEventType(NSEventTypeKeyDown.0));

extern_static!(NSKeyUp: NSEventType = NSEventType(NSEventTypeKeyUp.0));

extern_static!(NSFlagsChanged: NSEventType = NSEventType(NSEventTypeFlagsChanged.0));

extern_static!(NSAppKitDefined: NSEventType = NSEventType(NSEventTypeAppKitDefined.0));

extern_static!(NSSystemDefined: NSEventType = NSEventType(NSEventTypeSystemDefined.0));

extern_static!(NSApplicationDefined: NSEventType = NSEventType(NSEventTypeApplicationDefined.0));

extern_static!(NSPeriodic: NSEventType = NSEventType(NSEventTypePeriodic.0));

extern_static!(NSCursorUpdate: NSEventType = NSEventType(NSEventTypeCursorUpdate.0));

extern_static!(NSScrollWheel: NSEventType = NSEventType(NSEventTypeScrollWheel.0));

extern_static!(NSTabletPoint: NSEventType = NSEventType(NSEventTypeTabletPoint.0));

extern_static!(NSTabletProximity: NSEventType = NSEventType(NSEventTypeTabletProximity.0));

extern_static!(NSOtherMouseDown: NSEventType = NSEventType(NSEventTypeOtherMouseDown.0));

extern_static!(NSOtherMouseUp: NSEventType = NSEventType(NSEventTypeOtherMouseUp.0));

extern_static!(NSOtherMouseDragged: NSEventType = NSEventType(NSEventTypeOtherMouseDragged.0));

ns_options!(
    #[underlying(c_ulonglong)]
    pub enum NSEventMask {
        NSEventMaskLeftMouseDown = 1 << NSEventTypeLeftMouseDown.0,
        NSEventMaskLeftMouseUp = 1 << NSEventTypeLeftMouseUp.0,
        NSEventMaskRightMouseDown = 1 << NSEventTypeRightMouseDown.0,
        NSEventMaskRightMouseUp = 1 << NSEventTypeRightMouseUp.0,
        NSEventMaskMouseMoved = 1 << NSEventTypeMouseMoved.0,
        NSEventMaskLeftMouseDragged = 1 << NSEventTypeLeftMouseDragged.0,
        NSEventMaskRightMouseDragged = 1 << NSEventTypeRightMouseDragged.0,
        NSEventMaskMouseEntered = 1 << NSEventTypeMouseEntered.0,
        NSEventMaskMouseExited = 1 << NSEventTypeMouseExited.0,
        NSEventMaskKeyDown = 1 << NSEventTypeKeyDown.0,
        NSEventMaskKeyUp = 1 << NSEventTypeKeyUp.0,
        NSEventMaskFlagsChanged = 1 << NSEventTypeFlagsChanged.0,
        NSEventMaskAppKitDefined = 1 << NSEventTypeAppKitDefined.0,
        NSEventMaskSystemDefined = 1 << NSEventTypeSystemDefined.0,
        NSEventMaskApplicationDefined = 1 << NSEventTypeApplicationDefined.0,
        NSEventMaskPeriodic = 1 << NSEventTypePeriodic.0,
        NSEventMaskCursorUpdate = 1 << NSEventTypeCursorUpdate.0,
        NSEventMaskScrollWheel = 1 << NSEventTypeScrollWheel.0,
        NSEventMaskTabletPoint = 1 << NSEventTypeTabletPoint.0,
        NSEventMaskTabletProximity = 1 << NSEventTypeTabletProximity.0,
        NSEventMaskOtherMouseDown = 1 << NSEventTypeOtherMouseDown.0,
        NSEventMaskOtherMouseUp = 1 << NSEventTypeOtherMouseUp.0,
        NSEventMaskOtherMouseDragged = 1 << NSEventTypeOtherMouseDragged.0,
        NSEventMaskGesture = 1 << NSEventTypeGesture.0,
        NSEventMaskMagnify = 1 << NSEventTypeMagnify.0,
        NSEventMaskSwipe = 1 << NSEventTypeSwipe.0,
        NSEventMaskRotate = 1 << NSEventTypeRotate.0,
        NSEventMaskBeginGesture = 1 << NSEventTypeBeginGesture.0,
        NSEventMaskEndGesture = 1 << NSEventTypeEndGesture.0,
        NSEventMaskSmartMagnify = 1 << NSEventTypeSmartMagnify.0,
        NSEventMaskPressure = 1 << NSEventTypePressure.0,
        NSEventMaskDirectTouch = 1 << NSEventTypeDirectTouch.0,
        NSEventMaskChangeMode = 1 << NSEventTypeChangeMode.0,
        NSEventMaskAny = NSUIntegerMax as _,
    }
);

extern_static!(NSLeftMouseDownMask: NSEventMask = NSEventMask(NSEventMaskLeftMouseDown.0));

extern_static!(NSLeftMouseUpMask: NSEventMask = NSEventMask(NSEventMaskLeftMouseUp.0));

extern_static!(NSRightMouseDownMask: NSEventMask = NSEventMask(NSEventMaskRightMouseDown.0));

extern_static!(NSRightMouseUpMask: NSEventMask = NSEventMask(NSEventMaskRightMouseUp.0));

extern_static!(NSMouseMovedMask: NSEventMask = NSEventMask(NSEventMaskMouseMoved.0));

extern_static!(NSLeftMouseDraggedMask: NSEventMask = NSEventMask(NSEventMaskLeftMouseDragged.0));

extern_static!(NSRightMouseDraggedMask: NSEventMask = NSEventMask(NSEventMaskRightMouseDragged.0));

extern_static!(NSMouseEnteredMask: NSEventMask = NSEventMask(NSEventMaskMouseEntered.0));

extern_static!(NSMouseExitedMask: NSEventMask = NSEventMask(NSEventMaskMouseExited.0));

extern_static!(NSKeyDownMask: NSEventMask = NSEventMask(NSEventMaskKeyDown.0));

extern_static!(NSKeyUpMask: NSEventMask = NSEventMask(NSEventMaskKeyUp.0));

extern_static!(NSFlagsChangedMask: NSEventMask = NSEventMask(NSEventMaskFlagsChanged.0));

extern_static!(NSAppKitDefinedMask: NSEventMask = NSEventMask(NSEventMaskAppKitDefined.0));

extern_static!(NSSystemDefinedMask: NSEventMask = NSEventMask(NSEventMaskSystemDefined.0));

extern_static!(NSApplicationDefinedMask: NSEventMask = NSEventMask(NSEventMaskApplicationDefined.0));

extern_static!(NSPeriodicMask: NSEventMask = NSEventMask(NSEventMaskPeriodic.0));

extern_static!(NSCursorUpdateMask: NSEventMask = NSEventMask(NSEventMaskCursorUpdate.0));

extern_static!(NSScrollWheelMask: NSEventMask = NSEventMask(NSEventMaskScrollWheel.0));

extern_static!(NSTabletPointMask: NSEventMask = NSEventMask(NSEventMaskTabletPoint.0));

extern_static!(NSTabletProximityMask: NSEventMask = NSEventMask(NSEventMaskTabletProximity.0));

extern_static!(NSOtherMouseDownMask: NSEventMask = NSEventMask(NSEventMaskOtherMouseDown.0));

extern_static!(NSOtherMouseUpMask: NSEventMask = NSEventMask(NSEventMaskOtherMouseUp.0));

extern_static!(NSOtherMouseDraggedMask: NSEventMask = NSEventMask(NSEventMaskOtherMouseDragged.0));

extern_static!(NSAnyEventMask: NSEventMask = NSEventMask(NSUIntegerMax as _));

inline_fn!(
    pub unsafe fn NSEventMaskFromType(r#type: NSEventType) -> NSEventMask {
        todo!()
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSEventModifierFlags {
        NSEventModifierFlagCapsLock = 1 << 16,
        NSEventModifierFlagShift = 1 << 17,
        NSEventModifierFlagControl = 1 << 18,
        NSEventModifierFlagOption = 1 << 19,
        NSEventModifierFlagCommand = 1 << 20,
        NSEventModifierFlagNumericPad = 1 << 21,
        NSEventModifierFlagHelp = 1 << 22,
        NSEventModifierFlagFunction = 1 << 23,
        NSEventModifierFlagDeviceIndependentFlagsMask = 0xffff0000,
    }
);

extern_static!(NSAlphaShiftKeyMask: NSEventModifierFlags = NSEventModifierFlags(NSEventModifierFlagCapsLock.0));

extern_static!(NSShiftKeyMask: NSEventModifierFlags = NSEventModifierFlags(NSEventModifierFlagShift.0));

extern_static!(NSControlKeyMask: NSEventModifierFlags = NSEventModifierFlags(NSEventModifierFlagControl.0));

extern_static!(NSAlternateKeyMask: NSEventModifierFlags = NSEventModifierFlags(NSEventModifierFlagOption.0));

extern_static!(NSCommandKeyMask: NSEventModifierFlags = NSEventModifierFlags(NSEventModifierFlagCommand.0));

extern_static!(NSNumericPadKeyMask: NSEventModifierFlags = NSEventModifierFlags(NSEventModifierFlagNumericPad.0));

extern_static!(NSHelpKeyMask: NSEventModifierFlags = NSEventModifierFlags(NSEventModifierFlagHelp.0));

extern_static!(NSFunctionKeyMask: NSEventModifierFlags = NSEventModifierFlags(NSEventModifierFlagFunction.0));

extern_static!(NSDeviceIndependentModifierFlagsMask: NSEventModifierFlags = NSEventModifierFlags(NSEventModifierFlagDeviceIndependentFlagsMask.0));

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSPointingDeviceType {
        NSPointingDeviceTypeUnknown = 0,
        NSPointingDeviceTypePen = 1,
        NSPointingDeviceTypeCursor = 2,
        NSPointingDeviceTypeEraser = 3,
    }
);

extern_static!(NSUnknownPointingDevice: NSPointingDeviceType = NSPointingDeviceType(NSPointingDeviceTypeUnknown.0));

extern_static!(NSPenPointingDevice: NSPointingDeviceType = NSPointingDeviceType(NSPointingDeviceTypePen.0));

extern_static!(NSCursorPointingDevice: NSPointingDeviceType = NSPointingDeviceType(NSPointingDeviceTypeCursor.0));

extern_static!(NSEraserPointingDevice: NSPointingDeviceType = NSPointingDeviceType(NSPointingDeviceTypeEraser.0));

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSEventButtonMask {
        NSEventButtonMaskPenTip = 1,
        NSEventButtonMaskPenLowerSide = 2,
        NSEventButtonMaskPenUpperSide = 4,
    }
);

extern_static!(NSPenTipMask: NSEventButtonMask = NSEventButtonMask(NSEventButtonMaskPenTip.0));

extern_static!(NSPenLowerSideMask: NSEventButtonMask = NSEventButtonMask(NSEventButtonMaskPenLowerSide.0));

extern_static!(NSPenUpperSideMask: NSEventButtonMask = NSEventButtonMask(NSEventButtonMaskPenUpperSide.0));

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSEventPhase {
        NSEventPhaseNone = 0,
        NSEventPhaseBegan = 0x1 << 0,
        NSEventPhaseStationary = 0x1 << 1,
        NSEventPhaseChanged = 0x1 << 2,
        NSEventPhaseEnded = 0x1 << 3,
        NSEventPhaseCancelled = 0x1 << 4,
        NSEventPhaseMayBegin = 0x1 << 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSEventGestureAxis {
        NSEventGestureAxisNone = 0,
        NSEventGestureAxisHorizontal = 1,
        NSEventGestureAxisVertical = 2,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSEventSwipeTrackingOptions {
        NSEventSwipeTrackingLockDirection = 0x1 << 0,
        NSEventSwipeTrackingClampGestureAmount = 0x1 << 1,
    }
);

ns_enum!(
    #[underlying(c_short)]
    pub enum NSEventSubtype {
        NSEventSubtypeWindowExposed = 0,
        NSEventSubtypeApplicationActivated = 1,
        NSEventSubtypeApplicationDeactivated = 2,
        NSEventSubtypeWindowMoved = 4,
        NSEventSubtypeScreenChanged = 8,
        NSEventSubtypePowerOff = 1,
        NSEventSubtypeMouseEvent = 0,
        NSEventSubtypeTabletPoint = 1,
        NSEventSubtypeTabletProximity = 2,
        NSEventSubtypeTouch = 3,
    }
);

extern_static!(NSWindowExposedEventType: NSEventSubtype = NSEventSubtype(NSEventSubtypeWindowExposed.0));

extern_static!(NSApplicationActivatedEventType: NSEventSubtype = NSEventSubtype(NSEventSubtypeApplicationActivated.0));

extern_static!(NSApplicationDeactivatedEventType: NSEventSubtype = NSEventSubtype(NSEventSubtypeApplicationDeactivated.0));

extern_static!(NSWindowMovedEventType: NSEventSubtype = NSEventSubtype(NSEventSubtypeWindowMoved.0));

extern_static!(NSScreenChangedEventType: NSEventSubtype = NSEventSubtype(NSEventSubtypeScreenChanged.0));

extern_static!(NSAWTEventType: NSEventSubtype = NSEventSubtype(16));

extern_static!(NSPowerOffEventType: NSEventSubtype = NSEventSubtype(NSEventSubtypePowerOff.0));

extern_static!(NSMouseEventSubtype: NSEventSubtype = NSEventSubtype(NSEventSubtypeMouseEvent.0));

extern_static!(NSTabletPointEventSubtype: NSEventSubtype = NSEventSubtype(NSEventSubtypeTabletPoint.0));

extern_static!(NSTabletProximityEventSubtype: NSEventSubtype = NSEventSubtype(NSEventSubtypeTabletProximity.0));

extern_static!(NSTouchEventSubtype: NSEventSubtype = NSEventSubtype(NSEventSubtypeTouch.0));

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPressureBehavior {
        NSPressureBehaviorUnknown = -1,
        NSPressureBehaviorPrimaryDefault = 0,
        NSPressureBehaviorPrimaryClick = 1,
        NSPressureBehaviorPrimaryGeneric = 2,
        NSPressureBehaviorPrimaryAccelerator = 3,
        NSPressureBehaviorPrimaryDeepClick = 5,
        NSPressureBehaviorPrimaryDeepDrag = 6,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSEvent")]
    pub struct NSEvent;

    #[cfg(feature = "AppKit_NSEvent")]
    unsafe impl ClassType for NSEvent {
        type Super = NSObject;
        type Mutability = Immutable;
    }
);

#[cfg(feature = "AppKit_NSEvent")]
unsafe impl NSCoding for NSEvent {}

#[cfg(feature = "AppKit_NSEvent")]
unsafe impl NSCopying for NSEvent {}

#[cfg(feature = "AppKit_NSEvent")]
unsafe impl NSObjectProtocol for NSEvent {}

extern_methods!(
    #[cfg(feature = "AppKit_NSEvent")]
    unsafe impl NSEvent {
        #[method(type)]
        pub unsafe fn r#type(&self) -> NSEventType;

        #[method(modifierFlags)]
        pub unsafe fn modifierFlags(&self) -> NSEventModifierFlags;

        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> NSTimeInterval;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other window)]
        pub unsafe fn window(&self, mtm: MainThreadMarker) -> Option<Id<NSWindow>>;

        #[method(windowNumber)]
        pub unsafe fn windowNumber(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSGraphicsContext")]
        #[deprecated = "This method always returns nil. If you need access to the current drawing context, use [NSGraphicsContext currentContext] inside of a draw operation."]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Option<Id<NSGraphicsContext>>;

        #[method(clickCount)]
        pub unsafe fn clickCount(&self) -> NSInteger;

        #[method(buttonNumber)]
        pub unsafe fn buttonNumber(&self) -> NSInteger;

        #[method(eventNumber)]
        pub unsafe fn eventNumber(&self) -> NSInteger;

        #[method(pressure)]
        pub unsafe fn pressure(&self) -> c_float;

        #[method(locationInWindow)]
        pub unsafe fn locationInWindow(&self) -> NSPoint;

        #[method(deltaX)]
        pub unsafe fn deltaX(&self) -> CGFloat;

        #[method(deltaY)]
        pub unsafe fn deltaY(&self) -> CGFloat;

        #[method(deltaZ)]
        pub unsafe fn deltaZ(&self) -> CGFloat;

        #[method(hasPreciseScrollingDeltas)]
        pub unsafe fn hasPreciseScrollingDeltas(&self) -> bool;

        #[method(scrollingDeltaX)]
        pub unsafe fn scrollingDeltaX(&self) -> CGFloat;

        #[method(scrollingDeltaY)]
        pub unsafe fn scrollingDeltaY(&self) -> CGFloat;

        #[method(momentumPhase)]
        pub unsafe fn momentumPhase(&self) -> NSEventPhase;

        #[method(isDirectionInvertedFromDevice)]
        pub unsafe fn isDirectionInvertedFromDevice(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other characters)]
        pub unsafe fn characters(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other charactersIgnoringModifiers)]
        pub unsafe fn charactersIgnoringModifiers(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other charactersByApplyingModifiers:)]
        pub unsafe fn charactersByApplyingModifiers(
            &self,
            modifiers: NSEventModifierFlags,
        ) -> Option<Id<NSString>>;

        #[method(isARepeat)]
        pub unsafe fn isARepeat(&self) -> bool;

        #[method(keyCode)]
        pub unsafe fn keyCode(&self) -> c_ushort;

        #[method(trackingNumber)]
        pub unsafe fn trackingNumber(&self) -> NSInteger;

        #[method(userData)]
        pub unsafe fn userData(&self) -> *mut c_void;

        #[cfg(feature = "AppKit_NSTrackingArea")]
        #[method_id(@__retain_semantics Other trackingArea)]
        pub unsafe fn trackingArea(&self) -> Option<Id<NSTrackingArea>>;

        #[method(subtype)]
        pub unsafe fn subtype(&self) -> NSEventSubtype;

        #[method(data1)]
        pub unsafe fn data1(&self) -> NSInteger;

        #[method(data2)]
        pub unsafe fn data2(&self) -> NSInteger;

        #[method(eventRef)]
        pub unsafe fn eventRef(&self) -> *mut c_void;

        #[method_id(@__retain_semantics Other eventWithEventRef:)]
        pub unsafe fn eventWithEventRef(event_ref: NonNull<c_void>) -> Option<Id<NSEvent>>;

        #[method(isMouseCoalescingEnabled)]
        pub unsafe fn isMouseCoalescingEnabled() -> bool;

        #[method(setMouseCoalescingEnabled:)]
        pub unsafe fn setMouseCoalescingEnabled(mouse_coalescing_enabled: bool);

        #[method(magnification)]
        pub unsafe fn magnification(&self) -> CGFloat;

        #[method(deviceID)]
        pub unsafe fn deviceID(&self) -> NSUInteger;

        #[method(rotation)]
        pub unsafe fn rotation(&self) -> c_float;

        #[method(absoluteX)]
        pub unsafe fn absoluteX(&self) -> NSInteger;

        #[method(absoluteY)]
        pub unsafe fn absoluteY(&self) -> NSInteger;

        #[method(absoluteZ)]
        pub unsafe fn absoluteZ(&self) -> NSInteger;

        #[method(buttonMask)]
        pub unsafe fn buttonMask(&self) -> NSEventButtonMask;

        #[method(tilt)]
        pub unsafe fn tilt(&self) -> NSPoint;

        #[method(tangentialPressure)]
        pub unsafe fn tangentialPressure(&self) -> c_float;

        #[method_id(@__retain_semantics Other vendorDefined)]
        pub unsafe fn vendorDefined(&self) -> Id<AnyObject>;

        #[method(vendorID)]
        pub unsafe fn vendorID(&self) -> NSUInteger;

        #[method(tabletID)]
        pub unsafe fn tabletID(&self) -> NSUInteger;

        #[method(pointingDeviceID)]
        pub unsafe fn pointingDeviceID(&self) -> NSUInteger;

        #[method(systemTabletID)]
        pub unsafe fn systemTabletID(&self) -> NSUInteger;

        #[method(vendorPointingDeviceType)]
        pub unsafe fn vendorPointingDeviceType(&self) -> NSUInteger;

        #[method(pointingDeviceSerialNumber)]
        pub unsafe fn pointingDeviceSerialNumber(&self) -> NSUInteger;

        #[method(uniqueID)]
        pub unsafe fn uniqueID(&self) -> c_ulonglong;

        #[method(capabilityMask)]
        pub unsafe fn capabilityMask(&self) -> NSUInteger;

        #[method(pointingDeviceType)]
        pub unsafe fn pointingDeviceType(&self) -> NSPointingDeviceType;

        #[method(isEnteringProximity)]
        pub unsafe fn isEnteringProximity(&self) -> bool;

        #[cfg(all(
            feature = "AppKit_NSTouch",
            feature = "AppKit_NSView",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other touchesMatchingPhase:inView:)]
        pub unsafe fn touchesMatchingPhase_inView(
            &self,
            phase: NSTouchPhase,
            view: Option<&NSView>,
        ) -> Id<NSSet<NSTouch>>;

        #[cfg(all(feature = "AppKit_NSTouch", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other allTouches)]
        pub unsafe fn allTouches(&self) -> Id<NSSet<NSTouch>>;

        #[cfg(all(
            feature = "AppKit_NSTouch",
            feature = "AppKit_NSView",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other touchesForView:)]
        pub unsafe fn touchesForView(&self, view: &NSView) -> Id<NSSet<NSTouch>>;

        #[cfg(all(feature = "AppKit_NSTouch", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other coalescedTouchesForTouch:)]
        pub unsafe fn coalescedTouchesForTouch(&self, touch: &NSTouch) -> Id<NSArray<NSTouch>>;

        #[method(phase)]
        pub unsafe fn phase(&self) -> NSEventPhase;

        #[method(stage)]
        pub unsafe fn stage(&self) -> NSInteger;

        #[method(stageTransition)]
        pub unsafe fn stageTransition(&self) -> CGFloat;

        #[method(associatedEventsMask)]
        pub unsafe fn associatedEventsMask(&self) -> NSEventMask;

        #[method(pressureBehavior)]
        pub unsafe fn pressureBehavior(&self) -> NSPressureBehavior;

        #[method(isSwipeTrackingFromScrollEventsEnabled)]
        pub unsafe fn isSwipeTrackingFromScrollEventsEnabled() -> bool;

        #[method(trackSwipeEventWithOptions:dampenAmountThresholdMin:max:usingHandler:)]
        pub unsafe fn trackSwipeEventWithOptions_dampenAmountThresholdMin_max_usingHandler(
            &self,
            options: NSEventSwipeTrackingOptions,
            min_dampen_threshold: CGFloat,
            max_dampen_threshold: CGFloat,
            tracking_handler: &Block<dyn Fn(CGFloat, NSEventPhase, Bool, NonNull<Bool>)>,
        );

        #[method(startPeriodicEventsAfterDelay:withPeriod:)]
        pub unsafe fn startPeriodicEventsAfterDelay_withPeriod(
            delay: NSTimeInterval,
            period: NSTimeInterval,
        );

        #[method(stopPeriodicEvents)]
        pub unsafe fn stopPeriodicEvents();

        #[cfg(feature = "AppKit_NSGraphicsContext")]
        #[method_id(@__retain_semantics Other mouseEventWithType:location:modifierFlags:timestamp:windowNumber:context:eventNumber:clickCount:pressure:)]
        pub unsafe fn mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure(
            r#type: NSEventType,
            location: NSPoint,
            flags: NSEventModifierFlags,
            time: NSTimeInterval,
            w_num: NSInteger,
            unused_pass_nil: Option<&NSGraphicsContext>,
            e_num: NSInteger,
            c_num: NSInteger,
            pressure: c_float,
        ) -> Option<Id<NSEvent>>;

        #[cfg(all(feature = "AppKit_NSGraphicsContext", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other keyEventWithType:location:modifierFlags:timestamp:windowNumber:context:characters:charactersIgnoringModifiers:isARepeat:keyCode:)]
        pub unsafe fn keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
            r#type: NSEventType,
            location: NSPoint,
            flags: NSEventModifierFlags,
            time: NSTimeInterval,
            w_num: NSInteger,
            unused_pass_nil: Option<&NSGraphicsContext>,
            keys: &NSString,
            ukeys: &NSString,
            flag: bool,
            code: c_ushort,
        ) -> Option<Id<NSEvent>>;

        #[cfg(feature = "AppKit_NSGraphicsContext")]
        #[method_id(@__retain_semantics Other enterExitEventWithType:location:modifierFlags:timestamp:windowNumber:context:eventNumber:trackingNumber:userData:)]
        pub unsafe fn enterExitEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_trackingNumber_userData(
            r#type: NSEventType,
            location: NSPoint,
            flags: NSEventModifierFlags,
            time: NSTimeInterval,
            w_num: NSInteger,
            unused_pass_nil: Option<&NSGraphicsContext>,
            e_num: NSInteger,
            t_num: NSInteger,
            data: *mut c_void,
        ) -> Option<Id<NSEvent>>;

        #[cfg(feature = "AppKit_NSGraphicsContext")]
        #[method_id(@__retain_semantics Other otherEventWithType:location:modifierFlags:timestamp:windowNumber:context:subtype:data1:data2:)]
        pub unsafe fn otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2(
            r#type: NSEventType,
            location: NSPoint,
            flags: NSEventModifierFlags,
            time: NSTimeInterval,
            w_num: NSInteger,
            unused_pass_nil: Option<&NSGraphicsContext>,
            subtype: c_short,
            d1: NSInteger,
            d2: NSInteger,
        ) -> Option<Id<NSEvent>>;

        #[method(mouseLocation)]
        pub unsafe fn mouseLocation() -> NSPoint;

        #[method(modifierFlags)]
        pub unsafe fn modifierFlags_class() -> NSEventModifierFlags;

        #[method(pressedMouseButtons)]
        pub unsafe fn pressedMouseButtons() -> NSUInteger;

        #[method(doubleClickInterval)]
        pub unsafe fn doubleClickInterval() -> NSTimeInterval;

        #[method(keyRepeatDelay)]
        pub unsafe fn keyRepeatDelay() -> NSTimeInterval;

        #[method(keyRepeatInterval)]
        pub unsafe fn keyRepeatInterval() -> NSTimeInterval;

        #[method_id(@__retain_semantics Other addGlobalMonitorForEventsMatchingMask:handler:)]
        pub unsafe fn addGlobalMonitorForEventsMatchingMask_handler(
            mask: NSEventMask,
            block: &Block<dyn Fn(NonNull<NSEvent>)>,
        ) -> Option<Id<AnyObject>>;

        #[method_id(@__retain_semantics Other addLocalMonitorForEventsMatchingMask:handler:)]
        pub unsafe fn addLocalMonitorForEventsMatchingMask_handler(
            mask: NSEventMask,
            block: &Block<dyn Fn(NonNull<NSEvent>) -> *mut NSEvent>,
        ) -> Option<Id<AnyObject>>;

        #[method(removeMonitor:)]
        pub unsafe fn removeMonitor(event_monitor: &AnyObject);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSEvent")]
    unsafe impl NSEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

pub const NSUpArrowFunctionKey: c_uint = 0xF700;
pub const NSDownArrowFunctionKey: c_uint = 0xF701;
pub const NSLeftArrowFunctionKey: c_uint = 0xF702;
pub const NSRightArrowFunctionKey: c_uint = 0xF703;
pub const NSF1FunctionKey: c_uint = 0xF704;
pub const NSF2FunctionKey: c_uint = 0xF705;
pub const NSF3FunctionKey: c_uint = 0xF706;
pub const NSF4FunctionKey: c_uint = 0xF707;
pub const NSF5FunctionKey: c_uint = 0xF708;
pub const NSF6FunctionKey: c_uint = 0xF709;
pub const NSF7FunctionKey: c_uint = 0xF70A;
pub const NSF8FunctionKey: c_uint = 0xF70B;
pub const NSF9FunctionKey: c_uint = 0xF70C;
pub const NSF10FunctionKey: c_uint = 0xF70D;
pub const NSF11FunctionKey: c_uint = 0xF70E;
pub const NSF12FunctionKey: c_uint = 0xF70F;
pub const NSF13FunctionKey: c_uint = 0xF710;
pub const NSF14FunctionKey: c_uint = 0xF711;
pub const NSF15FunctionKey: c_uint = 0xF712;
pub const NSF16FunctionKey: c_uint = 0xF713;
pub const NSF17FunctionKey: c_uint = 0xF714;
pub const NSF18FunctionKey: c_uint = 0xF715;
pub const NSF19FunctionKey: c_uint = 0xF716;
pub const NSF20FunctionKey: c_uint = 0xF717;
pub const NSF21FunctionKey: c_uint = 0xF718;
pub const NSF22FunctionKey: c_uint = 0xF719;
pub const NSF23FunctionKey: c_uint = 0xF71A;
pub const NSF24FunctionKey: c_uint = 0xF71B;
pub const NSF25FunctionKey: c_uint = 0xF71C;
pub const NSF26FunctionKey: c_uint = 0xF71D;
pub const NSF27FunctionKey: c_uint = 0xF71E;
pub const NSF28FunctionKey: c_uint = 0xF71F;
pub const NSF29FunctionKey: c_uint = 0xF720;
pub const NSF30FunctionKey: c_uint = 0xF721;
pub const NSF31FunctionKey: c_uint = 0xF722;
pub const NSF32FunctionKey: c_uint = 0xF723;
pub const NSF33FunctionKey: c_uint = 0xF724;
pub const NSF34FunctionKey: c_uint = 0xF725;
pub const NSF35FunctionKey: c_uint = 0xF726;
pub const NSInsertFunctionKey: c_uint = 0xF727;
pub const NSDeleteFunctionKey: c_uint = 0xF728;
pub const NSHomeFunctionKey: c_uint = 0xF729;
pub const NSBeginFunctionKey: c_uint = 0xF72A;
pub const NSEndFunctionKey: c_uint = 0xF72B;
pub const NSPageUpFunctionKey: c_uint = 0xF72C;
pub const NSPageDownFunctionKey: c_uint = 0xF72D;
pub const NSPrintScreenFunctionKey: c_uint = 0xF72E;
pub const NSScrollLockFunctionKey: c_uint = 0xF72F;
pub const NSPauseFunctionKey: c_uint = 0xF730;
pub const NSSysReqFunctionKey: c_uint = 0xF731;
pub const NSBreakFunctionKey: c_uint = 0xF732;
pub const NSResetFunctionKey: c_uint = 0xF733;
pub const NSStopFunctionKey: c_uint = 0xF734;
pub const NSMenuFunctionKey: c_uint = 0xF735;
pub const NSUserFunctionKey: c_uint = 0xF736;
pub const NSSystemFunctionKey: c_uint = 0xF737;
pub const NSPrintFunctionKey: c_uint = 0xF738;
pub const NSClearLineFunctionKey: c_uint = 0xF739;
pub const NSClearDisplayFunctionKey: c_uint = 0xF73A;
pub const NSInsertLineFunctionKey: c_uint = 0xF73B;
pub const NSDeleteLineFunctionKey: c_uint = 0xF73C;
pub const NSInsertCharFunctionKey: c_uint = 0xF73D;
pub const NSDeleteCharFunctionKey: c_uint = 0xF73E;
pub const NSPrevFunctionKey: c_uint = 0xF73F;
pub const NSNextFunctionKey: c_uint = 0xF740;
pub const NSSelectFunctionKey: c_uint = 0xF741;
pub const NSExecuteFunctionKey: c_uint = 0xF742;
pub const NSUndoFunctionKey: c_uint = 0xF743;
pub const NSRedoFunctionKey: c_uint = 0xF744;
pub const NSFindFunctionKey: c_uint = 0xF745;
pub const NSHelpFunctionKey: c_uint = 0xF746;
pub const NSModeSwitchFunctionKey: c_uint = 0xF747;
