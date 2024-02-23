use log::info;

pub fn write_to_file(filename: &str, asm: &str) {
	std::fs::write(filename, asm).expect("Failed to write assembly to file");
	info!("Assembly code written to {}", filename);
}
