use makepad_router_core::makepad_live_id::*;
use makepad_router_core::makepad_micro_serde::{DeRon, SerRon};
use makepad_router_core::{
    NavigationHistory, Route, RouteParams, RoutePattern, RoutePatternRef, RouteQuery, RouterState,
};

#[test]
fn router_state_ron_roundtrip_preserves_history_and_query() {
    let mut params = RouteParams::default();
    params.add(live_id!(id), live_id!(user_42));

    let mut query = RouteQuery::default();
    query.set("tab", "settings");
    query.set("empty", "");

    let route = Route {
        id: live_id!(user_profile),
        params,
        query,
        hash: "#section".to_string(),
        pattern: Some(RoutePatternRef::new(
            RoutePattern::parse("/user/:id").unwrap(),
        )),
    };

    let history = NavigationHistory::from_parts(vec![Route::new(live_id!(home)), route], 1);
    let state = RouterState {
        history,
        url_path_override: Some("/admin/dashboard".to_string()),
    };

    let ron = state.serialize_ron();
    let de = RouterState::deserialize_ron(&ron).unwrap();
    assert_eq!(de, state);
}
