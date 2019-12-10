pub fn run() {
    let mut part_1 = 0;
    let mut part_2 = 0;

    // It is a six-digit number.
    for p_1 in 0..10 {
        for p_2 in 0..10 {
            for p_3 in 0..10 {
                for p_4 in 0..10 {
                    for p_5 in 0..10 {
                        for p_6 in 0..10 {
                            // Part 1 & Part 2

                            // The value is within the range given in your puzzle input.
                            let n = p_1 * 100000
                                + p_2 * 10000
                                + p_3 * 1000
                                + p_4 * 100
                                + p_5 * 10
                                + p_6;
                            if n < 156218 || 652527 < n {
                                continue;
                            }

                            // Two adjacent digits are the same (like 22 in 122345).
                            if !(p_1 == p_2 || p_2 == p_3 || p_3 == p_4 || p_4 == p_5 || p_5 == p_6)
                            {
                                continue;
                            }

                            // Going from left to right, the digits never decrease;
                            if p_1 > p_2 || p_2 > p_3 || p_3 > p_4 || p_4 > p_5 || p_5 > p_6 {
                                continue;
                            }

                            part_1 += 1;

                            // Only for part 2

                            // The two adjacent matching digits are not part of a larger group of matching digits.
                            let ps = [p_1, p_2, p_3, p_4, p_5, p_6];
                            let mut two = false;
                            let mut more = false;
                            let mut last = 11;
                            for p in ps.iter() {
                                if *p == last && two {
                                    two = false;
                                    more = true;
                                    continue;
                                }
                                if *p != last && two {
                                    break;
                                }
                                if *p == last && more {
                                    continue;
                                }
                                more = false;
                                if *p == last {
                                    two = true;
                                }
                                last = *p;
                            }

                            if !two {
                                continue;
                            }

                            part_2 += 1;
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
