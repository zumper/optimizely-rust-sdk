use fasthash::murmur3::hash32_with_seed as murmur3_hash;
use std::collections::BTreeMap;

/*
 Not a real test, but a playground to prototype new functionality
 Currently working on building the desicion bucketing
*/

#[test]
fn example_decision() {
    let mut json_data = json::parse(
        "[{\"entityId\":\"on\",\"endOfRange\":500},{\"entityId\":\"off\",\"endOfRange\":10000}]",
    )
    .unwrap();

    let map = json_data
        .take()
        .members_mut()
        .map(|value| {
            (
                value["endOfRange"].as_u32().unwrap(),
                value["entityId"].take_string().unwrap(),
            )
        })
        .collect::<BTreeMap<_, _>>();

    let hash_seed: u32 = 1;

    let user_id: &str = "mark";
    let experiment_id: &str = "9300000131788";

    // Concatenate user id and experiment id
    let bucketing_key = format!("{user_id}{experiment_id}");

    // To hash the bucket key it needs to be converted to an array of `u8` bytes
    // Use Murmur3 (32-bit) with seed
    let hash_value = murmur3_hash(bucketing_key.as_bytes(), hash_seed);

    // Bring the hash into a range of 0 to 10_000
    let bucket_value = ((hash_value as f64) / (u32::MAX as f64) * 10_000f64) as u32;

    // Use BTreeMap::range to find the variation in O(log(n))
    let variation = map.range(bucket_value..).next();

    assert_eq!(*variation.unwrap().1, "on");
}
