import { Provider } from '@project-serum/anchor'
import { Keypair } from '@solana/web3.js'
import { createToken } from '../tests/testUtils'
import { MINTER } from './minter'
require('dotenv').config()

const provider = Provider.local('https://api.devnet.solana.com', {
  // preflightCommitment: 'max',
  skipPreflight: true
})

const SOL_DECIMAL = 9
const USDT_DECIMAL = 6
const USDC_DECIMAL = 6

const main = async () => {
  const connection = provider.connection
  // @ts-expect-error
  const wallet = provider.wallet.payer as Keypair

  const solToken = await createToken(connection, wallet, MINTER, SOL_DECIMAL)
  const usdcToken = await createToken(connection, wallet, MINTER, USDC_DECIMAL)
  const usdtToken = await createToken(connection, wallet, MINTER, USDT_DECIMAL)
  const anaToken = await createToken(connection, wallet, MINTER, USDT_DECIMAL)
  const msolToken = await createToken(connection, wallet, MINTER, SOL_DECIMAL)

  console.log(`SOL: ${solToken.publicKey}`)
  console.log(`USDC: ${usdcToken.publicKey}`)
  console.log(`USDT: ${usdtToken.publicKey}`)
  console.log(`ANA: ${anaToken.publicKey}`)
  console.log(`MSOL: ${msolToken.publicKey}`)
}
main()
