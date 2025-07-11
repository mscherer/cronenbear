/*
use toml::Table;
use toml::Value;
use std::collections::HashMap;
use toml::map::Map;
use include_dir::{include_dir, Dir};
use std::fs;

*/
// TODO faire un type ?
//
//fn construct_calendar_url(

/*fn construct_calendar_url(id: HolidaysID) -> CalendarURL {
    let name = format!("{:?}.{:?}#holiday@group.v.calendar.google.com", LANGUAGE, id);
    CalendarURL(format!("{:?}{:?}{:?}", CALENDAR_PREFIX, name, CALENDAR_SUFFIX))
}

fn load_toml_file(file: &str) -> toml::Table {
// TODO make it less ugly with less unwrap
    let f = DATA_DIR.get_file(file).unwrap().contents_utf8().unwrap();
    return f.parse::<Table>().unwrap();
}
*/

mod country_calendar;
mod google_public_calendar;
use crate::country_calendar::CountryCalendar;
use crate::google_public_calendar::GooglePublicCalendar;

fn main() {
    let fr = CountryCalendar::try_from("fr").unwrap();
    let de = CountryCalendar::try_from("de").unwrap();
    println!("{}", de.construct_calendar_url());
    //  let mut merge_calendar = MergedCalendar::new();
    //  merge_calendar.add(fr);
    //  merge_calendar.add(de);
    //println!("{}", merge_calendar.generate_ical());

    //  load_all_calendars();
}
