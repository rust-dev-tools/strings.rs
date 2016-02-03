/// Generate implementation of common methods shared between
/// both Rope variants (rope & src_rope).
macro_rules! impl_rope {
    ($ty: ty) => {
        impl $ty {
            pub fn len(&self) -> usize {
                self.len
            }

            pub fn insert_copy(&mut self, start: usize, text: &str) {
                // FIXME If we did clever things with allocation, we could do better here.
                self.insert(start, text.to_string());
            }

            pub fn push(&mut self, text: String) {
                let len = self.len();
                self.insert(len, text);
            }

            pub fn push_copy(&mut self, text: &str) {
                // If we did clever things with allocation, we could do better here
                let len = self.len();
                self.insert(len, text.to_string());
            }

            fn remove_inner<F>(&mut self,
                               start: usize,
                               end: usize,
                               do_remove: F)
                where F: Fn(&mut Rope) -> NodeAction
            {
                assert!(end >= start);
                if start == end {
                    return;
                }

                match do_remove(self) {
                    NodeAction::None => {}
                    NodeAction::Remove => {
                        self.root = Node::empty_inner();
                        self.len = 0;
                    }
                    NodeAction::Adjust(adj) => self.len = (self.len as isize + adj) as usize,
                    NodeAction::Change(node, adj) => {
                        self.root = *node;
                        self.len = (self.len as isize + adj) as usize;
                    }
                }
            }

            // This can go horribly wrong if you overwrite a grapheme of different size.
            // It is the callers responsibility to ensure that the grapheme at point start
            // has the same size as new_char.
            pub fn replace(&mut self, start: usize, new_char: char) {
                assert!(start + new_char.len_utf8() <= self.len);
                // This is pretty wasteful in that we're allocating for no point, but
                // I think that is better than duplicating a bunch of code.
                // It should be possible to view a &char as a &[u8] somehow, and then
                // we can optimise this (FIXME).
                self.replace_str(start, &new_char.to_string()[..]);
            }

            pub fn replace_str(&mut self, start: usize, new_str: &str) {
                assert!(start + new_str.len() <= self.len);
                self.root.replace(start, new_str);
            }

            pub fn slice(&self, Range { start, end }: Range<usize>) -> RopeSlice {
                // This could be true for two cases
                //    1. The Rope is empty (start == end == self.len == 0)
                //    2. Attempting to slice the end of the rope (start == end == self.len)
                if start == end {
                    return RopeSlice::empty();
                }

                debug_assert!(end > start && start <= self.len && end <= self.len);

                let mut result = RopeSlice::empty();
                self.root.find_slice(start, end, &mut result);
                result
            }

            pub fn full_slice(&self) -> RopeSlice {
                self.slice(0..self.len)
            }

            pub fn chars(&self) -> RopeChars {
                RopeChars {
                    data: self.full_slice(),
                    cur_node: 0,
                    cur_byte: 0,
                    abs_byte: 0,
                }
            }
        }
    }
}
