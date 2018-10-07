use int_hash::IntHashMap;


use super::super::super::cache::cache;
use super::super::super::cache::cache::CategoryId;
use super::super::super::cache::cache::DayId;
use super::super::super::cache::cache::LocationId;
use super::super::super::cache::cache::LocationPollPrependLists;
use super::super::super::cache::cache::MonthId;
use super::super::super::cache::cache::PollId;
use super::super::super::cache::cache::TimezoneId;
use super::super::super::cache::cache::WeekId;


/**
 *
 *  Entering polls early - not needed for Minimum Viable Product.
 *
 *  Batching poll entries to reduce rehashing:
 *
 *    what are the chances of polls coming in for multiple
 *    categories at the same time - very high
 *
 *    what are the chances of polls coming in for the same
 *    category in the same location at the same time
 *          - not as high
 */
pub fn add_polls(
    rawData: &[u8]
) {}

fn add_day_after_tomorrows_poll(
    vcDayId: DayId,
    timezoneId: TimezoneId,
    globalLocationId: LocationId,
    globalCategoryIds: Vec<u64>,
    globalPollId: PollId,
) {
    let data: IntHashMap<LocationId, LocationPollPrependLists>
    = cache::DAY_AFTER_TOMORROWS_POLLS_BY_LOCATION.get(timezoneId).unwrap();

    if data.len() == data.capacity() {
        data.reserve(20);

    }
}

fn grow

fn add_tomorrows_poll(
    vcDayId: DayId,
    timezoneId: TimezoneId,
    globalLocationId: LocationId,
    globalCategoryIds: Vec<u64>,
    globalPollId: PollId,
) {}

fn add_week_poll(
    vcWeekId: WeekId,
    timezoneId: TimezoneId,
    globalLocationId: LocationId,
    globalCategoryIds: Vec<u64>,
    globalPollId: PollId,
) {}

fn add_month_poll(
    vcMonthId: MonthId,
    timezoneId: TimezoneId,
    globalLocationId: LocationId,
    globalCategoryIds: Vec<u64>,
    globalPollId: PollId,
) {}
