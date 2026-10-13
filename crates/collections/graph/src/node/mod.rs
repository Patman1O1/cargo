pub(super) struct Node<T> {
    data: T,
    edges: Vec<usize>
}

impl<T> Node {
    pub(super) fn new(data: T) -> Self {
        Self {
            data,
            edges: Vec::<usize>::new()
        }
    }
}
