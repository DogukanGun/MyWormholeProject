import { Connection, PublicKeyInitData, PublicKey } from "@solana/web3.js";
import { Program, Provider } from "@coral-xyz/anchor";
import { IDL, MyWormholeProject } from "../../target/types/my_wormhole_project";


export function createMyWormholeProjectProgramInterface(
  connection: Connection,
  programId: PublicKeyInitData,
  payer?: PublicKeyInitData
): Program<MyWormholeProject> {
  const provider: Provider = {
    connection,
    publicKey: payer == undefined ? undefined : new PublicKey(payer),
  };
  return new Program<MyWormholeProject>(
    IDL as any,
    new PublicKey(programId),
    provider
  );
}
