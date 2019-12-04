extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::ConditionalSelect;

impl<E, O> VMInstruction<E, O> for ConditionalSelect
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let condition = vm.pop()?;
        let if_true = vm.pop()?;
        let if_false = vm.pop()?;

        let selected = vm
            .get_operator()
            .conditional_select(condition, if_true, if_false)?;

        vm.push(selected)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::PushConst;

    #[test]
    fn test_cs() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 1337.into() })
            .add(PushConst { value: 42.into() })
            .add(PushConst { value: 0.into() })
            .add(ConditionalSelect)
            .add(PushConst { value: 420.into() })
            .add(PushConst { value: 69.into() })
            .add(PushConst { value: 1.into() })
            .add(ConditionalSelect)
            .test(&[69, 1337])
    }
}
