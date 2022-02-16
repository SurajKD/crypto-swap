import * as anchor from '@project-serum/anchor'
import { Provider, BN } from '@project-serum/anchor'
import { Token, TOKEN_PROGRAM_ID } from '@solana/spl-token'
import { Keypair } from '@solana/web3.js'
import { assert } from 'chai'
import { createToken, initEverything } from './testUtils'
import { Market, Pair, tou64, DENOMINATOR, Network } from '@invariant-labs/sdk'
import { CreateFeeTier, FeeTier, InitPoolAndPosition } from '@invariant-labs/sdk/lib/market'
import { fromFee } from '@invariant-labs/sdk/lib/utils'
import { toDecimal } from '@invariant-labs/sdk/src/utils'
import { CreateTick, InitPosition, Swap } from '@invariant-labs/sdk/src/market'
import { u64 } from '@project-serum/serum/lib/layout'

describe('swap', () => {
  const provider = Provider.local()
  const connection = provider.connection
  // @ts-expect-error
  const wallet = provider.wallet.payer as Keypair
  const mintAuthority = Keypair.generate()
  const admin = Keypair.generate()
  const feeTier: FeeTier = {
    fee: fromFee(new BN(600)),
    tickSpacing: 10
  }
  let market: Market
  let pair: Pair
  let tokenX: Token
  let tokenY: Token

  before(async () => {
    market = await Market.build(
      Network.LOCAL,
      provider.wallet,
      connection,
      anchor.workspace.Invariant.programId
    )

    // Request airdrops
    await Promise.all([
      connection.requestAirdrop(mintAuthority.publicKey, 1e9),
      connection.requestAirdrop(admin.publicKey, 1e9)
    ])
    // Create tokens
    const tokens = await Promise.all([
      createToken(connection, wallet, mintAuthority),
      createToken(connection, wallet, mintAuthority)
    ])

    pair = new Pair(tokens[0].publicKey, tokens[1].publicKey, feeTier)
    tokenX = new Token(connection, pair.tokenX, TOKEN_PROGRAM_ID, wallet)
    tokenY = new Token(connection, pair.tokenY, TOKEN_PROGRAM_ID, wallet)
  })

  it('#init()', async () => {
    await market.createState(admin.publicKey, admin)
    const createFeeTierVars: CreateFeeTier = {
      feeTier: pair.feeTier,
      admin: admin.publicKey
    }
    await market.createFeeTier(createFeeTierVars, admin)
  })

  it('#init Pool and position in a single tx', async () => {
    const owner = Keypair.generate()
    const [userTokenX, userTokenY] = await Promise.all([
      tokenX.createAccount(owner.publicKey),
      tokenX.createAccount(owner.publicKey),
      connection.requestAirdrop(owner.publicKey, 1e9)
    ])
    await Promise.all([
      tokenX.mintTo(userTokenX, mintAuthority, [], tou64(1e9)),
      tokenX.mintTo(userTokenY, mintAuthority, [], tou64(1e9))
    ])

    const props: InitPoolAndPosition = {
      pair,
      owner: owner.publicKey,
      userTokenX,
      userTokenY,
      lowerTick: 10,
      upperTick: 20,
      liquidityDelta: { v: new BN(1e9) },
      initTick: 0
    }

    await market.initPoolAndPosition(props, owner)
  })
})
