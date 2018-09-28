use head::AppendHeadList;

static DATA_LIST_LENGTH: u32 = 2; // 128
static BRANCH_LIST_LENGTH: u32 = 2; // 8
static MAX_SECOND_LEVEL_BLOCKS: u32 = DATA_LIST_LENGTH * DATA_LIST_LENGTH;
static BRANCH_LIST_LENGTH_NUM_BITS: u32 = 1; // 3

static IN_BLOCK_DIVIDER: char = ',';
static BLOCK_NOT_FOUND: char = '?';
static STRUCTURE_CHANGING: char = '~';
static ADDITIONAL_BLOCKS_AVAILABLE: char = '+';

static DEPTH_BASED_MAX_CAPACITY: [u64; 13] = [0, BRANCH_LIST_LENGTH,
BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH,
BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH,
BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH,
BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH,
BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH,
BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH,
BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH,
BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH,
BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH,
BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH,
BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH
* BRANCH_LIST_LENGTH * BRANCH_LIST_LENGTH,
];

pub struct PrependList<'a> {
    head: &'a AppendHeadList,

    // tree state
    tree: &'a Tree,
    topBranchDivider: u32,
    lastBlockIndex: u32,

    // cache state
    topCache: [u8],
    cacheCleared: bool,
    clearCacheTick: u32,

    // mutation state
    structureChanging: bool
}

impl PrependList {

    fn new() -> PrependList {
        PrependList {
            head: AppendHeadList::new(),
            tree: head,
            topBranchDivider: 1,
            lastBlockIndex: -1,
            topCache: [],
            cacheCleared: false,
            clearCacheTick: 0,
            structureChanging: false
        }
    }



    /**
     * NOTE: only one thread should ever prepend to a given prepend list.
     *
     * @param timePeriodLocationPollIds
     */
    pub fn prepend(
    timePeriodLocationPollIds
    ) {
    cacheCleared = true;
    clearCacheTick = checkCacheTick;

    PrependBlockNode firstPrependNode = null;
    PrependBlockNode currentPrependNode = null;

    int startingIncomingPollPosition;
    if (timePeriodLocationPollIds.length + headList.length >= DATA_LIST_LENGTH) {

    // Copy Poll Ids from input and headList into block lists
    var newLeafList = new int[DATA_LIST_LENGTH];
    firstPrependNode = new PrependBlockNode();
    firstPrependNode.data = newLeafList;
    currentPrependNode = firstPrependNode;

    var pollIdsToCopy = DATA_LIST_LENGTH - headList.length;
    startingIncomingPollPosition = timePeriodLocationPollIds.length - pollIdsToCopy;
    // Copy the tail end of the incoming Ids into the new LeafList
    System.arraycopy(
    timePeriodLocationPollIds, startingIncomingPollPosition,
    newLeafList, 0, pollIdsToCopy);

    // Copy HeadList entries
    var headListNode = headList.last;
    while (headListNode != null) {
    newLeafList[pollIdsToCopy++] = headListNode.timePeriodLocationId;
    headListNode = headListNode.previous;
    }
    headList.last = null;

    while (startingIncomingPollPosition > DATA_LIST_LENGTH) {
    // copy remaining full pages from incoming Ids
    startingIncomingPollPosition = startingIncomingPollPosition - DATA_LIST_LENGTH;
    newLeafList = new int[DATA_LIST_LENGTH];
    var lastPrependNode = currentPrependNode;
    currentPrependNode = new PrependBlockNode();
    currentPrependNode.data = newLeafList;
    lastPrependNode.next = currentPrependNode;
    System.arraycopy(
    timePeriodLocationPollIds, startingIncomingPollPosition,
    newLeafList, 0, DATA_LIST_LENGTH);
    }

    // last copied block is the most recent one
    mostRecentFullBlock = newLeafList;
    } else {
    startingIncomingPollPosition = timePeriodLocationPollIds.length;
    }

    if (startingIncomingPollPosition != 0) {
    // Copy remaining incoming Ids into the head list
    var last = headList.last;
    if (last == null) {
    // Setup a new last node
    startingIncomingPollPosition--;
    last = new HeadListNode();
    last.timePeriodLocationId = timePeriodLocationPollIds[startingIncomingPollPosition];
    headList.last = last;
    }

    HeadListNode newLast;
    while (startingIncomingPollPosition > 0) {
    // copy remaining incoming Ids
    startingIncomingPollPosition--;
    newLast = new HeadListNode();
    newLast.timePeriodLocationId = timePeriodLocationPollIds[startingIncomingPollPosition];
    newLast.previous = last;
    headList.last = newLast;
    }
    }

    if (currentPrependNode == null) {
    return;
    }
    mostRecentFullBlock = currentPrependNode.data;

    currentPrependNode = firstPrependNode;

    Object[] currentSubTree = null;
    int nextSubTreeIndex = BRANCH_LIST_LENGTH;

    while (currentPrependNode != null) {
    switch (lastBlockIndex) {
    case -1:
    // First time around don't create the earlierPollTree
    // no need for it at this time
    lastBlockIndex = 0;
    break;
    case 0:
    // Second time around create the root earlierPollTree
    // and add the already present first block and the new
    // second block
    earlierPollTree = new Object[BRANCH_LIST_LENGTH];
    earlierPollTree[0] = mostRecentFullBlock;
    earlierPollTree[1] = currentPrependNode.data;
    lastBlockIndex = 1;
    break;
//				case 1:
//				case 2:
//				case 3:
//				case 4:
//				case 5:
//				case 6:
//					// For all all blocks that would fit in only the top
//					// earlier poll tree blocks (expected to be a frequent scenario
//					// for most locations/location+categories)
//					// just add them to the root earlierPoolTree
//					lastBlockIndex++;
//					earlierPollTree[lastBlockIndex] = currentPrependNode.data;
//					break;

    default:
    // If there are more spaces in the current block, add
    // the list to the current block (useful if more than one
    // block is being submitted - a likely scenario under high
    // load for a popular location/location+category)
//					if (nextSubTreeIndex > 0 && nextSubTreeIndex != BRANCH_LIST_LENGTH) {
//						currentSubTree[nextSubTreeIndex] = currentPrependNode.data;
//						lastBlockIndex++;
//						nextSubTreeIndex++;
//						break;
//					}
    var blockIndex = lastBlockIndex + 1;

    // If the current tree has reached reached it's capacity: grow it
    if(DEPTH_BASED_MAX_CAPACITY[depth] == blockIndex) {
    delayStructuralModsUntilReadsComplete();
    var newFirstTopLevelBranchBlock = earlierPollTree;
    earlierPollTree = new Object[BRANCH_LIST_LENGTH];
    earlierPollTree[0] = newFirstTopLevelBranchBlock;
    currentSubTree = new Object[BRANCH_LIST_LENGTH];
    earlierPollTree[1] = currentSubTree;
    currentSubTree[0] = currentPrependNode.data;
    depth++;
    lastBlockIndex = blockIndex;
    structureChanging = false;
    break;
    }


    // If's its a depth2 tree use simpler cache to add values to it
    if (lastBlockIndex < MAX_SECOND_LEVEL_BLOCKS) {
    var nextBlockIndex = lastBlockIndex + 1;
    var secondStepId = nextBlockIndex % BRANCH_LIST_LENGTH;
    var branchNumber = nextBlockIndex >> BRANCH_LIST_LENGTH_NUM_BITS;
    if (secondStepId == 0) {
    currentSubTree = new Object[BRANCH_LIST_LENGTH];
    earlierPollTree[branchNumber] = currentSubTree;
    } else {
    currentSubTree = (Object[]) earlierPollTree[branchNumber];
    }
    currentSubTree[secondStepId] = currentPrependNode.data;
    lastBlockIndex = nextBlockIndex;
//						nextSubTreeIndex = ++nestStepId;
    break;
    }


    // FIXME: work here next
//					ADD_BLOCK_LOOP:
//					while (true) {
//						var previousBlockIndex = lastBlockIndex;

    var branchDivider = topBranchDivider;
    int nextStepId = blockIndex % branchDivider;
//						var nextStepLastBlockId = lastBlockIndex % branchDivider;
    int branchNumber = blockIndex / branchDivider;
    int currentDepth = 2;

    var parentBranch = earlierPollTree;
    var branch = (Object[]) earlierPollTree[branchNumber];
    while (true) {
    if (branch == null) {
    parentBranch[branchNumber] = currentPrependNode.data;
    lastBlockIndex = blockIndex;
    break;
    }

    branchDivider = branchDivider >> BRANCH_LIST_LENGTH_NUM_BITS;

    // First block is in nested tree
    var followingStepId = nextStepId % branchDivider;
//								var followingStepLastBlockId = nextStepId % branchDivider;

    // This is the terminating branch
    if (followingStepId < BRANCH_LIST_LENGTH) {
    branch[nextStepId / branchDivider] = currentPrependNode.data;
    break;
    } else {
    var isShorterBranch = false;
    // Keep on shortening the branch tree until the correct level
    while (followingStepId > branchDivider) {
    isShorterBranch = true;
    branchDivider = branchDivider >> BRANCH_LIST_LENGTH_NUM_BITS;
    currentDepth--;
    }
    if (isShorterBranch) {
    nextStepId = followingStepId;
    followingStepId = followingStepId % branchDivider;
//										followingStepLastBlockId = followingStepLastBlockId % branchDivider;
    }
    }
    branchNumber = nextStepId / branchDivider;
    nextStepId = followingStepId;
    //								nextStepLastBlockId = followingStepLastBlockId;
    parentBranch = branch;
    branch = (Object[]) branch[branchNumber];
    }
    //						}

    break;
    }
    currentPrependNode = currentPrependNode.next;
    }

    }
}

trait Tree {

}

trait Node {

}