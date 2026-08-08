fn subtract_need(need: [i32; 4], contribution: [i32; 4]) -> [i32; 4] {
    [
        (need[0] - contribution[0]).max(0),
        (need[1] - contribution[1]).max(0),
        (need[2] - contribution[2]).max(0),
        (need[3] - contribution[3]).max(0),
    ]
}

// Exact minimum number of digits needed to provide
// 2^a * 3^b * 5^c * 7^d.
fn min_digits(need: [i32; 4]) -> i32 {
    let [a, b, c, d] = need;

    // Factors 5 and 7 each require their own digit.
    let mut count = c + d;

    // For factors 2 and 3, try how many digit 6's we use.
    // 6 contributes one 2 and one 3.
    let mut best = i32::MAX;

    for sixes in 0..=a.min(b) {
        let rem2 = a - sixes;
        let rem3 = b - sixes;

        // Remaining 2s:
        // 8 supplies three 2s, 4 supplies two, 2 supplies one.
        let digits_for_2 = (rem2 + 2) / 3;

        // Remaining 3s:
        // 9 supplies two 3s, 3 supplies one.
        let digits_for_3 = (rem3 + 1) / 2;

        best = best.min(sixes + digits_for_2 + digits_for_3);
    }

    count += best;
    count
}

fn build_suffix(
    len: usize,
    mut need: [i32; 4],
    factors: &[[i32; 4]; 10],
) -> Option<String> {
    let mut result = String::new();

    for pos in 0..len {
        let remaining = len - pos - 1;

        // Try the smallest digit first.
        for digit in 1..=9 {
            let next_need = subtract_need(need, factors[digit]);

            // Only choose this digit if the remaining requirements
            // can still fit in the remaining positions.
            if min_digits(next_need) <= remaining as i32 {
                result.push(char::from(b'0' + digit as u8));
                need = next_need;
                break;
            }
        }
    }

    if need == [0, 0, 0, 0] {
        Some(result)
    } else {
        None
    }
}

fn smallest_number(num: String, t: i64) -> String {
    // --------------------------------------------------
    // STEP 1: Factor t using only 2, 3, 5, and 7
    // --------------------------------------------------

    let mut x = t;

    // need = [power of 2, power of 3, power of 5, power of 7]
    let mut need = [0i32; 4];

    let primes = [2i64, 3, 5, 7];

    for i in 0..4 {
        while x % primes[i] == 0 {
            need[i] += 1;
            x /= primes[i];
        }
    }

    // If something remains, t contains another prime factor.
    //
    // Example:
    // 26 = 2 * 13
    //
    // No digit from 1..9 can provide factor 13.
    if x != 1 {
        return "-1".to_string();
    }

    // --------------------------------------------------
    // STEP 2: Prime factors contributed by each digit
    // --------------------------------------------------

    // Each entry means:
    //
    // [power of 2, power of 3, power of 5, power of 7]

    let factors: [[i32; 4]; 10] = [
        [0, 0, 0, 0], // 0 - never allowed
        [0, 0, 0, 0], // 1
        [1, 0, 0, 0], // 2 = 2
        [0, 1, 0, 0], // 3 = 3
        [2, 0, 0, 0], // 4 = 2^2
        [0, 0, 1, 0], // 5 = 5
        [1, 1, 0, 0], // 6 = 2 * 3
        [0, 0, 0, 1], // 7 = 7
        [3, 0, 0, 0], // 8 = 2^3
        [0, 2, 0, 0], // 9 = 3^2
    ];

    // Convert "1234" -> [1, 2, 3, 4]
    let digits: Vec<u8> = num.bytes().map(|b| b - b'0').collect();

    let n = digits.len();

    // --------------------------------------------------
    // STEP 3: Calculate remaining requirement
    // after every prefix of num
    // --------------------------------------------------

    // prefix_need[i] tells us what factors are still needed
    // after using digits 0..i.
    let mut prefix_need = vec![[0i32; 4]; n + 1];

    prefix_need[0] = need;

    // Track whether a zero exists in the prefix.
    let mut prefix_zero = vec![false; n + 1];

    for i in 0..n {
        prefix_zero[i + 1] = prefix_zero[i] || digits[i] == 0;

        if digits[i] == 0 {
            prefix_need[i + 1] = prefix_need[i];
        } else {
            prefix_need[i + 1] =
                subtract_need(prefix_need[i], factors[digits[i] as usize]);
        }
    }

    // --------------------------------------------------
    // STEP 4: Check whether num itself already works
    // --------------------------------------------------

    if !prefix_zero[n] && prefix_need[n] == [0, 0, 0, 0] {
        return num;
    }

    // --------------------------------------------------
    // STEP 5: Try to make the smallest same-length
    // number greater than num
    // --------------------------------------------------

    // Start from the RIGHT.
    //
    // Changing a later digit gives a smaller number than
    // changing an earlier digit.
    for i in (0..n).rev() {
        // We cannot preserve a prefix containing zero,
        // because the final number must be zero-free.
        if prefix_zero[i] {
            continue;
        }

        let original = digits[i] as usize;

        // Make this position larger than num[i].
        //
        // Once this happens, our whole number is > num,
        // so the suffix can be as small as possible.
        let start = (original + 1).max(1);

        for digit in start..=9 {
            let next_need =
                subtract_need(prefix_need[i], factors[digit]);

            let remaining = n - i - 1;

            // Can the remaining factors fit in the remaining digits?
            if min_digits(next_need) <= remaining as i32 {
                let mut answer = String::new();

                // Keep the unchanged prefix.
                for &d in &digits[..i] {
                    answer.push(char::from(b'0' + d));
                }

                // Put the larger digit here.
                answer.push(char::from(b'0' + digit as u8));

                // Build the smallest possible suffix.
                if let Some(suffix) =
                    build_suffix(remaining, next_need, &factors)
                {
                    answer.push_str(&suffix);
                    return answer;
                }
            }
        }
    }

    // --------------------------------------------------
    // STEP 6: No same-length answer exists
    // --------------------------------------------------
    //
    // Any number with more digits than num is automatically
    // greater than num.

    let mut len = n + 1;

    loop {
        if min_digits(need) <= len as i32 {
            if let Some(answer) = build_suffix(len, need, &factors) {
                return answer;
            }
        }

        len += 1;
    }
}

fn main() {
    // -------------------------
    // Example 1
    // -------------------------

    let num = "1234".to_string();
    let t = 256;

    let result = smallest_number(num, t);

    println!("Example 1");
    println!("num    = {}", "1234");
    println!("t      = {}", t);
    println!("result = {}", result);
    println!();

    // -------------------------
    // Example 2
    // -------------------------

    let num = "12355".to_string();
    let t = 50;

    let result = smallest_number(num, t);

    println!("Example 2");
    println!("num    = {}", "12355");
    println!("t      = {}", t);
    println!("result = {}", result);
    println!();

    // -------------------------
    // Example 3
    // -------------------------

    let num = "11111".to_string();
    let t = 26;

    let result = smallest_number(num, t);

    println!("Example 3");
    println!("num    = {}", "11111");
    println!("t      = {}", t);
    println!("result = {}", result);
}
