use fatum_scene::NodeBehaviour;

trait Updatable : NodeBehaviour {
	fn update(&self, delta: std::time::Duration);
}
