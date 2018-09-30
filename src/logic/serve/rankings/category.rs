use std::mem::transmute;
use int_hash::IntHashMap;

use super::super::super::super::cache::cache;
use super::super::super::super::cache::cache::VoteCount;
use super::super::super::super::cache::cache::CategoryPeriodPollRankings;
use super::super::super::super::server::codes;
use super::super::super::super::data::byte_counts::ByteCounts;

// NOTE: max page size must fin into u16
const PAGE_SIZE: usize = 1024;

const INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category ids & vote counts
        PAGE_SIZE * (4 + 3)
        // space for the byte counts
        + PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category ids & vote counts
        PAGE_SIZE * (3 + 3)
        // space for the byte counts
        + PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category ids & vote counts
        PAGE_SIZE * (2 + 3)
        // space for the byte counts
        + PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_1_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category ids & vote counts
        PAGE_SIZE * (1 + 3)
        // space for the byte counts
        + PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

pub fn get_todays_category_rankings_by_global_id(
    vcDayId: u32,
    blockIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_PERIOD_IDS.todayVcDayId,
        vcDayId,
        cache::CATEGORY_TODAY_INDEX_MAP,
        cache::TODAY_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
    );
}

pub fn get_todays_category_rankings_by_cache_index(
    vcDayId: u32,
    blockIndex: u32,
    categoryCacheIndex: u32,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_PERIOD_IDS.todayVcDayId,
        vcDayId,
        cache::TODAY_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
    );
}

pub fn get_yesterdays_category_rankings_by_global_id(
    vcDayId: u32,
    blockIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_PERIOD_IDS.yesterdayVcDayId,
        vcDayId,
        cache::CATEGORY_YESTERDAY_INDEX_MAP,
        cache::YESTERDAY_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
    );
}

pub fn get_yesterdays_category_rankings_by_cache_index(
    vcDayId: u32,
    blockIndex: u32,
    categoryCacheIndex: u32,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_PERIOD_IDS.yesterdayVcDayId,
        vcDayId,
        cache::YESTERDAY_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
    );
}

pub fn get_day_b4_yesterdays_category_rankings_by_global_id(
    vcDayId: u32,
    blockIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_PERIOD_IDS.dayB4YesterdayVcDayId,
        vcDayId,
        cache::CATEGORY_DAY_B4_YESTERDAY_INDEX_MAP,
        cache::DAY_B4_YESTERDAY_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
    );
}

pub fn get_day_b4_yesterdays_category_rankings_by_cache_index(
    vcDayId: u32,
    blockIndex: u32,
    categoryCacheIndex: u32,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_PERIOD_IDS.dayB4YesterdayVcDayId,
        vcDayId,
        cache::DAY_B4_YESTERDAY_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
    );
}

pub fn get_this_weeks_category_rankings_by_global_id(
    vcWeekId: u32,
    blockIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_PERIOD_IDS.thisWeekVcWeekId,
        vcDayId,
        cache::CATEGORY_THIS_WEEK_INDEX_MAP,
        cache::THIS_WEEK_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
    );
}

pub fn get_this_weeks_category_rankings_by_cache_index(
    vcWeekId: u32,
    blockIndex: u32,
    categoryCacheIndex: u32,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_PERIOD_IDS.thisWeekVcWeekId,
        vcDayId,
        cache::THIS_WEEK_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
    );
}

pub fn get_last_weeks_category_rankings_by_global_id(
    vcWeekId: u32,
    blockIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_PERIOD_IDS.lastWeekVcWeekId,
        vcDayId,
        cache::CATEGORY_LAST_WEEK_INDEX_MAP,
        cache::LAST_WEEK_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
    );
}

pub fn get_last_weeks_category_rankings_by_cache_index(
    vcWeekId: u32,
    blockIndex: u32,
    categoryCacheIndex: u32,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_PERIOD_IDS.lastWeekVcWeekId,
        vcDayId,
        cache::LAST_WEEK_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
    );
}

pub fn get_this_months_category_rankings_by_global_id(
    vcMonthId: u32,
    blockIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_PERIOD_IDS.thisMonthVcMonthId,
        vcDayId,
        cache::CATEGORY_THIS_MONTH_INDEX_MAP,
        cache::THIS_MONTH_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
    );
}

pub fn get_this_months_category_rankings_by_cache_index(
    vcMonthId: u32,
    blockIndex: u32,
    categoryCacheIndex: u32,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_PERIOD_IDS.thisMonthVcMonthId,
        vcDayId,
        cache::THIS_MONTH_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
    );
}

pub fn get_last_months_category_rankings_by_global_id(
    vcMonthId: u32,
    blockIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_PERIOD_IDS.lastMonthVcMonthId,
        vcDayId,
        cache::CATEGORY_LAST_MONTH_INDEX_MAP,
        cache::LAST_MONTH_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
    );
}

pub fn get_last_months_category_rankings_by_cache_index(
    vcMonthId: u32,
    blockIndex: u32,
    categoryCacheIndex: u32,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_PERIOD_IDS.lastMonthVcMonthId,
        vcDayId,
        cache::LAST_MONTH_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
    );
}

fn get_category_rankings_by_global_id(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    categoryIndexMap: IntHashMap<u64, u32>,
    categoryPollRankings: CategoryPeriodPollRankings,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {
    if currentPeriodId != expectedPeriodId {
        return codes::INVALID_PERIOD_ID_RESPONSE;
    }

    let firstRecordIndex = PAGE_SIZE * blockIndex;

    match categoryIndexMap.get(&globalCategoryId) {
        None => {
            return codes::INVALID_GLOBAL_CATEGORY_ID_RESPONSE;
        }
        Some(categoryIndex) => {
            return get_category_rankings(firstRecordIndex, *categoryIndex, categoryPollRankings);
        }
    }
}

fn get_category_rankings_by_cache_index(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    categoryPollRankings: CategoryPeriodPollRankings,
    categoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {
    if currentPeriodId != expectedPeriodId {
        return codes::INVALID_PERIOD_ID_RESPONSE;
    }

    let firstRecordIndex = PAGE_SIZE * blockIndex;

    match categoryPollRankings.voteCountsByCategoryIndex.get(categoryCacheIndex) {
        None => {
            return codes::INVALID_CATEGORY_CACHE_INDEX_RESPONSE;
        }
        Some(_) => {
            return get_category_rankings(firstRecordIndex, categoryCacheIndex, categoryPollRankings);
        }
    }
}

fn get_category_rankings(
    firstRecordIndex: usize,
    categoryIndex: u32,
    pollRankings: CategoryPeriodPollRankings,
) -> Vec<u8> {
    let voteCountsForCategory = pollRankings.voteCountsByCategoryIndex[categoryIndex];
    match pollRankings.maxPollNumberBytes {
        1 => {
            return get1ByteRecentPolls(voteCountsForCategory, firstRecordIndex);
        }
        2 => {
            return get2ByteRecentPolls(voteCountsForCategory, firstRecordIndex);
        }
        3 => {
            return get3ByteRecentPolls(voteCountsForCategory, firstRecordIndex);
        }
        4 => {
            return get4ByteRecentPolls(voteCountsForCategory, firstRecordIndex);
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE;
}

fn get4ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
) -> Vec<u8> {
    let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES);
    response.push(0b00000000);

    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut byteCounts = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let tzAndPeriodPollIdBytes: [u8; 4] = unsafe {
// In this case tzAndPeriodPollId is the total number of polls in
// the period across all time zones
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                response.extend_from_slice(&tzAndPeriodPollIdBytes);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    byteCounts.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    byteCounts.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    byteCounts.add2();
                } else {
                    response.push(countBytes[3]);
                    byteCounts.add1();
                }
            }
        }
    }
    byteCounts.append(response);

    return response;
}

fn get3ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
) -> Vec<u8> {
    let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES);
    response.push(0b00000011);

    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut byteCounts = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let tzAndPeriodPollIdBytes: [u8; 4] = unsafe {
// In this case tzAndPeriodPollId is the total number of polls in
// the period across all time zones
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                response.extend_from_slice(&tzAndPeriodPollIdBytes[1..3]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    byteCounts.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    byteCounts.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    byteCounts.add2();
                } else {
                    response.push(countBytes[3]);
                    byteCounts.add1();
                }
            }
        }
    }
    byteCounts.append(response);

    return response;
}

fn get2ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
) -> Vec<u8> {
    let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES);
    response.push(0b00000010);

    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut byteCounts = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let tzAndPeriodPollIdBytes: [u8; 4] = unsafe {
// In this case tzAndPeriodPollId is the total number of polls in
// the period across all time zones
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                response.extend_from_slice(&tzAndPeriodPollIdBytes[2..3]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    byteCounts.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    byteCounts.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    byteCounts.add2();
                } else {
                    response.push(countBytes[3]);
                    byteCounts.add1();
                }
            }
        }
    }
    byteCounts.append(response);

    return response;
}

fn get1ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
) -> Vec<u8> {
    let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_1_POLL_BYTES);
    response.push(0b00000001);

    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut byteCounts = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let tzAndPeriodPollIdBytes: [u8; 4] = unsafe {
// In this case tzAndPeriodPollId is the total number of polls in
// the period across all time zones
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                response.push(tzAndPeriodPollIdBytes[3]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    byteCounts.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    byteCounts.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    byteCounts.add2();
                } else {
                    response.push(countBytes[3]);
                    byteCounts.add1();
                }
            }
        }
    }
    byteCounts.append(response);

    return response;
}
