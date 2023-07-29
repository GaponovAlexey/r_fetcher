use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum BitBlock {
    Block(i32, RefCell<Rc<BitBlock>>),
    Genesis,
}

use BitBlock::{ Block, Genesis };

impl BitBlock {
    fn get_prev_block(&self) -> Option<&RefCell<Rc<BitBlock>>> {
        match self {
            Self::Block(_, i) => Some(i),
            Genesis => None,
        }
    }
}

fn main() {
    let bl1 = Rc::new(Block(1, RefCell::new(Rc::new(BitBlock::Genesis))));
    println!("{:?}", Rc::strong_count(&bl1))
}
