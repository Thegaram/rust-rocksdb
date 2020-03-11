/// This file is generated from generate.py.
/// Re-generate it if you upgrade to a new version of RocksDB.

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum DBStatisticsTickerType {
    BlockCacheMiss = 0,
    BlockCacheHit = 1,
    BlockCacheAdd = 2,
    BlockCacheAddFailures = 3,
    BlockCacheIndexMiss = 4,
    BlockCacheIndexHit = 5,
    BlockCacheIndexAdd = 6,
    BlockCacheIndexBytesInsert = 7,
    BlockCacheIndexBytesEvict = 8,
    BlockCacheFilterMiss = 9,
    BlockCacheFilterHit = 10,
    BlockCacheFilterAdd = 11,
    BlockCacheFilterBytesInsert = 12,
    BlockCacheFilterBytesEvict = 13,
    BlockCacheDataMiss = 14,
    BlockCacheDataHit = 15,
    BlockCacheDataAdd = 16,
    BlockCacheDataBytesInsert = 17,
    BlockCacheBytesRead = 18,
    BlockCacheBytesWrite = 19,
    BloomFilterUseful = 20,
    BloomFilterFullPositive = 21,
    BloomFilterFullTruePositive = 22,
    BloomFilterMicros = 23,
    PersistentCacheHit = 24,
    PersistentCacheMiss = 25,
    SimBlockCacheHit = 26,
    SimBlockCacheMiss = 27,
    MemtableHit = 28,
    MemtableMiss = 29,
    GetHitL0 = 30,
    GetHitL1 = 31,
    GetHitL2AndUp = 32,
    CompactionKeyDropNewerEntry = 33,
    CompactionKeyDropObsolete = 34,
    CompactionKeyDropRangeDel = 35,
    CompactionKeyDropUser = 36,
    CompactionRangeDelDropObsolete = 37,
    CompactionOptimizedDelDropObsolete = 38,
    CompactionCancelled = 39,
    NumberKeysWritten = 40,
    NumberKeysRead = 41,
    NumberKeysUpdated = 42,
    BytesWritten = 43,
    BytesRead = 44,
    NumberDbSeek = 45,
    NumberDbNext = 46,
    NumberDbPrev = 47,
    NumberDbSeekFound = 48,
    NumberDbNextFound = 49,
    NumberDbPrevFound = 50,
    IterBytesRead = 51,
    NoFileCloses = 52,
    NoFileOpens = 53,
    NoFileErrors = 54,
    StallL0SlowdownMicros = 55,
    StallMemtableCompactionMicros = 56,
    StallL0NumFilesMicros = 57,
    StallMicros = 58,
    DbMutexWaitMicros = 59,
    RateLimitDelayMillis = 60,
    NoIterators = 61,
    NumberMultigetCalls = 62,
    NumberMultigetKeysRead = 63,
    NumberMultigetBytesRead = 64,
    NumberFilteredDeletes = 65,
    NumberMergeFailures = 66,
    BloomFilterPrefixChecked = 67,
    BloomFilterPrefixUseful = 68,
    NumberOfReseeksInIteration = 69,
    GetUpdatesSinceCalls = 70,
    BlockCacheCompressedMiss = 71,
    BlockCacheCompressedHit = 72,
    BlockCacheCompressedAdd = 73,
    BlockCacheCompressedAddFailures = 74,
    WalFileSynced = 75,
    WalFileBytes = 76,
    WriteDoneBySelf = 77,
    WriteDoneByOther = 78,
    WriteTimedout = 79,
    WriteWithWal = 80,
    CompactReadBytes = 81,
    CompactWriteBytes = 82,
    FlushWriteBytes = 83,
    NumberDirectLoadTableProperties = 84,
    NumberSuperversionAcquires = 85,
    NumberSuperversionReleases = 86,
    NumberSuperversionCleanups = 87,
    NumberBlockCompressed = 88,
    NumberBlockDecompressed = 89,
    NumberBlockNotCompressed = 90,
    MergeOperationTotalTime = 91,
    FilterOperationTotalTime = 92,
    RowCacheHit = 93,
    RowCacheMiss = 94,
    ReadAmpEstimateUsefulBytes = 95,
    ReadAmpTotalReadBytes = 96,
    NumberRateLimiterDrains = 97,
    NumberIterSkip = 98,
    BlobDbNumPut = 99,
    BlobDbNumWrite = 100,
    BlobDbNumGet = 101,
    BlobDbNumMultiget = 102,
    BlobDbNumSeek = 103,
    BlobDbNumNext = 104,
    BlobDbNumPrev = 105,
    BlobDbNumKeysWritten = 106,
    BlobDbNumKeysRead = 107,
    BlobDbBytesWritten = 108,
    BlobDbBytesRead = 109,
    BlobDbWriteInlined = 110,
    BlobDbWriteInlinedTtl = 111,
    BlobDbWriteBlob = 112,
    BlobDbWriteBlobTtl = 113,
    BlobDbBlobFileBytesWritten = 114,
    BlobDbBlobFileBytesRead = 115,
    BlobDbBlobFileSynced = 116,
    BlobDbBlobIndexExpiredCount = 117,
    BlobDbBlobIndexExpiredSize = 118,
    BlobDbBlobIndexEvictedCount = 119,
    BlobDbBlobIndexEvictedSize = 120,
    BlobDbGcNumFiles = 121,
    BlobDbGcNumNewFiles = 122,
    BlobDbGcFailures = 123,
    BlobDbGcNumKeysOverwritten = 124,
    BlobDbGcNumKeysExpired = 125,
    BlobDbGcNumKeysRelocated = 126,
    BlobDbGcBytesOverwritten = 127,
    BlobDbGcBytesExpired = 128,
    BlobDbGcBytesRelocated = 129,
    BlobDbFifoNumFilesEvicted = 130,
    BlobDbFifoNumKeysEvicted = 131,
    BlobDbFifoBytesEvicted = 132,
    TxnPrepareMutexOverhead = 133,
    TxnOldCommitMapMutexOverhead = 134,
    TxnDuplicateKeyOverhead = 135,
    TxnSnapshotMutexOverhead = 136,
    NumberMultigetKeysFound = 137,
    NoIteratorCreated = 138,
    NoIteratorDeleted = 139,
    BlockCacheCompressionDictMiss = 140,
    BlockCacheCompressionDictHit = 141,
    BlockCacheCompressionDictAdd = 142,
    BlockCacheCompressionDictBytesInsert = 143,
    BlockCacheCompressionDictBytesEvict = 144,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum DBStatisticsHistogramType {
    DbGet = 0,
    DbWrite = 1,
    CompactionTime = 2,
    CompactionCpuTime = 3,
    SubcompactionSetupTime = 4,
    TableSyncMicros = 5,
    CompactionOutfileSyncMicros = 6,
    WalFileSyncMicros = 7,
    ManifestFileSyncMicros = 8,
    TableOpenIoMicros = 9,
    DbMultiget = 10,
    ReadBlockCompactionMicros = 11,
    ReadBlockGetMicros = 12,
    WriteRawBlockMicros = 13,
    StallL0SlowdownCount = 14,
    StallMemtableCompactionCount = 15,
    StallL0NumFilesCount = 16,
    HardRateLimitDelayCount = 17,
    SoftRateLimitDelayCount = 18,
    NumFilesInSingleCompaction = 19,
    DbSeek = 20,
    WriteStall = 21,
    SstReadMicros = 22,
    NumSubcompactionsScheduled = 23,
    BytesPerRead = 24,
    BytesPerWrite = 25,
    BytesPerMultiget = 26,
    BytesCompressed = 27,
    BytesDecompressed = 28,
    CompressionTimesNanos = 29,
    DecompressionTimesNanos = 30,
    ReadNumMergeOperands = 31,
    BlobDbKeySize = 32,
    BlobDbValueSize = 33,
    BlobDbWriteMicros = 34,
    BlobDbGetMicros = 35,
    BlobDbMultigetMicros = 36,
    BlobDbSeekMicros = 37,
    BlobDbNextMicros = 38,
    BlobDbPrevMicros = 39,
    BlobDbBlobFileWriteMicros = 40,
    BlobDbBlobFileReadMicros = 41,
    BlobDbBlobFileSyncMicros = 42,
    BlobDbGcMicros = 43,
    BlobDbCompressionMicros = 44,
    BlobDbDecompressionMicros = 45,
    FlushTime = 46,
    SstBatchSize = 47,
    DbWriteWalTime = 48,
    HistogramEnumMax = 49,
}
