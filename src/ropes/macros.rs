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


/// Generate struct definition and implementation of RopeSlice
///
/// This is done through a macro because it relies on the Lnode type which is different
/// depending on which type of Rope you are working with. This macro is used in both the
/// rope & src_rope modules of this crate. In both places, the Lnode type from that
/// module is passed in so we can generate the correct struct definition.
macro_rules! generate_ropeslice_struct {
    ($lnode: ty) => {
        // A view over a portion of a Rope. Analagous to string slices (`str`);
        pub struct RopeSlice<'rope> {
            // All nodes which make up the slice, in order.
            nodes: Vec<&'rope $lnode>,
            // The offset of the start point in the first node.
            start: usize,
            // The length of text in the last node.
            len: usize,
            // The index of the current byte - only used for iterating
            cur_byte: usize,
            // The index of the current node - only used for iterating
            cur_node: usize,
        }

        impl<'rope> RopeSlice<'rope> {
            fn empty<'r>() -> RopeSlice<'r> {
                RopeSlice {
                    nodes: vec![],
                    start: 0,
                    len: 0,
                    cur_node: 0,
                    cur_byte: 0,
                }
            }
        }

        impl<'rope> Iterator for RopeSlice<'rope> {
            type Item = u8;

            fn next(&mut self) -> Option<u8> {
                if self.cur_node >= self.nodes.len() {
                    return None;
                }

                let node = self.nodes[self.cur_node];
                let len = node.len;
                let text = node.text;

                // if this is the first node, add the 'start' position to the current byte offset
                if self.cur_node == 0 && self.cur_byte == 0 && self.start > 0 {
                    self.cur_byte += self.start;
                }


                // if this is the last node and we have a start offset...
                if self.cur_node == (self.nodes.len() - 1) && self.start > 0 {
                    let end_pos = len - self.start;
                    let next_byte = self.cur_byte + 1;

                    if next_byte > end_pos {
                        return None
                    }
                }

                if self.cur_byte >= len {
                    self.cur_byte = 0;
                    self.cur_node += 1;
                    return self.next();
                }


                let byte = {
                    let addr = text as usize + self.cur_byte;
                    self.cur_byte += 1;
                    let addr = addr as *const u8;
                    unsafe {
                        *addr
                    }
                };

                return Some(byte);
            }
        }
    }
}
