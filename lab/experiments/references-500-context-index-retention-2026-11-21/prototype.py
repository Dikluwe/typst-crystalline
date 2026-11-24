"""Apply a root-batch location-map memo only to the laboratory RAM checkout."""
from pathlib import Path
import sys

root = Path(sys.argv[1]).resolve()
if not str(root).startswith("/dev/shm/"):
    raise ValueError("The experiment requires the RAM checkout.")

file = root / "03_infra/src/pipeline/context_stabilization.rs"
source = file.read_text()

def edit(old, new):
    global source
    assert source.count(old) == 1, old
    source = source.replace(old, new, 1)


edit("struct Session<'a> {", """/// The location projection of the provisional index, for one root batch.
#[derive(Default)]
struct LabTopologyLocations {
    locations: Option<HashMap<u64, Location>>,
    builds: usize,
    hits: usize,
    physical: usize,
    invalidations: usize,
    checked: usize,
    verify: bool,
}

impl LabTopologyLocations {
    fn changed(&mut self) {
        if self.locations.take().is_some() {
            self.invalidations += 1;
        }
    }
}

/// Session::body substitutes plain Empty for an absent or failed execution.
fn lab_empty_topology_product(node: &Producer) -> bool {
    node.current.as_ref().is_none_or(|execution| {
        matches!(&execution.result, Err(_) | Ok(Content::Empty))
    })
}

struct Session<'a> {""")

edit("""    fn realize(&mut self, index: usize, input: &Arc<TagIntrospector>) {
        if self.nodes[index].invalid {
            self.discard_descendants(index);""", """    fn realize(&mut self, index: usize, input: &Arc<TagIntrospector>) {
        self.realize_with_topology_locations(index, input, &mut None);
    }

    fn realize_with_topology_locations(
        &mut self,
        index: usize,
        input: &Arc<TagIntrospector>,
        retained: &mut Option<LabTopologyLocations>,
    ) {
        if self.nodes[index].invalid {
            if let Some(saved) = retained.as_mut() {
                if !self.nodes[index].children.is_empty() {
                    saved.changed();
                }
            }
            self.discard_descendants(index);""")

edit("""            let location = retained_location.or_else(|| {
                introspect_with_introspector(&self.running_topology())
                    .context_block_locations
                    .get(&self.nodes[index].elem.id)
                    .copied()
            });""", """            let id = self.nodes[index].elem.id;
            let location = match (retained_location, retained.as_mut()) {
                (Some(location), saved) => {
                    if let Some(saved) = saved {
                        saved.physical += 1;
                    }
                    Some(location)
                }
                (None, Some(saved)) => {
                    if let Some(locations) = &saved.locations {
                        saved.hits += 1;
                        if saved.verify {
                            let fresh = introspect_with_introspector(&self.running_topology());
                            assert_eq!(*locations, fresh.context_block_locations);
                            saved.checked += 1;
                        }
                    } else {
                        saved.builds += 1;
                        saved.locations = Some(
                            introspect_with_introspector(&self.running_topology())
                                .context_block_locations.clone(),
                        );
                    }
                    saved.locations.as_ref().unwrap().get(&id).copied()
                }
                (None, None) => introspect_with_introspector(&self.running_topology())
                    .context_block_locations
                    .get(&id)
                    .copied(),
            };""")
edit("""            self.execute(index, input, location);
            if self.impossible {""", """            let old_empty = retained.is_some() && lab_empty_topology_product(&self.nodes[index]);
            self.execute(index, input, location);
            if let Some(saved) = retained.as_mut() {
                if !old_empty || !lab_empty_topology_product(&self.nodes[index]) {
                    saved.changed();
                }
            }
            if self.impossible {""")
edit("""                Some(Some(blocks)) => {
                    for (elem, chain) in blocks {""", """                Some(Some(blocks)) => {
                    if let Some(saved) = retained.as_mut() {
                        if !blocks.is_empty() {
                            saved.changed();
                        }
                    }
                    for (elem, chain) in blocks {""")
edit("""        for child in self.nodes[index].children.clone() {
            self.realize(child, input);
            if self.impossible {""", """        for child in self.nodes[index].children.clone() {
            self.realize_with_topology_locations(child, input, retained);
            if self.impossible {""")

edit("""        let mut history = vec![seed];
        for attempt in 1..=5 {
            let snapshot = Arc::new(history.last().unwrap().clone());
            let input = &snapshot;
            for root in self.roots.clone() {
                self.realize(root, input);
                if self.impossible {
                    return (terminal(self.source, self.errors()), self.warnings());
                }
            }
            let content = self.content();""", """        let mut history = vec![seed];
        let lab_trace = std::env::var("TEKT_LAB_TOPOLOGY_INDEX_TRACE").ok();
        for attempt in 1..=5 {
            let snapshot = Arc::new(history.last().unwrap().clone());
            let input = &snapshot;
            // Origin, outline and running overlays are fixed throughout
            // this root batch. Drop the memo before updating any of them.
            let mut topology = (self.target == EvalTarget::Paged && !self.roots.is_empty())
                .then(|| LabTopologyLocations {
                    verify: lab_trace.as_deref() == Some("verify"),
                    ..LabTopologyLocations::default()
                });
            for root in self.roots.clone() {
                self.realize_with_topology_locations(root, input, &mut topology);
                if self.impossible {
                    return (terminal(self.source, self.errors()), self.warnings());
                }
            }
            if let Some(stats) = topology.as_ref().filter(|_| lab_trace.is_some()) {
                eprintln!(
                    "lab-topology-index attempt={attempt} roots={} builds={} hits={} physical={} invalidations={} checked={}",
                    self.roots.len(), stats.builds, stats.hits, stats.physical, stats.invalidations, stats.checked,
                );
            }
            drop(topology);
            let content = self.content();""")
file.write_text(source)
