use crate::{date, relative_to_easter_sunday, HolidayRegion};
use chrono::{Datelike, Duration, NaiveDate};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GermanHolidays {
    Neujahr,
    HeiligeDreiKoenige,
    Frauentag,
    Karfreitag,
    Ostermontag,
    ErsterMai,
    ChristiHimmelfahrt,
    Pfingstmontag,
    Fronleichnam,
    AugsburgerFriedensfest,
    MariaeHimmelfahrt,
    Weltkindertag,
    TagDerDeutschenEinheit,
    Reformationstag,
    Allerheiligen,
    BussUndBettag,
    ErsterWeihnachtsfeiertag,
    ZweiterWeihnachtsfeiertag,
}

pub enum Germany {
    BadenWuerttemberg,
    Bayern,
    Berlin,
    Brandenburg,
    Bremen,
    Hamburg,
    Hessen,
    MechlenburgVorpommern,
    Niedersachsen,
    NordrheinWestfalen,
    RheinlandPfalz,
    Saarland,
    Sachsen,
    SachsenAnhalt,
    SchleswigHolstein,
    Thueringen,
}

use crate::germany::GermanHolidays::*;
use crate::germany::Germany::*;

impl HolidayRegion for Germany {
    fn holidays_in_year(&self, year: i32) -> Vec<GermanHolidays> {
        if year < 1995 {
            return Vec::new();
        }
        let mut holidays = Vec::new();
        holidays.extend_from_slice(BUNDESWEITE_FEIERTAGE);
        let region_specific_holidays: &'static [GermanHolidays] = match self {
            BadenWuerttemberg => &[HeiligeDreiKoenige, Fronleichnam, Allerheiligen],
            Bayern => &[
                HeiligeDreiKoenige,
                Fronleichnam,
                MariaeHimmelfahrt,
                Allerheiligen,
            ],
            Berlin => {
                if year >= 2019 {
                    &[Frauentag]
                } else {
                    &[]
                }
            }
            Brandenburg => &[Reformationstag],
            Bremen => &[Reformationstag],
            Hamburg => &[Reformationstag],
            Hessen => &[Fronleichnam],
            MechlenburgVorpommern => &[Reformationstag],
            Niedersachsen => &[Reformationstag],
            NordrheinWestfalen => &[Fronleichnam, Allerheiligen],
            RheinlandPfalz => &[Fronleichnam, Allerheiligen],
            Saarland => &[Fronleichnam, MariaeHimmelfahrt, Allerheiligen],
            Sachsen => &[Reformationstag, BussUndBettag],
            SachsenAnhalt => &[HeiligeDreiKoenige, Reformationstag],
            SchleswigHolstein => &[Reformationstag],
            Thueringen => &[Weltkindertag, Reformationstag],
        };
        holidays.extend_from_slice(region_specific_holidays);
        holidays
    }
}

impl GermanHolidays {
    pub fn to_date(&self, year: i32) -> Option<NaiveDate> {
        match self {
            Neujahr => date(year, 1, 1),
            HeiligeDreiKoenige => date(year, 1, 6),
            Frauentag => date(year, 3, 8),
            Karfreitag => relative_to_easter_sunday(year, -2),
            Ostermontag => relative_to_easter_sunday(year, 1),
            ErsterMai => date(year, 5, 1),
            ChristiHimmelfahrt => relative_to_easter_sunday(year, 39),
            Pfingstmontag => relative_to_easter_sunday(year, 50),
            Fronleichnam => relative_to_easter_sunday(year, 60),
            AugsburgerFriedensfest => date(year, 8, 8),
            MariaeHimmelfahrt => date(year, 8, 15),
            Weltkindertag => date(year, 9, 20),
            TagDerDeutschenEinheit => date(year, 10, 3),
            Reformationstag => date(year, 10, 31),
            Allerheiligen => date(year, 11, 1),
            BussUndBettag => bus_und_bettag(year),
            ErsterWeihnachtsfeiertag => date(year, 12, 25),
            ZweiterWeihnachtsfeiertag => date(year, 12, 26),
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            Neujahr => "Neujahr",
            HeiligeDreiKoenige => "Heilige Drei Könige",
            Frauentag => "Frauentag",
            Karfreitag => "Karfreitag",
            Ostermontag => "Ostermontag",
            ErsterMai => "Erster Mai",
            ChristiHimmelfahrt => "Christi Himmelfahrt",
            Pfingstmontag => "Pfingstmontag",
            Fronleichnam => "Fronleichnam",
            AugsburgerFriedensfest => "Augsburger Friedensfest",
            MariaeHimmelfahrt => "Mariä Himmelfahrt",
            Weltkindertag => "Weltkindertag",
            TagDerDeutschenEinheit => "Tag der Deutschen Einheit",
            Reformationstag => "Reformationstag",
            Allerheiligen => "Allerheiligen",
            BussUndBettag => "Buß- und Bettag",
            ErsterWeihnachtsfeiertag => "1. Weihnachtsfeiertag",
            ZweiterWeihnachtsfeiertag => "2. Weihnachtsfeiertag",
        }
    }
}

const BUNDESWEITE_FEIERTAGE: &'static [GermanHolidays] = &[
    Neujahr,
    Karfreitag,
    Ostermontag,
    ErsterMai,
    ChristiHimmelfahrt,
    Pfingstmontag,
    TagDerDeutschenEinheit,
    ErsterWeihnachtsfeiertag,
    ZweiterWeihnachtsfeiertag,
];

fn bus_und_bettag(year: i32) -> Option<NaiveDate> {
    let reference_date = NaiveDate::from_ymd(year, 11, 23);
    let weekday_ordinal = i64::from(reference_date.weekday().num_days_from_monday());
    let duration_to_previous_wednesday = if weekday_ordinal < 3 {
        Duration::days(-(weekday_ordinal + 5))
    } else {
        Duration::days(2 - weekday_ordinal)
    };
    Some(reference_date + duration_to_previous_wednesday)
}

#[cfg(test)]
mod tests {
    use crate::germany::GermanHolidays::*;
    use crate::germany::Germany::*;
    use crate::germany::{bus_und_bettag, Germany};
    use crate::{date, HolidayRegion, ToHoliday};
    use chrono::{Datelike, NaiveDate, Weekday};
    use proptest::prelude::*;

    #[test]
    fn neujahr_feiertag_in_bayern() {
        let date = NaiveDate::from_ymd(2018, 01, 01);
        assert!(date.is_holiday(Bayern));
        assert_eq!(Some(Neujahr), date.holiday(Bayern));
    }

    #[test]
    fn total_number_holidays() {
        let number_holidays = |region: Germany| region.holidays_in_year(2019).len();
        assert_eq!(12, number_holidays(BadenWuerttemberg));
        assert_eq!(13, number_holidays(Bayern));
        assert_eq!(10, number_holidays(Berlin));
        assert_eq!(10, number_holidays(Brandenburg));
        assert_eq!(10, number_holidays(Bremen));
        assert_eq!(10, number_holidays(Hamburg));
        assert_eq!(10, number_holidays(Hessen));
        assert_eq!(10, number_holidays(MechlenburgVorpommern));
        assert_eq!(10, number_holidays(Niedersachsen));
        assert_eq!(11, number_holidays(NordrheinWestfalen));
        assert_eq!(11, number_holidays(RheinlandPfalz));
        assert_eq!(12, number_holidays(Saarland));
        assert_eq!(11, number_holidays(Sachsen));
        assert_eq!(11, number_holidays(SachsenAnhalt));
        assert_eq!(10, number_holidays(SchleswigHolstein));
        assert_eq!(11, number_holidays(Thueringen));
    }

    #[test]
    fn test_bus_und_bettag_calc() {
        assert_eq!(date(2018, 11, 21), bus_und_bettag(2018));
        assert_eq!(date(2019, 11, 20), bus_und_bettag(2019));
        assert_eq!(date(2020, 11, 18), bus_und_bettag(2020));
        assert_eq!(date(2021, 11, 17), bus_und_bettag(2021));
        assert_eq!(date(2022, 11, 16), bus_und_bettag(2022));
        assert_eq!(date(2023, 11, 22), bus_und_bettag(2023));
    }

    #[test]
    fn frauntag_in_berlin_since_2019() {
        assert!(!Berlin.holidays_in_year(2018).contains(&Frauentag));
        assert_eq!(None, NaiveDate::from_ymd(2018, 3, 8).holiday(Berlin));
        assert!(Berlin.holidays_in_year(2019).contains(&Frauentag));
        assert_eq!(
            Some(Frauentag),
            NaiveDate::from_ymd(2019, 3, 8).holiday(Berlin)
        );
    }

    #[test]
    fn only_provide_holidays_after_1995() {
        assert!(BadenWuerttemberg.holidays_in_year(1994).is_empty());
    }

    proptest! {
    #[test]
    fn test_bus_und_bettag_is_wed_before_23th_nov(y in 1i32..2999) {
        let date = bus_und_bettag(y).unwrap();
        assert_eq!(Weekday::Wed, date.weekday());
        let duration = date.signed_duration_since(NaiveDate::from_ymd(y, 11, 23));
        assert!(duration.num_days() <= -1);
        assert!(duration.num_days() >= -7);
    }
    }
}
