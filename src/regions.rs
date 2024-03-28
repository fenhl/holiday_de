use chrono::{Datelike, NaiveDate};

/// Represents all regions and their public holidays within Germany.
///
/// Holidays guaranteed to take place on sundays, e.g. easter sunday, are excluded by default.
/// However, holidays with a fixed date can still fall on a sunday.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GermanRegion {
    BadenWuerttemberg,
    /// Communities of Bavaria with Protestant majorities.
    BayernEv,
    /// Communities of Bavaria with Catholic majorities, not including Augsburg.
    BayernKath,
    Augsburg,
    Berlin,
    Brandenburg,
    Bremen,
    Hamburg,
    Hessen,
    MecklenburgVorpommern,
    Niedersachsen,
    NordrheinWestfalen,
    RheinlandPfalz,
    Saarland,
    /// Parts of Saxony where Fronleichnam is not a public holiday.
    SachsenOhneFronleichnam,
    /// Parts of Saxony where Fronleichnam is a public holiday.
    SachsenMitFronleichnam,
    SachsenAnhalt,
    SchleswigHolstein,
    /// Parts of Thuringia where Fronleichnam is not a public holiday.
    ThueringenOhneFronleichnam,
    /// Parts of Thuringia where Fronleichnam is a public holiday.
    ThueringenMitFronleichnam,
}

use crate::holidays::GermanHoliday;
use crate::holidays::GermanHoliday::*;
use crate::regions::GermanRegion::*;

impl GermanRegion {
    /// Returns all public holidays in the given year.
    /// Holidays guaranteed to take place on sundays, e.g. easter sunday, are excluded by default.
    ///
    /// For years before 1995 this list will be empty.
    pub fn holidays_in_year(&self, year: i32) -> Vec<GermanHoliday> {
        if year < 1995 {
            return Vec::new();
        }
        let mut holidays = Vec::new();
        holidays.extend_from_slice(BUNDESWEITE_FEIERTAGE);
        holidays.extend_from_slice(self.region_specific_holidays(year));
        if year == 2017 && !holidays.contains(&Reformationstag) {
            // BW: https://www.landesrecht-bw.de/perma?d=jlr-FeiertGBWV1P1a
            // BY: https://www.bayern.landtag.de/www/ElanTextAblage_WP17/Drucksachen/Folgedrucksachen/0000007000/0000007463.pdf
            // BE: https://gesetze.berlin.de/bsbe/document/aiz-jlr-FeiertGBErahmen%4020151025/part/x
            // HE: https://www.rv.hessenrecht.hessen.de/bshe/document/jlr-RefT2017VHErahmen/part/X
            // NW: https://www.landtag.nrw.de/portal/WWW/dokumentenarchiv/Dokument?Id=XMMGVB1528%7C496%7C496
            // RP: https://web.archive.org/web/20160305005630/https://www.rlp.de/fr/aktuelles/einzelansicht/news/detail/News/zusaetzlicher-feiertag-2017/
            // SL: https://web.archive.org/web/20160306062414/http://sl.juris.de/cgi-bin/landesrecht.py?d=http%3A%2F%2Fsl.juris.de%2Fsl%2Fgesamt%2FRefT2017V_SL.htm
            holidays.push(Reformationstag);
        }
        holidays
    }

    fn region_specific_holidays(&self, year: i32) -> &'static [GermanHoliday] {
        match self {
            BadenWuerttemberg => &[HeiligeDreiKoenige, Fronleichnam, Allerheiligen],
            BayernEv => &[
                HeiligeDreiKoenige,
                Fronleichnam,
                Allerheiligen,
            ],
            BayernKath => &[
                HeiligeDreiKoenige,
                Fronleichnam,
                MariaeHimmelfahrt,
                Allerheiligen,
            ],
            Augsburg => &[
                HeiligeDreiKoenige,
                Fronleichnam,
                AugsburgerFriedensfest,
                // https://statistik.bayern.de/statistik/gebiet_bevoelkerung/zensus/himmelfahrt/index.php
                // https://www.zensus2022.de/DE/Aktuelles/Zensus_2022_Ergebnisveroeffentlichung_verschiebt_sich_in_Sommer_2024.html
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
            Bremen => {
                if year >= 2017 {
                    &[Reformationstag]
                } else {
                    &[]
                }
            }
            Hamburg => {
                if year >= 2017 {
                    &[Reformationstag]
                } else {
                    &[]
                }
            }
            Hessen => &[Fronleichnam],
            MecklenburgVorpommern => {
                if year >= 2023 {
                    &[Frauentag, Reformationstag]
                } else {
                    &[Reformationstag]
                }
            }
            Niedersachsen => {
                if year >= 2017 {
                    &[Reformationstag]
                } else {
                    &[]
                }
            }
            NordrheinWestfalen => &[Fronleichnam, Allerheiligen],
            RheinlandPfalz => &[Fronleichnam, Allerheiligen],
            Saarland => &[Fronleichnam, MariaeHimmelfahrt, Allerheiligen],
            SachsenOhneFronleichnam => &[Reformationstag, BussUndBettag],
            SachsenMitFronleichnam => &[Fronleichnam, Reformationstag, BussUndBettag],
            SachsenAnhalt => &[HeiligeDreiKoenige, Reformationstag],
            SchleswigHolstein => {
                if year >= 2017 {
                    &[Reformationstag]
                } else {
                    &[]
                }
            }
            ThueringenOhneFronleichnam => {
                if year >= 2019 {
                    &[Weltkindertag, Reformationstag]
                } else {
                    &[Reformationstag]
                }
            }
            ThueringenMitFronleichnam => {
                if year >= 2019 {
                    &[Fronleichnam, Reformationstag]
                } else {
                    &[Fronleichnam, Weltkindertag, Reformationstag]
                }
            }
        }
    }

    /// Returns all holidays and their dates in the given year.
    /// Holidays guaranteed to take place on sundays, e.g. easter sunday, are excluded by default.
    ///
    /// For years before 1995 this list will be empty.
    pub fn holiday_dates_in_year(&self, year: i32) -> Vec<(NaiveDate, GermanHoliday)> {
        let mut holiday_dates: Vec<(NaiveDate, GermanHoliday)> = self
            .holidays_in_year(year)
            .into_iter()
            .flat_map(|holiday| holiday.date(year).map(|date| (date, holiday)))
            .collect();
        holiday_dates.sort_unstable_by_key(|(date, _)| *date);
        holiday_dates
    }

    /// Checks if a given date is a public holiday in the specific region.
    ///
    /// Always `false` for dates before 1995.
    pub fn is_holiday(&self, date: NaiveDate) -> bool {
        self.holiday_from_date(date).is_some()
    }

    /// Returns the holiday for a specific date if the date is a holiday in the specific region.
    ///
    /// Always `None` for dates before 1995.
    pub fn holiday_from_date(&self, date: NaiveDate) -> Option<GermanHoliday> {
        self.holidays_in_year(date.year())
            .into_iter()
            .find(|holiday| holiday.date(date.year()) == Some(date))
    }
}

const BUNDESWEITE_FEIERTAGE: &'static [GermanHoliday] = &[
    Neujahr,
    Karfreitag,
    Ostermontag,
    ErsterMai,
    ChristiHimmelfahrt,
    Pfingstmontag,
    // https://www.gesetze-im-internet.de/einigvtr/art_2.html
    TagDerDeutschenEinheit,
    ErsterWeihnachtsfeiertag,
    ZweiterWeihnachtsfeiertag,
];
