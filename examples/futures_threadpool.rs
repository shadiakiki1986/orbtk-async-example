use futures::executor::ThreadPool;

fn main() {
let pool = ThreadPool::new().unwrap();

let future = async {
    println!("Thread: Sleep 3 seconds");
  let sleep_dur1 = std::time::Duration::from_secs(3);
  std::thread::sleep(sleep_dur1);
    println!("Thread: wake up");
};

pool.spawn_ok(future);

println!("Master: Sleep 6 seconds");
let sleep_dur2 = std::time::Duration::from_secs(6);
std::thread::sleep(sleep_dur2);
println!("Master: wake up");

}
