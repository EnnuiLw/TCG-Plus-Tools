macro_cores::impl_value! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Prefecture {
        // 北海道・東北
        Hokkaido => "JP-01",
        Aomori => "JP-02",
        Iwate => "JP-03",
        Miyagi => "JP-04",
        Akita => "JP-05",
        Yamagata => "JP-6",
        Fukushima => "JP-7",

        // 関東
        Ibaraki => "JP-8",
        Tochigi => "JP-9",
        Gunma => "JP-10",
        Saitama => "JP-11",
        Chiba => "JP-12",
        Tokyo => "JP-13",
        Kanagawa => "JP-14",

        // 中部
        Niigata => "JP-15",
        Toyama => "JP-16",
        Ishikawa => "JP-17",
        Fukui => "JP-18",
        Yamanashi => "JP-19",
        Nagano => "JP-20",
        Gifu => "JP-21",
        Shizuoka => "JP-22",
        Aichi => "JP-23",

        // 近畿
        Mie => "JP-24",
        Shiga => "JP-25",
        Kyoto => "JP-26",
        Osaka => "JP-27",
        Hyogo => "JP-28",
        Nara => "JP-29",
        Wakayama => "JP-30",

        // 中国
        Tottori => "JP-31",
        Shimane => "JP-32",
        Okayama => "JP-33",
        Hiroshima => "JP-34",
        Yamaguchi => "JP-35",

        // 四国
        Tokushima => "JP-36",
        Kagawa => "JP-37",
        Ehime => "JP-38",
        Kochi => "JP-39",

        // 九州・沖縄
        Fukuoka => "JP-40",
        Saga => "JP-41",
        Nagasaki => "JP-42",
        Kumamoto => "JP-43",
        Oita => "JP-44",
        Miyazaki => "JP-45",
        Kagoshima => "JP-46",
        Okinawa => "JP-47"
    }
}