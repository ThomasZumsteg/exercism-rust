use std::fmt;
use std::collections::HashMap;

#[allow(unused_variables)]

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = usize;
pub type CallbackID = (CellID, usize);

struct Cell<T> {
    value: T,
    function: Option<Box<Fn(&Reactor<T>, &[CellID]) -> Result<T, ()>>>,
    dependencies: Vec<CellID>,
    callbacks: HashMap<usize, Box<FnMut(T) -> ()>>,
    callback_counter: usize,
}

impl <T> Cell<T> {
    fn new(value: T) -> Self {
        Cell {
            value: value,
            function: None,
            dependencies: Vec::new(),
            callbacks: HashMap::new(),
            callback_counter: 0,
        }
    }
}

impl <T: fmt::Debug> fmt::Debug for Cell<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cell: ({:?})", self.value)
    }
}

pub struct Reactor<T> {
    cells: Vec<Cell<T>>
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl <'reactor, T: fmt::Debug + Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Reactor { cells: Vec::new() }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        self.cells.push(Cell::new(initial));
        self.cells.len() - 1
    }

    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(&mut self, dependencies: &[CellID], compute_func: F) -> Result<CellID, ()> {
        let function: Box<Fn(&Reactor<T>, &[CellID]) -> Result<T, ()>> = 
            Box::new(move |reactor, dependencies| {
                let mut values: Vec<T> = Vec::new();
                for &d in dependencies {
                    match reactor.cells.get(d) {
                        Some(v) => values.push(v.value),
                        None => return Err(()),
                    }
                }
                Ok(compute_func(&values))
            });
        match function(self, dependencies) {
            Ok(initial) => {
                self.cells.push(Cell {
                    value: initial,
                    function: Some(function),
                    dependencies: dependencies.to_vec(),
                    callback_counter: 0,
                    callbacks: HashMap::new(),
                });
                Ok(self.cells.len() - 1)
            },
            Err(e) => Err(e)
        }
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

    pub fn add_callback<F: FnMut(T) -> () + 'reactor>(&mut self, id: CellID, callback: F) -> Result<CallbackID, ()> {
        let mut cell = match self.cells.get_mut(id) {
            Some(c) => c,
            _ => return Err(())
        };
        cell.callbacks.insert(cell.callback_counter, Box::new(callback));
        unimplemented!();
    }

    pub fn remove_callback(&mut self, cell: CellID, callback: CallbackID) -> Result<(), ()> {
        unimplemented!()
    }

    fn update(&mut self) {
        for i in 0..self.cells.len() {
            let new_value;
            let old_value;
            {
                let cell = self.cells.get(i).unwrap();
                if let Some(ref function) = cell.function {
                    old_value = cell.value;
                    new_value = function(&self, &cell.dependencies).unwrap();
                } else {
                    continue;
                }
            }
            if old_value != new_value {
                let mut cell = self.cells.get_mut(i).unwrap();
                cell.value = new_value;
            }
        }
    }
}
