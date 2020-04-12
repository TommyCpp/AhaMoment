use std::collections::BTreeMap;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct InputCellID(u32);

/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct ComputeCellID(u32);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct CallbackID(u32);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

type ComputeFunc<'a, T> = Box<dyn Fn(&'a [T]) -> T>;
type CallBackFunc<T> = Box<dyn FnMut(T) -> ()>;

#[derive(Clone)]
struct InputCell<T> {
    id: InputCellID,
    _val: T,
    compute_cells: Vec<ComputeCellID>,
}


struct ComputeCell<'a, T> {
    id: ComputeCellID,
    dependencies: Vec<CellID>,
    compute_cells: Vec<ComputeCellID>,
    func: ComputeFunc<'a, T>,
    _val: T,
    callbacks: BTreeMap<CallbackID, CallBackFunc<T>>, // callback will b.e called without order.
}

pub struct Reactor<'a, T> {
    input_cells: BTreeMap<InputCellID, InputCell<T>>,
    compute_cells: BTreeMap<ComputeCellID, ComputeCell<'a, T>>,
    id_generator: u32,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: BTreeMap::new(),
            compute_cells: BTreeMap::new(),
            id_generator: 0,
        }
    }

    pub fn get_next_id(&mut self) -> u32 {
        self.id_generator = self.id_generator + 1;
        self.id_generator
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, _initial: T) -> InputCellID {
        let cell = InputCell {
            id: InputCellID(self.get_next_id()),
            _val: _initial,
            compute_cells: vec![],
        };
        self.input_cells.insert(cell.id.clone(), cell.clone());
        cell.id.clone()
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
        _dependencies: &[CellID],
        _compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let mut input: Vec<T> = vec![];
        for cell_id in _dependencies {
            let val = self.value(cell_id.clone());
            if val.is_none() {
                return Err(cell_id.clone());
            } else {
                input.push(val.unwrap());
            }
        }

        let current_cell_id = ComputeCellID(self.get_next_id());
        for cell_id in _dependencies {
            match cell_id {
                CellID::Input(id) => {
                    self.input_cells.get_mut(id).unwrap().compute_cells.push(current_cell_id.clone());
                }
                CellID::Compute(id) => {
                    self.compute_cells.get_mut(id).unwrap().compute_cells.push(current_cell_id.clone());
                }
            }
        }

        let init = _compute_func(input.as_ref());
        let cell = ComputeCell {
            id: current_cell_id.clone(),
            dependencies: Vec::from(_dependencies),
            compute_cells: vec![],
            func: Box::new(_compute_func),
            _val: init,
            callbacks: BTreeMap::new(),
        };
        self.compute_cells.insert(current_cell_id.clone(), cell);
        Ok(current_cell_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(cell_id) => {
                match self.input_cells.get(&cell_id) {
                    Some(cell) => Some(cell._val),
                    None => None
                }
            }
            CellID::Compute(cell_id) => {
                match self.compute_cells.get(&cell_id) {
                    Some(cell) => Some(cell._val),
                    None => None
                }
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, _id: InputCellID, _new_value: T) -> bool {
        unimplemented!()
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
    pub fn add_callback<F: FnMut(T) -> ()>(
        &mut self,
        _id: ComputeCellID,
        _callback: F,
    ) -> Option<CallbackID> {
        unimplemented!()
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        unimplemented!(
            "Remove the callback identified by the CallbackID {:?} from the cell {:?}",
            callback,
            cell,
        )
    }
}
