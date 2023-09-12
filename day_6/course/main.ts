import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import "@polkadot/api-augment";

import type { FrameSystemAccountInfo } from "@polkadot/types/lookup";
import { KeyringPair } from "@polkadot/keyring/types";

const WEB_SOCKET = "ws://127.0.0.1:9944";

function sleep(time: number | undefined) {
  return new Promise((resolve) => setTimeout(resolve, time));
}

const connect = async () => {
  const wsProvider = new WsProvider(WEB_SOCKET);
  const api = await ApiPromise.create({ provider: wsProvider, types: {} });
  await api.isReady;
  return api;
};
/// 连接api
const test1 = async () => {
  const api = await connect();
  console.log(api);
};

/// 获取const 常量
const getConst = async (api: ApiPromise) => {
  const existentialDeposit = await api.consts.balances.existentialDeposit
    .toHuman();
  return existentialDeposit;
};
const test2 = async () => {
  const api = await connect();
  const deposit = await getConst(api);
  console.log("deposit is", deposit);
};

const getFreeBalance = async (api: ApiPromise, address: string) => {
  const { data: { free } }: FrameSystemAccountInfo = await api.query.system
    .account(address);
  return free;
};
/// 获取变量
const test3 = async () => {
  const api = await connect();
  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");
  const free = await getFreeBalance(api, alice.address);
  console.log("free is", free.toHuman());
};

const transfer = async (
  api: ApiPromise,
  alice: KeyringPair,
  bob: string,
  amount: number,
) => {
  await api.tx.balances
    .transfer(bob, amount)
    .signAndSend(alice, (res) => {
      console.log(`Tx status: ${res.status}`);
    });
};
/// 交易
const test4 = async () => {
  const api = await connect();
  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");
  const bob = keyring.addFromUri("//Bob");
  const bob_balance = await getFreeBalance(api, bob.address);

  console.log("bob_balance is", bob_balance.toHuman());
  await transfer(api, alice, bob.address, 10 ** 10 + 1);
  await sleep(10000);

  const bob_balance_after = await getFreeBalance(api, bob.address);
  console.log("bob_balance after is", bob_balance_after.toHuman());
};

///获取metadata
const getMetadata = async (api: ApiPromise) => {
  const metadata = await api.rpc.state.getMetadata();
  return metadata.toString();
};
const test5 = async () => {
  const api = await connect();
  console.log("metadata is", await getMetadata(api));
};

/// 订阅某个账号的balance值的改变, 订阅值的变化
const subscribe = async (api: ApiPromise, address: string) => {
  await api.query.system.account(address, (aliceInfo) => {
    const free = aliceInfo.data.free;
    console.log("free balance is", free.toHuman());
  });
};
const test6 = async () => {
  const api = await connect();
  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");
  await subscribe(api, alice.address);
  await sleep(1000000);
};

/// event 的调用
const subscribeEvent = async (api: ApiPromise) => {
  await api.query.system.events((events) => {
    events.forEach(function (event) {
      console.log("index ", event["event"]["index"].toHuman());
      console.log("data ", event["event"]["data"].toHuman());
      console.log("data ", event["event"].toHuman());
    });
  });
};

const test7 = async () => {
  const api = await connect();
  await subscribeEvent(api);
  await sleep(1000000);
};

const test8 = async () => {
  const api = await connect();
  await api.query.system.events((events) => {
    events.forEach(function (event) {
      if (event["event"]["data"]["method"] == "SomethingStored") {
        console.log("data ", event["event"]["data"].toHuman());
        console.log("event ", event["event"].toHuman());
        console.log("event ", event.toHuman());
      }
    });
  });
  await sleep(1000000);
};

const main = async () => {
  // await test1();
  // await test2();
  // await test3();
  // await test4();
  // await test5();
  // await test6();
  // await test7();
  await test8();
  console.log("main func");
};

main()
  .then(() => {
    console.log("exit with success");
    process.exit(0);
  })
  .catch((error) => {
    console.error("error is ", error);
    process.exit(1);
  });
