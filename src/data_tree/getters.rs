use super::DataTree;
use crate::size;

impl<Name, Size: size::Size> DataTree<Name, Size> {
    /// Extract name
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Get mutable reference to name.
    pub fn name_mut(&mut self) -> &mut Name {
        &mut self.name
    }

    /// Extract total disk usage
    pub fn size(&self) -> Size {
        self.size
    }

    /// Extract children
    pub fn children(&self) -> &Vec<Self> {
        &self.children
    }
}
