const TEMPLATE: &str = include_str! ("../../public/index.html");

fn generate (content: &str, path: &str) -> String {
	let html = TEMPLATE.replace("{content}", content).replace("{path}", path);

	html
}
