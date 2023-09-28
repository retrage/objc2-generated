//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MEMessageActionMessageColor {
        MEMessageActionMessageColorNone = 0,
        MEMessageActionMessageColorGreen = 1,
        MEMessageActionMessageColorYellow = 2,
        MEMessageActionMessageColorOrange = 3,
        MEMessageActionMessageColorRed = 4,
        MEMessageActionMessageColorPurple = 5,
        MEMessageActionMessageColorBlue = 6,
        MEMessageActionMessageColorGray = 7,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MEMessageActionFlag {
        MEMessageActionFlagNone = 0,
        MEMessageActionFlagDefaultColor = 1,
        MEMessageActionFlagRed = 2,
        MEMessageActionFlagOrange = 3,
        MEMessageActionFlagYellow = 4,
        MEMessageActionFlagGreen = 5,
        MEMessageActionFlagBlue = 6,
        MEMessageActionFlagPurple = 7,
        MEMessageActionFlagGray = 8,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MailKit_MEMessageAction")]
    pub struct MEMessageAction;

    #[cfg(feature = "MailKit_MEMessageAction")]
    unsafe impl ClassType for MEMessageAction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MailKit_MEMessageAction")]
unsafe impl NSCoding for MEMessageAction {}

#[cfg(feature = "MailKit_MEMessageAction")]
unsafe impl NSObjectProtocol for MEMessageAction {}

#[cfg(feature = "MailKit_MEMessageAction")]
unsafe impl NSSecureCoding for MEMessageAction {}

extern_methods!(
    #[cfg(feature = "MailKit_MEMessageAction")]
    unsafe impl MEMessageAction {
        #[method_id(@__retain_semantics Other moveToTrashAction)]
        pub unsafe fn moveToTrashAction() -> Id<MEMessageAction>;

        #[method_id(@__retain_semantics Other moveToArchiveAction)]
        pub unsafe fn moveToArchiveAction() -> Id<MEMessageAction>;

        #[method_id(@__retain_semantics Other moveToJunkAction)]
        pub unsafe fn moveToJunkAction() -> Id<MEMessageAction>;

        #[method_id(@__retain_semantics Other markAsReadAction)]
        pub unsafe fn markAsReadAction() -> Id<MEMessageAction>;

        #[method_id(@__retain_semantics Other markAsUnreadAction)]
        pub unsafe fn markAsUnreadAction() -> Id<MEMessageAction>;

        #[method_id(@__retain_semantics Other flagActionWithFlag:)]
        pub unsafe fn flagActionWithFlag(flag: MEMessageActionFlag) -> Id<Self>;

        #[method_id(@__retain_semantics Other setBackgroundColorActionWithColor:)]
        pub unsafe fn setBackgroundColorActionWithColor(
            color: MEMessageActionMessageColor,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
