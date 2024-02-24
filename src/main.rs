use bracket_lib::prelude::*;

mod state;

fn main() -> BError {
    println!("CURRENTLY RUNNING....!");

    let context = BTermBuilder::simple80x50().with_title("Flappy").build()?;

    main_loop(context, state::State::new())
}
