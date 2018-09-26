use super::super::data::prepend::list::PrependList;

pub static mut data: Vec<u32> = Vec::new();

pub static mut LAST_MONTH_LOCATION_POLL_RANKINGS: Vec<u32> = Vec::new();
pub static mut THIS_MONTH_LOCATION_POLL_RANKINGS: Vec<u32> = Vec::new();

pub static mut LAST_WEEK_LOCATION_POLL_RANKINGS: Vec<u32> = Vec::new();
pub static mut THIS_WEEK_LOCATION_POLL_RANKINGS: Vec<u32> = Vec::new();

pub static mut DAY_BEFORE_YESTERDAY_LOCATION_POLL_RANKINGS: Vec<u32> = Vec::new();
pub static mut YESTERDAY_LOCATION_POLL_RANKINGS: Vec<u32> = Vec::new();
pub static mut TODAY_LOCATION_POLL_RANKINGS: Vec<u32> = Vec::new();

pub static mut NEXT_MONTHS_LOCATION_POLLS: Vec<u32> = Vec::new();
pub static mut NEXT_WEEKS_LOCATION_POLLS: Vec<u32> = Vec::new();
pub static mut TOMORROWS_LOCATION_POLLS: Vec<u32> = Vec::new();
pub static mut DAY_AFTER_TOMORROWS_LOCATION_POLLS: Vec<u32> = Vec::new();


pub static mut LAST_MONTH_CATEGORY_POLL_RANKINGS: LsbShiftTree<[CategoryPollRanking]> = LsbShiftTree::new();
pub static mut THIS_MONTH_CATEGORY_POLL_RANKINGS: LsbShiftTree<[CategoryPollRanking]> = LsbShiftTree::new();

pub static mut LAST_WEEK_CATEGORY_POLL_RANKINGS: LsbShiftTree<[CategoryPollRanking]> = LsbShiftTree::new();
pub static mut THIS_WEEK_CATEGORY_POLL_RANKINGS: LsbShiftTree<[CategoryPollRanking]> = LsbShiftTree::new();

pub static mut DAY_BEFORE_YESTERDAY_CATEGORY_POLL_RANKINGS: LsbShiftTree<[CategoryPollRanking]> = LsbShiftTree::new();
pub static mut YESTERDAY_CATEGORY_POLL_RANKINGS: LsbShiftTree<[CategoryPollRanking]> = LsbShiftTree::new();
pub static mut TODAY_CATEGORY_POLL_RANKINGS: LsbShiftTree<[CategoryPollRanking]> = LsbShiftTree::new();

pub static mut NEXT_MONTHS_CATEGORY_POLLS: LsbShiftTree<PrependList> = LsbShiftTree::new();
pub static mut NEXT_WEEKS_CATEGORY_POLLS: LsbShiftTree<PrependList> = LsbShiftTree::new();
pub static mut TOMORROWS_CATEGORY_POLLS: LsbShiftTree<PrependList> = LsbShiftTree::new();
pub static mut DAY_AFTER_TOMORROWS_CATEGORY_POLLS: LsbShiftTree<PrependList> = LsbShiftTree::new();


pub static mut LAST_MONTH_CATEGORY_POLLS: LsbShiftTree<[u64]> = LsbShiftTree::new();

pub static mut LOCATION_TIMEZONE_MAP: LsbShiftTree<usize> = LsbShiftTree::new();
pub static mut LOCATIONS_BY_TIMEZONE: Vec<u32> = Vec::new();
pub static mut TIMEZONE_MODIFICATION_FLAGS: Vec<boolean> = Vec::new();

/**
Split by timezone:
*/

pub struct LocationPollRankings<'a> {
    location: &'a [LocationCategoryPollRanking],
    categoryLocations: &'a LsbShiftTree<[LocationCategoryPollRanking]>,
}


pub struct LocationPollPrependLists<'a> {
    location: &'a PrependList<'a>,
    categoryLocations: &'a LsbShiftTree<PrependList<'a>>,
}


pub struct LsbShiftTree<T> {}

impl<T> LsbShiftTree<T> {
    pub fn new() -> LsbShiftTree<T> {
        PollTree {}
    }
}


pub struct CategoryPoll {
    locationId: u32,
    pollId: u32,
}

pub struct CategoryPollRanking {
    pollId: u64,
    voteCount: u32,
}

pub struct LocationCategoryPollRanking {
    locationPeriodPollId: u32,
    voteCount: u32,
}

/*
 * At least upper 3 bytes in sums will be free, we can use this space for
 * additional threshold counts and flags.  Also the total sum of free
 * bytes will be at least 6 to 18.  This could be used to store additional
 * information about the poll.
 *
 * For example, the positional configuration of a 3D poll can be encoded
 * into a number of configurations.  Lets assume that it would take 2 bytes
 * (64K configurations).  In the 
 */

/*
 *
 */
pub struct ThreeDPoll {
    pollId: u64,
    voteCount: u64,
    dim1dir1Sum: u64,
    dim1dir2Sum: u64,
    dim2dir1Sum: u64,
    dim2dir2Sum: u64,
    dim3dir1Sum: u64,
    dim3dir2Sum: u64,
}

/*
 *
 */
pub struct TwoDPoll {
    pollId: u64,
    voteCount: u64,
    dim1dir1Sum: u64,
    dim1dir2Sum: u64,
    dim2dir1Sum: u64,
    dim2dir2Sum: u64,
}

pub struct OneDPoll {
    pollId: u64,
    voteCount: u64,
    dim1dir1Sum: u64,
    dim1dir2Sum: u64,
}

pub trait OneToThreeDPoll {
}

impl OneToThreeDPoll for ThreeDPoll {
}

impl OneToThreeDPoll for TwoDPoll {
}

impl OneToThreeDPoll for OneDPoll {
}