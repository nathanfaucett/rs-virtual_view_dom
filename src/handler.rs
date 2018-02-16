use view::{self, Transaction};
use stdweb::{Reference, Value};
use stdweb::unstable::{TryFrom, TryInto};
use stdweb::serde::ConversionError;
use stdweb::web::{Document, IEventTarget};
use stdweb::web::event::{ConcreteEvent, IEvent};
use serde::de::Error;
use serde_json::{from_str, to_value};

pub struct TransactionEvent(Reference);

impl AsRef<Reference> for TransactionEvent {
    #[inline]
    fn as_ref(&self) -> &Reference {
        &self.0
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
    const EVENT_TYPE: &'static str = "viewtransaction";
}

impl TransactionEvent {
    #[inline]
    pub fn new(transaction: Transaction) -> Self {
        let json = to_value(transaction).unwrap().to_string();

        let event = js! {
            let e = new Event("viewtransaction");
            e.transaction = @{json};
            return e;
        };

        TransactionEvent(event.into_reference().unwrap())
    }

    #[inline]
    pub fn transaction(&self) -> Transaction {
        let transaction = js! {
            return @{self.as_ref()}.transaction;
        };
        from_str(&transaction.into_string().unwrap()).unwrap()
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

impl view::Handler for Handler {
    #[inline]
    fn handle(&self, transaction: Transaction) {
        self.document
            .dispatch_event::<TransactionEvent>(&TransactionEvent::new(transaction));
    }
}
