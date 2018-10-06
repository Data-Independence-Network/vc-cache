use super::super::super::super::cache::cache::DayId;
use super::super::super::super::cache::cache::LocationId;
use super::super::super::super::cache::cache::MonthId;
use super::super::super::super::cache::cache::PollId;
use super::super::super::super::cache::cache::TimezoneId;
use super::super::super::super::cache::cache::WeekId;

pub fn get_tomorrows_location_polls(
    vcDayId: DayId,
    timezoneId: TimezoneId,
    blockIndex: u32,
    globalLocationId: LocationId,
) -> Vec<u8> {

}

pub fn get_day_after_tomorrows_location_polls(
    vcDayId: DayId,
    timezoneId: TimezoneId,
    blockIndex: u32,
    globalLocationId: LocationId,
) -> Vec<u8> {

}

pub fn get_next_weeks_location_polls(
    vcWeekId: WeekId,
    timezoneId: TimezoneId,
    blockIndex: u32,
    globalLocationId: LocationId,
) -> Vec<u8> {

}

pub fn get_next_months_location_polls(
    vcMonthId: MonthId,
    timezoneId: TimezoneId,
    blockIndex: u32,
    globalLocationId: LocationId,
) -> Vec<u8> {

}
