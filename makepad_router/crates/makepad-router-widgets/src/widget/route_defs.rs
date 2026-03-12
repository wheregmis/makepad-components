use makepad_widgets::*;

#[derive(Clone, Debug, Default)]
pub(super) struct RouteDefinition {
    pub(super) pattern: Option<String>,
    pub(super) transition: Option<LiveId>,
    pub(super) transition_duration: Option<f64>,
}

pub(super) fn script_value_to_string(vm: &mut ScriptVm, value: ScriptValue) -> Option<String> {
    vm.string_with(value, |_vm, value| value.to_string())
}

pub(super) fn route_definition_from_template(
    vm: &mut ScriptVm,
    template_obj: ScriptObject,
) -> RouteDefinition {
    let pattern = script_value_to_string(
        vm,
        vm.bx
            .heap
            .value(template_obj, id!(route_pattern).into(), NoTrap),
    );

    let route_transition_value =
        vm.bx
            .heap
            .value(template_obj, id!(route_transition).into(), NoTrap);
    let transition = route_transition_value.as_id().or_else(|| {
        script_value_to_string(vm, route_transition_value).map(|v| LiveId::from_str(&v))
    });

    let transition_duration = vm
        .bx
        .heap
        .value(template_obj, id!(route_transition_duration).into(), NoTrap)
        .as_number();

    RouteDefinition {
        pattern,
        transition,
        transition_duration,
    }
}
