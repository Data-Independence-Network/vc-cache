use std::mem::transmute;
use int_hash::IntHashMap;

use super::super::super::super::server::codes;
use super::super::super::super::cache::cache;
use super::super::super::super::cache::cache::VoteCount;
use super::super::super::super::cache::cache::LocationPollRankings;
use super::super::super::super::data::byte_counts::ByteCounts;

// NOTE: max page size must fin into u16
const PAGE_SIZE: usize = 1024;

const INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for location cache index (if any
        4 +
        // space for location ids & vote counts
        PAGE_SIZE * (4 + 3) +
        // space for poll type
        PAGE_SIZE / 4 +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for location cache index (if any
        4 +
        // space for location ids & vote counts
        PAGE_SIZE * (3 + 3) +
        // space for poll type
        PAGE_SIZE / 4 +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for location cache index (if any
        4 +
        // space for location ids & vote counts
        PAGE_SIZE * (2 + 3) +
        // space for poll type
        PAGE_SIZE / 4 +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_1_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for location cache index (if any
        4 +
        // space for location ids & vote counts
        PAGE_SIZE * (1 + 3) +
        // space for poll type
        PAGE_SIZE / 4 +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;
pub fn get_todays_location_category_rankings_by_global_location_and_category_ids(
    vcDayId: u32,
    globalLocationId: u64,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_todays_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vcDayId: u32,
    locationCacheIndex: u64,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_todays_location_category_rankings_by_cache_indexes(
    vcDayId: u32,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_yesterdays_location_category_rankings_by_global_location_and_category_ids(
    vcDayId: u32,
    globalLocationId: u64,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_yesterdays_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vcDayId: u32,
    locationCacheIndex: u64,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_yesterdays_location_category_rankings_by_cache_indexes(
    vcDayId: u32,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_day_b4_yesterdays_location_category_rankings_by_global_location_and_category_ids(
    vcDayId: u32,
    globalLocationId: u64,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}
pub fn get_day_b4_yesterdays_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vcDayId: u32,
    locationCacheIndex: u64,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_day_b4_yesterdays_location_category_rankings_by_cache_indexes(
    vcDayId: u32,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_this_weeks_location_category_rankings_by_global_location_and_category_ids(
    vcWeekId: u32,
    globalLocationId: u64,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_this_weeks_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vcWeekId: u32,
    locationCacheIndex: u64,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_this_weeks_location_category_rankings_by_cache_indexes(
    vcWeekId: u32,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_last_weeks_location_category_rankings_by_global_location_and_category_ids(
    vcWeekId: u32,
    globalLocationId: u64,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_last_weeks_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vcWeekId: u32,
    locationCacheIndex: u64,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_last_weeks_location_category_rankings_by_cache_indexes(
    vcWeekId: u32,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_this_months_location_category_rankings_by_global_location_and_category_ids(
    vcMonthId: u32,
    globalLocationId: u64,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_this_months_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vcMonthId: u32,
    locationCacheIndex: u64,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_this_months_location_category_rankings_by_cache_indexes(
    vcMonthId: u32,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_last_months_location_category_rankings_by_global_location_and_category_ids(
    vcMonthId: u32,
    globalLocationId: u64,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_last_months_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vcMonthId: u32,
    locationCacheIndex: u64,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_last_months_location_category_rankings_by_cache_indexes(
    vcMonthId: u32,
    locationCacheIndex: u32,
    locationCategoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {

}
