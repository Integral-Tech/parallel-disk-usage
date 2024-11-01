use super::DataTree;
use crate::size;
use rayon::prelude::*;

impl<Name, Size> DataTree<Name, Size>
where
    Self: Send,
    Size: size::Size,
{
    /// Recursively cull all descendants that do not satisfy given `predicate`, in parallel.
    pub fn par_retain(&mut self, predicate: impl Fn(&Self) -> bool + Copy + Sync) {
        self.children.retain(predicate);
        self.children
            .par_iter_mut()
            .for_each(|child| child.par_retain(predicate));
    }

    /// Process the tree via [`par_retain`](Self::par_retain) method.
    pub fn into_par_retained(mut self, predicate: impl Fn(&Self) -> bool + Copy + Sync) -> Self {
        self.par_retain(predicate);
        self
    }

    /// Recursively cull all descendants whose sizes are too small relative to root.
    #[cfg(feature = "cli")]
    pub fn par_cull_insignificant_data(&mut self, min_ratio: f32)
    where
        Size: Into<u64>,
    {
        let minimal = self.size().into() as f32 * min_ratio;
        self.par_retain(|descendant| descendant.size().into() as f32 >= minimal);
    }

    /// Process the tree via [`par_cull_insignificant_data`](Self::par_cull_insignificant_data) method.
    #[cfg(test)]
    #[cfg(feature = "cli")]
    fn into_insignificant_data_par_culled(mut self, min_ratio: f32) -> Self
    where
        Size: Into<u64>,
    {
        self.par_cull_insignificant_data(min_ratio);
        self
    }
}

#[cfg(test)]
#[cfg(feature = "cli")]
mod test;
