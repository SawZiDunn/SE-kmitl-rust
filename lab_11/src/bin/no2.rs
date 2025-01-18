fn main() {
    let weather_data = vec![
        (0.0, 65, false),
        (26.2, 70, false),
        (24.8, 62, false),
        (23.5, 78, true),
        (22.1, 82, true),
        (20.7, 85, true),
        (21.3, 80, true),
        (22.8, 73, false),
        (24.0, 68, false),
        (25.5, 60, false),
        (27.1, 55, false),
        (28.3, 52, false),
        (27.9, 58, false),
        (26.6, 64, false),
        (25.2, 70, true),
        (23.8, 75, true),
        (22.4, 80, true),
        (21.0, 83, true),
        (20.5, 86, true),
        (21.8, 82, true),
        (23.2, 77, false),
        (24.5, 70, false),
        (25.8, 63, false),
        (26.4, 58, false),
        (27.0, 53, false),
        (26.7, 56, false),
        (25.3, 62, false),
        (24.9, 68, true),
        (23.1, 74, true),
        (21.7, 79, true),
    ];

    let warmest_period = warmest_period(&weather_data, 3);
    println!("Warmest 3-day period: {:?}", warmest_period);

    let coldest_day = coldest_day(&weather_data);
    println!("Coldest day: {}", coldest_day);

    let rainy_days_count = count_rainy_days(&weather_data);
    println!("Number of rainy days: {}", rainy_days_count);

    let (temperature, humidity, _) = weather_data[0];
    let will_rain = predict_rain(temperature, humidity);
    println!("Will it rain on the first day? {}", will_rain);
}

fn warmest_period(weather_data: &[(f64, i32, bool)], k: usize) -> &[(f64, i32, bool)] {
    let mut start_index = 0;
    let mut highest_avg_temperature = f64::MIN; // 0.0

    for i in 0..=weather_data.len() - k {
        let three_day_period = &weather_data[i..i + k];
        let avg_temperature = three_day_period.iter().map(|&(t, _, _)| t).sum::<f64>() / k as f64;

        // map() applies a transform function to each item in the iterator and return a new iterator

        if avg_temperature > highest_avg_temperature {
            highest_avg_temperature = avg_temperature;
            start_index = i;
        }
    }

    &weather_data[start_index..start_index + k]
}

fn coldest_day(weather_data: &[(f64, i32, bool)]) -> usize {
    let mut index: usize = 0;
    let mut coldest_temperature = weather_data[0].0;

    for i in 0..weather_data.len() {
        if weather_data[i].0 < coldest_temperature {
            coldest_temperature = weather_data[i].0;
            index = i;
        }
    }

    index
}

fn count_rainy_days(weather_data: &[(f64, i32, bool)]) -> usize {
    // filter() takes a closure as an argument and applies to each item in the iterator to decide whether
    // to keep or discard the item depending on the condition

    weather_data
        .iter()
        .filter(|(_, _, is_rainy)| *is_rainy)
        .count()

    // count() is more effective since
    // collect() allocates memory for a new vector

    // use collect() when we need to use the filtered vector
    // .collect::<Vec<_>>()
    // .len()
}

fn predict_rain(temperature: f64, humidity: i32) -> bool {
    let rain_probability = 0.005 * humidity as f64 + 0.02 * temperature - 1.;
    rain_probability > 0.5
}
