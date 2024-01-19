## lesson 4
###  请回答链上随机数（如前面Kitties示例中）与链下随机数的区别
- 链上随机数(Randomness Module)是根据当前结点的之前81个block的哈希生成的，由于链上系统无法实现真正的不可预测的熵来保证生成的随机数的随机性，所以链上随机数pallet只推荐在test时使用。
- 链下随机数(Offchain Random)由于是在链下执行，本着链下数据不可信的原则，可以使用当前结点系统关联生成不可预测的熵，以确保生成数的随机性。


![01](01.png)

https://github.com/znycat/substrate_study/blob/main/substrate-7/lesson_4_polkadot-v1.0.0/substrate-node-template/pallets/offchain/src/lib.rs


```
#[pallet::call_index(3)]
#[pallet::weight(100)]
pub fn extrinsic(origin: OriginFor<T>, number: u64) -> DispatchResult {
	let who = ensure_signed(origin)?;

	let key = Self::derived_key(frame_system::Module::<T>::block_number());
	let data = IndexingData(b"submit_number_unsigned".to_vec(), number);
	offchain_index::set(&key, &data.encode());

	log::info!("OCW ==> in extrinsic submit_number_unsigned: {:?}", number);

	Ok(())
}
```
