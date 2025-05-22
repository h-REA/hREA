import { signalObject } from 'signal-utils/object';

const allowCaching = true;

// reactive entry lookup for hashes
export const entryLookup = signalObject<Record<string, any>>({});

// function to return entry if it exists in the store
export function getEntryFromStore(hash: string) {
    return entryLookup[hash] ?? null;
}

// function to add entry to the store
export function addEntryToStore(hash: string, entry: any) {
    if (!allowCaching) {
        return;
    }
    entryLookup[hash] = entry;
}

// reactive map for recently fetched revision IDs
export const recentlyFetchedRevisionIds = signalObject<Record<string, { revisionId: string, timestamp: number }>>({});

// function to get original id from the store
export function getLatestRevisionId(hash: string, passedSeconds: number) {
    const item = recentlyFetchedRevisionIds[hash];
    if (item && item.timestamp + passedSeconds * 1000 > Date.now()) {
        return item.revisionId;
    } else {
        return null;
    }
}

// get time of last update
export function getLastUpdateTime(hash: string) {
    const item = recentlyFetchedRevisionIds[hash];
    if (item) {
        return item.timestamp;
    } else {
        return null;
    }
}

// function to add/update original id
export function updateLatestRevision(originalId: string, revisionId: string) {
    if (!allowCaching) {
        return;
    }
    recentlyFetchedRevisionIds[originalId] = {
        revisionId,
        timestamp: Date.now()
    };
}
