use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use niri_ipc::{Action, Request, WorkspaceReferenceArg};

use crate::{Error, NiriState};

#[derive(Clone, Debug, Eq, PartialEq)]
struct SortState {
    idx: u8,
    id: u64,
}

impl Ord for SortState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id).then(self.idx.cmp(&other.idx))
    }
}

impl PartialOrd for SortState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl NiriState {
    /// Sort workspaces by Id across each monitor
    pub fn sort_workspaces(&mut self) -> Result<(), Error> {
        let mut outputs_by_monitor = self.group_by_output();
        for (output, ws_ids) in outputs_by_monitor.iter_mut() {
            match output {
                Some(_output) => self.apply_output_order(ws_ids)?,
                None => {}
            };
        }
        Ok(())
    }

    fn group_by_output(&self) -> HashMap<Option<String>, BinaryHeap<Reverse<SortState>>> {
        let mut outputs_by_monitor: HashMap<Option<String>, BinaryHeap<Reverse<SortState>>> =
            HashMap::new();

        for workspace in &self.workspaces {
            match outputs_by_monitor.get_mut(&workspace.output) {
                Some(heap) => heap.push(Reverse(SortState {
                    idx: workspace.idx,
                    id: workspace.id,
                })),
                None => {
                    let mut heap = BinaryHeap::new();
                    heap.push(Reverse(SortState {
                        idx: workspace.idx,
                        id: workspace.id,
                    }));
                    outputs_by_monitor.insert(workspace.output.clone(), heap);
                }
            };
        }

        return outputs_by_monitor;
    }

    fn apply_output_order(
        &mut self,
        workspaces: &mut BinaryHeap<Reverse<SortState>>,
    ) -> Result<(), Error> {
        // Only move workspaces which are at the incorrect index.
        // Every workspace before `count` is at the correct location,
        // meaning the list fully reflect the sorted state
        // by the time the priority queue has been exhausted
        let mut count = 0;
        while let Some(Reverse(workspace)) = workspaces.pop() {
            count += 1; // Niri workspaces are 1-indexed
            if workspace.idx == count {
                continue;
            }

            let request = Request::Action(Action::MoveWorkspaceToIndex {
                index: count as usize,
                reference: Some(WorkspaceReferenceArg::Id(workspace.id)),
            });
            self.socket.send(request)??;
        }

        Ok(())
    }
}
