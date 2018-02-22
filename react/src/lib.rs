#[allow(unused_variables)]

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = usize;
pub type CallbackID = (CellID, usize);

pub struct Reactor<T> {
    id: usize,
    values: Vec<T>,
    update: Vec<Option<Box<Fn(&[T]) -> T>>>,
    dependencies: Vec<Vec<CellID>>,
    callbacks: Vec<Vec<Box<FnMut(T) -> ()>>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl <T: Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Reactor { 
            id: 0,
            values: Vec::new(),
            update: Vec::new(),
            dependencies: Vec::new(),
            callbacks: Vec::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        self.id += 1;
        self.update.push(None);
        self.values.push(initial);
        self.callbacks.push(Vec::new());
        self.dependencies.push(Vec::new());
        self.id - 1
    }

    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(&mut self, dependencies: &[CellID], compute_func: F) -> Result<CellID, ()> {
        let mut values = Vec::new();
        for &d in dependencies {
            match self.values.get(d) {
                Some(&v) => values.push(v),
                None => return Err(()),
            }
        }
        let initial = compute_func(&values);
        let id = self.create_input(initial);
        self.update[id] = Some(Box::new(compute_func));
        self.dependencies[id] = dependencies.to_vec();
        Ok(id)
    }

    pub fn value(&self, id: CellID) -> Option<T> {
        self.values.get(id).map(|&v| v)
    }

    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), ()> {
        println!("Id is {}, self.id is {}, self.update\n", id, self.id);
        if id < self.id && self.update.get(id).unwrap().is_none() {
            self.values[id] = new_value;
            self.update();
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn add_callback<F: FnMut(T) -> () + 'static>(&mut self, id: CellID, callback: F) -> Result<CallbackID, ()> {
        if let Some(callbacks) = self.callbacks.get_mut(id) {
            callbacks.push(Box::new(callback)); 
            Ok((id, callbacks.len()-1))
        } else { Err(()) }
    }

    pub fn remove_callback(&mut self, cell: CellID, callback: CallbackID) -> Result<(), ()> {
        unimplemented!()
    }

    fn update(&mut self) {
        for id in 0..self.id {
            if let Some(&Some(ref update)) = self.update.get(id) {
                let mut values = Vec::new();
                for &d in self.dependencies.get(id).unwrap() {
                    values.push(self.values.get(d).unwrap().clone());
                }
                self.values[id] = update(&values);
            }
        }
    }
}
