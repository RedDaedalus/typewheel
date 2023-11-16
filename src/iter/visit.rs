use crate::Component;

pub enum Visit<'a> {
	Push(&'a Component),
	Pop(&'a Component),
}
