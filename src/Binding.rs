
struct Binding {
    key: String,
    command: String,
}

impl Binding {
    fn new(key: String, command: String) -> Binding {
        Binding { key, command, children: None }
    }

}

impl Descendable<Binding> for Binding {
    fn descend(&self, key: &char[]) -> Option<Binding> {
        if (key.len() == 0) {
            return Some(self);
        }

        let mut currentChar = key[0];
        if let Some(children) = self.children {
            for (chr, binding) in children.iter() {
                if (chr == currentChar) {
                    if (key.len() == 1) {
                        return Some(binding);
                    } else {
                        return binding.descend(key[2..]);
                    }
                }
            }
        }

        return None;
    }
}
