//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSRelativeDateTimeFormatterStyle {
        NSRelativeDateTimeFormatterStyleNumeric = 0,
        NSRelativeDateTimeFormatterStyleNamed = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSRelativeDateTimeFormatterUnitsStyle {
        NSRelativeDateTimeFormatterUnitsStyleFull = 0,
        NSRelativeDateTimeFormatterUnitsStyleSpellOut = 1,
        NSRelativeDateTimeFormatterUnitsStyleShort = 2,
        NSRelativeDateTimeFormatterUnitsStyleAbbreviated = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSRelativeDateTimeFormatter")]
    pub struct NSRelativeDateTimeFormatter;

    #[cfg(feature = "Foundation_NSRelativeDateTimeFormatter")]
    unsafe impl ClassType for NSRelativeDateTimeFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSRelativeDateTimeFormatter")]
unsafe impl NSCoding for NSRelativeDateTimeFormatter {}

#[cfg(feature = "Foundation_NSRelativeDateTimeFormatter")]
unsafe impl NSCopying for NSRelativeDateTimeFormatter {}

#[cfg(feature = "Foundation_NSRelativeDateTimeFormatter")]
unsafe impl NSObjectProtocol for NSRelativeDateTimeFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSRelativeDateTimeFormatter")]
    unsafe impl NSRelativeDateTimeFormatter {
        #[method(dateTimeStyle)]
        pub unsafe fn dateTimeStyle(&self) -> NSRelativeDateTimeFormatterStyle;

        #[method(setDateTimeStyle:)]
        pub unsafe fn setDateTimeStyle(&self, date_time_style: NSRelativeDateTimeFormatterStyle);

        #[method(unitsStyle)]
        pub unsafe fn unitsStyle(&self) -> NSRelativeDateTimeFormatterUnitsStyle;

        #[method(setUnitsStyle:)]
        pub unsafe fn setUnitsStyle(&self, units_style: NSRelativeDateTimeFormatterUnitsStyle);

        #[method(formattingContext)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;

        #[method(setFormattingContext:)]
        pub unsafe fn setFormattingContext(&self, formatting_context: NSFormattingContext);

        #[cfg(feature = "Foundation_NSCalendar")]
        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Id<NSCalendar>;

        #[cfg(feature = "Foundation_NSCalendar")]
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[cfg(all(
            feature = "Foundation_NSDateComponents",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other localizedStringFromDateComponents:)]
        pub unsafe fn localizedStringFromDateComponents(
            &self,
            date_components: &NSDateComponents,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringFromTimeInterval:)]
        pub unsafe fn localizedStringFromTimeInterval(
            &self,
            time_interval: NSTimeInterval,
        ) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localizedStringForDate:relativeToDate:)]
        pub unsafe fn localizedStringForDate_relativeToDate(
            &self,
            date: &NSDate,
            reference_date: &NSDate,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(&self, obj: Option<&AnyObject>) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSRelativeDateTimeFormatter")]
    unsafe impl NSRelativeDateTimeFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
