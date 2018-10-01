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

pub fn get_todays_location_rankings_by_global_id(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
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

    return get_location_rankings_by_global_id(
        currentPeriodIds.todaysVcDayId,
        vcDayId,
        timezoneId,
        cache::LOCATION_TODAYS_INDEX_MAP,
        cache::TODAYS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        blockIndex,
        cache::TODAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_todays_location_rankings_by_cache_index(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
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

    return get_category_rankings_by_cache_index(
        currentPeriodIds.todaysVcDayId,
        vcDayId,
        timezoneId,
        cache::TODAYS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        blockIndex,
        cache::TODAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_yesterdays_location_rankings_by_global_id(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
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

    return get_location_rankings_by_global_id(
        currentPeriodIds.yesterdaysVcDayId,
        vcDayId,
        timezoneId,
        cache::LOCATION_YESTERDAYS_INDEX_MAP,
        cache::YESTERDAY_LOCATION_POLL_RANKINGS,
        globalLocationId,
        blockIndex,
        cache::YESTERDAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_yesterdays_location_rankings_by_cache_index(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
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

    return get_category_rankings_by_cache_index(
        currentPeriodIds.yesterdaysVcDayId,
        vcDayId,
        timezoneId,
        cache::YESTERDAY_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        blockIndex,
        cache::YESTERDAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_day_b4_yesterdays_location_rankings_by_global_id(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
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

    return get_location_rankings_by_global_id(
        currentPeriodIds.dayB4YesterdaysVcDayId,
        vcDayId,
        timezoneId,
        cache::LOCATION_DAY_B4_YESTERDAYS_INDEX_MAP,
        cache::DAY_B4_YESTERDAYS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        blockIndex,
        cache::DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_day_b4_yesterdays_location_rankings_by_cache_index(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
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

    return get_category_rankings_by_cache_index(
        currentPeriodIds.dayB4YesterdaysVcDayId,
        vcDayId,
        timezoneId,
        cache::DAY_B4_YESTERDAYS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        blockIndex,
        cache::DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_this_weeks_location_rankings_by_global_id(
    vcWeekId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
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

    return get_location_rankings_by_global_id(
        currentPeriodIds.thisWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::LOCATION_THIS_WEEKS_INDEX_MAP,
        cache::THIS_WEEKS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        blockIndex,
        cache::THIS_WEEKS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_this_weeks_location_rankings_by_cache_index(
    vcWeekId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
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

    return get_category_rankings_by_cache_index(
        currentPeriodIds.thisWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::THIS_WEEKS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        blockIndex,
        cache::THIS_WEEKS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_last_weeks_location_rankings_by_global_id(
    vcWeekId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
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

    return get_location_rankings_by_global_id(
        currentPeriodIds.lastWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::LOCATION_LAST_WEEKS_INDEX_MAP,
        cache::LAST_WEEKS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        blockIndex,
        cache::LAST_WEEKS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_last_weeks_location_rankings_by_cache_index(
    vcWeekId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
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

    return get_category_rankings_by_cache_index(
        currentPeriodIds.lastWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::LAST_WEEKS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        blockIndex,
        cache::LAST_WEEKS_LOCATION_POLL_RANKINGS[timezoneId],
    );
}

pub fn get_this_months_location_rankings_by_global_id(
    vcMonthId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
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

    return get_location_rankings_by_global_id(
        currentPeriodIds.thisMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::LOCATION_THIS_MONTHS_INDEX_MAP,
        cache::THIS_MONTHS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        blockIndex,
        cache::THIS_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_this_months_location_rankings_by_cache_index(
    vcMonthId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
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

    return get_category_rankings_by_cache_index(
        currentPeriodIds.thisMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::THIS_MONTHS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        blockIndex,
        cache::THIS_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_last_months_location_rankings_by_global_id(
    vcMonthId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
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

    return get_location_rankings_by_global_id(
        currentPeriodIds.lastMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::LOCATION_LAST_MONTHS_INDEX_MAP,
        cache::LAST_MONTHS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        blockIndex,
        cache::LAST_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_last_months_location_rankings_by_cache_index(
    vcMonthId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
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

    return get_category_rankings_by_cache_index(
        currentPeriodIds.lastMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::LAST_MONTHS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        blockIndex,
        cache::LAST_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

fn get_location_rankings_by_global_id(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    timezoneId: u32,
    locationIndexMap: IntHashMap<u64, cache::LocationPeriodIds>,
    givenPeriodLocationPollRankings: Vec<Vec<LocationPollRankings>>,
    globalLocationId: u64,
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

    let locationCacheIndex: u32 = locationPeriodIds.locationCacheIndex;
    let locationPollRankings = givenPeriodLocationPollRankings[timezoneId][locationCacheIndex];
    let firstRecordIndex = PAGE_SIZE * blockIndex;

    return get_location_rankings_with_location_cache_index(
        timezoneId, firstRecordIndex, locationCacheIndex,
        locationPollRankings, maxPollNumberBytes);
}

fn get_category_rankings_by_cache_index(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    timezoneId: u32,
    givenPeriodLocationPollRankings: Vec<Vec<LocationPollRankings>>,
    locationCacheIndex: u32,
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

    let firstRecordIndex = PAGE_SIZE * blockIndex;

    return get_location_rankings(
        timezoneId, firstRecordIndex, categoryCacheIndex,
        locationPollRankings, maxPollNumberBytes);
}

#[inline]
fn get_location_rankings_with_location_cache_index(
    timezoneId: u32,
    firstRecordIndex: usize,
    locationCacheIndex: u32,
    locationPollRankings: LocationPollRankings,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    let voteCountsForLocation = locationPollRankings.location;
    let locationCacheIndexBytes: [u8; 4] = unsafe {
        std::mem::transmute(*locationCacheIndex);
    };

    match maxPollNumberBytes {
        1 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_1_POLL_BYTES);
            response.push(0b00000000);
            response.extend_from_slice(&locationCacheIndexBytes);

            return get1ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES);
            response.push(0b00000000);
            response.extend_from_slice(&locationCacheIndexBytes);

            return get2ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES);
            response.push(0b00000000);
            response.extend_from_slice(&locationCacheIndexBytes);

            return get3ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES);
            response.push(0b00000000);
            response.extend_from_slice(&locationCacheIndexBytes);

            return get4ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE;
}

#[inline]
fn get_location_rankings(
    timezoneId: u32,
    firstRecordIndex: usize,
    locationCacheIndex: u32,
    locationPollRankings: LocationPollRankings,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    let voteCountsForLocation = locationPollRankings.location;

    match maxPollNumberBytes {
        1 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_1_POLL_BYTES);
            response.push(0b00000000);

            return get1ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES);
            response.push(0b00000000);

            return get2ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES);
            response.push(0b00000000);

            return get3ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES);
            response.push(0b00000000);

            return get4ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE;
}

#[inline]
fn get4ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);
    let mut pollTypes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.pollTypeAndTz & 0b00000011 {
                    codes::POLL_TYPE_1D => {
                        pollTypes.add1();
                    }
                    codes::POLL_TYPE_2D => {
                        pollTypes.add2();
                    }
                    codes::POLL_TYPE_3D => {
                        pollTypes.add3();
                    }
                }

                let tzAndPeriodPollIdBytes: [u8; 4] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                response.extend_from_slice(&tzAndPeriodPollIdBytes);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    byteCounts.appendData(response);
    pollTypes.append(response);

    return response;
}

#[inline]
fn get3ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);
    let mut pollTypes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.pollTypeAndTz & 0b00000011 {
                    codes::POLL_TYPE_1D => {
                        pollTypes.add1();
                    }
                    codes::POLL_TYPE_2D => {
                        pollTypes.add2();
                    }
                    codes::POLL_TYPE_3D => {
                        pollTypes.add3();
                    }
                }

                let tzAndPeriodPollIdBytes: [u8; 4] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                response.extend_from_slice(&tzAndPeriodPollIdBytes[1..3]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    byteCounts.appendData(response);
    pollTypes.append(response);

    return response;
}

#[inline]
fn get2ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);
    let mut pollTypes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.pollTypeAndTz & 0b00000011 {
                    codes::POLL_TYPE_1D => {
                        pollTypes.add1();
                    }
                    codes::POLL_TYPE_2D => {
                        pollTypes.add2();
                    }
                    codes::POLL_TYPE_3D => {
                        pollTypes.add3();
                    }
                }

                let tzAndPeriodPollIdBytes: [u8; 4] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                response.extend_from_slice(&tzAndPeriodPollIdBytes[2..3]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    byteCounts.appendData(response);
    pollTypes.append(response);

    return response;
}

#[inline]
fn get1ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);
    let mut pollTypes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.pollTypeAndTz & 0b00000011 {
                    codes::POLL_TYPE_1D => {
                        pollTypes.add1();
                    }
                    codes::POLL_TYPE_2D => {
                        pollTypes.add2();
                    }
                    codes::POLL_TYPE_3D => {
                        pollTypes.add3();
                    }
                }

                let tzAndPeriodPollIdBytes: [u8; 4] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                response.push(tzAndPeriodPollIdBytes[3]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    byteCounts.appendData(response);
    pollTypes.append(response);

    return response;
}
