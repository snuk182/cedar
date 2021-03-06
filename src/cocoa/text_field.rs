
use cocoa::base::{id, nil, class, YES};
use cocoa::foundation::NSString;

use super::id::Id;
use super::widget::Widget;
use super::delegate;

use stream::Stream;

pub struct TextField<S> {
    id: Id,
    stream: Stream<S>,
}

impl<S: 'static> TextField<S> {
    pub fn new(stream: Stream<S>) -> Self {
        unsafe {
            let string = NSString::alloc(nil).init_str("");

            let field: id = msg_send![class("NSTextField"), alloc];
            let field: id = msg_send![field, init];

            msg_send![field, setStringValue: string];
            msg_send![field, setBezeled: YES];
            msg_send![field, setDrawsBackground: YES];
            msg_send![field, setEditable: YES];
            msg_send![field, setSelectable: YES];

            TextField {
                id: field.into(),
                stream: stream,
            }
        }
    }

    pub fn placeholder(self, text: &str) -> Self {
        unsafe {
            let text = NSString::alloc(nil).init_str(text);

            let string: id = msg_send![class("NSAttributedString"), alloc];
            let string: id = msg_send![string, initWithString: text];

            msg_send![*self.id, setPlaceholderAttributedString: string];

            // set "minimum size" through anchor constraint
            let anchor: id = msg_send![*self.id, widthAnchor];
            let constraint: id = msg_send![anchor, constraintGreaterThanOrEqualToConstant: 120.];
            msg_send![constraint, setActive: YES];
        }

        self
    }

    pub fn change<F: Fn(&str) -> S + 'static>(self, delegate: F) -> Self {
        let stream = self.stream.clone();
        let delegate = delegate::create(move |s| stream.push(delegate(s)));

        unsafe { msg_send![*self.id, setDelegate: delegate] };

        self
    }
}

impl<M, S> Widget<M> for TextField<S> {
    fn id(&self) -> &Id {
        &self.id
    }

    fn update(&mut self, _: &M) {}
}
