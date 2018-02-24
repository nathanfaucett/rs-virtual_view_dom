use virtual_view::{self, Transaction};
use stdweb::{InstanceOf, Reference, ReferenceType, Value};
use stdweb::unstable::{TryFrom, TryInto, Void};
use stdweb::serde::ConversionError;
use stdweb::web::{Document, IEventTarget};
use stdweb::web::event::{ConcreteEvent, IEvent};
use serde::de::Error;
use serde_json::{from_str, to_value};

pub struct TransactionEvent(Reference);

impl InstanceOf for TransactionEvent {
    #[inline]
    fn instance_of(reference: &Reference) -> bool {
        let is_instance_of: bool = js! {
            return @{reference} instanceOf Event;
        }.try_into()
            .unwrap();

        is_instance_of
    }
}

impl ReferenceType for TransactionEvent {
    #[inline]
    unsafe fn from_reference_unchecked(reference: Reference) -> Self {
        TransactionEvent(reference)
    }
}

impl AsRef<Reference> for TransactionEvent {
    #[inline]
    fn as_ref(&self) -> &Reference {
        &self.0
    }
}

impl TryFrom<Reference> for TransactionEvent {
    type Error = Void;

    #[inline]
    fn try_from(reference: Reference) -> Result<Self, Self::Error> {
        Ok(TransactionEvent(reference))
    }
}

impl TryFrom<Value> for TransactionEvent {
    type Error = ConversionError;

    #[inline]
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match TryInto::<Reference>::try_into(value) {
            Ok(reference) => Ok(TransactionEvent(reference)),
            Err(e) => Err(ConversionError::custom(e)),
        }
    }
}

impl IEvent for TransactionEvent {}
impl ConcreteEvent for TransactionEvent {
    const EVENT_TYPE: &'static str = "virtual_view_transaction";
}

impl TransactionEvent {
    #[inline]
    pub fn new(transaction: Transaction) -> Self {
        let value = to_value(transaction).unwrap().to_string();

        let event: Value = js! {
            let e = new Event("virtual_view_transaction");
            e.transaction = @{value};
            return e;
        };

        TransactionEvent(event.into_reference().unwrap())
    }

    #[inline]
    pub fn transaction(&self) -> Transaction {
        let transaction: String = js! {
            return @{self.as_ref()}.transaction;
        }.try_into()
            .unwrap();

        from_str(&transaction).unwrap()
    }
}

pub struct Handler {
    document: Document,
}

impl Handler {
    #[inline]
    pub fn new(document: Document) -> Self {
        Handler { document: document }
    }
}

impl virtual_view::Handler for Handler {
    #[inline]
    fn handle(&self, transaction: Transaction) {
        let _ = self.document
            .dispatch_event::<TransactionEvent>(&TransactionEvent::new(transaction));
    }
}
