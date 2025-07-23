# TCG-Plus Tools

![banner](https://i.imgur.com/FDuNR2D.png)
※ for only one piece card


# installation

```
tcg-plus-tools = { git = "https://github.com/EnnuiLw/TCG-Plus-Tools" }
```

<!-- 
```
cargo install tcg-plus-tools -F en
``` 
-->

> [Cargo.toml](https://github.com/EnnuiLw/TCG-Plus-Tools/blob/master/Cargo.toml)

you can choose the one natural language with -F
default is `JP`


# how to use

```rs
use tcg_plus_tools::Client;
use tcg_plus_tools::events_prelude::{EventTrait};

let client = Client::default();
let url = "https://www.bandai-tcg-plus.com/event/3679885";
let event = client.get_event_info(url).await.unwrap();

dbg!(event);
```

> [tests/api.rs](https://github.com/EnnuiLw/TCG-Plus-Tools/blob/master/tests/api.rs#L15)


### output
```
[tests/api.rs:14:9] event = PartialEvent {
    id: 3679885,
    title: "スタンダードバトル（6月開催）",
    event_series_id: "3924",
    series_header_img_url: "https://files.bandai-tcg-plus.com/...",
    event_series_type: 1,
    organizer_id: 6645,
    organizer_name: "BOOKOFF横浜東戸塚店",
    ...
}
```

> [Struct・PartialEvent](https://github.com/EnnuiLw/TCG-Plus-Tools/blob/master/src/models/event.rs#L19) 
