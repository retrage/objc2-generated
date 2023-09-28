//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSDatePicker")]
    pub struct NSDatePicker;

    #[cfg(feature = "AppKit_NSDatePicker")]
    unsafe impl ClassType for NSDatePicker {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSDatePicker")]
unsafe impl NSAccessibility for NSDatePicker {}

#[cfg(feature = "AppKit_NSDatePicker")]
unsafe impl NSAccessibilityElementProtocol for NSDatePicker {}

#[cfg(feature = "AppKit_NSDatePicker")]
unsafe impl NSAnimatablePropertyContainer for NSDatePicker {}

#[cfg(feature = "AppKit_NSDatePicker")]
unsafe impl NSAppearanceCustomization for NSDatePicker {}

#[cfg(feature = "AppKit_NSDatePicker")]
unsafe impl NSCoding for NSDatePicker {}

#[cfg(feature = "AppKit_NSDatePicker")]
unsafe impl NSDraggingDestination for NSDatePicker {}

#[cfg(feature = "AppKit_NSDatePicker")]
unsafe impl NSObjectProtocol for NSDatePicker {}

#[cfg(feature = "AppKit_NSDatePicker")]
unsafe impl NSUserInterfaceItemIdentification for NSDatePicker {}

extern_methods!(
    #[cfg(feature = "AppKit_NSDatePicker")]
    unsafe impl NSDatePicker {
        #[method(datePickerStyle)]
        pub unsafe fn datePickerStyle(&self) -> NSDatePickerStyle;

        #[method(setDatePickerStyle:)]
        pub unsafe fn setDatePickerStyle(&self, date_picker_style: NSDatePickerStyle);

        #[method(isBezeled)]
        pub unsafe fn isBezeled(&self) -> bool;

        #[method(setBezeled:)]
        pub unsafe fn setBezeled(&self, bezeled: bool);

        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;

        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, text_color: &NSColor);

        #[method(datePickerMode)]
        pub unsafe fn datePickerMode(&self) -> NSDatePickerMode;

        #[method(setDatePickerMode:)]
        pub unsafe fn setDatePickerMode(&self, date_picker_mode: NSDatePickerMode);

        #[method(datePickerElements)]
        pub unsafe fn datePickerElements(&self) -> NSDatePickerElementFlags;

        #[method(setDatePickerElements:)]
        pub unsafe fn setDatePickerElements(&self, date_picker_elements: NSDatePickerElementFlags);

        #[cfg(feature = "Foundation_NSCalendar")]
        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Option<Id<NSCalendar>>;

        #[cfg(feature = "Foundation_NSCalendar")]
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Option<Id<NSLocale>>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Id<NSTimeZone>>;

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other dateValue)]
        pub unsafe fn dateValue(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setDateValue:)]
        pub unsafe fn setDateValue(&self, date_value: &NSDate);

        #[method(timeInterval)]
        pub unsafe fn timeInterval(&self) -> NSTimeInterval;

        #[method(setTimeInterval:)]
        pub unsafe fn setTimeInterval(&self, time_interval: NSTimeInterval);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other minDate)]
        pub unsafe fn minDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setMinDate:)]
        pub unsafe fn setMinDate(&self, min_date: Option<&NSDate>);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other maxDate)]
        pub unsafe fn maxDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setMaxDate:)]
        pub unsafe fn setMaxDate(&self, max_date: Option<&NSDate>);

        #[method(presentsCalendarOverlay)]
        pub unsafe fn presentsCalendarOverlay(&self) -> bool;

        #[method(setPresentsCalendarOverlay:)]
        pub unsafe fn setPresentsCalendarOverlay(&self, presents_calendar_overlay: bool);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSDatePickerCellDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSDatePickerCellDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSDatePicker")]
    unsafe impl NSDatePicker {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSDatePicker")]
    unsafe impl NSDatePicker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSDatePicker")]
    unsafe impl NSDatePicker {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
