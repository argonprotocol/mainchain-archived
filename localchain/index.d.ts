/* auto-generated by NAPI-RS */
/* eslint-disable */
export class AccountStore {
  getDepositAccount(notaryId?: number | undefined | null): Promise<LocalAccount>
  getTaxAccount(notaryId?: number | undefined | null): Promise<LocalAccount>
  get(address: string, accountType: AccountType, notaryId: number): Promise<LocalAccount>
  getById(id: number): Promise<LocalAccount>
  hasAccount(address: string, accountType: AccountType, notaryId: number): Promise<boolean>
  /** Finds an account with no balance that is not waiting for a send claim */
  findIdleJumpAccount(accountType: AccountType, notaryId: number): Promise<LocalAccount | null>
  insert(address: string, accountType: AccountType, notaryId: number, hdPath?: string | undefined | null): Promise<LocalAccount>
  list(includeJumpAccounts?: boolean | undefined | null): Promise<Array<LocalAccount>>
}

export class BalanceChange {
  id: number
  accountId: number
  changeNumber: number
  balance: string
  netBalanceChange: string
  escrowHoldNoteJson?: string
  notaryId: number
  notesJson: string
  proofJson?: string
  finalizedBlockNumber?: number
  status: BalanceChangeStatus
  transactionId?: number
  notarizationId?: number
}
export type BalanceChangeRow = BalanceChange

export class BalanceChangeBuilder {
  accountType: AccountType
  address: string
  localAccountId: number
  changeNumber: number
  syncStatus?: BalanceChangeStatus
  static newAccount(address: string, localAccountId: number, accountType: AccountType): BalanceChangeBuilder
  isEmptySignature(): Promise<boolean>
  get balance(): Promise<bigint>
  get accountId32(): Promise<Uint8Array>
  isPendingClaim(): Promise<boolean>
  send(amount: bigint, restrictToAddresses?: Array<string> | undefined | null): Promise<void>
  claim(amount: bigint): Promise<ClaimResult>
  claimEscrow(amount: bigint): Promise<ClaimResult>
  claimFromMainchain(transfer: LocalchainTransfer): Promise<void>
  sendToMainchain(amount: bigint): Promise<void>
  createEscrowHold(amount: bigint, dataDomain: string, dataDomainAddress: string, delegatedSignerAddress?: string | undefined | null): Promise<void>
  createPrivateServerEscrowHold(amount: bigint, paymentAddress: string, delegatedSignerAddress?: string | undefined | null): Promise<void>
  sendToVote(amount: bigint): Promise<void>
  /** Lease a data domain. DataDomain leases are converted in full to tax. */
  leaseDataDomain(): Promise<bigint>
  /** Create scale encoded signature message for the balance change. */
  static toSigningMessage(balanceChangeJson: string): Uint8Array
}

export class BalanceChangeStore {
  allForAccount(accountId: number): Promise<Array<BalanceChange>>
  getLatestForAccount(accountId: number): Promise<BalanceChange | null>
  cancel(id: number): Promise<void>
  getById(id: number): Promise<BalanceChange>
  findUnsettled(): Promise<Array<BalanceChange>>
}

export class BalanceSync {
  constructor(localchain: Localchain)
  sync(options?: EscrowCloseOptions | undefined | null): Promise<BalanceSyncResult>
  consolidateJumpAccounts(): Promise<Array<NotarizationTracker>>
  syncUnsettledBalances(): Promise<Array<BalanceChange>>
  syncMainchainTransfers(): Promise<Array<NotarizationTracker>>
  syncBalanceChange(balanceChange: BalanceChange): Promise<BalanceChange>
  processPendingEscrows(options?: EscrowCloseOptions | undefined | null): Promise<Array<NotarizationBuilder>>
}

export class BalanceSyncResult {
  get balanceChanges(): Array<BalanceChange>
  get escrowNotarizations(): Array<NotarizationBuilder>
  get mainchainTransfers(): Array<NotarizationTracker>
  get jumpAccountConsolidations(): Array<NotarizationTracker>
}

export class BalanceTipResult {
  balanceTip: Uint8Array
  notebookNumber: number
  tick: number
}

export class DataDomainLease {
  id: number
  name: string
  tld: string
  registeredToAddress: string
  notarizationId: number
  registeredAtTick: number
}
export type DataDomainRow = DataDomainLease

export class DataDomainStore {
  static tldFromString(tld: string): DataTLD
  get list(): Promise<Array<DataDomainLease>>
  hashDomain(domain: JsDataDomain): Uint8Array
  static getHash(domain: string): Uint8Array
  static parse(domain: string): DataDomain
  get(id: number): Promise<DataDomainLease>
}

export class Escrow {
  id: string
  initialBalanceChangeJson: string
  notaryId: number
  fromAddress: string
  delegatedSignerAddress?: string
  toAddress: string
  dataDomainHash?: Array<number>
  expirationTick: number
  balanceChangeNumber: number
  notarizationId?: number
  isClient: boolean
  missedClaimWindow: boolean
  get holdAmount(): bigint
  get settledAmount(): bigint
  get settledSignature(): Uint8Array
  isPastClaimPeriod(currentTick: number): boolean
}

export class Keystore {
  useExternal(defaultAddress: string, sign: (address: string, signatureMessage: Uint8Array) => Promise<Uint8Array>, derive: (hdPath: string) => Promise<string>): Promise<void>
  /** Bootstrap this localchain with a new key. Must be empty or will throw an error! Defaults to SR25519 if no scheme is provided. */
  bootstrap(scheme?: CryptoScheme | undefined | null, passwordOption?: KeystorePasswordOption | undefined | null): Promise<string>
  /** Import a known keypair into the embedded keystore. */
  importSuri(suri: string, scheme: CryptoScheme, passwordOption?: KeystorePasswordOption | undefined | null): Promise<string>
  unlock(passwordOption?: KeystorePasswordOption | undefined | null): Promise<void>
  lock(): Promise<void>
  isUnlocked(): Promise<boolean>
  deriveAccountId(hdPath: string): Promise<string>
  sign(address: string, message: Uint8Array): Promise<Uint8Array>
}

export class LocalAccount {
  id: number
  address: string
  hdPath?: string
  accountId32: string
  notaryId: number
  accountType: AccountType
  createdAt: number
  updatedAt: number
  origin?: NotaryAccountOrigin
}

export class Localchain {
  path: string
  static load(config: LocalchainConfig): Promise<Localchain>
  static loadWithoutMainchain(path: string, tickerConfig: TickerConfig, keystorePassword?: KeystorePasswordOption | undefined | null): Promise<Localchain>
  attachMainchain(mainchainClient: MainchainClient): Promise<void>
  updateTicker(ntpSyncPoolUrl?: string | undefined | null): Promise<void>
  close(): Promise<void>
  accountOverview(): Promise<LocalchainOverview>
  static getDefaultDir(): string
  static setDefaultDir(value: string): void
  static getDefaultPath(): string
  get address(): Promise<string>
  get name(): string
  get currentTick(): number
  durationToNextTick(): bigint
  get ticker(): TickerRef
  get keystore(): Keystore
  get mainchainClient(): Promise<MainchainClient | null>
  get mainchainTransfers(): MainchainTransferStore
  get notaryClients(): NotaryClients
  get accounts(): AccountStore
  get balanceChanges(): BalanceChangeStore
  get dataDomains(): DataDomainStore
  get openEscrows(): OpenEscrowsStore
  get balanceSync(): BalanceSync
  get transactions(): Transactions
  beginChange(): NotarizationBuilder
}

export class MainchainClient {
  host: string
  close(): Promise<void>
  static connect(host: string, timeoutMillis: number): Promise<MainchainClient>
  getTicker(): Promise<Ticker>
  getBestBlockHash(): Promise<Uint8Array>
  getVoteBlockHash(currentTick: number): Promise<BestBlockForVote | null>
  getDataDomainRegistration(domainName: string, tld: DataTLD): Promise<DataDomainRegistration | null>
  getDataDomainZoneRecord(domainName: string, tld: DataTLD): Promise<ZoneRecord | null>
  getNotaryDetails(notaryId: number): Promise<NotaryDetails | null>
  getAccount(address: string): Promise<AccountInfo>
  getUlixees(address: string): Promise<BalancesAccountData>
  getTransferToLocalchainFinalizedBlock(transferId: number): Promise<number | null>
  waitForLocalchainTransfer(transferId: number): Promise<LocalchainTransfer | null>
  getAccountChangesRoot(notaryId: number, notebookNumber: number): Promise<Uint8Array>
  latestFinalizedNumber(): Promise<number>
  waitForNotebookImmortalized(notaryId: number, notebookNumber: number): Promise<number>
}

export class MainchainTransferStore {
  sendToLocalchain(amount: bigint, notaryId?: number | undefined | null): Promise<LocalchainTransfer>
}

export class NotarizationBuilder {
  set notaryId(notaryId: number)
  get notaryId(): Promise<number>
  set transaction(transaction: LocalchainTransaction)
  get transaction(): Promise<LocalchainTransaction | null>
  get isFinalized(): Promise<boolean>
  get unclaimedTax(): Promise<bigint>
  get escrows(): Promise<Array<Escrow>>
  get accounts(): Promise<Array<LocalAccount>>
  get balanceChangeBuilders(): Promise<Array<BalanceChangeBuilder>>
  get unusedVoteFunds(): Promise<bigint>
  get unusedDomainFunds(): Promise<bigint>
  get unclaimedDeposits(): Promise<bigint>
  getBalanceChange(account: LocalAccount): Promise<BalanceChangeBuilder>
  addAccount(address: string, accountType: AccountType, notaryId: number): Promise<BalanceChangeBuilder>
  addAccountById(localAccountId: number): Promise<BalanceChangeBuilder>
  getJumpAccount(accountType: AccountType): Promise<BalanceChangeBuilder>
  defaultDepositAccount(): Promise<BalanceChangeBuilder>
  defaultTaxAccount(): Promise<BalanceChangeBuilder>
  loadAccount(account: LocalAccount): Promise<BalanceChangeBuilder>
  canAddEscrow(escrow: OpenEscrow): Promise<boolean>
  cancelEscrow(openEscrow: OpenEscrow): Promise<void>
  claimEscrow(openEscrow: OpenEscrow): Promise<void>
  addVote(vote: BlockVote): Promise<void>
  leaseDataDomain(dataDomain: string, registerToAddress: string): Promise<void>
  /** Calculates the transfer tax on the given amount */
  getTransferTaxAmount(amount: bigint): bigint
  /** Calculates the total needed to end up with the given balance */
  getTotalForAfterTaxBalance(finalBalance: bigint): bigint
  getEscrowTaxAmount(amount: bigint): bigint
  claimFromMainchain(transfer: LocalchainTransfer): Promise<BalanceChangeBuilder>
  claimAndPayTax(milligonsPlusTax: bigint, depositAccountId: number | undefined | null, useDefaultTaxAccount: boolean): Promise<BalanceChangeBuilder>
  fundJumpAccount(milligons: bigint): Promise<BalanceChangeBuilder>
  acceptArgonFileRequest(argonFileJson: string): Promise<void>
  importArgonFile(argonFileJson: string): Promise<void>
  /**
   * Exports an argon file from this notarization builder with the intention that these will be sent to another
   * user (who will import into their own localchain).
   */
  exportAsFile(fileType: ArgonFileType): Promise<string>
  toJSON(): Promise<string>
  notarizeAndWaitForNotebook(): Promise<NotarizationTracker>
  notarize(): Promise<NotarizationTracker>
  verify(): Promise<void>
  sign(): Promise<void>
}

export class NotarizationTracker {
  notebookNumber: number
  tick: number
  notaryId: number
  notarizationId: number
  notarizedBalanceChanges: number
  notarizedVotes: number
  /** Returns the balance changes that were submitted to the notary indexed by the stringified account id (napi doesn't allow numbers as keys) */
  get balanceChangesByAccountId(): Promise<Record<string, BalanceChange>>
  waitForNotebook(): Promise<void>
  /** Asks the notary for proof the transaction was included in a notebook header. If this notebook has not been finalized yet, it will return an error. */
  getNotebookProof(): Promise<Array<NotebookProof>>
  /** Confirms the root added to the mainchain */
  waitForImmortalized(mainchainClient: MainchainClient): Promise<ImmortalizedBlock>
}

export class NotaryClient {
  notaryId: number
  autoVerifyHeaderSignatures: boolean
  isConnected(): Promise<boolean>
  static connect(notaryId: number, publicKey: Uint8Array, host: string, autoVerifyHeaderSignatures: boolean): Promise<NotaryClient>
  getBalanceTip(address: string, accountType: AccountType): Promise<BalanceTipResult>
  get metadata(): Promise<NotebookMeta>
}

export class NotaryClients {
  static new(mainchainClient: MainchainClient): NotaryClients
  close(): Promise<void>
  useClient(client: NotaryClient): Promise<void>
  get(notaryId: number): Promise<NotaryClient>
}

export class OpenEscrow {
  get escrow(): Promise<Escrow>
  sign(settledAmount: bigint): Promise<SignatureResult>
  exportForSend(): Promise<string>
  recordUpdatedSettlement(milligons: bigint, signature: Uint8Array): Promise<void>
}

export class OpenEscrowsStore {
  get(id: string): Promise<OpenEscrow>
  open(escrow: Escrow): OpenEscrow
  getClaimable(): Promise<Array<OpenEscrow>>
  /** Import an escrow from a JSON string. Verifies with the notary that the escrow hold is valid. */
  importEscrow(escrowJson: string): Promise<OpenEscrow>
  /** Create a new escrow as a client. You must first notarize an escrow hold note to the notary for the `client_address`. */
  openClientEscrow(accountId: number): Promise<OpenEscrow>
}

export class OverviewStore {
  get(): Promise<LocalchainOverview>
}

export class TickerRef {
  get current(): number
  tickForTime(timestampMillis: number): number
  timeForTick(tick: number): bigint
  millisToNextTick(): bigint
}

export class Transactions {
  create(transactionType: TransactionType): Promise<LocalchainTransaction>
  request(milligons: bigint): Promise<string>
  createEscrow(escrowMilligons: bigint, recipientAddress: string, dataDomain?: string | undefined | null, notaryId?: number | undefined | null, delegatedSignerAddress?: string | undefined | null): Promise<OpenEscrow>
  send(milligons: bigint, to?: Array<string> | undefined | null): Promise<string>
  importArgons(argonFile: string): Promise<NotarizationTracker>
  acceptArgonRequest(argonFile: string): Promise<NotarizationTracker>
}

export interface AccountInfo {
  nonce: number
  consumers: number
  providers: number
  sufficients: number
  data: BalancesAccountData
}

export enum AccountType {
  Tax = 'tax',
  Deposit = 'deposit'
}

export const ADDRESS_PREFIX: number

/** The version of the Argon file format. */
export const ARGON_FILE_VERSION: string

export enum ArgonFileType {
  Send = 0,
  Request = 1
}

export interface BalanceChangeGroup {
  netBalanceChange: bigint
  netTax: bigint
  heldBalance: bigint
  timestamp: number
  notes: Array<string>
  finalizedBlockNumber?: number
  status: BalanceChangeStatus
  notarizationId?: number
  transactionId?: number
  transactionType?: TransactionType
  balanceChanges: Array<BalanceChangeSummary>
  notebookNumber?: number
}

export enum BalanceChangeStatus {
  /** The balance change has been notarized by a notary. It isn't necessarily in a notebook yet. */
  Notarized = 'Notarized',
  /** A balance change that doesn't get final proof because it is one of many in a single notebook. Aka, another balance change superseded it in the notebook. */
  SupersededInNotebook = 'SupersededInNotebook',
  /** Proof has been obtained from a notebook */
  NotebookPublished = 'NotebookPublished',
  /** The mainchain has included the notebook with the balance change */
  Immortalized = 'Immortalized',
  /** A balance change has been sent to another user to claim. Keep checking until it is claimed. */
  WaitingForSendClaim = 'WaitingForSendClaim',
  /** A pending balance change that was canceled before being claimed by another user (escrow or send). */
  Canceled = 'Canceled'
}

export interface BalanceChangeSummary {
  id: number
  finalBalance: bigint
  holdBalance: bigint
  netBalanceChange: bigint
  changeNumber: number
  accountId: number
  accountType: AccountType
  isJumpAccount: boolean
  notes: Array<string>
  status: BalanceChangeStatus
  notebookNumber?: number
  finalizedBlockNumber?: number
}

export interface BalancesAccountData {
  free: bigint
  reserved: bigint
  frozen: bigint
  flags: bigint
}

export interface BestBlockForVote {
  blockHash: Uint8Array
  voteMinimum: bigint
}

export interface BlockVote {
  /** The creator of the seal */
  address: string
  /** The block hash being voted on. Must be in last 2 ticks. */
  blockHash: Array<number>
  /** A unique index per account for this notebook */
  index: number
  /** The voting power of this vote, determined from the amount of tax */
  power: bigint
  /** The data domain used to create this vote */
  dataDomainHash: Array<number>
  /** The data domain payment address used to create this vote */
  dataDomainAddress: string
  /** The mainchain address where rewards will be sent */
  blockRewardsAddress: string
  /** A signature of the vote by the account_id */
  signature: Array<number>
}

export interface ClaimResult {
  claimed: bigint
  tax: bigint
}

export enum CryptoScheme {
  Ed25519 = 0,
  Sr25519 = 1,
  Ecdsa = 2
}

/** Cost to lease a data domain for 1 year */
export const DATA_DOMAIN_LEASE_COST: bigint

/** Minimum data domain name length */
export const DATA_DOMAIN_MIN_NAME_LENGTH: number

export interface DataDomain {
  domainName: string
  topLevelDomain: DataTLD
}

export interface DataDomainRegistration {
  registeredToAddress: string
  registeredAtTick: number
}

/** Max versions that can be in a datastore zone record */
export const DATASTORE_MAX_VERSIONS: number

export enum DataTLD {
  Analytics = 0,
  Automotive = 1,
  Bikes = 2,
  Business = 3,
  Cars = 4,
  Communication = 5,
  Entertainment = 6,
  Finance = 7,
  Flights = 8,
  Health = 9,
  Hotels = 10,
  Jobs = 11,
  News = 12,
  RealEstate = 13,
  Restaurants = 14,
  Shopping = 15,
  Sports = 16,
  Transportation = 17,
  Travel = 18,
  Weather = 19
}

/** Number of ticks past the expiration of an escrow that a recipient has to claim. After this point, sender can recoup the escrowed funds */
export const ESCROW_CLAWBACK_TICKS: number

/** Number of ticks past the notarization of an escrow hold that an escrow can be claimed (and no longer used) */
export const ESCROW_EXPIRATION_TICKS: number

/** Minimum milligons that can be settled in an escrow */
export const ESCROW_MINIMUM_SETTLEMENT: bigint

export interface EscrowCloseOptions {
  votesAddress?: string
  /** What's the minimum amount of tax we should wait for before voting on blocks */
  minimumVoteAmount?: number
}

export interface ImmortalizedBlock {
  immortalizedBlock: number
  accountChangesMerkleRoot: Uint8Array
}

/**
 * Options to provide the password for a keystore. NOTE that this library cannot clear out memory in javascript.
 * Only a single option should be picked.
 */
export interface KeystorePasswordOption {
  /** Provides a password directly for the keystore. Converted to a SecretString inside Rust, but not cleared out in javascript or napi. */
  password?: Buffer
  /** Initiate a prompt from the cli to load the password. */
  interactiveCli?: boolean
  /** Load the password from a file. */
  passwordFile?: string
}

export interface LocalchainConfig {
  path: string
  mainchainUrl: string
  ntpPoolUrl?: string
  keystorePassword?: KeystorePasswordOption
}

export interface LocalchainOverview {
  /** The name of this localchain */
  name: string
  /** The primary localchain address */
  address: string
  /** The current account balance */
  balance: bigint
  /** The net pending balance change acceptance/confirmation */
  pendingBalanceChange: bigint
  /** Balance held in escrow */
  heldBalance: bigint
  /** Tax accumulated for the account */
  tax: bigint
  /** The net pending tax balance change */
  pendingTaxChange: bigint
  /** Changes to the account ordered from most recent to oldest */
  changes: Array<BalanceChangeGroup>
  /** The mainchain balance */
  mainchainBalance: bigint
  /** The net pending mainchain balance pending movement in/out of the localchain */
  processingMainchainBalanceChange: bigint
}

export interface LocalchainTransaction {
  id: number
  transactionType: TransactionType
}

export interface LocalchainTransfer {
  address: string
  amount: bigint
  notaryId: number
  expirationTick: number
  transferId: number
}

/** Max balance changes that can be in a single notarization */
export const NOTARIZATION_MAX_BALANCE_CHANGES: number

/** Max notarizations that can be in a single notarization */
export const NOTARIZATION_MAX_BLOCK_VOTES: number

/** Max data domains that can be in a single notarization */
export const NOTARIZATION_MAX_DOMAINS: number

export interface NotaryAccountOrigin {
  notaryId: number
  notebookNumber: number
  accountUid: number
}

export interface NotaryDetails {
  id: number
  hosts: Array<string>
  publicKey: Uint8Array
}

export interface NotebookMeta {
  finalizedNotebookNumber: number
  finalizedTick: number
}

export interface NotebookProof {
  address: string
  accountType: AccountType
  notebookNumber: number
  balance: bigint
  /** H256 hash of the balance tip */
  balanceTip: Uint8Array
  changeNumber: number
  accountOrigin: NotaryAccountOrigin
  escrowHoldNoteJson?: string
  leafIndex: number
  numberOfLeaves: number
  proof: Array<Uint8Array>
}

export function runCli(): Promise<void>

export interface SignatureResult {
  signature: Uint8Array
  milligons: bigint
}

export interface Ticker {
  tickDurationMillis: number
  genesisUtcTime: number
}

export interface TickerConfig {
  tickDurationMillis: number
  genesisUtcTime: number
  ntpPoolUrl?: string
}

export enum TransactionType {
  Send = 0,
  Request = 1,
  OpenEscrow = 2,
  Consolidation = 3
}

export interface VersionHost {
  /** Datastore id is a 2-50 char string that uniquely identifies a data domain. */
  datastoreId: string
  /** The host address where the data domain can be accessed. */
  host: string
}

export interface ZoneRecord {
  paymentAddress: string
  notaryId: number
  /** A mapping of versions to host addresses. */
  versions: Record<string, VersionHost>
}
