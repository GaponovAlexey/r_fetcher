use r_fetcher::*;
use futures::executor::block_on;

fn main() {
    // block_on(eat_play());
    block_on(eat_play_concurrently())
    // task_print()
}
