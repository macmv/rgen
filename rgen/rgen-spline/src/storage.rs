pub trait SplineStorage {
  fn len(&self) -> usize;
  fn get(&self, index: usize) -> (f64, f64);
  fn binary_search(&self, key: f64) -> usize;
}

impl SplineStorage for Vec<(f64, f64)> {
  fn len(&self) -> usize { self.len() }
  fn get(&self, index: usize) -> (f64, f64) { self[index] }
  fn binary_search(&self, key: f64) -> usize {
    match self.binary_search_by(|(k, _)| k.partial_cmp(&key).unwrap()) {
      Ok(i) => i,
      Err(i) => i,
    }
  }
}

impl SplineStorage for [(f64, f64)] {
  fn len(&self) -> usize { self.as_ref().len() }
  fn get(&self, index: usize) -> (f64, f64) { self[index] }
  fn binary_search(&self, key: f64) -> usize {
    match self.binary_search_by(|(k, _)| k.partial_cmp(&key).unwrap()) {
      Ok(i) => i,
      Err(i) => i,
    }
  }
}

impl SplineStorage for &[(f64, f64)] {
  fn len(&self) -> usize { self.as_ref().len() }
  fn get(&self, index: usize) -> (f64, f64) { self[index] }
  fn binary_search(&self, key: f64) -> usize {
    match self.binary_search_by(|(k, _)| k.partial_cmp(&key).unwrap()) {
      Ok(i) => i,
      Err(i) => i,
    }
  }
}
