pub fn get_weather_art(weather_code: i32) -> &'static str {
    match weather_code {
        0 => { // Clear sky
            r#"
    \   /
     .-.
  ― (   ) ―
     `-'
    /   \
            "#
        },
        1..=3 => { // Partly cloudy
            r#"
   \  /
 _ /"".-.
   \_(   ).
   /(___(__)
            "#
        },
        45 | 48 => { // Fog
            r#"
 _ - _ - _ -
  _ - _ - _
 _ - _ - _ -
   _ - _ - _
            "#
        },
        51..=67 => { // Drizzle and rain
            r#"
     .-.
    (   ).
   (___(__)
    ' ' ' '
   ' ' ' '
            "#
        },
        71..=77 => { // Snow
            r#"
     .-.
    (   ).
   (___(__)
    * * * *
   * * * *
            "#
        },
        80..=82 => { // Rain showers
            r#"
   _`/"".-.
    ,\_(   ).
     /(___(__)
      ‚'‚'‚'‚'
     ‚'‚'‚'‚'
            "#
        },
        85..=86 => { // Snow showers
            r#"
   _`/"".-.
    ,\_(   ).
     /(___(__)
       *  *  *
      *  *  *
            "#
        },
        95..=99 => { // Thunderstorm
            r#"
     .-.
    (   ).
   (___(__)
    ⚡⚡⚡⚡
   ⚡⚡⚡⚡
            "#
        },
        _ => { // Unknown or unsupported
            r#"
    ?
   ???
  ?????
            "#
        }
    }
}