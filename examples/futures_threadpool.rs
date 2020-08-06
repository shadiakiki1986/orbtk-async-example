use futures::executor::ThreadPool;

fn main() {
let pool = ThreadPool::new().unwrap();

let future = async {
    println!("Sleep 3 seconds");
  let sleep_dur = std::time::Duration::from_secs(3);
  std::thread::sleep(sleep_dur);
    println!("{}", 123);
};
pool.spawn_ok(future);
}
