use std::collections::HashMap;

#[allow(unused_variables)]

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = usize;
pub type CallbackID = (CellID, usize);

struct Cell<'cell, T> {
    value: T,
    function: Option<Box<Fn(&Reactor<T>) -> Result<T, ()> + 'cell>>,
    callbacks: HashMap<usize, Box<FnMut(T) -> ()>>,
    callback_counter: usize,
}

impl <'cell, T> Cell<'cell, T> {
    fn new(value: T) -> Self {
        Cell {
            value: value,
            function: None,
            callbacks: HashMap::new(),
            callback_counter: 0,
        }
    }
}

pub struct Reactor<'cell, T> {
    cells: Vec<Cell<'cell, T>>
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'r, T: Copy + PartialEq> Reactor<'r, T> {
    pub fn new() -> Self {
        Reactor { cells: Vec::new() }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        self.cells.push(Cell::new(initial));
        self.cells.len() - 1
    }

    pub fn create_compute<F: Fn(&[T]) -> T>(&'r mut self, dependencies: &[CellID], compute_func: F) -> Result<CellID, ()> {
        let function = |reactor: &Reactor<T>| {
                let mut values = Vec::new();
                for &d in dependencies {
                    if let Some(value) = reactor.value(d) { values.push(value); }
                    else { return Err(()); }
                }
                Ok(compute_func(&values))
            };
        // let function = |_: &Reactor<T>| Err(());
        let mut cell: Cell<T> = Cell::new(function(&self)?);
        cell.function = Some(Box::new(function));
        self.cells.push(cell);
        Ok(self.cells.len() - 1)
    }

    pub fn value(&self, id: CellID) -> Option<T> {
        self.cells.get(id).map(|c| c.value)
    }

    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), ()> {
        let mut result = Err(());
        if let Some(cell) = self.cells.get_mut(id) {
            if cell.function.is_none() {
                cell.value = new_value;
                result = Ok(());
            }
        }
        self.update();
        result
    }

    pub fn add_callback<F: FnMut(T) -> ()>(&mut self, id: CellID, callback: F) -> Result<CallbackID, ()> {
        unimplemented!();
    }

    pub fn remove_callback(&mut self, cell: CellID, callback: CallbackID) -> Result<(), ()> {
        unimplemented!()
    }

    fn update(&mut self) {
        for id in 0..self.cells.len() {
            let new_value = {
                let cell = self.cells.get(id).unwrap();
                if let Some(ref function) = cell.function {
                    function(self).unwrap()
                } else { continue; }
            };
            let mut cell = self.cells.get_mut(id).unwrap();
            cell.value = new_value;
        }
    }
}
