use futures::executor::ThreadPool;

fn main() {
let pool = ThreadPool::new().unwrap();

let future = async {
    println!("{}", 123);
};
pool.spawn_ok(future);
}
