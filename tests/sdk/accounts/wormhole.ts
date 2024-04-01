import { deriveAddress } from "@certusone/wormhole-sdk/lib/cjs/solana";
import { deriveWormholeEmitterKey } from "@certusone/wormhole-sdk/lib/cjs/solana/wormhole";
import { Connection, PublicKeyInitData } from "@solana/web3.js";
import { createMyWormholeProjectProgramInterface } from "../program";

export { deriveWormholeEmitterKey };

export function deriveWormholeMessageKey(
  programId: PublicKeyInitData,
  sequence: bigint
) {
  return deriveAddress(
    [
      Buffer.from("sent"),
      (() => {
        const buf = Buffer.alloc(8);
        buf.writeBigUInt64LE(sequence);
        return buf;
      })(),
    ],
    programId
  );
}

export interface WormholeEmitterData {
  bump: number;
}

export async function getWormholeEmitterData(
  connection: Connection,
  programId: PublicKeyInitData
): Promise<WormholeEmitterData> {
  return createMyWormholeProjectProgramInterface(connection, programId)
    .account.wormholeEmitter.fetch(deriveWormholeEmitterKey(programId));
}
