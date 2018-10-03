use std::mem::transmute;
use int_hash::IntHashMap;

use super::super::super::super::server::codes;
use super::super::super::super::cache::cache;
use super::super::super::super::cache::cache::VoteCount;
use super::super::super::super::cache::cache::LocationPollRankings;
use super::super::super::super::data::byte_counts::ByteCounts;

use super::location_and_loc_category::INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES;
use super::location_and_loc_category::INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES;
use super::location_and_loc_category::INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES;
use super::location_and_loc_category::INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES;
use super::location_and_loc_category::INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES;
use super::location_and_loc_category::INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES;
use super::location_and_loc_category::INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES;
use super::location_and_loc_category::get2ByteRecentPolls;
use super::location_and_loc_category::get3ByteRecentPolls;
use super::location_and_loc_category::get4ByteRecentPolls;
use super::location_and_loc_category::get4ByteRecentPolls;
use super::location_and_loc_category::get5ByteRecentPolls;
use super::location_and_loc_category::get6ByteRecentPolls;
use super::location_and_loc_category::get7ByteRecentPolls;
use super::location_and_loc_category::get8ByteRecentPolls;

pub fn get_todays_location_category_rankings_by_global_ids(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
    globalCategoryId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_global_ids(
        currentPeriodIds.todaysVcDayId,
        vcDayId,
        timezoneId,
        cache::LOCATION_TODAYS_INDEX_MAP,
        cache::TODAYS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        globalCategoryId,
        blockIndex,
        cache::TODAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_todays_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_location_cache_index_and_global_category_id(
        currentPeriodIds.todaysVcDayId,
        vcDayId,
        timezoneId,
        cache::LOCATION_CATEGORY_TODAYS_INDEX_MAP,
        cache::TODAYS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        globalCategoryId,
        blockIndex,
        cache::TODAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_todays_location_category_rankings_by_cache_indexes(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_cache_indexes(
        currentPeriodIds.todaysVcDayId,
        vcDayId,
        timezoneId,
        cache::TODAYS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        locationCategoryCacheIndex,
        blockIndex,
        cache::TODAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_yesterdays_location_category_rankings_by_global_ids(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
    globalCategoryId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_global_ids(
        currentPeriodIds.yesterdaysVcDayId,
        vcDayId,
        timezoneId,
        cache::LOCATION_YESTERDAYS_INDEX_MAP,
        cache::YESTERDAYS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        globalCategoryId,
        blockIndex,
        cache::YESTERDAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_yesterdays_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_location_cache_index_and_global_category_id(
        currentPeriodIds.yesterdaysVcDayId,
        vcDayId,
        timezoneId,
        cache::LOCATION_CATEGORY_YESTERDAYS_INDEX_MAP,
        cache::YESTERDAYS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        globalCategoryId,
        blockIndex,
        cache::YESTERDAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_yesterdays_location_category_rankings_by_cache_indexes(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_cache_indexes(
        currentPeriodIds.yesterdaysVcDayId,
        vcDayId,
        timezoneId,
        cache::YESTERDAYS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        locationCategoryCacheIndex,
        blockIndex,
        cache::YESTERDAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_day_b4_yesterdays_location_category_rankings_by_global_ids(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
    globalCategoryId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_global_ids(
        currentPeriodIds.dayB4YesterdaysVcDayId,
        vcDayId,
        timezoneId,
        cache::LOCATION_DAY_B4_YESTERDAYS_INDEX_MAP,
        cache::DAY_B4_YESTERDAYS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        globalCategoryId,
        blockIndex,
        cache::DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}
pub fn get_day_b4_yesterdays_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_location_cache_index_and_global_category_id(
        currentPeriodIds.dayB4YesterdaysVcDayId,
        vcDayId,
        timezoneId,
        cache::LOCATION_CATEGORY_DAY_B4_YESTERDAYS_INDEX_MAP,
        cache::DAY_B4_YESTERDAYS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        globalCategoryId,
        blockIndex,
        cache::DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_day_b4_yesterdays_location_category_rankings_by_cache_indexes(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_cache_indexes(
        currentPeriodIds.dayB4YesterdaysVcDayId,
        vcDayId,
        timezoneId,
        cache::DAY_B4_YESTERDAYS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        locationCategoryCacheIndex,
        blockIndex,
        cache::DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_this_weeks_location_category_rankings_by_global_ids(
    vcWeekId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
    globalCategoryId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_global_ids(
        currentPeriodIds.thisWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::LOCATION_THIS_WEEKS_INDEX_MAP,
        cache::THIS_WEEKS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        globalCategoryId,
        blockIndex,
        cache::THIS_WEEKS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_this_weeks_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vcWeekId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_location_cache_index_and_global_category_id(
        currentPeriodIds.thisWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::LOCATION_CATEGORY_THIS_WEEKS_INDEX_MAP,
        cache::THIS_WEEKS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        globalCategoryId,
        blockIndex,
        cache::THIS_WEEKS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_this_weeks_location_category_rankings_by_cache_indexes(
    vcWeekId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_cache_indexes(
        currentPeriodIds.thisWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::THIS_WEEKS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        locationCategoryCacheIndex,
        blockIndex,
        cache::THIS_WEEKS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_last_weeks_location_category_rankings_by_global_ids(
    vcWeekId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
    globalCategoryId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_global_ids(
        currentPeriodIds.lastWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::LOCATION_LAST_WEEKS_INDEX_MAP,
        cache::LAST_WEEKS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        globalCategoryId,
        blockIndex,
        cache::LAST_WEEKS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_last_weeks_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vcWeekId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_location_cache_index_and_global_category_id(
        currentPeriodIds.lastWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::LOCATION_CATEGORY_LAST_WEEKS_INDEX_MAP,
        cache::LAST_WEEKS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        globalCategoryId,
        blockIndex,
        cache::LAST_WEEKS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_last_weeks_location_category_rankings_by_cache_indexes(
    vcWeekId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_cache_indexes(
        currentPeriodIds.lastWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::LAST_WEEKS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        locationCategoryCacheIndex,
        blockIndex,
        cache::LAST_WEEKS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_this_months_location_category_rankings_by_global_ids(
    vcMonthId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
    globalCategoryId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_global_ids(
        currentPeriodIds.thisMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::LOCATION_THIS_MONTHS_INDEX_MAP,
        cache::THIS_MONTHS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        globalCategoryId,
        blockIndex,
        cache::THIS_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_this_months_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vcMonthId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_location_cache_index_and_global_category_id(
        currentPeriodIds.thisMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::LOCATION_CATEGORY_THIS_MONTHS_INDEX_MAP,
        cache::THIS_MONTHS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        globalCategoryId,
        blockIndex,
        cache::THIS_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_this_months_location_category_rankings_by_cache_indexes(
    vcMonthId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_cache_indexes(
        currentPeriodIds.thisMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::THIS_MONTHS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        locationCategoryCacheIndex,
        blockIndex,
        cache::THIS_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_last_months_location_category_rankings_by_global_ids(
    vcMonthId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
    globalCategoryId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_global_ids(
        currentPeriodIds.lastMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::LOCATION_LAST_MONTHS_INDEX_MAP,
        cache::LAST_MONTHS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        globalCategoryId,
        blockIndex,
        cache::LAST_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_last_months_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vcMonthId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_location_cache_index_and_global_category_id(
        currentPeriodIds.lastMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::LOCATION_CATEGORY_LAST_MONTHS_INDEX_MAP,
        cache::LAST_MONTHS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        globalCategoryId,
        blockIndex,
        cache::LAST_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_last_months_location_category_rankings_by_cache_indexes(
    vcMonthId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_category_rankings_by_cache_indexes(
        currentPeriodIds.lastMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::LAST_MONTHS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        locationCategoryCacheIndex,
        blockIndex,
        cache::LAST_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

#[inline]
fn get_location_category_rankings_by_global_ids(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    timezoneId: u32,
    locationIndexMap: IntHashMap<u64, cache::LocationPeriodIds>,
    givenPeriodLocationPollRankings: Vec<Vec<LocationPollRankings>>,
    globalLocationId: u64,
    globalCategoryId: u64,
    blockIndex: u32,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    if currentPeriodId != expectedPeriodId {
        return codes::INVALID_PERIOD_ID_RESPONSE;
    }

    let locationPeriodIds: cache::LocationPeriodIds = match locationIndexMap.get(*globalLocationId) {
        None => {
            return codes::INVALID_GLOBAL_LOCATION_ID_RESPONSE;
        }
        Some(locationPeriodIds) => {
            locationPeriodIds
        }
    };

    let locationCategoryCacheIndex: u32 = match locationPeriodIds
        .locationCategoryCacheIndexMap.get(*globalCategoryId) {
        None => {
            return codes::INVALID_GLOBAL_CATEGORY_ID_RESPONSE;
        }
        Some(categoryCacheIndex) => {
            categoryCacheIndex
        }
    };


    let locationCacheIndex: u32 = locationPeriodIds.locationCacheIndex;
    let locationPollRankings = givenPeriodLocationPollRankings[timezoneId][locationCacheIndex];
    let firstRecordIndex = PAGE_SIZE * blockIndex;

    return get_location_category_rankings_with_cache_indexes(
        timezoneId, firstRecordIndex, locationCacheIndex, locationCategoryCacheIndex,
        locationPollRankings, maxPollNumberBytes);
}

#[inline]
fn get_location_category_rankings_by_location_cache_index_and_global_category_id(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    timezoneId: u32,
    locationCategoryIndexMap: IntHashMap<u32, cache::LocationPeriodIds>,
    givenPeriodLocationPollRankings: Vec<Vec<LocationPollRankings>>,
    locationCacheIndex: u32,
    globalCategoryId: u64,
    blockIndex: u32,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    if currentPeriodId != expectedPeriodId {
        return codes::INVALID_PERIOD_ID_RESPONSE;
    }

    let locationPollRankings: LocationPollRankings
    = match givenPeriodLocationPollRankings[timezoneId].get(locationCacheIndex) {
        None => {
            return codes::INVALID_LOCATION_CACHE_INDEX_RESPONSE;
        }
        Some(locationPollRankings) => {
            locationPollRankings
        }
    };

    let locationPeriodIds: cache::LocationPeriodIds
    = matlocationCategoryIndexMap.get(*locationCacheIndex).unwrap();

    let locationCategoryCacheIndex: u32 = match locationPeriodIds
        .locationCategoryCacheIndexMap.get(*globalCategoryId) {
        None => {
            return codes::INVALID_GLOBAL_CATEGORY_ID_RESPONSE;
        }
        Some(categoryCacheIndex) => {
            categoryCacheIndex
        }
    };

    let locationPollRankings = givenPeriodLocationPollRankings[timezoneId][locationCacheIndex];
    let firstRecordIndex = PAGE_SIZE * blockIndex;

    return get_location_category_rankings_with_category_cache_index(
        timezoneId, firstRecordIndex, locationCacheIndex, locationCategoryCacheIndex,
        locationPollRankings, maxPollNumberBytes);
}

#[inline]
fn get_location_category_rankings_by_cache_indexes(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    timezoneId: u32,
    givenPeriodLocationPollRankings: Vec<Vec<LocationPollRankings>>,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
    blockIndex: u32,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    if currentPeriodId != expectedPeriodId {
        return codes::INVALID_PERIOD_ID_RESPONSE;
    }

    let locationPollRankings: LocationPollRankings
    = match givenPeriodLocationPollRankings[timezoneId].get(locationCacheIndex) {
        None => {
            return codes::INVALID_LOCATION_CACHE_INDEX_RESPONSE;
        }
        Some(locationPollRankings) => {
            locationPollRankings
        }
    };

    let locationCategoryVoteCounts: Vec<VoteCount>
    = match locationPollRankings.categoryLocations.get(locationCategoryCacheIndex) {
        None => {
            return codes::INVALID_CATEGORY_CACHE_INDEX_RESPONSE;
        }
        Some(locationCategoryVoteCounts) => {
            locationCategoryVoteCounts
        }
    };

    let firstRecordIndex = PAGE_SIZE * blockIndex;

    return get_location_category_rankings(
        firstRecordIndex, locationCategoryVoteCounts,maxPollNumberBytes);
}

#[inline]
fn get_location_category_rankings_with_cache_indexes(
    timezoneId: u32,
    firstRecordIndex: usize,
    locationCacheIndex: u32,
    categoryCacheIndex: u32,
    locationPollRankings: LocationPollRankings,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    let voteCountsForLocation
    = locationPollRankings.categoryLocations[categoryCacheIndex];
    let locationCacheIndexBytes: [u8; 4] = unsafe {
        std::mem::transmute(*locationCacheIndex);
    };
    let categoryCacheIndexBytes: [u8; 4] = unsafe {
        std::mem::transmute(*categoryCacheIndex);
    };

    match maxPollNumberBytes {
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES);
            response.push(0b00000010);
            response.extend_from_slice(&locationCacheIndexBytes);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get2ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES);
            response.push(0b00000011);
            response.extend_from_slice(&locationCacheIndexBytes);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get3ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES);
            response.push(0b00000100);
            response.extend_from_slice(&locationCacheIndexBytes);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        5 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES);
            response.push(0b00000101);
            response.extend_from_slice(&locationCacheIndexBytes);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        6 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES);
            response.push(0b00000110);
            response.extend_from_slice(&locationCacheIndexBytes);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        7 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES);
            response.push(0b00000111);
            response.extend_from_slice(&locationCacheIndexBytes);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        8 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES);
            response.push(0b00000000);
            response.extend_from_slice(&locationCacheIndexBytes);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE;
}

#[inline]
fn get_location_category_rankings_with_category_cache_index(
    timezoneId: u32,
    firstRecordIndex: usize,
    locationCacheIndex: u32,
    categoryCacheIndex: u32,
    locationPollRankings: LocationPollRankings,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    let voteCountsForLocation
    = locationPollRankings.categoryLocations[categoryCacheIndex];
    let categoryCacheIndexBytes: [u8; 4] = unsafe {
        std::mem::transmute(*categoryCacheIndex);
    };

    match maxPollNumberBytes {
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES);
            response.push(0b00000010);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get2ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES);
            response.push(0b00000011);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get3ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES);
            response.push(0b00000100);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        5 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES);
            response.push(0b00000101);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        6 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES);
            response.push(0b00000110);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        7 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES);
            response.push(0b00000111);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        8 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES);
            response.push(0b00000000);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE;
}

#[inline]
fn get_location_category_rankings(
    firstRecordIndex: usize,
    locationCategoryVoteCounts: Vec<VoteCount>,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    let voteCountsForLocation = locationPollRankings.location;

    match maxPollNumberBytes {
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES);
            response.push(0b00000010);

            return get2ByteRecentPolls(locationCategoryVoteCounts, firstRecordIndex, response);
        }
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES);
            response.push(0b00000011);

            return get3ByteRecentPolls(locationCategoryVoteCounts, firstRecordIndex, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES);
            response.push(0b00000100);

            return get4ByteRecentPolls(locationCategoryVoteCounts, firstRecordIndex, response);
        }
        5 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES);
            response.push(0b00000101);

            return get4ByteRecentPolls(locationCategoryVoteCounts, firstRecordIndex, response);
        }
        6 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES);
            response.push(0b00000110);

            return get4ByteRecentPolls(locationCategoryVoteCounts, firstRecordIndex, response);
        }
        7 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES);
            response.push(0b00000111);

            return get4ByteRecentPolls(locationCategoryVoteCounts, firstRecordIndex, response);
        }
        8 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES);
            response.push(0b00000000);

            return get4ByteRecentPolls(locationCategoryVoteCounts, firstRecordIndex, response);
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE;
}
