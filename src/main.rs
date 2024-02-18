use rand::Rng;

extern crate rand;

// sample size of experiement.
const EPOCH: u32 = 10000000;

fn main() {
    /*
    * holds count of times when:
        * 6 dice get exactly one 6
        * 12 dice get exactly two 6s
        * 18 dice get exactly three 6s
    */
    let mut exactly_n_6s: [u32; 3] = [0; 3];

    /*
    * holds count of times when:
        * 6 dice get atleast one 6
        * 12 dice get atleast two 6s
        * 18 dice get atleast three 6s
    */
    let mut atleast_n_6s: [u32; 3] = [0; 3];

    let mut thread_rng = rand::thread_rng();
    let start_time = std::time::Instant::now();

    for _ in 1..=EPOCH {

        // roll 6 dice
        let number_of_6s = roll_n_dice(6, &mut thread_rng);
        if number_of_6s == 1 {exactly_n_6s[0] += 1}
        if number_of_6s >= 1 {atleast_n_6s[0] += 1}

        // roll 12 dice
        let number_of_6s = roll_n_dice(12, &mut thread_rng);
        if number_of_6s == 2 {exactly_n_6s[1] += 1}
        if number_of_6s >= 2 {atleast_n_6s[1] += 1}

        // roll 18 dice
        let number_of_6s = roll_n_dice(18, &mut thread_rng);
        if number_of_6s == 3 {exactly_n_6s[2] += 1}
        if number_of_6s >= 3 {atleast_n_6s[2] += 1}
    }

    // log amount of time taken
    let end_time = std::time::Instant::now();
    let elapsed = end_time.duration_since(start_time).as_millis() as f64 / 1000.0;
    println!("Completed in {}s", elapsed);

    // print results of expiriment
    for i in 0..3 {
        let dice_count = (i + 1) * 6;
        let expected_amount_of_6s = dice_count / 6;
        let exact_percent = exactly_n_6s[i] as f64 / EPOCH as f64 * 100.0;
        let atleast_percent = atleast_n_6s[i] as f64 / EPOCH as f64 * 100.0;
        println!("{dice_count} dice: exactly {expected_amount_of_6s} six(es) {}%; atleast {expected_amount_of_6s} six(es) {}%.",
            exact_percent, atleast_percent);
    }

}

// rolls n dice and returns the number of 6s
fn roll_n_dice(n: u32, thread_rng: &mut rand::prelude::ThreadRng) -> u32 {
    let mut number_of_6s = 0;
    for _ in 0..n {
        let dice_result = thread_rng.gen_range(1..=6);
        if dice_result == 6 {number_of_6s += 1;}
    }
    number_of_6s
}
