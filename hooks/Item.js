import * as anchor from '@project-serum/anchor'
import { useEffect, useMemo, useState } from 'react'
import { PublicKey , SystemProgram } from '@solana/web3.js'
import ItemIdl from '../Idl/idl.json'
import { useAnchorWallet, useConnection, useWallet } from '@solana/wallet-adapter-react'






export function Item() {
    const { connection } = useConnection()
    const { publicKey } = useWallet()
    const anchorWallet = useAnchorWallet()

    const program = new PublicKey("6MhT8RYRYk13xPAc8e9jjTbHokTSwaZpXhp3ojToYZEC");
}