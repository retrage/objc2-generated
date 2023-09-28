//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSCalendarDate")]
    #[deprecated = "Use NSCalendar and NSDateComponents and NSDateFormatter instead"]
    pub struct NSCalendarDate;

    #[cfg(feature = "Foundation_NSCalendarDate")]
    unsafe impl ClassType for NSCalendarDate {
        #[inherits(NSObject)]
        type Super = NSDate;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSCalendarDate")]
unsafe impl NSCoding for NSCalendarDate {}

#[cfg(feature = "Foundation_NSCalendarDate")]
unsafe impl NSCopying for NSCalendarDate {}

#[cfg(feature = "Foundation_NSCalendarDate")]
unsafe impl NSObjectProtocol for NSCalendarDate {}

#[cfg(feature = "Foundation_NSCalendarDate")]
unsafe impl NSSecureCoding for NSCalendarDate {}

extern_methods!(
    #[cfg(feature = "Foundation_NSCalendarDate")]
    unsafe impl NSCalendarDate {
        #[deprecated = "Use NSCalendar instead"]
        #[method_id(@__retain_semantics Other calendarDate)]
        pub unsafe fn calendarDate() -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSDateFormatter instead"]
        #[method_id(@__retain_semantics Other dateWithString:calendarFormat:locale:)]
        pub unsafe fn dateWithString_calendarFormat_locale(
            description: &NSString,
            format: &NSString,
            locale: Option<&AnyObject>,
        ) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSDateFormatter instead"]
        #[method_id(@__retain_semantics Other dateWithString:calendarFormat:)]
        pub unsafe fn dateWithString_calendarFormat(
            description: &NSString,
            format: &NSString,
        ) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[deprecated = "Use NSCalendar and NSDateComponents instead"]
        #[method_id(@__retain_semantics Other dateWithYear:month:day:hour:minute:second:timeZone:)]
        pub unsafe fn dateWithYear_month_day_hour_minute_second_timeZone(
            year: NSInteger,
            month: NSUInteger,
            day: NSUInteger,
            hour: NSUInteger,
            minute: NSUInteger,
            second: NSUInteger,
            a_time_zone: Option<&NSTimeZone>,
        ) -> Id<AnyObject>;

        #[deprecated = "Use NSCalendar instead"]
        #[method_id(@__retain_semantics Other dateByAddingYears:months:days:hours:minutes:seconds:)]
        pub unsafe fn dateByAddingYears_months_days_hours_minutes_seconds(
            &self,
            year: NSInteger,
            month: NSInteger,
            day: NSInteger,
            hour: NSInteger,
            minute: NSInteger,
            second: NSInteger,
        ) -> Id<NSCalendarDate>;

        #[deprecated]
        #[method(dayOfCommonEra)]
        pub unsafe fn dayOfCommonEra(&self) -> NSInteger;

        #[deprecated]
        #[method(dayOfMonth)]
        pub unsafe fn dayOfMonth(&self) -> NSInteger;

        #[deprecated]
        #[method(dayOfWeek)]
        pub unsafe fn dayOfWeek(&self) -> NSInteger;

        #[deprecated]
        #[method(dayOfYear)]
        pub unsafe fn dayOfYear(&self) -> NSInteger;

        #[deprecated]
        #[method(hourOfDay)]
        pub unsafe fn hourOfDay(&self) -> NSInteger;

        #[deprecated]
        #[method(minuteOfHour)]
        pub unsafe fn minuteOfHour(&self) -> NSInteger;

        #[deprecated]
        #[method(monthOfYear)]
        pub unsafe fn monthOfYear(&self) -> NSInteger;

        #[deprecated]
        #[method(secondOfMinute)]
        pub unsafe fn secondOfMinute(&self) -> NSInteger;

        #[deprecated]
        #[method(yearOfCommonEra)]
        pub unsafe fn yearOfCommonEra(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other calendarFormat)]
        pub unsafe fn calendarFormat(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptionWithCalendarFormat:locale:)]
        pub unsafe fn descriptionWithCalendarFormat_locale(
            &self,
            format: &NSString,
            locale: Option<&AnyObject>,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptionWithCalendarFormat:)]
        pub unsafe fn descriptionWithCalendarFormat(&self, format: &NSString) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&AnyObject>) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[deprecated]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Id<NSTimeZone>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSDateFormatter instead"]
        #[method_id(@__retain_semantics Init initWithString:calendarFormat:locale:)]
        pub unsafe fn initWithString_calendarFormat_locale(
            this: Allocated<Self>,
            description: &NSString,
            format: &NSString,
            locale: Option<&AnyObject>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSDateFormatter instead"]
        #[method_id(@__retain_semantics Init initWithString:calendarFormat:)]
        pub unsafe fn initWithString_calendarFormat(
            this: Allocated<Self>,
            description: &NSString,
            format: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSDateFormatter instead"]
        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Allocated<Self>,
            description: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[deprecated = "Use NSCalendar and NSDateComponents instead"]
        #[method_id(@__retain_semantics Init initWithYear:month:day:hour:minute:second:timeZone:)]
        pub unsafe fn initWithYear_month_day_hour_minute_second_timeZone(
            this: Allocated<Self>,
            year: NSInteger,
            month: NSUInteger,
            day: NSUInteger,
            hour: NSUInteger,
            minute: NSUInteger,
            second: NSUInteger,
            a_time_zone: Option<&NSTimeZone>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setCalendarFormat:)]
        pub unsafe fn setCalendarFormat(&self, format: Option<&NSString>);

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[deprecated]
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, a_time_zone: Option<&NSTimeZone>);

        #[deprecated]
        #[method(years:months:days:hours:minutes:seconds:sinceDate:)]
        pub unsafe fn years_months_days_hours_minutes_seconds_sinceDate(
            &self,
            yp: *mut NSInteger,
            mop: *mut NSInteger,
            dp: *mut NSInteger,
            hp: *mut NSInteger,
            mip: *mut NSInteger,
            sp: *mut NSInteger,
            date: &NSCalendarDate,
        );

        #[deprecated]
        #[method_id(@__retain_semantics Other distantFuture)]
        pub unsafe fn distantFuture() -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other distantPast)]
        pub unsafe fn distantPast() -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDate`
    #[cfg(feature = "Foundation_NSCalendarDate")]
    unsafe impl NSCalendarDate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithTimeIntervalSinceReferenceDate:)]
        pub unsafe fn initWithTimeIntervalSinceReferenceDate(
            this: Allocated<Self>,
            ti: NSTimeInterval,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSCalendarDate")]
    unsafe impl NSCalendarDate {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSCalendarDateExtras
    #[cfg(feature = "Foundation_NSDate")]
    unsafe impl NSDate {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Create an NSDateFormatter with `init` and set the dateFormat property instead."]
        #[method_id(@__retain_semantics Other dateWithNaturalLanguageString:locale:)]
        pub unsafe fn dateWithNaturalLanguageString_locale(
            string: &NSString,
            locale: Option<&AnyObject>,
        ) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Create an NSDateFormatter with `init` and set the dateFormat property instead."]
        #[method_id(@__retain_semantics Other dateWithNaturalLanguageString:)]
        pub unsafe fn dateWithNaturalLanguageString(string: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSDateFormatter instead"]
        #[method_id(@__retain_semantics Other dateWithString:)]
        pub unsafe fn dateWithString(a_string: &NSString) -> Id<AnyObject>;

        #[cfg(all(
            feature = "Foundation_NSCalendarDate",
            feature = "Foundation_NSString",
            feature = "Foundation_NSTimeZone"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Other dateWithCalendarFormat:timeZone:)]
        pub unsafe fn dateWithCalendarFormat_timeZone(
            &self,
            format: Option<&NSString>,
            a_time_zone: Option<&NSTimeZone>,
        ) -> Id<NSCalendarDate>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSTimeZone"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptionWithCalendarFormat:timeZone:locale:)]
        pub unsafe fn descriptionWithCalendarFormat_timeZone_locale(
            &self,
            format: Option<&NSString>,
            a_time_zone: Option<&NSTimeZone>,
            locale: Option<&AnyObject>,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSDateFormatter instead"]
        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Allocated<Self>,
            description: &NSString,
        ) -> Option<Id<Self>>;
    }
);
