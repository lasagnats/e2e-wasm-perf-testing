use leptos::*;

pub mod app;

use app::HomePage;


fn main() {
	mount_to_body(|| {
		view! {
			<HomePage />
		}
	})
}
