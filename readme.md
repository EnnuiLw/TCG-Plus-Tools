# TCG-Plus Tools(API Wrapper)

※ for only one piece card


# installation
`Cargo.toml`
```
tcg-plus-tools = { git = "https://github.com/EnnuiLw/TCG-Plus-Tools" }
```

<!-- 
```
cargo install tcg-plus-tools -F en
``` 
-->

you can choose the one natural language with -F
default is `JP`


# how to use

(tests/api.rs)[~tests/api.rs]
```rs
use tcg_plus_tools::Client;
use tcg_plus_tools::events_prelude::{EventTrait};

let client = Client::default();
let url = "https://www.bandai-tcg-plus.com/event/3679885";
let event = client.get_event_info(url).await.unwrap();

dbg!(event);
```

### output
```
[tests/api.rs:14:9] event = PartialEvent {
    id: 3679885,
    title: "スタンダードバトル（6月開催）",
    event_series_id: "3924",
    series_header_img_url: "https://files.bandai-tcg-plus.com/event_series_file/3924/user/%E3%83%AF%E3%83%B3%E3%83%94%E3%83%BC%E3%82%B9%E3%82%B9%E3%82%BF%E3%83%B3%E3%83%80%E3%83%BC%E3%83%89%E3%83%90%E3%83%88%E3%83%AB320x320.png",
    event_series_type: 1,
    organizer_id: 6645,
    organizer_name: "BOOKOFF横浜東戸塚店",
    ...
}
```