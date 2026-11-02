use valuable::{Value, Visit};

use crate::Uuid;

impl valuable::Valuable for Uuid {
    fn as_value(&self) -> Value<'_> {
        Value::U128(self.as_u128())
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_value(self.as_value());
    }
}

#[cfg(test)]
mod tests {
    use valuable::{Valuable, Value, Visit};

    use crate::Uuid;

    #[test]
    fn test_valuable_as_value() {
        let uuid = Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8);

        assert!(matches!(
            uuid.as_value(),
            Value::U128(value) if value == uuid.as_u128()
        ));
    }

    #[test]
    fn test_valuable_visit() {
        struct Capture(Option<u128>);

        impl Visit for Capture {
            fn visit_value(&mut self, value: Value<'_>) {
                self.0 = value.as_u128();
            }
        }

        let uuid = Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8);
        let mut visitor = Capture(None);

        uuid.visit(&mut visitor);

        assert_eq!(visitor.0, Some(uuid.as_u128()));
    }
}