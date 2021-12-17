use crate::{Map, Route};

#[derive(Debug)]
pub struct NaiveFinder<'a> {
    map: &'a Map,
    current_position: (usize, usize),
    current_route: usize,
    routes: Vec<Route>,
}

impl<'a> NaiveFinder<'a> {
    pub fn new(map: &'a Map) -> NaiveFinder<'a> {
        NaiveFinder {
            map,
            current_position: (0, 0),
            current_route: 0,
            routes: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        *self = NaiveFinder::new(self.map);
    }

    pub fn start(&mut self) -> Route {
        self.reset();

        loop {
            let mut new_routes = Vec::new();
            let mut not_looped = true;
            for route in self.routes.iter_mut() {
                not_looped = false;
                let mut founds = Vec::new();
                let mut minimum = usize::MAX;
                for (x, y) in route.next_steps() {
                    if route.contains(&(x, y)) {
                        continue;
                    };
                    if let Some(cost) = self.map.get(x, y).copied() {
                        let cost = cost as usize;
                        if cost < minimum {
                            founds = vec![(x, y)];
                            minimum = cost;
                            continue;
                        }
                        if cost == minimum {
                            founds.push((x, y));
                            minimum = cost;
                            continue;
                        }
                    }
                }

                for found in founds {
                    let mut new_route = route.clone();
                    new_route.push(found, minimum);
                    new_routes.push(new_route);
                }
            }
            if let Some(end_found) = self.routes.iter().find_map(|x| {
                if x.last() == Some(&self.map.endpoint()) {
                    Some(x)
                } else {
                    None
                }
            }) {
                return end_found.clone();
            }

            new_routes.sort_by_key(|x| x.cost / x.path.len());
            new_routes.truncate(50);
            self.routes = new_routes;
            if not_looped {
                break;
            }
        }

        Route::new()
    }
}
