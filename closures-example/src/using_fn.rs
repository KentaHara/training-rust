use std::thread;
use std::time::Duration;

fn main() {
	let simulated_user_specified_value = 10;
	let simulated_random_number = 7;

	generate_workout(simulated_user_specified_value, simulated_random_number);
}

///
/// 実行に約2秒かかる仮想関数
///
fn simulated_expensive_calculation(intensity: u32) -> u32 {
	println!("calculating slowly...");
	let sec: u64 = 2;
	let duration: Duration = Duration::from_secs(sec);
	thread::sleep(duration);
	intensity
}

///
/// トレーニング計画を出力する関数
///
fn generate_workout(intensity: u32, random_number: u32) {
	let expensive_result = simulated_expensive_calculation(intensity);

	if intensity < 25 {
		println!("Today, do {} pushups!", expensive_result);
		println!("Next, do {} situps!", expensive_result);
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!("Today, run for {} minutes!", expensive_result);
		}
	}
}
