use jokeapi_rs::Joke;

fn main() {
    let res = Joke::new()
        .of_type("single")
        .categories(
            ["programming", "pun"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        )
        .blacklist(["sexist", "nsfw"].iter().map(|x| x.to_string()).collect())
        //.safe() // Uncomment to enable safe mode
        .fetch()
        .joke();
    println!("{}", res);
}
