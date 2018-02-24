use std::fmt;

#[allow(unused_variables)]

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = usize;
pub type CallbackID = (CellID, usize);

struct Cell<T> {
    value: T,
    function: Option<Box<Fn(&Reactor<T>) -> Result<T, ()>>>,
    callbacks: Vec<Box<FnMut(T)>>,
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
impl <T: fmt::Debug + Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Reactor { cells: Vec::new() }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        self.cells.push(Cell {
            value: initial,
            function: None,
            callbacks: Vec::new() });
        self.cells.len() - 1
    }

    pub fn create_compute<F: Fn(&[T]) -> T>(&mut self, dependencies: &[CellID], compute_func: F) -> Result<CellID, ()> {
        let function: Box<Fn(&Reactor<T>) -> Result<T, ()>> = 
            Box::new( |reactor| {
                let mut values = Vec::new();
                for &d in dependencies {
                    match reactor.cells.get(d) {
                        Some(v) => values.push(v.value),
                        None => return Err(()),
                    }
                }
                Ok(compute_func(&values))
            });
        match function(self) {
            Ok(initial) => {
                self.cells.push(Cell {
                    value: initial,
                    function: None,
                    callbacks: Vec::new(),
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

    pub fn add_callback<F: FnMut(T) -> ()>(&mut self, id: CellID, callback: F) -> Result<CallbackID, ()> {
        unimplemented!()
    }

    pub fn remove_callback(&mut self, cell: CellID, callback: CallbackID) -> Result<(), ()> {
        unimplemented!()
    }

    fn update(&mut self) {
        for i in 0..self.cells.len() {
            let mut cell = self.cells.get_mut(i).unwrap();
            let function = match cell.function {
                Some(ref function) => function,
                _ => continue
            };
            let new_value = function(&self).unwrap();
            cell.value = new_value;
        }
    }
}
