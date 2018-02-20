#[allow(unused_variables)]

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = usize;
pub type CallbackID = usize;

struct Cell<T> {
    value: T, 
    dependencies: Vec<Cell<T>>,
    update: Option<Box<Fn(&[T]) -> T>>,
    callbacks: Vec<Box<Fn(T)>>,
}

impl <T: Copy + PartialEq> Cell<T> {
    pub fn set_value(&mut self, new_value: T) {
        if new_value == self.value { return }
        self.value = new_value;
    }
}

pub struct Reactor<T> {
    cells: Vec<Cell<T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl <T: Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Reactor { cells: Vec::new() }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        self.cells.push(Cell {
            value: initial,
            callbacks: Vec::new(),
            update: None,
            dependencies: Vec::new(),
        });
        self.cells.len() - 1
    }

    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(&mut self, dependencies: &[CellID], compute_func: F) -> Result<CellID, ()> {
        let cells: Vec<Cell<T>> = dependencies.iter().map(|d| 
            match self.cells.get(d) {
                Some(c) => c,
                None => return Err(()),
            }).collect();
        let values = cells.iter().map(|c| c.value).collect();
        self.cells.push(Cell{
            value: compute_func(values),
            update: Some(Box::new(compute_func)),
            dependencies: cells,
            callbacks: Vec::new()
        });
        Ok(self.cells.len() - 1)
    }

    pub fn value(&self, id: CellID) -> Option<T> {
        self.cells.get(id).and_then(|c| Some(c.value))
    }

    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), ()> {
        let mut result = Err(());
        if let Some(cell) = self.cells.get_mut(id) {
            if cell.update.is_none() {
                cell.set_value(new_value);
                result = Ok(());
            }
        }
        if result.is_ok() { self.update(); }
        result
    }

    pub fn add_callback<F: FnMut(T) -> ()>(&mut self, id: CellID, callback: F) -> Result<CallbackID, ()> {
        unimplemented!()
    }

    pub fn remove_callback(&mut self, cell: CellID, callback: CallbackID) -> Result<(), ()> {
        unimplemented!()
    }

    fn update(&mut self) {
        for cell in &mut self.cells.iter_mut() {
            if cell.update.is_none() { continue; }
            unimplemented!()
        }
    }
}
