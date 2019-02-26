use dedup::dedup_via_btreeset;

fn main() {
    dedup_via_btreeset(&std::env::args().collect::<Vec<_>>()).iter().for_each(|e| println!("{:?}", e));
}

