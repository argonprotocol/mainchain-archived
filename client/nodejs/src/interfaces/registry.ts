// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/types/registry';

import type { FinalityGrandpaEquivocationPrecommit, FinalityGrandpaEquivocationPrevote, FinalityGrandpaPrecommit, FinalityGrandpaPrevote, FrameMetadataHashExtensionCheckMetadataHash, FrameMetadataHashExtensionMode, FrameSupportDispatchDispatchClass, FrameSupportDispatchDispatchInfo, FrameSupportDispatchPays, FrameSupportDispatchPerDispatchClassU32, FrameSupportDispatchPerDispatchClassWeight, FrameSupportDispatchPerDispatchClassWeightsPerClass, FrameSupportPalletId, FrameSupportTokensMiscBalanceStatus, FrameSupportTokensMiscIdAmountRuntimeFreezeReason, FrameSupportTokensMiscIdAmountRuntimeHoldReason, FrameSystemAccountInfo, FrameSystemCall, FrameSystemCodeUpgradeAuthorization, FrameSystemError, FrameSystemEvent, FrameSystemEventRecord, FrameSystemExtensionsCheckGenesis, FrameSystemExtensionsCheckNonZeroSender, FrameSystemExtensionsCheckNonce, FrameSystemExtensionsCheckSpecVersion, FrameSystemExtensionsCheckTxVersion, FrameSystemExtensionsCheckWeight, FrameSystemLastRuntimeUpgradeInfo, FrameSystemLimitsBlockLength, FrameSystemLimitsBlockWeights, FrameSystemLimitsWeightsPerClass, FrameSystemPhase, PalletBalancesAccountData, PalletBalancesAdjustmentDirection, PalletBalancesBalanceLock, PalletBalancesCall, PalletBalancesError, PalletBalancesEvent, PalletBalancesReasons, PalletBalancesReserveData, PalletBitcoinUtxosCall, PalletBitcoinUtxosError, PalletBitcoinUtxosEvent, PalletBlockRewardsCall, PalletBlockRewardsError, PalletBlockRewardsEvent, PalletBlockRewardsFreezeReason, PalletBlockRewardsHoldReason, PalletBlockSealCall, PalletBlockSealError, PalletBlockSealSpecCall, PalletBlockSealSpecError, PalletBlockSealSpecEvent, PalletBondCall, PalletBondError, PalletBondEvent, PalletBondHoldReason, PalletBondUtxoCosignRequest, PalletBondUtxoState, PalletChainTransferCall, PalletChainTransferError, PalletChainTransferEvent, PalletChainTransferQueuedTransferOut, PalletDataDomainCall, PalletDataDomainDataDomainRegistration, PalletDataDomainError, PalletDataDomainEvent, PalletGrandpaCall, PalletGrandpaError, PalletGrandpaEvent, PalletGrandpaStoredPendingChange, PalletGrandpaStoredState, PalletMiningSlotCall, PalletMiningSlotError, PalletMiningSlotEvent, PalletMiningSlotHoldReason, PalletMiningSlotMinerHistory, PalletMiningSlotMiningSlotBid, PalletMintCall, PalletMintError, PalletMintEvent, PalletMintMintType, PalletMultisigCall, PalletMultisigError, PalletMultisigEvent, PalletMultisigMultisig, PalletMultisigTimepoint, PalletNotariesCall, PalletNotariesError, PalletNotariesEvent, PalletNotebookCall, PalletNotebookError, PalletNotebookEvent, PalletOffencesEvent, PalletPriceIndexCall, PalletPriceIndexError, PalletPriceIndexEvent, PalletPriceIndexPriceIndex, PalletProxyAnnouncement, PalletProxyCall, PalletProxyError, PalletProxyEvent, PalletProxyProxyDefinition, PalletSessionCall, PalletSessionError, PalletSessionEvent, PalletSudoCall, PalletSudoError, PalletSudoEvent, PalletTicksCall, PalletTicksError, PalletTimestampCall, PalletTransactionPaymentChargeTransactionPayment, PalletTransactionPaymentEvent, PalletTransactionPaymentReleases, PalletTxPauseCall, PalletTxPauseError, PalletTxPauseEvent, PalletVaultsCall, PalletVaultsError, PalletVaultsEvent, PalletVaultsHoldReason, PalletVaultsVaultConfig, SpArithmeticArithmeticError, SpConsensusGrandpaAppPublic, SpConsensusGrandpaAppSignature, SpConsensusGrandpaEquivocation, SpConsensusGrandpaEquivocationProof, SpCoreCryptoKeyTypeId, SpRuntimeDigest, SpRuntimeDigestDigestItem, SpRuntimeDispatchError, SpRuntimeModuleError, SpRuntimeMultiSignature, SpRuntimeTokenError, SpRuntimeTransactionalError, SpSessionMembershipProof, SpStakingOffenceOffenceDetails, SpVersionRuntimeVersion, SpWeightsRuntimeDbWeight, SpWeightsWeightV2Weight, UlxNodeRuntimeOpaqueSessionKeys, UlxNodeRuntimeProxyType, UlxNodeRuntimeRuntime, UlxNodeRuntimeRuntimeFreezeReason, UlxNodeRuntimeRuntimeHoldReason, UlxNotaryAuditAccountHistoryLookupError, UlxNotaryAuditErrorVerifyError, UlxPrimitivesAccountAccountType, UlxPrimitivesBalanceChangeAccountOrigin, UlxPrimitivesBalanceChangeMerkleProof, UlxPrimitivesBitcoinBitcoinBlock, UlxPrimitivesBitcoinBitcoinCosignScriptPubkey, UlxPrimitivesBitcoinBitcoinNetwork, UlxPrimitivesBitcoinBitcoinRejectedReason, UlxPrimitivesBitcoinBitcoinXPub, UlxPrimitivesBitcoinCompressedBitcoinPubkey, UlxPrimitivesBitcoinH256Le, UlxPrimitivesBitcoinNetworkKind, UlxPrimitivesBitcoinOpaqueBitcoinXpub, UlxPrimitivesBitcoinUtxoRef, UlxPrimitivesBitcoinUtxoValue, UlxPrimitivesBlockSealAppPublic, UlxPrimitivesBlockSealAppSignature, UlxPrimitivesBlockSealBlockPayout, UlxPrimitivesBlockSealMiningRegistration, UlxPrimitivesBlockSealRewardDestination, UlxPrimitivesBlockSealRewardSharing, UlxPrimitivesBlockVoteBlockVoteT, UlxPrimitivesBond, UlxPrimitivesBondBondError, UlxPrimitivesBondBondExpiration, UlxPrimitivesBondBondType, UlxPrimitivesBondVault, UlxPrimitivesBondVaultArgons, UlxPrimitivesBondVaultTerms, UlxPrimitivesDataDomainSemver, UlxPrimitivesDataDomainVersionHost, UlxPrimitivesDataDomainZoneRecord, UlxPrimitivesDigestsBlockVoteDigest, UlxPrimitivesDigestsNotebookDigest, UlxPrimitivesDigestsNotebookDigestRecord, UlxPrimitivesDigestsParentVotingKeyDigest, UlxPrimitivesInherentsBitcoinUtxoSync, UlxPrimitivesInherentsBlockSealInherent, UlxPrimitivesNotaryNotaryMeta, UlxPrimitivesNotaryNotaryNotebookKeyDetails, UlxPrimitivesNotaryNotaryNotebookVoteDigestDetails, UlxPrimitivesNotaryNotaryRecord, UlxPrimitivesNotebookChainTransfer, UlxPrimitivesNotebookNotebookHeader, UlxPrimitivesNotebookSignedNotebookHeader, UlxPrimitivesProvidersBlockSealerInfo } from '@polkadot/types/lookup';

declare module '@polkadot/types/types/registry' {
  interface InterfaceTypes {
    FinalityGrandpaEquivocationPrecommit: FinalityGrandpaEquivocationPrecommit;
    FinalityGrandpaEquivocationPrevote: FinalityGrandpaEquivocationPrevote;
    FinalityGrandpaPrecommit: FinalityGrandpaPrecommit;
    FinalityGrandpaPrevote: FinalityGrandpaPrevote;
    FrameMetadataHashExtensionCheckMetadataHash: FrameMetadataHashExtensionCheckMetadataHash;
    FrameMetadataHashExtensionMode: FrameMetadataHashExtensionMode;
    FrameSupportDispatchDispatchClass: FrameSupportDispatchDispatchClass;
    FrameSupportDispatchDispatchInfo: FrameSupportDispatchDispatchInfo;
    FrameSupportDispatchPays: FrameSupportDispatchPays;
    FrameSupportDispatchPerDispatchClassU32: FrameSupportDispatchPerDispatchClassU32;
    FrameSupportDispatchPerDispatchClassWeight: FrameSupportDispatchPerDispatchClassWeight;
    FrameSupportDispatchPerDispatchClassWeightsPerClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
    FrameSupportPalletId: FrameSupportPalletId;
    FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
    FrameSupportTokensMiscIdAmountRuntimeFreezeReason: FrameSupportTokensMiscIdAmountRuntimeFreezeReason;
    FrameSupportTokensMiscIdAmountRuntimeHoldReason: FrameSupportTokensMiscIdAmountRuntimeHoldReason;
    FrameSystemAccountInfo: FrameSystemAccountInfo;
    FrameSystemCall: FrameSystemCall;
    FrameSystemCodeUpgradeAuthorization: FrameSystemCodeUpgradeAuthorization;
    FrameSystemError: FrameSystemError;
    FrameSystemEvent: FrameSystemEvent;
    FrameSystemEventRecord: FrameSystemEventRecord;
    FrameSystemExtensionsCheckGenesis: FrameSystemExtensionsCheckGenesis;
    FrameSystemExtensionsCheckNonZeroSender: FrameSystemExtensionsCheckNonZeroSender;
    FrameSystemExtensionsCheckNonce: FrameSystemExtensionsCheckNonce;
    FrameSystemExtensionsCheckSpecVersion: FrameSystemExtensionsCheckSpecVersion;
    FrameSystemExtensionsCheckTxVersion: FrameSystemExtensionsCheckTxVersion;
    FrameSystemExtensionsCheckWeight: FrameSystemExtensionsCheckWeight;
    FrameSystemLastRuntimeUpgradeInfo: FrameSystemLastRuntimeUpgradeInfo;
    FrameSystemLimitsBlockLength: FrameSystemLimitsBlockLength;
    FrameSystemLimitsBlockWeights: FrameSystemLimitsBlockWeights;
    FrameSystemLimitsWeightsPerClass: FrameSystemLimitsWeightsPerClass;
    FrameSystemPhase: FrameSystemPhase;
    PalletBalancesAccountData: PalletBalancesAccountData;
    PalletBalancesAdjustmentDirection: PalletBalancesAdjustmentDirection;
    PalletBalancesBalanceLock: PalletBalancesBalanceLock;
    PalletBalancesCall: PalletBalancesCall;
    PalletBalancesError: PalletBalancesError;
    PalletBalancesEvent: PalletBalancesEvent;
    PalletBalancesReasons: PalletBalancesReasons;
    PalletBalancesReserveData: PalletBalancesReserveData;
    PalletBitcoinUtxosCall: PalletBitcoinUtxosCall;
    PalletBitcoinUtxosError: PalletBitcoinUtxosError;
    PalletBitcoinUtxosEvent: PalletBitcoinUtxosEvent;
    PalletBlockRewardsCall: PalletBlockRewardsCall;
    PalletBlockRewardsError: PalletBlockRewardsError;
    PalletBlockRewardsEvent: PalletBlockRewardsEvent;
    PalletBlockRewardsFreezeReason: PalletBlockRewardsFreezeReason;
    PalletBlockRewardsHoldReason: PalletBlockRewardsHoldReason;
    PalletBlockSealCall: PalletBlockSealCall;
    PalletBlockSealError: PalletBlockSealError;
    PalletBlockSealSpecCall: PalletBlockSealSpecCall;
    PalletBlockSealSpecError: PalletBlockSealSpecError;
    PalletBlockSealSpecEvent: PalletBlockSealSpecEvent;
    PalletBondCall: PalletBondCall;
    PalletBondError: PalletBondError;
    PalletBondEvent: PalletBondEvent;
    PalletBondHoldReason: PalletBondHoldReason;
    PalletBondUtxoCosignRequest: PalletBondUtxoCosignRequest;
    PalletBondUtxoState: PalletBondUtxoState;
    PalletChainTransferCall: PalletChainTransferCall;
    PalletChainTransferError: PalletChainTransferError;
    PalletChainTransferEvent: PalletChainTransferEvent;
    PalletChainTransferQueuedTransferOut: PalletChainTransferQueuedTransferOut;
    PalletDataDomainCall: PalletDataDomainCall;
    PalletDataDomainDataDomainRegistration: PalletDataDomainDataDomainRegistration;
    PalletDataDomainError: PalletDataDomainError;
    PalletDataDomainEvent: PalletDataDomainEvent;
    PalletGrandpaCall: PalletGrandpaCall;
    PalletGrandpaError: PalletGrandpaError;
    PalletGrandpaEvent: PalletGrandpaEvent;
    PalletGrandpaStoredPendingChange: PalletGrandpaStoredPendingChange;
    PalletGrandpaStoredState: PalletGrandpaStoredState;
    PalletMiningSlotCall: PalletMiningSlotCall;
    PalletMiningSlotError: PalletMiningSlotError;
    PalletMiningSlotEvent: PalletMiningSlotEvent;
    PalletMiningSlotHoldReason: PalletMiningSlotHoldReason;
    PalletMiningSlotMinerHistory: PalletMiningSlotMinerHistory;
    PalletMiningSlotMiningSlotBid: PalletMiningSlotMiningSlotBid;
    PalletMintCall: PalletMintCall;
    PalletMintError: PalletMintError;
    PalletMintEvent: PalletMintEvent;
    PalletMintMintType: PalletMintMintType;
    PalletMultisigCall: PalletMultisigCall;
    PalletMultisigError: PalletMultisigError;
    PalletMultisigEvent: PalletMultisigEvent;
    PalletMultisigMultisig: PalletMultisigMultisig;
    PalletMultisigTimepoint: PalletMultisigTimepoint;
    PalletNotariesCall: PalletNotariesCall;
    PalletNotariesError: PalletNotariesError;
    PalletNotariesEvent: PalletNotariesEvent;
    PalletNotebookCall: PalletNotebookCall;
    PalletNotebookError: PalletNotebookError;
    PalletNotebookEvent: PalletNotebookEvent;
    PalletOffencesEvent: PalletOffencesEvent;
    PalletPriceIndexCall: PalletPriceIndexCall;
    PalletPriceIndexError: PalletPriceIndexError;
    PalletPriceIndexEvent: PalletPriceIndexEvent;
    PalletPriceIndexPriceIndex: PalletPriceIndexPriceIndex;
    PalletProxyAnnouncement: PalletProxyAnnouncement;
    PalletProxyCall: PalletProxyCall;
    PalletProxyError: PalletProxyError;
    PalletProxyEvent: PalletProxyEvent;
    PalletProxyProxyDefinition: PalletProxyProxyDefinition;
    PalletSessionCall: PalletSessionCall;
    PalletSessionError: PalletSessionError;
    PalletSessionEvent: PalletSessionEvent;
    PalletSudoCall: PalletSudoCall;
    PalletSudoError: PalletSudoError;
    PalletSudoEvent: PalletSudoEvent;
    PalletTicksCall: PalletTicksCall;
    PalletTicksError: PalletTicksError;
    PalletTimestampCall: PalletTimestampCall;
    PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
    PalletTransactionPaymentEvent: PalletTransactionPaymentEvent;
    PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
    PalletTxPauseCall: PalletTxPauseCall;
    PalletTxPauseError: PalletTxPauseError;
    PalletTxPauseEvent: PalletTxPauseEvent;
    PalletVaultsCall: PalletVaultsCall;
    PalletVaultsError: PalletVaultsError;
    PalletVaultsEvent: PalletVaultsEvent;
    PalletVaultsHoldReason: PalletVaultsHoldReason;
    PalletVaultsVaultConfig: PalletVaultsVaultConfig;
    SpArithmeticArithmeticError: SpArithmeticArithmeticError;
    SpConsensusGrandpaAppPublic: SpConsensusGrandpaAppPublic;
    SpConsensusGrandpaAppSignature: SpConsensusGrandpaAppSignature;
    SpConsensusGrandpaEquivocation: SpConsensusGrandpaEquivocation;
    SpConsensusGrandpaEquivocationProof: SpConsensusGrandpaEquivocationProof;
    SpCoreCryptoKeyTypeId: SpCoreCryptoKeyTypeId;
    SpRuntimeDigest: SpRuntimeDigest;
    SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
    SpRuntimeDispatchError: SpRuntimeDispatchError;
    SpRuntimeModuleError: SpRuntimeModuleError;
    SpRuntimeMultiSignature: SpRuntimeMultiSignature;
    SpRuntimeTokenError: SpRuntimeTokenError;
    SpRuntimeTransactionalError: SpRuntimeTransactionalError;
    SpSessionMembershipProof: SpSessionMembershipProof;
    SpStakingOffenceOffenceDetails: SpStakingOffenceOffenceDetails;
    SpVersionRuntimeVersion: SpVersionRuntimeVersion;
    SpWeightsRuntimeDbWeight: SpWeightsRuntimeDbWeight;
    SpWeightsWeightV2Weight: SpWeightsWeightV2Weight;
    UlxNodeRuntimeOpaqueSessionKeys: UlxNodeRuntimeOpaqueSessionKeys;
    UlxNodeRuntimeProxyType: UlxNodeRuntimeProxyType;
    UlxNodeRuntimeRuntime: UlxNodeRuntimeRuntime;
    UlxNodeRuntimeRuntimeFreezeReason: UlxNodeRuntimeRuntimeFreezeReason;
    UlxNodeRuntimeRuntimeHoldReason: UlxNodeRuntimeRuntimeHoldReason;
    UlxNotaryAuditAccountHistoryLookupError: UlxNotaryAuditAccountHistoryLookupError;
    UlxNotaryAuditErrorVerifyError: UlxNotaryAuditErrorVerifyError;
    UlxPrimitivesAccountAccountType: UlxPrimitivesAccountAccountType;
    UlxPrimitivesBalanceChangeAccountOrigin: UlxPrimitivesBalanceChangeAccountOrigin;
    UlxPrimitivesBalanceChangeMerkleProof: UlxPrimitivesBalanceChangeMerkleProof;
    UlxPrimitivesBitcoinBitcoinBlock: UlxPrimitivesBitcoinBitcoinBlock;
    UlxPrimitivesBitcoinBitcoinCosignScriptPubkey: UlxPrimitivesBitcoinBitcoinCosignScriptPubkey;
    UlxPrimitivesBitcoinBitcoinNetwork: UlxPrimitivesBitcoinBitcoinNetwork;
    UlxPrimitivesBitcoinBitcoinRejectedReason: UlxPrimitivesBitcoinBitcoinRejectedReason;
    UlxPrimitivesBitcoinBitcoinXPub: UlxPrimitivesBitcoinBitcoinXPub;
    UlxPrimitivesBitcoinCompressedBitcoinPubkey: UlxPrimitivesBitcoinCompressedBitcoinPubkey;
    UlxPrimitivesBitcoinH256Le: UlxPrimitivesBitcoinH256Le;
    UlxPrimitivesBitcoinNetworkKind: UlxPrimitivesBitcoinNetworkKind;
    UlxPrimitivesBitcoinOpaqueBitcoinXpub: UlxPrimitivesBitcoinOpaqueBitcoinXpub;
    UlxPrimitivesBitcoinUtxoRef: UlxPrimitivesBitcoinUtxoRef;
    UlxPrimitivesBitcoinUtxoValue: UlxPrimitivesBitcoinUtxoValue;
    UlxPrimitivesBlockSealAppPublic: UlxPrimitivesBlockSealAppPublic;
    UlxPrimitivesBlockSealAppSignature: UlxPrimitivesBlockSealAppSignature;
    UlxPrimitivesBlockSealBlockPayout: UlxPrimitivesBlockSealBlockPayout;
    UlxPrimitivesBlockSealMiningRegistration: UlxPrimitivesBlockSealMiningRegistration;
    UlxPrimitivesBlockSealRewardDestination: UlxPrimitivesBlockSealRewardDestination;
    UlxPrimitivesBlockSealRewardSharing: UlxPrimitivesBlockSealRewardSharing;
    UlxPrimitivesBlockVoteBlockVoteT: UlxPrimitivesBlockVoteBlockVoteT;
    UlxPrimitivesBond: UlxPrimitivesBond;
    UlxPrimitivesBondBondError: UlxPrimitivesBondBondError;
    UlxPrimitivesBondBondExpiration: UlxPrimitivesBondBondExpiration;
    UlxPrimitivesBondBondType: UlxPrimitivesBondBondType;
    UlxPrimitivesBondVault: UlxPrimitivesBondVault;
    UlxPrimitivesBondVaultArgons: UlxPrimitivesBondVaultArgons;
    UlxPrimitivesBondVaultTerms: UlxPrimitivesBondVaultTerms;
    UlxPrimitivesDataDomainSemver: UlxPrimitivesDataDomainSemver;
    UlxPrimitivesDataDomainVersionHost: UlxPrimitivesDataDomainVersionHost;
    UlxPrimitivesDataDomainZoneRecord: UlxPrimitivesDataDomainZoneRecord;
    UlxPrimitivesDigestsBlockVoteDigest: UlxPrimitivesDigestsBlockVoteDigest;
    UlxPrimitivesDigestsNotebookDigest: UlxPrimitivesDigestsNotebookDigest;
    UlxPrimitivesDigestsNotebookDigestRecord: UlxPrimitivesDigestsNotebookDigestRecord;
    UlxPrimitivesDigestsParentVotingKeyDigest: UlxPrimitivesDigestsParentVotingKeyDigest;
    UlxPrimitivesInherentsBitcoinUtxoSync: UlxPrimitivesInherentsBitcoinUtxoSync;
    UlxPrimitivesInherentsBlockSealInherent: UlxPrimitivesInherentsBlockSealInherent;
    UlxPrimitivesNotaryNotaryMeta: UlxPrimitivesNotaryNotaryMeta;
    UlxPrimitivesNotaryNotaryNotebookKeyDetails: UlxPrimitivesNotaryNotaryNotebookKeyDetails;
    UlxPrimitivesNotaryNotaryNotebookVoteDigestDetails: UlxPrimitivesNotaryNotaryNotebookVoteDigestDetails;
    UlxPrimitivesNotaryNotaryRecord: UlxPrimitivesNotaryNotaryRecord;
    UlxPrimitivesNotebookChainTransfer: UlxPrimitivesNotebookChainTransfer;
    UlxPrimitivesNotebookNotebookHeader: UlxPrimitivesNotebookNotebookHeader;
    UlxPrimitivesNotebookSignedNotebookHeader: UlxPrimitivesNotebookSignedNotebookHeader;
    UlxPrimitivesProvidersBlockSealerInfo: UlxPrimitivesProvidersBlockSealerInfo;
  } // InterfaceTypes
} // declare module
