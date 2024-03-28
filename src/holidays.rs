use chrono::{Datelike, Duration, NaiveDate};
use computus;

/// All reoccurring holidays in Germany.
/// This list contains both public and non-public holidays.
///
/// For public holidays use `GermanRegion` instead, since
/// public holidays differ from region to region.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GermanHoliday {
    Neujahr,
    HeiligeDreiKoenige,
    Frauentag,
    Faschingsdienstag,
    Aschermittwoch,
    Gruendonnerstag,
    Karfreitag,
    Ostersonntag,
    Ostermontag,
    ErsterMai,
    ChristiHimmelfahrt,
    Pfingstsonntag,
    Pfingstmontag,
    Fronleichnam,
    AugsburgerFriedensfest,
    MariaeHimmelfahrt,
    Weltkindertag,
    TagDerDeutschenEinheit,
    Reformationstag,
    Allerheiligen,
    BussUndBettag,
    Heiligabend,
    ErsterWeihnachtsfeiertag,
    ZweiterWeihnachtsfeiertag,
    Silvester,
}

use GermanHoliday::*;

impl GermanHoliday {
    /// Calculates the date for a specific year.
    ///
    /// `None` if it cannot be calculated.
    pub fn date(&self, year: i32) -> Option<NaiveDate> {
        match self {
            Neujahr => date(year, 1, 1),
            HeiligeDreiKoenige => date(year, 1, 6),
            Frauentag => date(year, 3, 8),
            Faschingsdienstag => relative_to_easter_sunday(year, -47),
            Aschermittwoch => relative_to_easter_sunday(year, -46),
            Gruendonnerstag => relative_to_easter_sunday(year, -3),
            Karfreitag => relative_to_easter_sunday(year, -2),
            Ostersonntag => relative_to_easter_sunday(year, 0),
            Ostermontag => relative_to_easter_sunday(year, 1),
            ErsterMai => date(year, 5, 1),
            ChristiHimmelfahrt => relative_to_easter_sunday(year, 39),
            Pfingstsonntag => relative_to_easter_sunday(year, 49),
            Pfingstmontag => relative_to_easter_sunday(year, 50),
            Fronleichnam => relative_to_easter_sunday(year, 60),
            AugsburgerFriedensfest => date(year, 8, 8),
            MariaeHimmelfahrt => date(year, 8, 15),
            Weltkindertag => date(year, 9, 20),
            TagDerDeutschenEinheit => date(year, 10, 3),
            Reformationstag => date(year, 10, 31),
            Allerheiligen => date(year, 11, 1),
            BussUndBettag => bus_und_bettag(year),
            Heiligabend => date(year, 12, 24),
            ErsterWeihnachtsfeiertag => date(year, 12, 25),
            ZweiterWeihnachtsfeiertag => date(year, 12, 26),
            Silvester => date(year, 12, 31),
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            Neujahr => "Neujahr",
            HeiligeDreiKoenige => "Heilige Drei Könige",
            Frauentag => "Frauentag",
            Faschingsdienstag => "Faschingsdienstag",
            Aschermittwoch => "Aschermittwoch",
            Gruendonnerstag => "Gründonnerstag",
            Karfreitag => "Karfreitag",
            Ostersonntag => "Ostersonntag",
            Ostermontag => "Ostermontag",
            ErsterMai => "Erster Mai",
            ChristiHimmelfahrt => "Christi Himmelfahrt",
            Pfingstsonntag => "Pfingstsonntag",
            Pfingstmontag => "Pfingstmontag",
            Fronleichnam => "Fronleichnam",
            AugsburgerFriedensfest => "Augsburger Friedensfest",
            MariaeHimmelfahrt => "Mariä Himmelfahrt",
            Weltkindertag => "Weltkindertag",
            TagDerDeutschenEinheit => "Tag der Deutschen Einheit",
            Reformationstag => "Reformationstag",
            Allerheiligen => "Allerheiligen",
            BussUndBettag => "Buß- und Bettag",
            Heiligabend => "Heiligabend",
            ErsterWeihnachtsfeiertag => "Erster Weihnachtsfeiertag",
            ZweiterWeihnachtsfeiertag => "Zweiter Weihnachtsfeiertag",
            Silvester => "Silvester",
        }
    }
}

fn bus_und_bettag(year: i32) -> Option<NaiveDate> {
    let reference_date = NaiveDate::from_ymd_opt(year, 11, 23)?;
    let weekday_ordinal = i64::from(reference_date.weekday().num_days_from_monday());
    let duration_to_previous_wednesday = if weekday_ordinal < 3 {
        Duration::days(-(weekday_ordinal + 5))
    } else {
        Duration::days(2 - weekday_ordinal)
    };
    Some(reference_date + duration_to_previous_wednesday)
}

fn date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(year, month, day)
}

fn relative_to_easter_sunday(year: i32, days_offset: i64) -> Option<NaiveDate> {
    let easter_sunday = computus::gregorian(year).ok()?;
    let date = NaiveDate::from_ymd_opt(easter_sunday.year, easter_sunday.month, easter_sunday.day)?;
    Some(date + Duration::days(days_offset))
}
