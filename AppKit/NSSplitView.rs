//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSSplitViewAutosaveName = NSString;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSSplitViewDividerStyle {
        NSSplitViewDividerStyleThick = 1,
        NSSplitViewDividerStyleThin = 2,
        NSSplitViewDividerStylePaneSplitter = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSplitView")]
    pub struct NSSplitView;

    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl ClassType for NSSplitView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSSplitView")]
unsafe impl NSAccessibility for NSSplitView {}

#[cfg(feature = "AppKit_NSSplitView")]
unsafe impl NSAccessibilityElementProtocol for NSSplitView {}

#[cfg(feature = "AppKit_NSSplitView")]
unsafe impl NSAnimatablePropertyContainer for NSSplitView {}

#[cfg(feature = "AppKit_NSSplitView")]
unsafe impl NSAppearanceCustomization for NSSplitView {}

#[cfg(feature = "AppKit_NSSplitView")]
unsafe impl NSCoding for NSSplitView {}

#[cfg(feature = "AppKit_NSSplitView")]
unsafe impl NSDraggingDestination for NSSplitView {}

#[cfg(feature = "AppKit_NSSplitView")]
unsafe impl NSObjectProtocol for NSSplitView {}

#[cfg(feature = "AppKit_NSSplitView")]
unsafe impl NSUserInterfaceItemIdentification for NSSplitView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl NSSplitView {
        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;

        #[method(setVertical:)]
        pub unsafe fn setVertical(&self, vertical: bool);

        #[method(dividerStyle)]
        pub unsafe fn dividerStyle(&self) -> NSSplitViewDividerStyle;

        #[method(setDividerStyle:)]
        pub unsafe fn setDividerStyle(&self, divider_style: NSSplitViewDividerStyle);

        #[method_id(@__retain_semantics Other autosaveName)]
        pub unsafe fn autosaveName(&self) -> Option<Id<NSSplitViewAutosaveName>>;

        #[method(setAutosaveName:)]
        pub unsafe fn setAutosaveName(&self, autosave_name: Option<&NSSplitViewAutosaveName>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSSplitViewDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSplitViewDelegate>>,
        );

        #[method(drawDividerInRect:)]
        pub unsafe fn drawDividerInRect(&self, rect: NSRect);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other dividerColor)]
        pub unsafe fn dividerColor(&self) -> Id<NSColor>;

        #[method(dividerThickness)]
        pub unsafe fn dividerThickness(&self) -> CGFloat;

        #[method(adjustSubviews)]
        pub unsafe fn adjustSubviews(&self);

        #[method(isSubviewCollapsed:)]
        pub unsafe fn isSubviewCollapsed(&self, subview: &NSView) -> bool;

        #[method(minPossiblePositionOfDividerAtIndex:)]
        pub unsafe fn minPossiblePositionOfDividerAtIndex(
            &self,
            divider_index: NSInteger,
        ) -> CGFloat;

        #[method(maxPossiblePositionOfDividerAtIndex:)]
        pub unsafe fn maxPossiblePositionOfDividerAtIndex(
            &self,
            divider_index: NSInteger,
        ) -> CGFloat;

        #[method(setPosition:ofDividerAtIndex:)]
        pub unsafe fn setPosition_ofDividerAtIndex(
            &self,
            position: CGFloat,
            divider_index: NSInteger,
        );

        #[method(holdingPriorityForSubviewAtIndex:)]
        pub unsafe fn holdingPriorityForSubviewAtIndex(
            &self,
            subview_index: NSInteger,
        ) -> NSLayoutPriority;

        #[method(setHoldingPriority:forSubviewAtIndex:)]
        pub unsafe fn setHoldingPriority_forSubviewAtIndex(
            &self,
            priority: NSLayoutPriority,
            subview_index: NSInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl NSSplitView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl NSSplitView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl NSSplitView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    /// NSSplitViewArrangedSubviews
    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl NSSplitView {
        #[method(arrangesAllSubviews)]
        pub unsafe fn arrangesAllSubviews(&self) -> bool;

        #[method(setArrangesAllSubviews:)]
        pub unsafe fn setArrangesAllSubviews(&self, arranges_all_subviews: bool);

        #[cfg(all(feature = "AppKit_NSView", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other arrangedSubviews)]
        pub unsafe fn arrangedSubviews(&self) -> Id<NSArray<NSView>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(addArrangedSubview:)]
        pub unsafe fn addArrangedSubview(&self, view: &NSView);

        #[cfg(feature = "AppKit_NSView")]
        #[method(insertArrangedSubview:atIndex:)]
        pub unsafe fn insertArrangedSubview_atIndex(&self, view: &NSView, index: NSInteger);

        #[cfg(feature = "AppKit_NSView")]
        #[method(removeArrangedSubview:)]
        pub unsafe fn removeArrangedSubview(&self, view: &NSView);
    }
);

extern_protocol!(
    pub unsafe trait NSSplitViewDelegate: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(all(feature = "AppKit_NSSplitView", feature = "AppKit_NSView"))]
        #[optional]
        #[method(splitView:canCollapseSubview:)]
        unsafe fn splitView_canCollapseSubview(
            &self,
            split_view: &NSSplitView,
            subview: &NSView,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSSplitView", feature = "AppKit_NSView"))]
        #[deprecated = "NSSplitView no longer supports collapsing sections via double-click. This delegate method is never called."]
        #[optional]
        #[method(splitView:shouldCollapseSubview:forDoubleClickOnDividerAtIndex:)]
        unsafe fn splitView_shouldCollapseSubview_forDoubleClickOnDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            subview: &NSView,
            divider_index: NSInteger,
        ) -> bool;

        #[cfg(feature = "AppKit_NSSplitView")]
        #[optional]
        #[method(splitView:constrainMinCoordinate:ofSubviewAt:)]
        unsafe fn splitView_constrainMinCoordinate_ofSubviewAt(
            &self,
            split_view: &NSSplitView,
            proposed_minimum_position: CGFloat,
            divider_index: NSInteger,
        ) -> CGFloat;

        #[cfg(feature = "AppKit_NSSplitView")]
        #[optional]
        #[method(splitView:constrainMaxCoordinate:ofSubviewAt:)]
        unsafe fn splitView_constrainMaxCoordinate_ofSubviewAt(
            &self,
            split_view: &NSSplitView,
            proposed_maximum_position: CGFloat,
            divider_index: NSInteger,
        ) -> CGFloat;

        #[cfg(feature = "AppKit_NSSplitView")]
        #[optional]
        #[method(splitView:constrainSplitPosition:ofSubviewAt:)]
        unsafe fn splitView_constrainSplitPosition_ofSubviewAt(
            &self,
            split_view: &NSSplitView,
            proposed_position: CGFloat,
            divider_index: NSInteger,
        ) -> CGFloat;

        #[cfg(feature = "AppKit_NSSplitView")]
        #[optional]
        #[method(splitView:resizeSubviewsWithOldSize:)]
        unsafe fn splitView_resizeSubviewsWithOldSize(
            &self,
            split_view: &NSSplitView,
            old_size: NSSize,
        );

        #[cfg(all(feature = "AppKit_NSSplitView", feature = "AppKit_NSView"))]
        #[optional]
        #[method(splitView:shouldAdjustSizeOfSubview:)]
        unsafe fn splitView_shouldAdjustSizeOfSubview(
            &self,
            split_view: &NSSplitView,
            view: &NSView,
        ) -> bool;

        #[cfg(feature = "AppKit_NSSplitView")]
        #[optional]
        #[method(splitView:shouldHideDividerAtIndex:)]
        unsafe fn splitView_shouldHideDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            divider_index: NSInteger,
        ) -> bool;

        #[cfg(feature = "AppKit_NSSplitView")]
        #[optional]
        #[method(splitView:effectiveRect:forDrawnRect:ofDividerAtIndex:)]
        unsafe fn splitView_effectiveRect_forDrawnRect_ofDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            proposed_effective_rect: NSRect,
            drawn_rect: NSRect,
            divider_index: NSInteger,
        ) -> NSRect;

        #[cfg(feature = "AppKit_NSSplitView")]
        #[optional]
        #[method(splitView:additionalEffectiveRectOfDividerAtIndex:)]
        unsafe fn splitView_additionalEffectiveRectOfDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            divider_index: NSInteger,
        ) -> NSRect;

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(splitViewWillResizeSubviews:)]
        unsafe fn splitViewWillResizeSubviews(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(splitViewDidResizeSubviews:)]
        unsafe fn splitViewDidResizeSubviews(&self, notification: &NSNotification);
    }

    unsafe impl ProtocolType for dyn NSSplitViewDelegate {}
);

extern_static!(NSSplitViewWillResizeSubviewsNotification: &'static NSNotificationName);

extern_static!(NSSplitViewDidResizeSubviewsNotification: &'static NSNotificationName);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl NSSplitView {
        #[deprecated]
        #[method(setIsPaneSplitter:)]
        pub unsafe fn setIsPaneSplitter(&self, flag: bool);

        #[deprecated]
        #[method(isPaneSplitter)]
        pub unsafe fn isPaneSplitter(&self) -> bool;
    }
);
