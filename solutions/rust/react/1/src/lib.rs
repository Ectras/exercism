use std::collections::{HashMap, HashSet, VecDeque};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

enum Cell<'a, T> {
    Input(T),
    Compute(
        T,
        Vec<CellId>,
        Box<dyn Fn(&[T]) -> T>,
        Vec<Option<Box<dyn FnMut(T) + 'a>>>,
    ),
}

pub struct Reactor<'a, T> {
    cells: Vec<Cell<'a, T>>,
    listeners: HashMap<CellId, Vec<ComputeCellId>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            listeners: HashMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        self.cells.push(Cell::Input(initial));
        InputCellId(self.cells.len() - 1)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        // Check dependencies
        for id in dependencies {
            let valid = match id {
                CellId::Input(InputCellId(idx)) => {
                    matches!(self.cells.get(*idx), Some(Cell::Input(..)))
                }
                CellId::Compute(ComputeCellId(idx)) => {
                    matches!(self.cells.get(*idx), Some(Cell::Compute(..)))
                }
            };
            if !valid {
                return Err(*id);
            }
        }

        // Compute the initial value
        let args = dependencies
            .iter()
            .map(|&dp| self.value(dp).unwrap())
            .collect::<Vec<_>>();
        let result = compute_func(&args);

        // Create the cell
        let cell = Cell::Compute(
            result,
            dependencies.to_vec(),
            Box::new(compute_func),
            Vec::default(),
        );

        // Add it to the cells
        self.cells.push(cell);
        let cell_id = ComputeCellId(self.cells.len() - 1);

        // Add it as listener to all dependencies
        for dependency in dependencies {
            self.listeners.entry(*dependency).or_default().push(cell_id);
        }

        Ok(cell_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(InputCellId(idx)) => {
                let cell = self.cells.get(idx)?;
                let Cell::Input(value) = cell else { panic!() };
                Some(*value)
            }
            CellId::Compute(ComputeCellId(idx)) => {
                let cell = self.cells.get(idx)?;
                let Cell::Compute(value, ..) = cell else {
                    panic!()
                };
                Some(*value)
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if let Some(Cell::Input(value)) = self.cells.get_mut(id.0) {
            // Set the new value
            *value = new_value;

            // Collect all primary listeners
            let Some(listeners) = self.listeners.get(&CellId::Input(id)) else {
                return true;
            };
            let mut needs_update = VecDeque::from_iter(listeners);
            let mut were_updated = HashSet::new();

            // Recursively update all listeners
            while let Some(outdated) = needs_update.pop_front() {
                // Compute the value again (using the updated value)
                let Cell::Compute(_, dependencies, compute_func, _) = &self.cells[outdated.0]
                else {
                    panic!()
                };
                let args = dependencies
                    .iter()
                    .map(|&dp| self.value(dp).unwrap())
                    .collect::<Vec<_>>();
                let result = compute_func(&args);

                // Get the current value
                let Cell::Compute(value, _, _, callbacks) = &mut self.cells[outdated.0] else {
                    panic!()
                };

                // Update the value (if it changed)
                if *value != result {
                    *value = result;
                    were_updated.insert(outdated);

                    // Add all further listeners
                    if let Some(listeners) = self.listeners.get(&CellId::Compute(*outdated)) {
                        needs_update.extend(listeners);
                    }
                }
            }

            // Trigger callbacks
            for updated in were_updated {
                let Cell::Compute(value, _, _, callbacks) = &mut self.cells[updated.0] else {
                    panic!()
                };
                for callback in callbacks {
                    if let Some(callback) = callback {
                        callback(*value)
                    }
                }
            }
            true
        } else {
            false
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        // Get callbacks from cell
        let cell = self.cells.get_mut(id.0)?;
        let Cell::Compute(_, _, _, callbacks) = cell else {
            panic!()
        };

        // Add to callbacks
        callbacks.push(Some(Box::new(callback)));
        Some(CallbackId(callbacks.len() - 1))
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        // Get callbacks from cell
        let cell = self
            .cells
            .get_mut(cell.0)
            .ok_or(RemoveCallbackError::NonexistentCell)?;
        let Cell::Compute(_, _, _, callbacks) = cell else {
            panic!()
        };

        if callbacks[callback.0].is_some() {
            callbacks[callback.0] = None;
            Ok(())
        } else {
            Err(RemoveCallbackError::NonexistentCallback)
        }
    }
}
