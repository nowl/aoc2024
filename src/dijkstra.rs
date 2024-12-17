use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    hash::Hash,
    ops::Add,
    time::Instant,
};

/// A user type needs to implement this trait in order for the
/// algorithm to operate.
pub trait DijkstraInput {
    /// This represents the type of the `cost` attribute in the graph
    /// to move between nodes.
    type Cost;

    /// This represents the type of the identifier for nodes in the
    /// graph.
    type Index;

    /// This takes an index representing a node and must return a list
    /// of adjacent nodes and the cost required to reach each of those
    /// nodes.
    fn get_adjacent(&self, x: &Self::Index) -> Vec<(Self::Cost, Self::Index)>;
}

pub struct DijkstraMap<'a, T>
where
    T: DijkstraInput,
    T::Cost: Ord + Eq + PartialEq,
    T::Index: Eq + PartialEq,
    (T::Cost, T::Index): Ord,
{
    dijkstra_input: &'a T,
    unvisited_best_paths: HashMap<T::Index, (T::Cost, T::Index)>,
    unvisited: BinaryHeap<Reverse<(T::Cost, T::Index)>>,
    visited: HashMap<T::Index, (T::Cost, T::Index)>,

    config: DijkstraConfig,
}

#[derive(Default)]
pub struct DijkstraConfig {
    pub print_1000: bool,
}

impl<'a, T> DijkstraMap<'a, T>
where
    T: DijkstraInput,
    T::Cost: Ord + Eq + PartialEq + Debug + Add<Output = T::Cost> + Clone + Copy,
    T::Index: Eq + PartialEq + PartialOrd + Debug + Hash + Clone + Copy,
    (T::Cost, T::Index): Ord,
{
    /// Pass in an instance of your type implementing DijkstraInput
    pub fn new(dijkstra_input: &'a T, config: DijkstraConfig) -> Self {
        Self {
            dijkstra_input,
            unvisited_best_paths: HashMap::new(),
            unvisited: BinaryHeap::new(),
            visited: HashMap::new(),
            config,
        }
    }

    /// Run the algorithm from a starting location. Returns a mapping
    /// from all reachable indexes (from the starting location) to a
    /// previous index and the cost required to reach that index.
    pub fn run(&mut self, start: (T::Cost, T::Index)) -> &HashMap<T::Index, (T::Cost, T::Index)> {
        self.unvisited_best_paths
            .insert(start.1, (start.0, start.1));
        self.unvisited.push(Reverse((start.0, start.1)));

        let mut done = false;
        let mut timer = Instant::now();

        while !done {
            if self.config.print_1000 && self.visited.len() % 10000 == 0 {
                println!(
                    "dijkstra, unvisited: {}, visited: {}, elapsed: {}",
                    self.unvisited.len(),
                    self.visited.len(),
                    timer.elapsed().as_secs_f32()
                );
                timer = Instant::now();
            }

            let cur_index = self.unvisited.pop();

            if let Some(cur_index) = cur_index {
                let cur_index = cur_index.0;
                let (cost, cur_index) = cur_index;
                let (old_cost, prev_idx) = self.unvisited_best_paths.remove(&cur_index).unwrap();
                self.visited.insert(cur_index, (old_cost, prev_idx));

                let indexes = self.dijkstra_input.get_adjacent(&cur_index);
                for (neighbor_cost, neighbor_index) in indexes
                    .iter()
                    .filter(|(_, index)| !self.visited.contains_key(index))
                {
                    let alt_cost = *neighbor_cost + cost;
                    if let Some(val) = self.unvisited_best_paths.get_mut(neighbor_index) {
                        if val.0 > alt_cost {
                            val.0 = alt_cost;
                            val.1 = cur_index;
                        }
                    } else {
                        self.unvisited.push(Reverse((alt_cost, *neighbor_index)));
                        let new_item = (alt_cost, cur_index);
                        self.unvisited_best_paths.insert(*neighbor_index, new_item);
                    }
                }
            }
            if self.unvisited.is_empty() {
                done = true;
            }
        }

        &self.visited
    }
}

pub struct DijkstraMapAll<'a, T>
where
    T: DijkstraInput,
    T::Cost: Ord + Eq + PartialEq,
    T::Index: Eq + PartialEq,
    (T::Cost, T::Index): Ord,
{
    dijkstra_input: &'a T,
    unvisited_best_paths: HashMap<T::Index, (T::Cost, Vec<T::Index>)>,
    unvisited: BinaryHeap<Reverse<(T::Cost, T::Index)>>,
    visited: HashMap<T::Index, (T::Cost, Vec<T::Index>)>,

    config: DijkstraConfig,
}

impl<'a, T> DijkstraMapAll<'a, T>
where
    T: DijkstraInput,
    T::Cost: Ord + Eq + PartialEq + Debug + Add<Output = T::Cost> + Clone + Copy,
    T::Index: Eq + PartialEq + PartialOrd + Debug + Hash + Clone + Copy,
    (T::Cost, T::Index): Ord,
{
    /// Pass in an instance of your type implementing DijkstraInput
    pub fn new(dijkstra_input: &'a T, config: DijkstraConfig) -> Self {
        Self {
            dijkstra_input,
            unvisited_best_paths: HashMap::new(),
            unvisited: BinaryHeap::new(),
            visited: HashMap::new(),
            config,
        }
    }

    /// Run the algorithm from a starting location. Returns a mapping
    /// from all reachable indexes (from the starting location) to a
    /// previous index and the cost required to reach that index.
    pub fn run(
        &mut self,
        start: (T::Cost, T::Index),
    ) -> &HashMap<T::Index, (T::Cost, Vec<T::Index>)> {
        self.unvisited_best_paths
            .insert(start.1, (start.0, vec![start.1]));
        self.unvisited.push(Reverse((start.0, start.1)));

        let mut done = false;
        let mut timer = Instant::now();

        while !done {
            if self.config.print_1000 && self.visited.len() % 10000 == 0 {
                println!(
                    "dijkstra, unvisited: {}, visited: {}, elapsed: {}",
                    self.unvisited.len(),
                    self.visited.len(),
                    timer.elapsed().as_secs_f32()
                );
                timer = Instant::now();
            }

            let cur_index = self.unvisited.pop();

            if let Some(cur_index) = cur_index {
                let cur_index = cur_index.0;
                let (cost, cur_index) = cur_index;
                let (old_cost, prev_idxs) = self.unvisited_best_paths.remove(&cur_index).unwrap();
                self.visited.insert(cur_index, (old_cost, prev_idxs));

                let indexes = self.dijkstra_input.get_adjacent(&cur_index);
                for (neighbor_cost, neighbor_index) in indexes
                    .iter()
                    .filter(|(_, index)| !self.visited.contains_key(index))
                {
                    let alt_cost = *neighbor_cost + cost;
                    if let Some(val) = self.unvisited_best_paths.get_mut(neighbor_index) {
                        if val.0 > alt_cost {
                            val.0 = alt_cost;
                            val.1 = vec![cur_index];
                        } else if val.0 == alt_cost {
                            val.1.push(cur_index);
                        }
                    } else {
                        self.unvisited.push(Reverse((alt_cost, *neighbor_index)));
                        let new_item = (alt_cost, vec![cur_index]);
                        self.unvisited_best_paths.insert(*neighbor_index, new_item);
                    }
                }
            }
            if self.unvisited.is_empty() {
                done = true;
            }
        }

        &self.visited
    }

    fn extract_path_from_aux(
        start: &T::Index,
        end: &T::Index,
        data: &HashMap<T::Index, (T::Cost, Vec<T::Index>)>,
        prev: Vec<T::Index>,
    ) -> Vec<Vec<T::Index>> {
        if start == end {
            vec![prev]
        } else {
            let pre_ends = data.get(end).unwrap();
            let mut paths = vec![];
            for pre_end in pre_ends.1.iter() {
                let mut prev = prev.clone();
                prev.push(pre_end.clone());
                let new_paths = Self::extract_path_from_aux(start, pre_end, data, prev);
                paths.extend(new_paths);
            }
            paths
        }
    }

    pub fn extract_path_from(
        start: &T::Index,
        end: &T::Index,
        data: &HashMap<T::Index, (T::Cost, Vec<T::Index>)>,
    ) -> Vec<Vec<T::Index>> {
        Self::extract_path_from_aux(start, end, data, vec![end.clone()])
    }
}
