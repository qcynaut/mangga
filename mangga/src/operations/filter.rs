/// Filter
///
/// Filter used to filter documents
pub struct Filter<T, M> {
    f: T,
    _model: std::marker::PhantomData<M>,
}
