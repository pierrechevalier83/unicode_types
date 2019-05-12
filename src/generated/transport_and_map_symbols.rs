
/// An enum to represent all characters in the TransportandMapSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TransportandMapSymbols {
    /// \u{1f680}: '🚀'
    Rocket,
    /// \u{1f681}: '🚁'
    Helicopter,
    /// \u{1f682}: '🚂'
    SteamLocomotive,
    /// \u{1f683}: '🚃'
    RailwayCar,
    /// \u{1f684}: '🚄'
    HighDashSpeedTrain,
    /// \u{1f685}: '🚅'
    HighDashSpeedTrainWithBulletNose,
    /// \u{1f686}: '🚆'
    Train,
    /// \u{1f687}: '🚇'
    Metro,
    /// \u{1f688}: '🚈'
    LightRail,
    /// \u{1f689}: '🚉'
    Station,
    /// \u{1f68a}: '🚊'
    Tram,
    /// \u{1f68b}: '🚋'
    TramCar,
    /// \u{1f68c}: '🚌'
    Bus,
    /// \u{1f68d}: '🚍'
    OncomingBus,
    /// \u{1f68e}: '🚎'
    Trolleybus,
    /// \u{1f68f}: '🚏'
    BusStop,
    /// \u{1f690}: '🚐'
    Minibus,
    /// \u{1f691}: '🚑'
    Ambulance,
    /// \u{1f692}: '🚒'
    FireEngine,
    /// \u{1f693}: '🚓'
    PoliceCar,
    /// \u{1f694}: '🚔'
    OncomingPoliceCar,
    /// \u{1f695}: '🚕'
    Taxi,
    /// \u{1f696}: '🚖'
    OncomingTaxi,
    /// \u{1f697}: '🚗'
    Automobile,
    /// \u{1f698}: '🚘'
    OncomingAutomobile,
    /// \u{1f699}: '🚙'
    RecreationalVehicle,
    /// \u{1f69a}: '🚚'
    DeliveryTruck,
    /// \u{1f69b}: '🚛'
    ArticulatedLorry,
    /// \u{1f69c}: '🚜'
    Tractor,
    /// \u{1f69d}: '🚝'
    Monorail,
    /// \u{1f69e}: '🚞'
    MountainRailway,
    /// \u{1f69f}: '🚟'
    SuspensionRailway,
    /// \u{1f6a0}: '🚠'
    MountainCableway,
    /// \u{1f6a1}: '🚡'
    AerialTramway,
    /// \u{1f6a2}: '🚢'
    Ship,
    /// \u{1f6a3}: '🚣'
    Rowboat,
    /// \u{1f6a4}: '🚤'
    Speedboat,
    /// \u{1f6a5}: '🚥'
    HorizontalTrafficLight,
    /// \u{1f6a6}: '🚦'
    VerticalTrafficLight,
    /// \u{1f6a7}: '🚧'
    ConstructionSign,
    /// \u{1f6a8}: '🚨'
    PoliceCarsRevolvingLight,
    /// \u{1f6a9}: '🚩'
    TriangularFlagOnPost,
    /// \u{1f6aa}: '🚪'
    Door,
    /// \u{1f6ab}: '🚫'
    NoEntrySign,
    /// \u{1f6ac}: '🚬'
    SmokingSymbol,
    /// \u{1f6ad}: '🚭'
    NoSmokingSymbol,
    /// \u{1f6ae}: '🚮'
    PutLitterInItsPlaceSymbol,
    /// \u{1f6af}: '🚯'
    DoNotLitterSymbol,
    /// \u{1f6b0}: '🚰'
    PotableWaterSymbol,
    /// \u{1f6b1}: '🚱'
    NonDashPotableWaterSymbol,
    /// \u{1f6b2}: '🚲'
    Bicycle,
    /// \u{1f6b3}: '🚳'
    NoBicycles,
    /// \u{1f6b4}: '🚴'
    Bicyclist,
    /// \u{1f6b5}: '🚵'
    MountainBicyclist,
    /// \u{1f6b6}: '🚶'
    Pedestrian,
    /// \u{1f6b7}: '🚷'
    NoPedestrians,
    /// \u{1f6b8}: '🚸'
    ChildrenCrossing,
    /// \u{1f6b9}: '🚹'
    MensSymbol,
    /// \u{1f6ba}: '🚺'
    WomensSymbol,
    /// \u{1f6bb}: '🚻'
    Restroom,
    /// \u{1f6bc}: '🚼'
    BabySymbol,
    /// \u{1f6bd}: '🚽'
    Toilet,
    /// \u{1f6be}: '🚾'
    WaterCloset,
    /// \u{1f6bf}: '🚿'
    Shower,
    /// \u{1f6c0}: '🛀'
    Bath,
    /// \u{1f6c1}: '🛁'
    Bathtub,
    /// \u{1f6c2}: '🛂'
    PassportControl,
    /// \u{1f6c3}: '🛃'
    Customs,
    /// \u{1f6c4}: '🛄'
    BaggageClaim,
    /// \u{1f6c5}: '🛅'
    LeftLuggage,
    /// \u{1f6c6}: '🛆'
    TriangleWithRoundedCorners,
    /// \u{1f6c7}: '🛇'
    ProhibitedSign,
    /// \u{1f6c8}: '🛈'
    CircledInformationSource,
    /// \u{1f6c9}: '🛉'
    BoysSymbol,
    /// \u{1f6ca}: '🛊'
    GirlsSymbol,
    /// \u{1f6cb}: '🛋'
    CouchAndLamp,
    /// \u{1f6cc}: '🛌'
    SleepingAccommodation,
    /// \u{1f6cd}: '🛍'
    ShoppingBags,
    /// \u{1f6ce}: '🛎'
    BellhopBell,
    /// \u{1f6cf}: '🛏'
    Bed,
    /// \u{1f6d0}: '🛐'
    PlaceOfWorship,
    /// \u{1f6d1}: '🛑'
    OctagonalSign,
    /// \u{1f6d2}: '🛒'
    ShoppingTrolley,
    /// \u{1f6d3}: '🛓'
    Stupa,
    /// \u{1f6d4}: '🛔'
    Pagoda,
    /// \u{1f6d5}: '🛕'
    HinduTemple,
    /// \u{1f6e0}: '🛠'
    HammerAndWrench,
    /// \u{1f6e1}: '🛡'
    Shield,
    /// \u{1f6e2}: '🛢'
    OilDrum,
    /// \u{1f6e3}: '🛣'
    Motorway,
    /// \u{1f6e4}: '🛤'
    RailwayTrack,
    /// \u{1f6e5}: '🛥'
    MotorBoat,
    /// \u{1f6e6}: '🛦'
    UpDashPointingMilitaryAirplane,
    /// \u{1f6e7}: '🛧'
    UpDashPointingAirplane,
    /// \u{1f6e8}: '🛨'
    UpDashPointingSmallAirplane,
    /// \u{1f6e9}: '🛩'
    SmallAirplane,
    /// \u{1f6ea}: '🛪'
    NortheastDashPointingAirplane,
    /// \u{1f6eb}: '🛫'
    AirplaneDeparture,
    /// \u{1f6ec}: '🛬'
    AirplaneArriving,
    /// \u{1f6f0}: '🛰'
    Satellite,
    /// \u{1f6f1}: '🛱'
    OncomingFireEngine,
    /// \u{1f6f2}: '🛲'
    DieselLocomotive,
    /// \u{1f6f3}: '🛳'
    PassengerShip,
    /// \u{1f6f4}: '🛴'
    Scooter,
    /// \u{1f6f5}: '🛵'
    MotorScooter,
    /// \u{1f6f6}: '🛶'
    Canoe,
    /// \u{1f6f7}: '🛷'
    Sled,
    /// \u{1f6f8}: '🛸'
    FlyingSaucer,
    /// \u{1f6f9}: '🛹'
    Skateboard,
    /// \u{1f6fa}: '🛺'
    AutoRickshaw,
}

impl Into<char> for TransportandMapSymbols {
    fn into(self) -> char {
        match self {
            TransportandMapSymbols::Rocket => '🚀',
            TransportandMapSymbols::Helicopter => '🚁',
            TransportandMapSymbols::SteamLocomotive => '🚂',
            TransportandMapSymbols::RailwayCar => '🚃',
            TransportandMapSymbols::HighDashSpeedTrain => '🚄',
            TransportandMapSymbols::HighDashSpeedTrainWithBulletNose => '🚅',
            TransportandMapSymbols::Train => '🚆',
            TransportandMapSymbols::Metro => '🚇',
            TransportandMapSymbols::LightRail => '🚈',
            TransportandMapSymbols::Station => '🚉',
            TransportandMapSymbols::Tram => '🚊',
            TransportandMapSymbols::TramCar => '🚋',
            TransportandMapSymbols::Bus => '🚌',
            TransportandMapSymbols::OncomingBus => '🚍',
            TransportandMapSymbols::Trolleybus => '🚎',
            TransportandMapSymbols::BusStop => '🚏',
            TransportandMapSymbols::Minibus => '🚐',
            TransportandMapSymbols::Ambulance => '🚑',
            TransportandMapSymbols::FireEngine => '🚒',
            TransportandMapSymbols::PoliceCar => '🚓',
            TransportandMapSymbols::OncomingPoliceCar => '🚔',
            TransportandMapSymbols::Taxi => '🚕',
            TransportandMapSymbols::OncomingTaxi => '🚖',
            TransportandMapSymbols::Automobile => '🚗',
            TransportandMapSymbols::OncomingAutomobile => '🚘',
            TransportandMapSymbols::RecreationalVehicle => '🚙',
            TransportandMapSymbols::DeliveryTruck => '🚚',
            TransportandMapSymbols::ArticulatedLorry => '🚛',
            TransportandMapSymbols::Tractor => '🚜',
            TransportandMapSymbols::Monorail => '🚝',
            TransportandMapSymbols::MountainRailway => '🚞',
            TransportandMapSymbols::SuspensionRailway => '🚟',
            TransportandMapSymbols::MountainCableway => '🚠',
            TransportandMapSymbols::AerialTramway => '🚡',
            TransportandMapSymbols::Ship => '🚢',
            TransportandMapSymbols::Rowboat => '🚣',
            TransportandMapSymbols::Speedboat => '🚤',
            TransportandMapSymbols::HorizontalTrafficLight => '🚥',
            TransportandMapSymbols::VerticalTrafficLight => '🚦',
            TransportandMapSymbols::ConstructionSign => '🚧',
            TransportandMapSymbols::PoliceCarsRevolvingLight => '🚨',
            TransportandMapSymbols::TriangularFlagOnPost => '🚩',
            TransportandMapSymbols::Door => '🚪',
            TransportandMapSymbols::NoEntrySign => '🚫',
            TransportandMapSymbols::SmokingSymbol => '🚬',
            TransportandMapSymbols::NoSmokingSymbol => '🚭',
            TransportandMapSymbols::PutLitterInItsPlaceSymbol => '🚮',
            TransportandMapSymbols::DoNotLitterSymbol => '🚯',
            TransportandMapSymbols::PotableWaterSymbol => '🚰',
            TransportandMapSymbols::NonDashPotableWaterSymbol => '🚱',
            TransportandMapSymbols::Bicycle => '🚲',
            TransportandMapSymbols::NoBicycles => '🚳',
            TransportandMapSymbols::Bicyclist => '🚴',
            TransportandMapSymbols::MountainBicyclist => '🚵',
            TransportandMapSymbols::Pedestrian => '🚶',
            TransportandMapSymbols::NoPedestrians => '🚷',
            TransportandMapSymbols::ChildrenCrossing => '🚸',
            TransportandMapSymbols::MensSymbol => '🚹',
            TransportandMapSymbols::WomensSymbol => '🚺',
            TransportandMapSymbols::Restroom => '🚻',
            TransportandMapSymbols::BabySymbol => '🚼',
            TransportandMapSymbols::Toilet => '🚽',
            TransportandMapSymbols::WaterCloset => '🚾',
            TransportandMapSymbols::Shower => '🚿',
            TransportandMapSymbols::Bath => '🛀',
            TransportandMapSymbols::Bathtub => '🛁',
            TransportandMapSymbols::PassportControl => '🛂',
            TransportandMapSymbols::Customs => '🛃',
            TransportandMapSymbols::BaggageClaim => '🛄',
            TransportandMapSymbols::LeftLuggage => '🛅',
            TransportandMapSymbols::TriangleWithRoundedCorners => '🛆',
            TransportandMapSymbols::ProhibitedSign => '🛇',
            TransportandMapSymbols::CircledInformationSource => '🛈',
            TransportandMapSymbols::BoysSymbol => '🛉',
            TransportandMapSymbols::GirlsSymbol => '🛊',
            TransportandMapSymbols::CouchAndLamp => '🛋',
            TransportandMapSymbols::SleepingAccommodation => '🛌',
            TransportandMapSymbols::ShoppingBags => '🛍',
            TransportandMapSymbols::BellhopBell => '🛎',
            TransportandMapSymbols::Bed => '🛏',
            TransportandMapSymbols::PlaceOfWorship => '🛐',
            TransportandMapSymbols::OctagonalSign => '🛑',
            TransportandMapSymbols::ShoppingTrolley => '🛒',
            TransportandMapSymbols::Stupa => '🛓',
            TransportandMapSymbols::Pagoda => '🛔',
            TransportandMapSymbols::HinduTemple => '🛕',
            TransportandMapSymbols::HammerAndWrench => '🛠',
            TransportandMapSymbols::Shield => '🛡',
            TransportandMapSymbols::OilDrum => '🛢',
            TransportandMapSymbols::Motorway => '🛣',
            TransportandMapSymbols::RailwayTrack => '🛤',
            TransportandMapSymbols::MotorBoat => '🛥',
            TransportandMapSymbols::UpDashPointingMilitaryAirplane => '🛦',
            TransportandMapSymbols::UpDashPointingAirplane => '🛧',
            TransportandMapSymbols::UpDashPointingSmallAirplane => '🛨',
            TransportandMapSymbols::SmallAirplane => '🛩',
            TransportandMapSymbols::NortheastDashPointingAirplane => '🛪',
            TransportandMapSymbols::AirplaneDeparture => '🛫',
            TransportandMapSymbols::AirplaneArriving => '🛬',
            TransportandMapSymbols::Satellite => '🛰',
            TransportandMapSymbols::OncomingFireEngine => '🛱',
            TransportandMapSymbols::DieselLocomotive => '🛲',
            TransportandMapSymbols::PassengerShip => '🛳',
            TransportandMapSymbols::Scooter => '🛴',
            TransportandMapSymbols::MotorScooter => '🛵',
            TransportandMapSymbols::Canoe => '🛶',
            TransportandMapSymbols::Sled => '🛷',
            TransportandMapSymbols::FlyingSaucer => '🛸',
            TransportandMapSymbols::Skateboard => '🛹',
            TransportandMapSymbols::AutoRickshaw => '🛺',
        }
    }
}

impl std::convert::TryFrom<char> for TransportandMapSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '🚀' => Ok(TransportandMapSymbols::Rocket),
            '🚁' => Ok(TransportandMapSymbols::Helicopter),
            '🚂' => Ok(TransportandMapSymbols::SteamLocomotive),
            '🚃' => Ok(TransportandMapSymbols::RailwayCar),
            '🚄' => Ok(TransportandMapSymbols::HighDashSpeedTrain),
            '🚅' => Ok(TransportandMapSymbols::HighDashSpeedTrainWithBulletNose),
            '🚆' => Ok(TransportandMapSymbols::Train),
            '🚇' => Ok(TransportandMapSymbols::Metro),
            '🚈' => Ok(TransportandMapSymbols::LightRail),
            '🚉' => Ok(TransportandMapSymbols::Station),
            '🚊' => Ok(TransportandMapSymbols::Tram),
            '🚋' => Ok(TransportandMapSymbols::TramCar),
            '🚌' => Ok(TransportandMapSymbols::Bus),
            '🚍' => Ok(TransportandMapSymbols::OncomingBus),
            '🚎' => Ok(TransportandMapSymbols::Trolleybus),
            '🚏' => Ok(TransportandMapSymbols::BusStop),
            '🚐' => Ok(TransportandMapSymbols::Minibus),
            '🚑' => Ok(TransportandMapSymbols::Ambulance),
            '🚒' => Ok(TransportandMapSymbols::FireEngine),
            '🚓' => Ok(TransportandMapSymbols::PoliceCar),
            '🚔' => Ok(TransportandMapSymbols::OncomingPoliceCar),
            '🚕' => Ok(TransportandMapSymbols::Taxi),
            '🚖' => Ok(TransportandMapSymbols::OncomingTaxi),
            '🚗' => Ok(TransportandMapSymbols::Automobile),
            '🚘' => Ok(TransportandMapSymbols::OncomingAutomobile),
            '🚙' => Ok(TransportandMapSymbols::RecreationalVehicle),
            '🚚' => Ok(TransportandMapSymbols::DeliveryTruck),
            '🚛' => Ok(TransportandMapSymbols::ArticulatedLorry),
            '🚜' => Ok(TransportandMapSymbols::Tractor),
            '🚝' => Ok(TransportandMapSymbols::Monorail),
            '🚞' => Ok(TransportandMapSymbols::MountainRailway),
            '🚟' => Ok(TransportandMapSymbols::SuspensionRailway),
            '🚠' => Ok(TransportandMapSymbols::MountainCableway),
            '🚡' => Ok(TransportandMapSymbols::AerialTramway),
            '🚢' => Ok(TransportandMapSymbols::Ship),
            '🚣' => Ok(TransportandMapSymbols::Rowboat),
            '🚤' => Ok(TransportandMapSymbols::Speedboat),
            '🚥' => Ok(TransportandMapSymbols::HorizontalTrafficLight),
            '🚦' => Ok(TransportandMapSymbols::VerticalTrafficLight),
            '🚧' => Ok(TransportandMapSymbols::ConstructionSign),
            '🚨' => Ok(TransportandMapSymbols::PoliceCarsRevolvingLight),
            '🚩' => Ok(TransportandMapSymbols::TriangularFlagOnPost),
            '🚪' => Ok(TransportandMapSymbols::Door),
            '🚫' => Ok(TransportandMapSymbols::NoEntrySign),
            '🚬' => Ok(TransportandMapSymbols::SmokingSymbol),
            '🚭' => Ok(TransportandMapSymbols::NoSmokingSymbol),
            '🚮' => Ok(TransportandMapSymbols::PutLitterInItsPlaceSymbol),
            '🚯' => Ok(TransportandMapSymbols::DoNotLitterSymbol),
            '🚰' => Ok(TransportandMapSymbols::PotableWaterSymbol),
            '🚱' => Ok(TransportandMapSymbols::NonDashPotableWaterSymbol),
            '🚲' => Ok(TransportandMapSymbols::Bicycle),
            '🚳' => Ok(TransportandMapSymbols::NoBicycles),
            '🚴' => Ok(TransportandMapSymbols::Bicyclist),
            '🚵' => Ok(TransportandMapSymbols::MountainBicyclist),
            '🚶' => Ok(TransportandMapSymbols::Pedestrian),
            '🚷' => Ok(TransportandMapSymbols::NoPedestrians),
            '🚸' => Ok(TransportandMapSymbols::ChildrenCrossing),
            '🚹' => Ok(TransportandMapSymbols::MensSymbol),
            '🚺' => Ok(TransportandMapSymbols::WomensSymbol),
            '🚻' => Ok(TransportandMapSymbols::Restroom),
            '🚼' => Ok(TransportandMapSymbols::BabySymbol),
            '🚽' => Ok(TransportandMapSymbols::Toilet),
            '🚾' => Ok(TransportandMapSymbols::WaterCloset),
            '🚿' => Ok(TransportandMapSymbols::Shower),
            '🛀' => Ok(TransportandMapSymbols::Bath),
            '🛁' => Ok(TransportandMapSymbols::Bathtub),
            '🛂' => Ok(TransportandMapSymbols::PassportControl),
            '🛃' => Ok(TransportandMapSymbols::Customs),
            '🛄' => Ok(TransportandMapSymbols::BaggageClaim),
            '🛅' => Ok(TransportandMapSymbols::LeftLuggage),
            '🛆' => Ok(TransportandMapSymbols::TriangleWithRoundedCorners),
            '🛇' => Ok(TransportandMapSymbols::ProhibitedSign),
            '🛈' => Ok(TransportandMapSymbols::CircledInformationSource),
            '🛉' => Ok(TransportandMapSymbols::BoysSymbol),
            '🛊' => Ok(TransportandMapSymbols::GirlsSymbol),
            '🛋' => Ok(TransportandMapSymbols::CouchAndLamp),
            '🛌' => Ok(TransportandMapSymbols::SleepingAccommodation),
            '🛍' => Ok(TransportandMapSymbols::ShoppingBags),
            '🛎' => Ok(TransportandMapSymbols::BellhopBell),
            '🛏' => Ok(TransportandMapSymbols::Bed),
            '🛐' => Ok(TransportandMapSymbols::PlaceOfWorship),
            '🛑' => Ok(TransportandMapSymbols::OctagonalSign),
            '🛒' => Ok(TransportandMapSymbols::ShoppingTrolley),
            '🛓' => Ok(TransportandMapSymbols::Stupa),
            '🛔' => Ok(TransportandMapSymbols::Pagoda),
            '🛕' => Ok(TransportandMapSymbols::HinduTemple),
            '🛠' => Ok(TransportandMapSymbols::HammerAndWrench),
            '🛡' => Ok(TransportandMapSymbols::Shield),
            '🛢' => Ok(TransportandMapSymbols::OilDrum),
            '🛣' => Ok(TransportandMapSymbols::Motorway),
            '🛤' => Ok(TransportandMapSymbols::RailwayTrack),
            '🛥' => Ok(TransportandMapSymbols::MotorBoat),
            '🛦' => Ok(TransportandMapSymbols::UpDashPointingMilitaryAirplane),
            '🛧' => Ok(TransportandMapSymbols::UpDashPointingAirplane),
            '🛨' => Ok(TransportandMapSymbols::UpDashPointingSmallAirplane),
            '🛩' => Ok(TransportandMapSymbols::SmallAirplane),
            '🛪' => Ok(TransportandMapSymbols::NortheastDashPointingAirplane),
            '🛫' => Ok(TransportandMapSymbols::AirplaneDeparture),
            '🛬' => Ok(TransportandMapSymbols::AirplaneArriving),
            '🛰' => Ok(TransportandMapSymbols::Satellite),
            '🛱' => Ok(TransportandMapSymbols::OncomingFireEngine),
            '🛲' => Ok(TransportandMapSymbols::DieselLocomotive),
            '🛳' => Ok(TransportandMapSymbols::PassengerShip),
            '🛴' => Ok(TransportandMapSymbols::Scooter),
            '🛵' => Ok(TransportandMapSymbols::MotorScooter),
            '🛶' => Ok(TransportandMapSymbols::Canoe),
            '🛷' => Ok(TransportandMapSymbols::Sled),
            '🛸' => Ok(TransportandMapSymbols::FlyingSaucer),
            '🛹' => Ok(TransportandMapSymbols::Skateboard),
            '🛺' => Ok(TransportandMapSymbols::AutoRickshaw),
            _ => Err(()),
        }
    }
}

impl Into<u32> for TransportandMapSymbols {
    fn into(self) -> u32 {
        let c: char = self.into();
        let hex = c
            .escape_unicode()
            .to_string()
            .replace("\\u{", "")
            .replace("}", "");
        u32::from_str_radix(&hex, 16).unwrap()
    }
}

impl std::convert::TryFrom<u32> for TransportandMapSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for TransportandMapSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl TransportandMapSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        TransportandMapSymbols::Rocket
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("TransportandMapSymbols{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
