use index_database::index::{
    events::events_function, subscribe_block_number::subscribe_block,
    subscribe_blocks::subscribe_to_blocks,
};

fn main() {
    // let _result = events_function();
    // let _result2 = subscribe_to_blocks();
    let _result3 = subscribe_block(500);
}
