use std::collections::HashMap;

/*
 * Complete the 'sockMerchant' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER_ARRAY ar
 */
pub fn find_pairs(ar: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in ar {
        map.entry(*i).and_modify(|count| *count+=1).or_insert(1);
    }
    let pairs: i32 = map.values().into_iter().map(|a| a-(a%2)).sum::<i32>()/2;
    pairs
}

