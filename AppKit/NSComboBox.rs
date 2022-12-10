//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSComboBoxWillPopUpNotification: &'static NSNotificationName);

extern_static!(NSComboBoxWillDismissNotification: &'static NSNotificationName);

extern_static!(NSComboBoxSelectionDidChangeNotification: &'static NSNotificationName);

extern_static!(NSComboBoxSelectionIsChangingNotification: &'static NSNotificationName);

extern_protocol!(
    pub struct NSComboBoxDataSource;

    unsafe impl ProtocolType for NSComboBoxDataSource {
        #[optional]
        #[method(numberOfItemsInComboBox:)]
        pub unsafe fn numberOfItemsInComboBox(&self, comboBox: &NSComboBox) -> NSInteger;

        #[optional]
        #[method_id(@__retain_semantics Other comboBox:objectValueForItemAtIndex:)]
        pub unsafe fn comboBox_objectValueForItemAtIndex(
            &self,
            comboBox: &NSComboBox,
            index: NSInteger,
        ) -> Option<Id<Object, Shared>>;

        #[optional]
        #[method(comboBox:indexOfItemWithStringValue:)]
        pub unsafe fn comboBox_indexOfItemWithStringValue(
            &self,
            comboBox: &NSComboBox,
            string: &NSString,
        ) -> NSUInteger;

        #[optional]
        #[method_id(@__retain_semantics Other comboBox:completedString:)]
        pub unsafe fn comboBox_completedString(
            &self,
            comboBox: &NSComboBox,
            string: &NSString,
        ) -> Option<Id<NSString, Shared>>;
    }
);

extern_protocol!(
    pub struct NSComboBoxDelegate;

    unsafe impl ProtocolType for NSComboBoxDelegate {
        #[optional]
        #[method(comboBoxWillPopUp:)]
        pub unsafe fn comboBoxWillPopUp(&self, notification: &NSNotification);

        #[optional]
        #[method(comboBoxWillDismiss:)]
        pub unsafe fn comboBoxWillDismiss(&self, notification: &NSNotification);

        #[optional]
        #[method(comboBoxSelectionDidChange:)]
        pub unsafe fn comboBoxSelectionDidChange(&self, notification: &NSNotification);

        #[optional]
        #[method(comboBoxSelectionIsChanging:)]
        pub unsafe fn comboBoxSelectionIsChanging(&self, notification: &NSNotification);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSComboBox;

    unsafe impl ClassType for NSComboBox {
        type Super = NSTextField;
    }
);

extern_methods!(
    unsafe impl NSComboBox {
        #[method(hasVerticalScroller)]
        pub unsafe fn hasVerticalScroller(&self) -> bool;

        #[method(setHasVerticalScroller:)]
        pub unsafe fn setHasVerticalScroller(&self, hasVerticalScroller: bool);

        #[method(intercellSpacing)]
        pub unsafe fn intercellSpacing(&self) -> NSSize;

        #[method(setIntercellSpacing:)]
        pub unsafe fn setIntercellSpacing(&self, intercellSpacing: NSSize);

        #[method(itemHeight)]
        pub unsafe fn itemHeight(&self) -> CGFloat;

        #[method(setItemHeight:)]
        pub unsafe fn setItemHeight(&self, itemHeight: CGFloat);

        #[method(numberOfVisibleItems)]
        pub unsafe fn numberOfVisibleItems(&self) -> NSInteger;

        #[method(setNumberOfVisibleItems:)]
        pub unsafe fn setNumberOfVisibleItems(&self, numberOfVisibleItems: NSInteger);

        #[method(isButtonBordered)]
        pub unsafe fn isButtonBordered(&self) -> bool;

        #[method(setButtonBordered:)]
        pub unsafe fn setButtonBordered(&self, buttonBordered: bool);

        #[method(reloadData)]
        pub unsafe fn reloadData(&self);

        #[method(noteNumberOfItemsChanged)]
        pub unsafe fn noteNumberOfItemsChanged(&self);

        #[method(usesDataSource)]
        pub unsafe fn usesDataSource(&self) -> bool;

        #[method(setUsesDataSource:)]
        pub unsafe fn setUsesDataSource(&self, usesDataSource: bool);

        #[method(scrollItemAtIndexToTop:)]
        pub unsafe fn scrollItemAtIndexToTop(&self, index: NSInteger);

        #[method(scrollItemAtIndexToVisible:)]
        pub unsafe fn scrollItemAtIndexToVisible(&self, index: NSInteger);

        #[method(selectItemAtIndex:)]
        pub unsafe fn selectItemAtIndex(&self, index: NSInteger);

        #[method(deselectItemAtIndex:)]
        pub unsafe fn deselectItemAtIndex(&self, index: NSInteger);

        #[method(indexOfSelectedItem)]
        pub unsafe fn indexOfSelectedItem(&self) -> NSInteger;

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method(completes)]
        pub unsafe fn completes(&self) -> bool;

        #[method(setCompletes:)]
        pub unsafe fn setCompletes(&self, completes: bool);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSComboBoxDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSComboBoxDelegate>);

        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Id<NSComboBoxDataSource, Shared>>;

        #[method(setDataSource:)]
        pub unsafe fn setDataSource(&self, dataSource: Option<&NSComboBoxDataSource>);

        #[method(addItemWithObjectValue:)]
        pub unsafe fn addItemWithObjectValue(&self, object: &Object);

        #[method(addItemsWithObjectValues:)]
        pub unsafe fn addItemsWithObjectValues(&self, objects: &NSArray);

        #[method(insertItemWithObjectValue:atIndex:)]
        pub unsafe fn insertItemWithObjectValue_atIndex(&self, object: &Object, index: NSInteger);

        #[method(removeItemWithObjectValue:)]
        pub unsafe fn removeItemWithObjectValue(&self, object: &Object);

        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

        #[method(removeAllItems)]
        pub unsafe fn removeAllItems(&self);

        #[method(selectItemWithObjectValue:)]
        pub unsafe fn selectItemWithObjectValue(&self, object: Option<&Object>);

        #[method_id(@__retain_semantics Other itemObjectValueAtIndex:)]
        pub unsafe fn itemObjectValueAtIndex(&self, index: NSInteger) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other objectValueOfSelectedItem)]
        pub unsafe fn objectValueOfSelectedItem(&self) -> Option<Id<Object, Shared>>;

        #[method(indexOfItemWithObjectValue:)]
        pub unsafe fn indexOfItemWithObjectValue(&self, object: &Object) -> NSInteger;

        #[method_id(@__retain_semantics Other objectValues)]
        pub unsafe fn objectValues(&self) -> Id<NSArray, Shared>;
    }
);
