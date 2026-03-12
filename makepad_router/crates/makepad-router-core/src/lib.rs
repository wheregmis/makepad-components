pub use makepad_live_id;
pub use makepad_micro_serde;

pub mod navigation;
pub mod pattern;
pub mod registry;
pub mod route;
pub mod router;
pub mod state;
pub mod url;

pub use crate::navigation::NavigationHistory;
pub use crate::pattern::{RouteParams, RoutePattern, RoutePatternRef, RouteSegment};
pub use crate::registry::RouteRegistry;
pub use crate::route::{Route, RouteQuery};
pub use crate::router::{Router, RouterAction};
pub use crate::state::RouterState;
pub use crate::url::{build_query_string, parse_query_map, RouterUrl};

pub mod prelude {
    pub use crate::pattern::{RouteParams, RoutePattern, RoutePatternRef, RouteSegment};
    pub use crate::route::{Route, RouteQuery};
    pub use crate::router::{Router, RouterAction};
    pub use crate::state::RouterState;
    pub use crate::url::RouterUrl;
}
