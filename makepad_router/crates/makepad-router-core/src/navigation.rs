use crate::route::Route;
use makepad_live_id::*;
use makepad_micro_serde::*;
use std::collections::HashMap;

/// Navigation history stack for managing route navigation
#[derive(Clone, Debug, Default)]
pub struct NavigationHistory {
    /// Stack of routes representing navigation history
    stack: Vec<Route>,
    /// Current position in the history (for back/forward navigation)
    current_index: usize,
    /// Reverse index for stack-style operations (not serialized).
    index: HashMap<LiveId, Vec<usize>>,
}

impl NavigationHistory {
    /// Create a new navigation history with an initial route
    pub fn new(initial_route: Route) -> Self {
        let mut out = Self {
            stack: vec![initial_route],
            current_index: 0,
            index: HashMap::new(),
        };
        out.rebuild_index();
        out
    }

    /// Create an empty navigation history
    pub fn empty() -> Self {
        Self {
            stack: Vec::new(),
            current_index: 0,
            index: HashMap::new(),
        }
    }

    /// Get the current route
    pub fn current(&self) -> Option<&Route> {
        self.stack.get(self.current_index)
    }

    /// Push a new route onto the history
    pub fn push(&mut self, route: Route) {
        // Remove any forward history when pushing a new route
        self.stack.truncate(self.current_index + 1);
        self.rebuild_index_for_truncate(self.current_index + 1);
        self.stack.push(route);
        self.current_index = self.stack.len() - 1;
        self.index_entry(self.current_index);
    }

    /// Replace the current route without adding to history
    pub fn replace(&mut self, route: Route) {
        if !self.stack.is_empty() {
            let idx = self.current_index;
            self.unindex_entry(idx);
            self.stack[idx] = route;
            self.index_entry(idx);
        } else {
            self.stack.push(route);
            self.current_index = 0;
            self.index_entry(0);
        }
    }

    /// Go back in history
    pub fn back(&mut self) -> bool {
        if self.can_go_back() {
            self.current_index -= 1;
            true
        } else {
            false
        }
    }

    /// Go forward in history
    pub fn forward(&mut self) -> bool {
        if self.can_go_forward() {
            self.current_index += 1;
            true
        } else {
            false
        }
    }

    /// Check if we can go back
    pub fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    /// Check if we can go forward
    pub fn can_go_forward(&self) -> bool {
        self.current_index < self.stack.len().saturating_sub(1)
    }

    /// Get the depth of the history stack
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Clear all history except the current route
    pub fn clear(&mut self) {
        if let Some(current) = self.current().cloned() {
            self.stack = vec![current];
            self.current_index = 0;
            self.rebuild_index();
        } else {
            self.stack.clear();
            self.current_index = 0;
            self.index.clear();
        }
    }

    /// Reset to a specific route, clearing all history
    pub fn reset(&mut self, route: Route) {
        self.stack = vec![route];
        self.current_index = 0;
        self.index.clear();
        self.index_entry(0);
    }

    /// Get all routes in the stack
    pub fn all_routes(&self) -> &[Route] {
        &self.stack
    }

    /// Current index into the history stack.
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    /// Preview the route selected by a `back()` operation without mutating history.
    pub fn preview_back(&self) -> Option<&Route> {
        if !self.can_go_back() {
            return None;
        }
        self.stack.get(self.current_index - 1)
    }

    /// Preview the route selected by a `forward()` operation without mutating history.
    pub fn preview_forward(&self) -> Option<&Route> {
        if !self.can_go_forward() {
            return None;
        }
        self.stack.get(self.current_index + 1)
    }

    /// Preview the route selected by a `pop()` operation without mutating history.
    pub fn preview_pop(&self) -> Option<&Route> {
        if self.stack.len() <= 1 {
            return None;
        }
        self.stack.get(self.stack.len() - 2)
    }

    /// Preview the route selected by a `pop_to()` operation without mutating history.
    pub fn preview_pop_to(&self, route_id: LiveId) -> Option<&Route> {
        if self.current().map(|r| r.id) == Some(route_id) {
            return None;
        }
        let pos = self.index.get(&route_id).and_then(|v| v.last().copied())?;
        self.stack.get(pos)
    }

    /// Preview the route selected by a `pop_to_root()` operation without mutating history.
    pub fn preview_pop_to_root(&self) -> Option<&Route> {
        if self.stack.len() <= 1 {
            return None;
        }
        self.stack.first()
    }

    /// Split history into stack + current index (for persistence).
    pub fn into_parts(self) -> (Vec<Route>, usize) {
        (self.stack, self.current_index)
    }

    /// Restore history from stack + current index.
    pub fn from_parts(stack: Vec<Route>, current_index: usize) -> Self {
        if stack.is_empty() {
            return Self::empty();
        }
        let current_index = current_index.min(stack.len().saturating_sub(1));
        let mut out = Self {
            stack,
            current_index,
            index: HashMap::new(),
        };
        out.rebuild_index();
        out
    }

    /// Sets the entire stack (stack-style semantics).
    ///
    /// - If `stack` is empty, the history becomes empty.
    /// - If `stack` is non-empty, the current route becomes the last element.
    pub fn set_stack(&mut self, stack: Vec<Route>) {
        if stack.is_empty() {
            self.stack.clear();
            self.current_index = 0;
            self.index.clear();
            return;
        }
        self.stack = stack;
        self.current_index = self.stack.len() - 1;
        self.rebuild_index();
    }

    /// Pops the current route (stack-style semantics).
    ///
    /// Unlike `back()`, this removes the current route from the stack and does not keep forward history.
    pub fn pop(&mut self) -> bool {
        if self.stack.len() <= 1 {
            return false;
        }
        let idx = self.stack.len() - 1;
        self.unindex_entry(idx);
        self.stack.pop();
        self.current_index = self.stack.len() - 1;
        true
    }

    /// Pops routes until `route_id` is the current route (stack-style semantics).
    ///
    /// Returns `false` if `route_id` does not exist in the stack or it is already the current route.
    pub fn pop_to(&mut self, route_id: LiveId) -> bool {
        let current = self.current().map(|r| r.id);
        if current == Some(route_id) {
            return false;
        }
        let Some(pos) = self.index.get(&route_id).and_then(|v| v.last().copied()) else {
            return false;
        };
        self.stack.truncate(pos + 1);
        self.current_index = pos;
        self.rebuild_index_for_truncate(pos + 1);
        true
    }

    /// Pops to the root route (stack-style semantics).
    pub fn pop_to_root(&mut self) -> bool {
        if self.stack.len() <= 1 {
            return false;
        }
        self.stack.truncate(1);
        self.current_index = 0;
        self.rebuild_index_for_truncate(1);
        true
    }

    fn rebuild_index(&mut self) {
        self.index.clear();
        for (i, r) in self.stack.iter().enumerate() {
            self.index.entry(r.id).or_default().push(i);
        }
        // Clamp in case stack was externally mutated.
        if self.stack.is_empty() {
            self.current_index = 0;
        } else {
            self.current_index = self.current_index.min(self.stack.len() - 1);
        }
    }

    fn index_entry(&mut self, idx: usize) {
        if let Some(route) = self.stack.get(idx) {
            self.index.entry(route.id).or_default().push(idx);
        }
    }

    fn unindex_entry(&mut self, idx: usize) {
        if let Some(route) = self.stack.get(idx) {
            if let Some(list) = self.index.get_mut(&route.id) {
                if let Some(pos) = list.iter().position(|&v| v == idx) {
                    list.remove(pos);
                }
                if list.is_empty() {
                    self.index.remove(&route.id);
                }
            }
        }
    }

    fn rebuild_index_for_truncate(&mut self, new_len: usize) {
        let keys: Vec<LiveId> = self.index.keys().copied().collect();
        for key in keys {
            if let Some(list) = self.index.get_mut(&key) {
                list.retain(|&idx| idx < new_len);
                if list.is_empty() {
                    self.index.remove(&key);
                }
            }
        }
        if self.stack.is_empty() {
            self.current_index = 0;
        } else {
            self.current_index = self.current_index.min(self.stack.len() - 1);
        }
    }
}

impl PartialEq for NavigationHistory {
    fn eq(&self, other: &Self) -> bool {
        self.stack == other.stack && self.current_index == other.current_index
    }
}

impl Eq for NavigationHistory {}

impl SerBin for NavigationHistory {
    fn ser_bin(&self, s: &mut Vec<u8>) {
        self.stack.ser_bin(s);
        self.current_index.ser_bin(s);
    }
}

impl DeBin for NavigationHistory {
    fn de_bin(o: &mut usize, d: &[u8]) -> Result<Self, DeBinErr> {
        let stack = <Vec<Route>>::de_bin(o, d)?;
        let current_index = usize::de_bin(o, d)?;
        Ok(Self::from_parts(stack, current_index))
    }
}

impl SerRon for NavigationHistory {
    fn ser_ron(&self, d: usize, s: &mut SerRonState) {
        s.st_pre();
        s.field(d + 1, "stack");
        self.stack.ser_ron(d + 1, s);
        s.conl();
        s.field(d + 1, "current_index");
        self.current_index.ser_ron(d + 1, s);
        s.out.push('\n');
        s.st_post(d);
    }
}

impl DeRon for NavigationHistory {
    fn de_ron(s: &mut DeRonState, i: &mut std::str::Chars) -> Result<Self, DeRonErr> {
        s.paren_open(i)?;
        let mut stack: Option<Vec<Route>> = None;
        let mut current_index: Option<usize> = None;
        loop {
            match s.tok {
                DeRonTok::ParenClose => {
                    s.paren_close(i)?;
                    break;
                }
                DeRonTok::Ident => {
                    let key = s.identbuf.clone();
                    s.ident(i)?;
                    s.colon(i)?;
                    match key.as_str() {
                        "stack" => stack = Some(Vec::<Route>::de_ron(s, i)?),
                        "current_index" => current_index = Some(usize::de_ron(s, i)?),
                        _ => {
                            return Err(DeRonErr {
                                msg: format!("Unexpected field {}", key),
                                line: s.line,
                                col: s.col,
                            });
                        }
                    }
                    s.eat_comma_paren(i)?;
                }
                _ => return Err(s.err_token("Identifier or )")),
            }
        }
        let stack = stack.unwrap_or_default();
        let current_index = current_index.unwrap_or(0);
        Ok(Self::from_parts(stack, current_index))
    }
}

#[cfg(test)]
mod tests {
    use makepad_live_id::live_id;

    use super::*;

    #[test]
    fn test_navigation_push() {
        let mut history = NavigationHistory::new(Route::new(live_id!(home)));
        assert_eq!(history.current().unwrap().id, live_id!(home));

        history.push(Route::new(live_id!(settings)));
        assert_eq!(history.current().unwrap().id, live_id!(settings));
        assert_eq!(history.depth(), 2);
    }

    #[test]
    fn test_navigation_back() {
        let mut history = NavigationHistory::new(Route::new(live_id!(home)));
        history.push(Route::new(live_id!(settings)));

        assert!(history.can_go_back());
        assert!(history.back());
        assert_eq!(history.current().unwrap().id, live_id!(home));
        assert!(!history.can_go_back());
    }

    #[test]
    fn test_navigation_replace() {
        let mut history = NavigationHistory::new(Route::new(live_id!(home)));
        history.replace(Route::new(live_id!(settings)));

        assert_eq!(history.current().unwrap().id, live_id!(settings));
        assert_eq!(history.depth(), 1);
        assert!(!history.can_go_back());
    }

    #[test]
    fn test_stack_pop() {
        let mut history = NavigationHistory::new(Route::new(live_id!(home)));
        history.push(Route::new(live_id!(settings)));
        history.push(Route::new(live_id!(profile)));

        assert!(history.pop());
        assert_eq!(history.current().unwrap().id, live_id!(settings));
        assert_eq!(history.depth(), 2);
        assert!(!history.can_go_forward());
    }

    #[test]
    fn test_stack_pop_to() {
        let mut history = NavigationHistory::new(Route::new(live_id!(home)));
        history.push(Route::new(live_id!(settings)));
        history.push(Route::new(live_id!(profile)));

        assert!(history.pop_to(live_id!(home)));
        assert_eq!(history.current().unwrap().id, live_id!(home));
        assert_eq!(history.depth(), 1);
    }

    #[test]
    fn test_stack_set_stack() {
        let mut history = NavigationHistory::empty();
        history.set_stack(vec![
            Route::new(live_id!(home)),
            Route::new(live_id!(settings)),
        ]);
        assert_eq!(history.current().unwrap().id, live_id!(settings));
        assert_eq!(history.depth(), 2);
        assert!(!history.can_go_forward());
    }
}
