use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct List<T: Serialize> {
    pub total_page: u64,
    pub list: Vec<T>,
}

impl<T> List<T>
where
    T: Serialize,
{
    pub fn new(total_page: u64) -> Self {
        Self {
            total_page,
            list: Vec::new(),
        }
    }

    pub fn from_vec(items: Vec<T>, total_page: u64) -> Self {
        let mut list = Self::new(total_page);

        for item in items {
            list.push(item);
        }

        return list;
    }

    pub fn push(&mut self, item: T) {
        self.list.push(item);
    }
}
