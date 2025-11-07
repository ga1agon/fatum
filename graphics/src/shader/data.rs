use std::rc::Rc;

use bytemuck::Pod;

use crate::shader::ShaderProgram;

pub trait ShaderData<D> {
	fn push(&self);

	fn handle(&self) -> u64;
	fn name(&self) -> &str;
	fn binding(&self) -> u32;

	fn set_data(&mut self, data: Rc<Vec<D>>);

	// fn get_size(&self) -> usize;
	// fn set_size(&mut self, size: usize);

	// fn recalc_size(&mut self) {
	// 	let size = std::mem::size_of_val(self.get_data());
	// 	self.set_size(size);
	// }
}
