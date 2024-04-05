## Weather-CLI
- A simple weather cli application to get current weather based on the city using `api.openweathermap.org`
### Pre-requisite
`$ export WEATHER_API_KEY=API_KEY`

## build
`$ cargo build --release`

### run
`$ ./target/release/weather-cli "New York"`

`$ weather-cli "New York"`

## Example Response:
`
{"coord":{"lon":-74.006,"lat":40.7143},
    "weather":[{"id":804,"main":"Clouds","description":"overcast clouds","icon":"04d"}],
    "base":"stations",
    "main":{
        "temp":43.59,
        "feels_like":35.83,
        "temp_min":40.66,
        "temp_max":46.31,
        "pressure":997,
        "humidity":71
    },
    "visibility":10000,
    "wind":{
        "speed":17.27,
        "deg":330,
        "gust":23.02
    },
    "clouds":{
        "all":100
    },
    "dt":1712245928,
    "sys":{
        "type":2,
        "id":2008101,
        "country":"US",
        "sunrise":1712226822,
        "sunset":1712273017
    },
    "timezone":-14400,
    "id":5128581,
    "name":"New York",
    "cod":200}
`