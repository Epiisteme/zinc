//!
//! The interpreter element value.
//!

mod array;
mod boolean;
mod error;
mod integer;
mod structure;
mod tuple;

pub use self::array::Array;
pub use self::array::Error as ArrayError;
pub use self::boolean::Boolean;
pub use self::boolean::Error as BooleanError;
pub use self::error::Error;
pub use self::integer::Error as IntegerError;
pub use self::integer::Integer;
pub use self::structure::Error as StructureError;
pub use self::structure::Structure;
pub use self::tuple::Tuple;

use std::fmt;

use parser::BooleanLiteral;
use parser::IntegerLiteral;
use parser::TypeVariant;
use r1cs::Bn256;
use r1cs::ConstraintSystem;

#[derive(Clone, PartialEq)]
pub enum Value {
    Unit,
    Boolean(Boolean),
    Integer(Integer),
    Array(Array),
    Tuple(Tuple),
    Structure(Structure),
    Enumeration(usize),
}

impl Value {
    pub fn new_unit() -> Self {
        Self::Unit
    }

    pub fn new_boolean_from_literal<S: ConstraintSystem<Bn256>>(
        system: S,
        literal: BooleanLiteral,
    ) -> Result<Self, Error> {
        Boolean::new_from_literal(system, literal)
            .map(Self::Boolean)
            .map_err(Error::Boolean)
    }

    pub fn new_integer_from_literal<S: ConstraintSystem<Bn256>>(
        system: S,
        literal: IntegerLiteral,
    ) -> Result<Self, Error> {
        Integer::new_from_literal(system, literal)
            .map(Self::Integer)
            .map_err(Error::Integer)
    }

    pub fn new_integer_from_usize<S: ConstraintSystem<Bn256>>(
        system: S,
        value: usize,
    ) -> Result<Self, Error> {
        Integer::new_from_usize(system, value)
            .map(Self::Integer)
            .map_err(Error::Integer)
    }

    pub fn new_array(values: Vec<Self>) -> Result<Self, Error> {
        let mut array = Array::with_capacity(values.len());
        for element in values.into_iter() {
            array.push(element).map_err(Error::Array)?;
        }
        Ok(Self::Array(array))
    }

    pub fn new_tuple(values: Vec<Self>) -> Result<Self, Error> {
        let mut tuple = Tuple::with_capacity(values.len());
        for element in values.into_iter() {
            tuple.push(element);
        }
        Ok(Self::Tuple(tuple))
    }

    pub fn new_structure(fields: Vec<(String, Self)>) -> Result<Self, Error> {
        let mut structure = Structure::new();
        for (key, value) in fields.into_iter() {
            structure.push(key, value).map_err(Error::Structure)?;
        }
        Ok(Self::Structure(structure))
    }

    pub fn type_variant(&self) -> TypeVariant {
        match self {
            Self::Unit => TypeVariant::new_unit(),
            Self::Boolean(value) => value.type_variant(),
            Self::Integer(value) => value.type_variant(),
            Self::Array(value) => value.type_variant(),
            Self::Tuple(value) => value.type_variant(),
            Self::Structure(value) => value.type_variant(),
            Self::Enumeration(_value) => TypeVariant::new_integer_unsigned(8),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unit, Self::Unit) => true,
            (Self::Boolean(..), Self::Boolean(..)) => true,
            (Self::Integer(value_1), Self::Integer(value_2)) => {
                value_1.has_the_same_type_as(value_2)
            }
            (Self::Array(value_1), Self::Array(value_2)) => value_1.has_the_same_type_as(value_2),
            (Self::Tuple(value_1), Self::Tuple(value_2)) => value_1.has_the_same_type_as(value_2),
            (Self::Structure(value_1), Self::Structure(value_2)) => {
                value_1.has_the_same_type_as(value_2)
            }
            _ => false,
        }
    }

    pub fn or<S: ConstraintSystem<Bn256>>(self, mut system: S, other: Self) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Boolean(value) => value,
            value => return Err(Error::ExpectedBoolean("or", value)),
        };

        let value_2 = match other {
            Self::Boolean(value) => value,
            value => return Err(Error::ExpectedBoolean("or", value)),
        };

        value_1
            .or(system.namespace(|| "value_or"), value_2)
            .map(Self::Boolean)
            .map_err(Error::Boolean)
    }

    pub fn xor<S: ConstraintSystem<Bn256>>(
        self,
        mut system: S,
        other: Self,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Boolean(value) => value,
            value => return Err(Error::ExpectedBoolean("xor", value)),
        };

        let value_2 = match other {
            Self::Boolean(value) => value,
            value => return Err(Error::ExpectedBoolean("xor", value)),
        };

        value_1
            .xor(system.namespace(|| "value_xor"), value_2)
            .map(Self::Boolean)
            .map_err(Error::Boolean)
    }

    pub fn and<S: ConstraintSystem<Bn256>>(
        self,
        mut system: S,
        other: Self,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Boolean(value) => value,
            value => return Err(Error::ExpectedBoolean("and", value)),
        };

        let value_2 = match other {
            Self::Boolean(value) => value,
            value => return Err(Error::ExpectedBoolean("and", value)),
        };

        value_1
            .and(system.namespace(|| "value_and"), value_2)
            .map(Self::Boolean)
            .map_err(Error::Boolean)
    }

    pub fn equals<S: ConstraintSystem<Bn256>>(
        &self,
        mut system: S,
        other: &Self,
    ) -> Result<Self, Error> {
        match (self, other) {
            (Self::Unit, Self::Unit) => {
                Boolean::new_from_bool(system.namespace(|| "value_equals"), true)
                    .map(Self::Boolean)
                    .map_err(Error::Boolean)
            }
            (Self::Boolean(value_1), Self::Boolean(value_2)) => value_1
                .equals(system.namespace(|| "value_equals"), value_2)
                .map(Self::Boolean)
                .map_err(Error::Boolean),
            (Self::Boolean(..), value_2) => {
                Err(Error::ExpectedBoolean("equals", value_2.to_owned()))
            }
            (Self::Integer(value_1), Self::Integer(value_2)) => value_1
                .equals(system.namespace(|| "value_equals"), value_2)
                .map(Self::Boolean)
                .map_err(Error::Integer),
            (Self::Integer(..), value_2) => {
                Err(Error::ExpectedInteger("equals", value_2.to_owned()))
            }
            (value_1, value_2) => Err(Error::OperandTypesMismatch(
                value_1.to_owned(),
                value_2.to_owned(),
            )),
        }
    }

    pub fn not_equals<S: ConstraintSystem<Bn256>>(
        &self,
        mut system: S,
        other: &Self,
    ) -> Result<Self, Error> {
        match (self, other) {
            (Self::Unit, Self::Unit) => {
                Boolean::new_from_bool(system.namespace(|| "value_not_equals"), false)
                    .map(Self::Boolean)
                    .map_err(Error::Boolean)
            }
            (Self::Boolean(value_1), Self::Boolean(value_2)) => value_1
                .not_equals(system.namespace(|| "value_not_equals"), value_2)
                .map(Self::Boolean)
                .map_err(Error::Boolean),
            (Self::Boolean(..), value_2) => {
                Err(Error::ExpectedBoolean("not_equals", value_2.to_owned()))
            }
            (Self::Integer(value_1), Self::Integer(value_2)) => value_1
                .not_equals(system.namespace(|| "value_not_equals"), value_2)
                .map(Self::Boolean)
                .map_err(Error::Integer),
            (Self::Integer(..), value_2) => {
                Err(Error::ExpectedInteger("not_equals", value_2.to_owned()))
            }
            (value_1, value_2) => Err(Error::OperandTypesMismatch(
                value_1.to_owned(),
                value_2.to_owned(),
            )),
        }
    }

    pub fn greater_equals<S: ConstraintSystem<Bn256>>(
        self,
        mut system: S,
        other: &Self,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("greater_equals", value)),
        };

        let value_2 = match other {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("greater_equals", value.to_owned())),
        };

        value_1
            .greater_equals(system.namespace(|| "value_greater_equals"), &value_2)
            .map(Self::Boolean)
            .map_err(Error::Integer)
    }

    pub fn lesser_equals<S: ConstraintSystem<Bn256>>(
        self,
        mut system: S,
        other: &Self,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("lesser_equals", value)),
        };

        let value_2 = match other {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("lesser_equals", value.to_owned())),
        };

        value_1
            .lesser_equals(system.namespace(|| "value_lesser_equals"), &value_2)
            .map(Self::Boolean)
            .map_err(Error::Integer)
    }

    pub fn greater<S: ConstraintSystem<Bn256>>(
        self,
        mut system: S,
        other: &Self,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("greater", value)),
        };

        let value_2 = match other {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("greater", value.to_owned())),
        };

        value_1
            .greater(system.namespace(|| "value_greater"), &value_2)
            .map(Self::Boolean)
            .map_err(Error::Integer)
    }

    pub fn lesser<S: ConstraintSystem<Bn256>>(
        self,
        mut system: S,
        other: &Self,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("lesser", value)),
        };

        let value_2 = match other {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("lesser", value.to_owned())),
        };

        value_1
            .lesser(system.namespace(|| "value_lesser"), &value_2)
            .map(Self::Boolean)
            .map_err(Error::Integer)
    }

    pub fn add<S: ConstraintSystem<Bn256>>(
        self,
        mut system: S,
        other: Self,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("add", value)),
        };

        let value_2 = match other {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("add", value)),
        };

        value_1
            .add(system.namespace(|| "value_add"), value_2)
            .map(Self::Integer)
            .map_err(Error::Integer)
    }

    pub fn subtract<S: ConstraintSystem<Bn256>>(
        self,
        mut system: S,
        other: Self,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("subtract", value)),
        };

        let value_2 = match other {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("subtract", value)),
        };

        value_1
            .subtract(system.namespace(|| "value_subtract"), value_2)
            .map(Self::Integer)
            .map_err(Error::Integer)
    }

    pub fn multiply<S: ConstraintSystem<Bn256>>(
        self,
        mut system: S,
        other: Self,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("multiply", value)),
        };

        let value_2 = match other {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("multiply", value)),
        };

        value_1
            .multiply(system.namespace(|| "value_multiply"), value_2)
            .map(Self::Integer)
            .map_err(Error::Integer)
    }

    pub fn divide<S: ConstraintSystem<Bn256>>(
        self,
        mut system: S,
        other: Self,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("divide", value)),
        };

        let value_2 = match other {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("divide", value)),
        };

        value_1
            .divide(system.namespace(|| "value_divide"), value_2)
            .map(Self::Integer)
            .map_err(Error::Integer)
    }

    pub fn modulo<S: ConstraintSystem<Bn256>>(
        self,
        mut system: S,
        other: Self,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("modulo", value)),
        };

        let value_2 = match other {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("modulo", value)),
        };

        value_1
            .modulo(system.namespace(|| "value_modulo"), value_2)
            .map(Self::Integer)
            .map_err(Error::Integer)
    }

    pub fn negate<S: ConstraintSystem<Bn256>>(self, mut system: S) -> Result<Self, Error> {
        let value = match self {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("negate", value)),
        };

        value
            .negate(system.namespace(|| "value_negate"))
            .map(Self::Integer)
            .map_err(Error::Integer)
    }

    pub fn not<S: ConstraintSystem<Bn256>>(self, mut system: S) -> Result<Self, Error> {
        let value = match self {
            Self::Boolean(value) => value,
            value => return Err(Error::ExpectedBoolean("not", value)),
        };

        value
            .not(system.namespace(|| "value_not"))
            .map(Self::Boolean)
            .map_err(Error::Boolean)
    }

    pub fn cast<S: ConstraintSystem<Bn256>>(
        self,
        mut system: S,
        type_variant: TypeVariant,
    ) -> Result<Self, Error> {
        let value = match self {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("cast", value)),
        };

        value
            .cast(system.namespace(|| "value_cast"), type_variant)
            .map(Self::Integer)
            .map_err(Error::Integer)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Integer(value) => write!(f, "{}", value),
            Self::Array(value) => write!(f, "{}", value),
            Self::Tuple(value) => write!(f, "{}", value),
            Self::Structure(value) => write!(f, "{}", value),
            Self::Enumeration(value) => write!(f, "{}", value),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
