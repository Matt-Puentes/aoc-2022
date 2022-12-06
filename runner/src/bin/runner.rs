use took::Timer;

fn main() {
    let timer = Timer::new();
    runner::jobs().into_iter().for_each(|(j, _, i)| j(i));
    timer.took().describe("everything");
}
