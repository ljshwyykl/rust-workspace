use tracing::info;
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();
    let number_of_yaks = 3;
    info!(number_of_yaks,"preparing to shave yaks");

    // let number_shaved = yak_shave::shave_all(number_of_yaks);
    // info!(
    //     all_yaks_shaved = number_shaved == number_of_yaks,
    //     "yak shaving completed."
    // );
}
