use makepad_widgets::*;

fn first_arg(vm: &mut ScriptVm, args: ScriptValue) -> Option<ScriptValue> {
    let args_obj = args.as_object()?;
    let trap = vm.bx.threads.cur().trap.pass();
    Some(vm.bx.heap.vec_value(args_obj, 0, trap))
}

pub(crate) fn bool_arg(vm: &mut ScriptVm, args: ScriptValue) -> Option<bool> {
    first_arg(vm, args).and_then(|value| value.as_bool())
}

pub(crate) fn number_arg(vm: &mut ScriptVm, args: ScriptValue) -> Option<f64> {
    first_arg(vm, args).and_then(|value| value.as_number())
}

pub(crate) fn string_arg(vm: &mut ScriptVm, args: ScriptValue) -> Option<String> {
    let value = first_arg(vm, args)?;
    if !value.is_string_like() {
        return None;
    }
    let mut output = String::new();
    vm.bx.heap.cast_to_string(value, &mut output);
    Some(output)
}
