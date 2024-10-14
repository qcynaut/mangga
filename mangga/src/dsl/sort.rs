use crate::Field;

/// Sort
pub struct Sort(bson::Document);

/// Sorts
pub trait Sorts {
    /// Ascending
    fn asc(self) -> Sort;

    /// Descending
    fn desc(self) -> Sort;
}

impl<T> Sorts for T
where
    T: Field,
{
    fn asc(self) -> Sort {
        let name = T::NAME.to_string();
        let mut doc = bson::Document::new();
        doc.insert(name, 1);
        Sort(doc)
    }

    fn desc(self) -> Sort {
        let name = T::NAME.to_string();
        let mut doc = bson::Document::new();
        doc.insert(name, -1);
        Sort(doc)
    }
}

impl From<Sort> for Option<bson::Document> {
    fn from(value: Sort) -> Self {
        Some(value.0)
    }
}

pub trait IntoSort {
    fn into_sort(self) -> bson::Document;
}

impl IntoSort for Vec<Sort> {
    fn into_sort(self) -> bson::Document {
        if self.is_empty() {
            bson::Document::new()
        } else {
            let mut doc = bson::Document::new();
            for sort in self {
                let sd = sort.0;
                doc.extend(sd);
            }
            doc
        }
    }
}
