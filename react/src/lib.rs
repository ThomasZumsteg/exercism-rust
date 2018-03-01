use std::fmt;
use std::collections::HashMap;

#[allow(unused_variables)]

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = usize;
pub type CallbackID = (CellID, usize);

struct Cell<T> {
    value: T,
    function: Option<Box<Fn(&[T]) -> T>>,
    dependencies: Vec<CellID>,
    callbacks: HashMap<usize, Box<FnMut(T) -> ()>>,
    callback_counter: usize,
}

impl <T> Cell<T> {
    fn new(value: T) -> Self {
        Cell {
            value: value,
            dependencies: Vec::new(),
            function: None,
            callbacks: HashMap::new(),
            callback_counter: 0,
        }
    }
}

pub struct Reactor<T> {
    cells: Vec<Cell<T>>
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<T: fmt::Debug + Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Reactor { cells: Vec::new() }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        self.cells.push(Cell::new(initial));
        self.cells.len() - 1
    }

    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(&mut self, dependencies: &[CellID], compute_func: F) -> Result<CellID, ()> {
        let function = Box::new(compute_func);
        let value = self.compute_value(&function, dependencies);
        let mut cell = Cell::new(value?);
        cell.function = Some(function);
        cell.dependencies = dependencies.to_vec();
        self.cells.push(cell);
        Ok(self.cells.len() - 1)
    }

    fn compute_value<F: Fn(&[T]) -> T>(&self, compute_func: &Box<F>, dependencies: &[CellID]) -> Result<T, ()> {
        let mut values = Vec::new();
        for &d in dependencies {
            if let Some(v) = self.value(d) {
                values.push(v);
            } else { return Err(()) }
        }
        Ok(compute_func(&values))
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
            let function;
            let new_value;
            let dependencies;
            {
                // replace function on cell with None
                let mut cell = self.cells.get_mut(id).unwrap();
                function = cell.function.take();
                dependencies = cell.dependencies.clone();
            }
            if let Some(function) = function {
                // assign new_value to cell.value
                let mut values = Vec::new();
                for d in dependencies {
                    values.push(self.value(d).unwrap());
                }
                new_value = function(&values);
                let mut cell = self.cells.get_mut(id).unwrap();
                cell.value = new_value;
                cell.function = Some(function);
            }
        }
    }
}
