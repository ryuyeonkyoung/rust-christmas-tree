mod animation;
mod colors;
mod ornament;
mod tree;

use animation::AnimationLoop;

fn main() {
    println!("🎄 Rust Christmas Tree Animation 🎄");
    println!("Starting animation...\n");

    let mut animation = AnimationLoop::new();

    if let Err(e) = animation.run() {
        eprintln!("Error running animation: {}", e);
        std::process::exit(1);
    }

    println!("\n🎅 Merry Christmas! 🎅");
}
