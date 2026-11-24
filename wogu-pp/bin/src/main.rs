fn main() {
    let map = rosu_pp::Beatmap::from_path("./ooi.osu").unwrap();

    map.check_suspicion().unwrap();
    // Calculate difficulty attributes
    let diff_attrs = rosu_pp::Difficulty::new()
        .mods(0) // HDHR
        .calculate(&map); // or `checked_calculate`

    let stars = diff_attrs.stars();

    // Calculate performance attributes
    let perf_attrs = rosu_pp::Performance::new(diff_attrs)
        // To speed up the calculation, we used the previous attributes.
        // **Note** that this should only be done if the map and all difficulty
        // settings stay the same, otherwise the final attributes will be incorrect!
        .mods(0) // HDHR, must be the same as before
        .combo(200)
        .accuracy(99.14)
        .misses(0)
        .calculate();

    let pp = perf_attrs.pp();

    // Again, we re-use the previous attributes for maximum efficiency.
    let max_pp = perf_attrs
        .performance()
        .mods(0) // Still the same
        .calculate()
        .pp();

    println!("Stars: {stars} | PP: {pp}/{max_pp}");
}
