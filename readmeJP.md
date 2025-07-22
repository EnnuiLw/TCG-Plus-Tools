# TCG-Plus Tools(API Wrapper)

※ ワンピのみ


# インストール

```
tcg-plus-tools = { git = "https://github.com/EnnuiLw/TCG-Plus-Tools" }
```

<!-- 
```
cargo install tcg-plus-tools -F en
``` 
-->

> [Cargo.toml](https://github.com/EnnuiLw/TCG-Plus-Tools/blob/master/Cargo.toml)

-Fを用いて一つだけ自然言語を選択できます。
デフォルトは日本語。


## 使い方

```rs
use tcg_plus_tools::Client;
use tcg_plus_tools::events_prelude::{EventTrait};

let client = Client::default();
let url = "https://www.bandai-tcg-plus.com/event/3679885";
let event = client.get_event_info(url).await.unwrap();

dbg!(event);
```

> [tests/api.rs](https://github.com/EnnuiLw/TCG-Plus-Tools/blob/master/tests/api.rs#L15)


### 出力結果
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

> [構造体・PartialEvent](https://github.com/EnnuiLw/TCG-Plus-Tools/blob/master/src/models/event.rs#L19) 
