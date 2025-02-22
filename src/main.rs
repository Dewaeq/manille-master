use bench::bench;

mod array;
mod bench;
mod bits;
mod card;
mod game;
mod players;
mod trick;

fn main() {
    romu::seed();

    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"bench".to_owned()) {
        let size = args.last().and_then(|x| x.parse::<usize>().ok());
        bench(size);
    }
}
