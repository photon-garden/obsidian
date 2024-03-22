pub trait VecExtension<Element> {
    /// ```rust
    /// use library_of_babel::extensions::VecExtension;
    ///
    /// let numbers = vec![1, 0, 2, 3, 0, 4, 5, 6];
    /// let without_zeros = numbers.split(|number| *number == 0);
    /// assert_eq!(without_zeros, vec![
    ///   vec![1],
    ///   vec![2, 3],
    ///   vec![4, 5, 6],
    /// ]);
    /// ```
    fn split(self, should_split: impl FnMut(&Element) -> bool) -> Vec<Vec<Element>>;
}

impl<Element> VecExtension<Element> for Vec<Element> {
    fn split(self, mut should_split: impl FnMut(&Element) -> bool) -> Vec<Vec<Element>> {
        let mut sub_vecs = vec![];
        let mut current_sub_vec = vec![];

        let mut save_current_sub_vec = |current_sub_vec: Vec<Element>| {
            if current_sub_vec.is_empty() {
                return;
            }
            sub_vecs.push(current_sub_vec);
        };

        for element in self {
            if should_split(&element) {
                save_current_sub_vec(current_sub_vec);
                current_sub_vec = vec![];
            } else {
                current_sub_vec.push(element);
            }
        }

        save_current_sub_vec(current_sub_vec);

        sub_vecs
    }
}
