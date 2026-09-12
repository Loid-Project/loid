use strum::IntoEnumIterator;

use crate::shared::types::syntax::operator::Operator;

pub fn get_operators() -> Vec<Operator> {
    Operator::iter().collect()
}
