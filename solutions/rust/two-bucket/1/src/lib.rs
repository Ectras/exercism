use std::cmp::min;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut buckets = [0, 0];
    let capacities = [capacity_1, capacity_2];
    let (main_bucket, side_bucket) = match start_bucket {
        Bucket::One => (0, 1),
        Bucket::Two => (1, 0),
    };
    let mut moves = 0;

    if capacities[side_bucket] == goal {
        return Some(BucketStats {
            moves: 2,
            goal_bucket: Bucket::Two,
            other_bucket: capacities[main_bucket],
        });
    }

    loop {
        if buckets[0] == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: buckets[1],
            });
        } else if buckets[1] == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: buckets[0],
            });
        }

        if buckets[main_bucket] > 0 && buckets[side_bucket] < capacities[side_bucket] {
            // Pour water from main to side bucket
            let amount = min(
                buckets[main_bucket],
                capacities[side_bucket] - buckets[side_bucket],
            );
            buckets[main_bucket] -= amount;
            buckets[side_bucket] += amount;
        } else if buckets[main_bucket] == 0 {
            // Fill main bucket
            buckets[main_bucket] = capacities[main_bucket];
        } else {
            // Empty side bucket
            buckets[side_bucket] = 0;
        }
        moves += 1;

        if buckets[main_bucket] == 0 && buckets[side_bucket] == capacities[side_bucket] {
            return None;
        }
    }
}
