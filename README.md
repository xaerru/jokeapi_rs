# jokeapi_rs

Rust wrapper for [JokeAPI](https://v2.jokeapi.dev/)

## Get a joke

```rust
use jokeapi_rs::Joke;

fn main() {
    println!("{}", Joke::new().fetch().joke());
}
```

## Get a joke of a certain type

Type can either be "single" or "twopart"

```rust
use jokeapi::Joke;

fn main() {
    println!("{}", Joke::new().of_type("single").fetch().joke());
}
```

## Get a joke which belongs to certain categories

```rust
fn main() {
    println!(
        "{}",
        Joke::new()
            .categories(
                ["programming", "spooky"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            )
            .fetch()
            .joke()
    )
}
```

## Get a Data struct which contains all the fields of the the result

```rust
use jokeapi_rs::structs::joke::Data;
use jokeapi_rs::structs::joke::DataKind;
use jokeapi_rs::Joke;

fn main() {
    let res: Data = Joke::new()
        .of_type("single")
        .categories(
            ["programming", "pun"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        )
        .blacklist(["sexist", "nsfw"].iter().map(|x| x.to_string()).collect())
        //.safe() // Uncomment to enable safe mode
        .fetch();

    // Get the joke
    // .joke() method on res does the following for you
    match res.kind.clone() {
        DataKind::TwoPart { setup, delivery } => {
            println!("{}\n{}", setup, delivery)
        }
        DataKind::Single { joke } => println!("{}", joke),
    }

    // Access all the json fields
    println!("{:#?}", res);
}
```
