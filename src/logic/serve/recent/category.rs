use super::super::super::super::cache::cache;
use super::super::super::super::cache::cache::PollId;
use super::super::super::super::data::prepend::GlobalNode;

const noResults: Vec<u8> = Vec::new();

pub fn get_tomorrows_category_polls(
    // 1 based index
    blockNumber: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_global_category_polls(
        cache::TOMORROWS_POLLS_BY_CATEGORY,
        blockNumber, globalCategoryId,
        cache::TOMORROWS_POLL_ID_BYTE_COUNTS[38]);
}

pub fn get_day_after_tomorrows_category_polls(
    // 1 based index
    blockNumber: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_global_category_polls(
        cache::DAY_AFTER_TOMORROWS_POLLS_BY_CATEGORY,
        blockNumber, globalCategoryId,
        cache::DAY_AFTER_TOMORROWS_POLL_ID_BYTE_COUNTS[38]);
}

pub fn get_next_weeks_category_polls(
    // 1 based index
    blockNumber: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_global_category_polls(
        cache::NEXT_WEEKS_POLLS_BY_CATEGORY,
        blockNumber, globalCategoryId,
        cache::NEXT_WEEKS_POLL_ID_BYTE_COUNTS[38]);
}

pub fn get_next_months_category_polls(
    // 1 based index
    blockNumber: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_global_category_polls(
        cache::NEXT_MONTHS_POLLS_BY_CATEGORY,
        blockNumber, globalCategoryId,
        cache::NEXT_MONTHS_POLL_ID_BYTE_COUNTS[38]);
}


fn get_global_category_polls(
    globalCategoryPolls: GlobalNode<Vec<Vec<PollId>>>,
    // 1 based index
    blockNumber: u32,
    globalCategoryId: u64,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    let categoryPolls: Vec<Vec<PollId>> = match globalCategoryPolls.get(globalCategoryId) {
        None => {
            return noResults;
        }
        Some(polls) => {
            polls
        }
    };
    let pollsBlock: Vec<PollId> = match categoryPolls.get(categoryPolls.len() - blockNumber) {
        None => {
            return noResults;
        }
        Some(block) => {
            block
        }
    };
    let mut response: Vec<u8> = Vec::with_capacity(maxPollNumberBytes * pollsBlock.len() + 1);

    match maxPollNumberBytes {
        3 => {
            response.push(0b00000011);
            return get3ByteRecentPollIds(pollsBlock, response);
        }
        4 => {
            response.push(0b00000100);
            return get4ByteRecentPollIds(pollsBlock, response);
        }
        2 => {
            response.push(0b00000010);
            return get2ByteRecentPollIds(pollsBlock, response);
        }
        5 => {
            response.push(0b00000101);
            return get5ByteRecentPollIds(pollsBlock, response);
        }
        6 => {
            response.push(0b00000110);
            return get6ByteRecentPollIds(pollsBlock, response);
        }
        7 => {
            response.push(0b00000111);
            return get7ByteRecentPollIds(pollsBlock, response);
        }
        8 => {
            response.push(0b00000000);
            return get8ByteRecentPollIds(pollsBlock, response);
        }
    }
}

#[inline]
fn get2ByteRecentPollIds(
    pollIds: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in pollIds {
        let pollIdBytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*voteCount.tzAndPeriodPollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&pollIdBytes[6..7]);
    }

    return response;
}

#[inline]
fn get3ByteRecentPollIds(
    pollIds: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in pollIds {
        let pollIdBytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*voteCount.tzAndPeriodPollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&pollIdBytes[5..7]);
    }

    return response;
}

#[inline]
fn get4ByteRecentPollIds(
    pollIds: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in pollIds {
        let pollIdBytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*voteCount.tzAndPeriodPollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&pollIdBytes[4..7]);
    }

    return response;
}

#[inline]
fn get5ByteRecentPollIds(
    pollIds: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in pollIds {
        let pollIdBytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*voteCount.tzAndPeriodPollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&pollIdBytes[3..7]);
    }

    return response;
}

#[inline]
fn get6ByteRecentPollIds(
    pollIds: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in pollIds {
        let pollIdBytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*voteCount.tzAndPeriodPollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&pollIdBytes[2..7]);
    }

    return response;
}

#[inline]
fn get7ByteRecentPollIds(
    pollIds: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in pollIds {
        let pollIdBytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*voteCount.tzAndPeriodPollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&pollIdBytes[1..7]);
    }

    return response;
}

#[inline]
fn get8ByteRecentPollIds(
    pollIds: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in pollIds {
        let pollIdBytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*voteCount.tzAndPeriodPollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&pollIdBytes);
    }

    return response;
}