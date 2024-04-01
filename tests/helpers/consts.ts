import { PublicKey, Keypair } from "@solana/web3.js";
import { CONTRACTS } from "@certusone/wormhole-sdk";
import { MockGuardians } from "@certusone/wormhole-sdk/lib/cjs/mock";

export const NETWORK = "DEVNET";

export const WORMHOLE_CONTRACTS = CONTRACTS[NETWORK];
export const CORE_BRIDGE_PID = new PublicKey("3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5");
export const TOKEN_BRIDGE_PID = new PublicKey("DZnkkTmCiFWfYTfT41X3Rd1kDgozqzxWaHqsw6W4x2oe");

export const LOCALHOST = "http://localhost:8899";
export const DEVNET = "https://api.devnet.solana.com";

export const PAYER_KEYPAIR = Keypair.fromSecretKey(Uint8Array.from([113, 67, 183, 229, 97, 122, 151, 85, 26, 62, 212, 142, 226, 2, 96, 89, 19, 194, 211, 90, 101, 107, 97, 84, 17, 21, 235, 117, 160, 66, 118, 96, 99, 53, 116, 217, 100, 121, 131, 207, 142, 78, 112, 232, 228, 211, 151, 206, 212, 17, 211, 161, 30, 240, 29, 197, 252, 16, 112, 61, 207, 91, 106, 115]));
export const RELAYER_KEYPAIR = Keypair.fromSecretKey(Uint8Array.from([
  209, 193, 148, 98, 190, 29, 112, 141, 167, 133, 181, 253, 103, 0, 148, 205,
  111, 214, 146, 194, 94, 126, 194, 28, 188, 221, 72, 105, 190, 41, 91, 39, 237,
  124, 31, 221, 91, 218, 22, 33, 230, 41, 14, 203, 176, 164, 200, 245, 31, 19,
  161, 61, 30, 188, 11, 120, 155, 236, 178, 241, 114, 240, 67, 3,
]));

//this is the WETH mainnet address - but any address will do for local testing
export const WETH_ADDRESS = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

export const GOVERNANCE_EMITTER_ADDRESS = new PublicKey("11111111111111111111111111111115");

export const MOCK_GUARDIANS =
  new MockGuardians(0, ["cfb12303a19cde580bb4dd771639b0d26bc68353645571a8cff516ab2ee113a0"]);

export const MINTS_WITH_DECIMALS =
  new Map<number, { privateKey: Uint8Array; publicKey: PublicKey }>([
    [
      8,
      {
        privateKey: Uint8Array.from([76, 170, 108, 66, 202, 13, 82, 128, 179, 149, 61, 183, 229, 184, 29, 40, 200, 15, 101, 25, 147, 242, 75, 153, 118, 174, 191, 39, 165, 125, 133, 8, 126, 106, 29, 8, 252, 6, 29, 167, 242, 236, 187, 237, 54, 203, 34, 195, 29, 174, 252, 102, 130, 242, 24, 204, 35, 229, 4, 109, 179, 183, 187, 219]),
        publicKey: new PublicKey("DgMtX3f3gCe41apKkAPrPVBcPNRUyWgrL5wqFwEdP2mn"),
      },
    ],
    [
      9,
      {
        privateKey: Uint8Array.from([
          98, 139, 243, 120, 236, 152, 36, 219, 202, 42, 72, 178, 107, 155, 181, 134,
          120, 36, 55, 108, 253, 218, 96, 139, 80, 99, 85, 54, 116, 145, 94, 40, 227,
          10, 159, 48, 118, 75, 67, 84, 239, 36, 177, 138, 6, 214, 73, 149, 26, 100,
          255, 28, 218, 167, 251, 229, 93, 236, 25, 225, 152, 104, 223, 54,
        ]),
        publicKey: new PublicKey("GHGwbrTCsynp7yJ9keowy2Roe5DzxFbayAaAwLyAvRKj"),
      }
    ]
  ]);
