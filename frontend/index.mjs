import { ApiPromise } from "@polkadot/api";
import { u8aToString, stringToU8a } from "@polkadot/util";

async function main() {
  // Create our API with a default connection to the local node
  const api = await ApiPromise.create();

  // 获取 Offchain 数据
  const value = await api.rpc.offchain.localStorageGet(
    "PERSISTENT",
    "node-template::storage::"
  );

  const hexValue = value.toHex();
  const u8aValue = new Uint8Array(
    (hexValue.match(/.{1,2}/g) || []).map((byte) => parseInt(byte, 16))
  );
  const stringValue = u8aToString(u8aValue);
  console.log("value in offchain storage: ", hexValue);
}

main().catch((error) => {
  console.error(error);
  process.exit(-1);
});
