# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## [0.44.0](https://github.com/maidsafe/sn_routing/compare/v0.43.4...v0.44.0) (2021-02-23)


### ⚠ BREAKING CHANGES

* **deps:** removes send_message_to_client api,

### Features

* **enduser:** add mapping between socketaddr and pk ([1ff902d](https://github.com/maidsafe/sn_routing/commit/1ff902da2f28d89ed6ecb3efe502efea8476135e))
* **messages:** remove MsgEnvelope ([57df069](https://github.com/maidsafe/sn_routing/commit/57df069f6ed5d7c9afe3b665158181cce70ceb15))


### Bug Fixes

* add missing routing to client of relayed client message ([fbde5b1](https://github.com/maidsafe/sn_routing/commit/fbde5b10d734fcf5037b0d37767ba5093376e46e))
* addresss review comments ([27dcac5](https://github.com/maidsafe/sn_routing/commit/27dcac57b78daa9b41c481ed3970453d172720b4))
* post-rebase issues ([906ef03](https://github.com/maidsafe/sn_routing/commit/906ef031585f3db19a546928e76c8304a7f3c7f3))
* remove unnecessary error mapping ([0f3418b](https://github.com/maidsafe/sn_routing/commit/0f3418b2ea4d66f438604ded2682d76f95e70d6f))
* remove use of wildcard match and unimplemented macro ([84c53d8](https://github.com/maidsafe/sn_routing/commit/84c53d8db16f1ab237c46ad5e8221b2a80758d54))


* **deps:** update sn_messaging, sn_data_types ([367b673](https://github.com/maidsafe/sn_routing/commit/367b6731b90b7211679282b2fcaa8852f3449ccd))

### [0.43.4](https://github.com/maidsafe/sn_routing/compare/v0.43.3...v0.43.4) (2021-02-23)

### [0.43.3](https://github.com/maidsafe/sn_routing/compare/v0.43.2...v0.43.3) (2021-02-18)

### [0.43.2](https://github.com/maidsafe/sn_routing/compare/v0.43.1...v0.43.2) (2021-02-16)


### Features

* having EldersInfo change candidate considered as DKG in progress ([6137123](https://github.com/maidsafe/sn_routing/commit/61371230e2eab7ceff4fd80073843d6b46ff4adf))
* notify client of incorrect section_key ([c54f034](https://github.com/maidsafe/sn_routing/commit/c54f034fdc304106d1a3e56e00012773b1e85a9d))
* updates for section key response changes ([71f89d8](https://github.com/maidsafe/sn_routing/commit/71f89d8c54008ff9974f740eff5be9ac2b893f26))

### [0.43.1](https://github.com/maidsafe/sn_routing/compare/v0.43.0...v0.43.1) (2021-02-15)


### Bug Fixes

* remove offline elder from vote recipients ([3bcea21](https://github.com/maidsafe/sn_routing/commit/3bcea21ceb00feacc52843cc435fe875d3ed3f84))

## [0.43.0](https://github.com/maidsafe/sn_routing/compare/v0.42.4...v0.43.0) (2021-02-15)


### ⚠ BREAKING CHANGES

* this changes the return type of State::new

### Bug Fixes

* **comm:** dont hold on to messages sent on a channel that is unused ([92856cd](https://github.com/maidsafe/sn_routing/commit/92856cd8daf51af109405d1b9b58b7fa0a5f2d9c))


* fix clippy errors with version 1.50.0 of rust ([b6b385a](https://github.com/maidsafe/sn_routing/commit/b6b385aa1d05a8ac908f568d4537bb64589cd470))

### [0.42.4](https://github.com/maidsafe/sn_routing/compare/v0.42.3...v0.42.4) (2021-02-09)

### [0.42.3](https://github.com/maidsafe/sn_routing/compare/v0.42.2...v0.42.3) (2021-02-08)

### [0.42.2](https://github.com/maidsafe/sn_routing/compare/v0.42.1...v0.42.2) (2021-02-04)


### Bug Fixes

* redirect to our elders on mismatching GetSectionRequest as adult ([22c4745](https://github.com/maidsafe/sn_routing/commit/22c47453a50d79b47771c9afb682d3cac88aeb12))

### [0.42.1](https://github.com/maidsafe/sn_routing/compare/v0.42.0...v0.42.1) (2021-02-04)

## [0.42.0](https://github.com/maidsafe/sn_routing/compare/v0.41.6...v0.42.0) (2021-02-04)


### ⚠ BREAKING CHANGES

* remove unused Error::NodeMessaging variant

### Features

* add Envelope and InfrastructureQuery ([e0b999f](https://github.com/maidsafe/sn_routing/commit/e0b999f961b971b068cad65bfe8e8f938bf4ab41))
* make use of sn_messaging crate for messaging serialisation/deserialisation ([cbc4802](https://github.com/maidsafe/sn_routing/commit/cbc48026e6d1e32cde8a3f1f7ab92ca7aed801ad))
* modify bootstrap to use infrastructure queries ([9fb438f](https://github.com/maidsafe/sn_routing/commit/9fb438f6a3c209a50733fd6b894cf4e4ca2861bc))
* remove unused Error::NodeMessaging variant ([0b70c28](https://github.com/maidsafe/sn_routing/commit/0b70c28792076599af88dd61f9f6482116c2f3e4))


### Bug Fixes

* send messages with correct MessageKind byte ([6756b43](https://github.com/maidsafe/sn_routing/commit/6756b43d969d26afe9305ee4ff2851c6e9193495))

### [0.41.6](https://github.com/maidsafe/sn_routing/compare/v0.41.5...v0.41.6) (2021-02-01)


### Features

* **keycache:** adds a key cache and removes exposure of secret key ([b312446](https://github.com/maidsafe/sn_routing/commit/b312446b6db2c2beaf6007d39619dd8969fc8428))


### Bug Fixes

* **clippy:** remove clone (undetected in local clippy check) ([da6cbc7](https://github.com/maidsafe/sn_routing/commit/da6cbc7af578649907202d74676caee1623af278))

### [0.41.5](https://github.com/maidsafe/sn_routing/compare/v0.41.4...v0.41.5) (2021-01-27)

### [0.41.4](https://github.com/maidsafe/sn_routing/compare/v0.41.3...v0.41.4) (2021-01-26)


### Features

* log send to client error ([ddeff5e](https://github.com/maidsafe/sn_routing/commit/ddeff5e0bf41dfdba3430a8df9ed4b51224822f9))

### [0.41.3](https://github.com/maidsafe/sn_routing/compare/v0.41.2...v0.41.3) (2021-01-21)

### [0.41.2](https://github.com/maidsafe/sn_routing/compare/v0.41.1...v0.41.2) (2021-01-20)

### [0.41.1](https://github.com/maidsafe/sn_routing/compare/v0.41.0...v0.41.1) (2021-01-20)


### Features

* do not create connection when failed to send to client ([d5eadd8](https://github.com/maidsafe/sn_routing/commit/d5eadd8dc2ae88af2ed26f2e9b0d58c20a69a516))

## [0.41.0](https://github.com/maidsafe/sn_routing/compare/v0.40.0...v0.41.0) (2021-01-19)


### ⚠ BREAKING CHANGES

*     - remove `Error::BadLocation` (use the more specific `InvalidSrcLocation` / `InvalidDstLocation` instead)
    - rename `Error::InvalidSource` to `Error::InvalidSrcLocation`
    - rename `Error::InvalidDestination` to `Error::InvalidDstLocation`

### Features

* improve fork diagnostics ([dbf9807](https://github.com/maidsafe/sn_routing/commit/dbf98072a98bba734c6e0458936fa3aaa56ddeb6))
* **stress test:** improve output ([33eac1b](https://github.com/maidsafe/sn_routing/commit/33eac1b61383f231d0c34657db98d00cc84cf7c3))
* remove old DKG sessions ([c8db72f](https://github.com/maidsafe/sn_routing/commit/c8db72f8120c538ed41cbe1d036106ba3c0c04d9))
* support multiple concurrent DKGs ([98fc101](https://github.com/maidsafe/sn_routing/commit/98fc10194ddd73387a5539ad1e29423a224583d5))


### Bug Fixes

* allow only one relocation at the time per node ([0e4d05f](https://github.com/maidsafe/sn_routing/commit/0e4d05f7f06349512a63a912a832cbab0631e429))
* don't fail in update_state if secret key share is missing ([97d8266](https://github.com/maidsafe/sn_routing/commit/97d8266042d1c21c02b8015aa5be38ad009c8224))
* ignore elders update with incorrect prefix ([dfc9c60](https://github.com/maidsafe/sn_routing/commit/dfc9c60278fe78fdd5fbb4de14b5cc2721dbf570))
* ignore invalid bootstrap response ([3d8cfd5](https://github.com/maidsafe/sn_routing/commit/3d8cfd583c16cff7c25b82c142c80aa6348852e3))
* **stress test:** ignore InvalidSource errors when sending probes ([adabf82](https://github.com/maidsafe/sn_routing/commit/adabf82f7da0f8b6669e8f28cc9fb7fca02f67b2))
* send Sync messages on split even when demoted ([5f42b78](https://github.com/maidsafe/sn_routing/commit/5f42b78c4bcd68720399d9553512057d8b7d4d0d))


* remove Error::BadLocation ([3391c7f](https://github.com/maidsafe/sn_routing/commit/3391c7f1d49e050ae2fe580816a10add68388d14))

## [0.40.0](https://github.com/maidsafe/sn_routing/compare/v0.39.16...v0.40.0) (2021-01-13)


### ⚠ BREAKING CHANGES

* this affects the `Error` type which is a part of the public API.

### Bug Fixes

* trust check failure of Sync message sent to non-elders ([5520c18](https://github.com/maidsafe/sn_routing/commit/5520c182e1f3c29ce560d3fbb6e1e7e74324ac47))
* use keys not key indices to check whether elders changed ([a99a07f](https://github.com/maidsafe/sn_routing/commit/a99a07f80b706d7fe84b5d970ee10999910db395))


* remove `Error::UntrustedMessage` ([dbcf0db](https://github.com/maidsafe/sn_routing/commit/dbcf0db471f2b234342fdeeb68d5cf7aaff50846))

### [0.39.16](https://github.com/maidsafe/sn_routing/compare/v0.39.15...v0.39.16) (2021-01-13)

### [0.39.15](https://github.com/maidsafe/sn_routing/compare/v0.39.14...v0.39.15) (2021-01-13)

### [0.39.14](https://github.com/maidsafe/sn_routing/compare/v0.39.13...v0.39.14) (2021-01-12)


### Features

* add stress test example ([cf25c48](https://github.com/maidsafe/sn_routing/commit/cf25c48d3ba613db0a1e631620727a31f87d2661))


### Bug Fixes

* **stress test:** fix log to file and probe message destination ([c933605](https://github.com/maidsafe/sn_routing/commit/c933605df7847b05afd5a0b497cc315381e99955))
* **stress test:** fix sent probe messages counter ([b9b7530](https://github.com/maidsafe/sn_routing/commit/b9b7530fe383142541da5164e46455fe84287565))

### [0.39.13](https://github.com/maidsafe/sn_routing/compare/v0.39.12...v0.39.13) (2021-01-05)

### [0.39.12](https://github.com/maidsafe/sn_routing/compare/v0.39.11...v0.39.12) (2020-12-29)

### [0.39.11](https://github.com/maidsafe/sn_routing/compare/v0.39.10...v0.39.11) (2020-12-29)

### [0.39.10](https://github.com/maidsafe/sn_routing/compare/v0.39.9...v0.39.10) (2020-12-25)


### Bug Fixes

* avoid over relocation ([989529c](https://github.com/maidsafe/sn_routing/commit/989529cafd1903e9009f4f66b1d111819d89be9c))

### [0.39.9](https://github.com/maidsafe/sn_routing/compare/v0.39.8...v0.39.9) (2020-12-24)


### Features

* set filter number boundary ([c129bff](https://github.com/maidsafe/sn_routing/commit/c129bff69d92400202bcefd1983eb028e3a26155))

### [0.39.8](https://github.com/maidsafe/sn_routing/compare/v0.39.7...v0.39.8) (2020-12-16)


### Bug Fixes

* use age assigned by section ([4db6351](https://github.com/maidsafe/sn_routing/commit/4db63514d4b4f8fe226bc76d97ca33f0e646165a))

### [0.39.7](https://github.com/maidsafe/sn_routing/compare/v0.39.6...v0.39.7) (2020-12-15)


### Bug Fixes

* reject SectionInfo votes not voted by a participant ([c40dc12](https://github.com/maidsafe/sn_routing/commit/c40dc12bb1f64f6b22695c0541479c2dbc26fd8f))

### [0.39.6](https://github.com/maidsafe/sn_routing/compare/v0.39.5...v0.39.6) (2020-12-14)


### Bug Fixes

* handle message send to self ([a1c26ff](https://github.com/maidsafe/sn_routing/commit/a1c26ff62d2dfd0bab12e35b266ce46eee024b77))

### [0.39.5](https://github.com/maidsafe/sn_routing/compare/v0.39.4...v0.39.5) (2020-12-11)

### [0.39.4](https://github.com/maidsafe/sn_routing/compare/v0.39.3...v0.39.4) (2020-12-09)


### Bug Fixes

* **test:** account for relocations in test_startup_section_bootstrapping ([53196a5](https://github.com/maidsafe/sn_routing/commit/53196a5ef8a82a073383f56f1f58ac84dbf28b9f))
* consider also relocated current elders for elder candidates ([fffc946](https://github.com/maidsafe/sn_routing/commit/fffc94696fc82b73711b48c1ba4d83d21e2dd09b))
* do not require resource proof for relocated node + test ([667e1fb](https://github.com/maidsafe/sn_routing/commit/667e1fb156aa1dfad68388f95082d87807898a3f))
* forward ResurceChallenge to the bootstrap task ([2552f06](https://github.com/maidsafe/sn_routing/commit/2552f0631e32a8e442ef1291b537c1ef969bca6d))
* **test:** dont assert new joining node is not instantly relocated ([9a18b4c](https://github.com/maidsafe/sn_routing/commit/9a18b4c0230142dda3a3c64a5fd8aaa0c67fc3b6))

### [0.39.3](https://github.com/maidsafe/sn_routing/compare/v0.39.2...v0.39.3) (2020-12-09)


### Features

* use tracing for logging ([a68af40](https://github.com/maidsafe/sn_routing/commit/a68af409d0700eaf6c25d1ccac65afc0626902d0))

### [0.39.2](https://github.com/maidsafe/sn_routing/compare/v0.39.1...v0.39.2) (2020-12-07)

### [0.39.1](https://github.com/maidsafe/sn_routing/compare/v0.39.0...v0.39.1) (2020-12-03)


### Features

* carry out resource proofing during bootstrap ([a047ca1](https://github.com/maidsafe/sn_routing/commit/a047ca1f88c65cc1d9b99c0602b856bb7acb4f9b))
* relocation during startup no-longer required ([cf937e4](https://github.com/maidsafe/sn_routing/commit/cf937e47bf41cc8b8724e7496f5040e69f95d67e))

## [0.39.0](https://github.com/maidsafe/sn_routing/compare/v0.38.0...v0.39.0) (2020-12-02)


### ⚠ BREAKING CHANGES

*     - remove `Routing::secret_key_share` (use `Routing::sign_with_secret_key_share` instead).
    - Rename `Error::InvalidElderDkgResult` to `Error::MissingSecretKeyShare`
    - `Routing::public_key_set` and `Routing::our_index` now return `MissingSecretKeyShare` instead of `InvalidState` on error.

### Features

* do not expose BLS secret key share ([e8fa12e](https://github.com/maidsafe/sn_routing/commit/e8fa12e4b528ce1e23657c2a2450f48adc3d20de))

## [0.38.0](https://github.com/maidsafe/sn_routing/compare/v0.37.0...v0.38.0) (2020-11-30)


### ⚠ BREAKING CHANGES

* use `use sn_routing::Event;` instead of `use sn_routing::event::Event;`.
* `Event` changes:

- Remove `Event::Connected` - not needed because `Routing::new` now returns fully connected routing instance.
- Add `Event::Relocated` - replaces `Event::Connected(Connected::Relocate)`
- Remove `Event::InfantJoined` - merged with `MemberJoined`
- Change `Event::MemberJoined::previous_name` to `Option` to allow distinguishing between new and relocated peers.
* remove size fields within routing::Config
* remove NetworkParams
* some methods of `Routing` that previosuly returned `Option<T>` or `Result<T>` now return just T.
* rename Instance to Routing
* `Node` and `NodeConfig` are part of the public API.

### Features

* add bootstrap message backlog ([75f0a5c](https://github.com/maidsafe/sn_routing/commit/75f0a5c751835aba15a3cd42ae3b30900f6b1428))
* allow rejoin with same name ([ded038d](https://github.com/maidsafe/sn_routing/commit/ded038d8526246fab6c8a9c63918a74a02a4848e))
* cancel running timers on drop ([d8f420f](https://github.com/maidsafe/sn_routing/commit/d8f420f239ef3c2e0311681f4b620c230326d250))
* expose `Event` directly, hide `event` module ([d940b77](https://github.com/maidsafe/sn_routing/commit/d940b77effde39376b8c7671dbf94f6607ce46ba))
* implement DKG message bouncing ([551c427](https://github.com/maidsafe/sn_routing/commit/551c4276b0c737269716fe05da83fc2b34cfd63c))
* implement lost peer detection ([cbc57ba](https://github.com/maidsafe/sn_routing/commit/cbc57baea9d44637d7439d62872dd8bde0df40b9))
* implement message resending ([cc2fcbd](https://github.com/maidsafe/sn_routing/commit/cc2fcbd163eb80ec85a567b0eb8bc160fc84a312))
* implement proper node termination ([0fbced8](https://github.com/maidsafe/sn_routing/commit/0fbced8a2efaac6be063aee2fb30b8f74f2e7df8))
* improve Comm api and documentation ([9ecfe8a](https://github.com/maidsafe/sn_routing/commit/9ecfe8a5cf949ec741d6cf197930a83515538412))
* joins_allowed flag to toggle accept new node or not ([5def794](https://github.com/maidsafe/sn_routing/commit/5def79408bfe16e37d7455b5c83037415429ce78))
* make the log identifier work again ([48d7ce7](https://github.com/maidsafe/sn_routing/commit/48d7ce79d15f6b7da1cea328980aff835690b4ca))
* make the resend delay configurable ([8a0d043](https://github.com/maidsafe/sn_routing/commit/8a0d043dc4079a4ff677b211c07bc4ffccdf9fdb))
* minor changes to the Event enum ([56e658f](https://github.com/maidsafe/sn_routing/commit/56e658fe6a2fb0b2e1aeac8018f126512c944345))
* notify when key got changed during relocation ([2540a27](https://github.com/maidsafe/sn_routing/commit/2540a27a3aafac61979d6b664e62655796c795ad))
* ping peers on connection loss to detect if they went offline ([d6be64f](https://github.com/maidsafe/sn_routing/commit/d6be64f087341f31838d51dfbdfb067ed24895df))
* relocate all joining infants during startup phase ([492f4d7](https://github.com/maidsafe/sn_routing/commit/492f4d7a5715fe48d1d1757b100fc8ac186ba669))
* relocate one infant with age increased by one when set joins_allowed flag ([03d9827](https://github.com/maidsafe/sn_routing/commit/03d9827e591bf79fa5ecb775801ff8c325109fde))
* **age:** add age getter API ([07430a0](https://github.com/maidsafe/sn_routing/commit/07430a07f5c4772014fc9db7108d3c9404f5702a))
* **comm:** detect lost connections ([f4e9e3a](https://github.com/maidsafe/sn_routing/commit/f4e9e3a00ce5b8905be06d7d6ffa6ea522108466))
* remove resend delay ([9b0971e](https://github.com/maidsafe/sn_routing/commit/9b0971e1aea11b2ada4cc56d70d1d0195631aaad))
* remove Variant::Ping ([18a9d40](https://github.com/maidsafe/sn_routing/commit/18a9d40f9e8a8210b53a00afbe40bada2abcac3f))
* start the first node with higher age ([d23914e](https://github.com/maidsafe/sn_routing/commit/d23914ed998eb415a0e0f7af616eca6bf6ea4333))
* **upnp:** use new version of qp2p with UPnP and echo service ([afb609e](https://github.com/maidsafe/sn_routing/commit/afb609e030acf3002599e2cee14e80f81dae7b21))
* relocate only the oldest peers that pass the relocation check ([d7855b5](https://github.com/maidsafe/sn_routing/commit/d7855b5cf3e18d49517f7f4daac96f0add47a8cf))
* remove join timeout - to be handled by the upper layers instead ([cb4f6fe](https://github.com/maidsafe/sn_routing/commit/cb4f6feb6dc9949e1b865f6c8876d34cfd93322f))
* use unbounded channel for Events ([fb5a3aa](https://github.com/maidsafe/sn_routing/commit/fb5a3aa2eb1af018d82fcdfbe11a9a3b156525b1))
* **api:** expose an async event stream API, and adapt node module to use qp2p async API ([a42b065](https://github.com/maidsafe/sn_routing/commit/a42b065edad3225ccbcad30ed9755e7eff78cd10))
* **node:** cache Connections to nodes ([a78c305](https://github.com/maidsafe/sn_routing/commit/a78c30596400e360b880caafb41a8c94c3bc5b67))


### Bug Fixes

* prevent losing incoming messages during bootstrapping ([3c9357e](https://github.com/maidsafe/sn_routing/commit/3c9357e9cc9d77d5da35df5fb856b08f3ac674b3))
* **dkg:** backlog messages with unknown DKG key ([03873c1](https://github.com/maidsafe/sn_routing/commit/03873c11224d26bf587a4b3366d51e6847b91f06))
* **dkg:** handle delayed DKG outcome ([c58611b](https://github.com/maidsafe/sn_routing/commit/c58611b5bc8343bffef08f3a5464bed3109380f8))
* **dkg:** handle DKG with single participant ([00c2efa](https://github.com/maidsafe/sn_routing/commit/00c2efa6fb042a2e97008713f10a28e9b27a62e7))
* bounce DKG message only if node has no ongoing session ([350b75d](https://github.com/maidsafe/sn_routing/commit/350b75db30fbdec86e14d48ff4f1740be39ddc00))
* clear peer_mgr candidate post pfx change. ([57cd490](https://github.com/maidsafe/sn_routing/commit/57cd490069c961098e3a242fcf439ab2f1631bb5))
* don't ack hop messages in Client state ([9539c05](https://github.com/maidsafe/sn_routing/commit/9539c05f3133a487dd5f0806418283a880eb411e))
* expand ConnInfoReq handling conditions. ([d081800](https://github.com/maidsafe/sn_routing/commit/d0818004f90d5f67e5d03f974967ba8829ae2a6a))
* handle invalid bootstrap response by retuning error ([d5ee338](https://github.com/maidsafe/sn_routing/commit/d5ee338bf79c21d7e136bd8becb84d49fd3a2997))
* lost peer handling ([1d95194](https://github.com/maidsafe/sn_routing/commit/1d95194f7a074d0561a4199cf106cca541af70f4))
* no longer use serde macro derive ([2116420](https://github.com/maidsafe/sn_routing/commit/2116420e2d205499c3c030acafa036df73c9664c))
* Remove old compatible neighbour pfx not restricted to a strict parent/child prefix in Chain on updating neighbour_infos. ([#1579](https://github.com/maidsafe/sn_routing/issues/1579)) ([6d23fa3](https://github.com/maidsafe/sn_routing/commit/6d23fa3390cac5462988ac069e93ad5199dcc57f))
* rename mock/quick_p2p to mock/quic_p2p ([067fab0](https://github.com/maidsafe/sn_routing/commit/067fab09f2e2dcf185dd8bd5987bf8c99c88029d))
* resolve clippy errors of non-mock tests ([94eda60](https://github.com/maidsafe/sn_routing/commit/94eda60e3eae1fd033903038e4271a955c729112))
* resolve failing example ([121ce95](https://github.com/maidsafe/sn_routing/commit/121ce952993ad7e29e055d27b33f164331cd9252))
* send Event::Connected only after transition to Approved ([dbe0593](https://github.com/maidsafe/sn_routing/commit/dbe059361876c09f00323b7eb7fd8d95bcb151ee))
* take ages into account when calculating DkgKey ([824d229](https://github.com/maidsafe/sn_routing/commit/824d2293f17e3d64a6282544556d0ffec3d5e744))
* **comm:** try to re-connect after previously failed send ([08d9410](https://github.com/maidsafe/sn_routing/commit/08d9410b575cfb26f80cc3efe896a73da432f98d))
* **event:** export qp2p SendStream and RecvStream for consumers to use ([65af16f](https://github.com/maidsafe/sn_routing/commit/65af16fd62055999460dd7aeec91b2e0eaab6c68))
* use the latest section key when updating non-elders ([219f98d](https://github.com/maidsafe/sn_routing/commit/219f98d9b3e1a51e5c7eb32fd3857a5de592081f))
* vote for sibling knowledge after parsec reset ([090663f](https://github.com/maidsafe/sn_routing/commit/090663f24dcb165b98d0ccb16b1f5d32614f3b91))


* remove the Routing state machine ([cfa19ff](https://github.com/maidsafe/sn_routing/commit/cfa19ff2151976996d425a3a10e863b03abf6331))
* rename Instance to Routing ([a227e3f](https://github.com/maidsafe/sn_routing/commit/a227e3fe03894545956c7899d8b120b375065281))
* rename Node to Instance and NodeConfig to Config ([d8d6314](https://github.com/maidsafe/sn_routing/commit/d8d63149fce5742af1d2151b91ee974c24ada269))


### api

* remove NetworkParams ([686c248](https://github.com/maidsafe/sn_routing/commit/686c2482358e03b94779c0cde9a61af2b83b6575))
* remove size fields within routing::Config ([9dfb935](https://github.com/maidsafe/sn_routing/commit/9dfb935afd9bdfe4dcc65d37e1cdbb93ac21fa06))

### [0.37.0](https://github.com/maidsafe/sn_routing/compare/v0.36.0...v0.37.0) (2018-08-28)
* Upgrade unwrap version to 1.2.0
* Use rust 1.28.0 stable / 2018-07-07 nightly
* rustfmt 0.99.2 and clippy-0.0.212
* Update license to mention GPLv3 only

### [0.36.0](https://github.com/maidsafe/sn_routing/compare/v0.35.0...v0.36.0) (2018-04-05)
* Use rust 1.24.0 stable / 2018-02-05 nightly
* rustfmt 0.9.0 and clippy-0.0.186

### [0.35.0]
* Use rust 1.22.1 stable / 2017-11-23 nightly
* rustfmt 0.9.0 and clippy-0.0.174

### [0.34.0]
* Fix rounding error in test.
* Depend on Crust 0.29.0.
* Depend on rust_sodium 0.6.0.

### [0.33.2]
* Depend on Crust 0.28.1.

### [0.33.1]
* Increase MAX_MUTABLE_DATA_ENTRIES from 100 to 1000.

### [0.33.0]
* Rate limiter refund on overcharge for get response.
* Rate limiter having soft capacity for clients.
* Bugfix to not mutate peer on receiving conn_info_response.
* Bugfix to remove expired peers(normalise peers) when receiving TunnelSuccess or TunnelRequest.
* Enforce one client per IP only on bootstrap request.
* Add Rand impl for PermissionSet.
* Resend rate exceeded user message parts and remove Event::ProxyRateLimitExceeded
* Bugfix to not reject BootstrapRequest messages invalidly.

### [0.32.2]
* Bugfix to avoid adding nodes to disconnected client list.

### [0.32.1]
* Bugfix to avoid handling recently-disconnected client direct messages.

### [0.32.0]
* Allow mock-crust network to support multiple nodes/clients with the same IPs.
* Allow only one client ip per proxy.
* Modify the client Rate Limiter paradigm to not put an upper threshold on number of clients with unique IPs to bootstrap off a proxy.
* Add dev configuration options for routing such that these options can be supplied in a routing config file that routing would read to tweak parameters such as disable resource proof etc.
* Update to use Rust Stable 1.19.0 / Nightly 2017-07-20, clippy 0.0.144, and rustfmt 0.9.0.
* Make MutableData errors more descriptive.

### [0.31.0]
* Remove support for Structured, PrivAppendable and PubAppendable Data
* Add Support for MutableData instead.
* Introduce mock-crypto which provides efficient mocking of the crypto primitives for faster test runs for cases where we don't care about tightness of security.
* Code rate-limiter to restrict proxy from relaying more than the agreed threshold to the Network on behalf of the clients (each client being identified on IP level).
* Detect malformed messages and invalid/disallowed RPCs and ban such a sender on IP level.

### [0.30.0]
* Replace all sha256 from rust_sodium with sha3_256 from tiny_keccak.
* Move `AccountPacket` type required by vaults and clients into a `common_types` module.

### [0.29.0]
* Integration with templatised Crust where now routing specifies what to use as a UID so that crust and routing use a common UID to identify peer.
* Peer manager clean up as connect success now tells us everything about the peer. Previously we needed to wait additionally for NodeIdentify for instance as crust-uid (PeerId) and routing-uid (PublicId) were separate and each layer informed about the id specific to that layer only.

### [0.28.5]
* Add section update requests to make merges more stable.
* Don't approve new node if routing table is invalid.
* Work around cases where `OtherSectionMerge` would not accumulate.
* Several fixes to tunnel nodes and peer manager.
* Remove more sources of randomness to make tests deterministic.
* Add new error types related to invitation-based account creation.
* Replace rustc-serialize with serde.

### [0.28.4]
* Don't try to reconnect to candidates that are not yet approved.
* Don't include peers in `sent_to` that are not directly connected.
* Use SHA3 everywhere instead of SipHash.
* `PrefixSection` authorities now always refer to all sections _compatible_
  with the given prefix.
* Cache `OwnSectionMerge` messages until both merging sections have sent one.
  Only then update the routing table.
* Cache any routing table updates while merging, and relay them afterwards.
* Other merge and tunnel fixes, and additional tests for merging and tunnels.
* Try to reconnect after a connection drops.

### [0.28.2]
* Extend the tests for tunnelling and the churn tests.
* Fix several peer manager issues related to tunnel nodes and candidates.
* Send `SectionSplit` messages as `PrefixSection` authority to allow resending.
* Fix several issues related to merging sections.
* Some improvements to the log messages.

### [0.28.1]
* Retry preparing connection info if Crust returns an error.

### [0.28.0]
* Profile the bandwidth of nodes joining the network. Reject slow nodes.
* Organise nodes into disjoint sections. Introduce the `Section` and
  `PrefixSection` authorities.
* Maintain lists of signatures of neighbouring sections, which will enable
  secure  message validation.
* Accumulate messages in the sending group/section, then send the messages with
  all the signatures across a single route. This reduces the number of total
  invididual hop messages that need to be sent.
* Routes are now disjoint: Retrying to send a message along a different route
  cannot potentially fail on the same node again.
* Merge the routing_table crate into routing.
* Remove the internal event handling thread. Events should be handled in the
  upper library's event loop. That way, some message passing can be replaced by
  direct calls to routing methods.
* Remove the `PlainData` type which was only used in tests.

### [0.27.1]
* Increase the ID size limit by 10 kB.

### [0.27.0]
* add `NO_OWNER_PUB_KEY` to make data effectively immutable
* disallow that key together with other owners (new error `InvalidOwners`)
* provide API for data chunk size validation (new error `DataTooLarge`)
* support new deletion paradigm for structured data

### [0.26.0]
* Add the public and private appendable data types.
* Allow whitelisting nodes via the crust config file.
* Randomise message handling order in the mock crust tests.

### [0.25.1]
* Fix a panic in ack manager.

### [0.25.0]
* Refactoring: Further split up and reorganise the states and move more logic
  into the peer manager module.
* Several bug fixes and test improvements.

### [0.24.2]
* Refactoring: Turn `Core` into a state machine with `Client` and `Node` states.
  Move some more logic onto the `PeerManager`.
* Fix a bug that caused some nodes to refuse to close an unneeded connection.

### [0.24.1]
* Fix redundant calls to Crust `connect`.

### [0.24.0]
* Fix sodiumoxide to v0.0.10 as the new released v0.0.12 does not support
  rustc-serializable types anymore and breaks builds.
* Avoid redundant hash calculations by making the data `name` method a simple
  getter.
* Fix ack handling when resending a message.
* Some refactoring and test updates.

### [0.23.2]
* Don't cache as a member of recipient group: this can cause redundant
  responses.
* Disconnect previous bootstrap node when retrying to bootstrap.

### [0.23.1]
* Fix tests involving sorting nodes by names.
* Fix random seeds when multiple tests are run at once.

### [0.23.0]
* Add seeded rng support to mock crust tests.
* Add support for response caching.
* Add various mock crust tests.
* Prevent multiple routing nodes from starting on same LAN.

### [0.22.0]
* Migrate to the mio-based Crust.
* Replace redundant group messages by hashes to save bandwidth.
* Split up large messages into 20 kB chunks.
* Improve message statistics; add total message size and count failures.
* Restart with blacklist if the proxy node denied the connection.
* Merge message_filter into routing.
* Some refactoring to clean up the logic in `Core`.
* Several bug fixes.

### [0.21.0]
* Reduce the `XorName` size from 512 to 256 bits.

### [0.20.0]
* Send acknowledgement messages (acks) and resend via a different route only if
  no ack is received. Previously, several routes were used simultaneously,
  wasting a lot of bandwidth.
* Merge xor_name into routing.
* Simplify the message type hierarchy and the API.
* Fix sending redundant connection info.

### [0.19.1]
* network size < GROUP_SIZE will only accept new nodes via first node

### [0.19.0]
* Only start a network if explicitly designated as first node.
* Use a Crust priority based on message type.

### [0.18.5]
* Don't send `Tick` events to clients.
* Use a size limit for the data cache instead of a timeout.
* More detailed message stats logging.

### [0.18.4]
* Allow up to 40 tunnel client pairs.
* Migrate to Crust 0.12.0.
* Add sequence diagrams to the documentation.
* Improve logging.
* Fix several bugs.

### [0.18.3]
* Depend on latest Crust.
* Add the 'Stats' prefix to all statistics log messages.

### [0.18.2]
* Add a periodic tick event.
* Increase the timeout for polling bucket groups.
* Extract the statistics module and gather more statistics.

### [0.18.1]
* Some improvements to the log messages.
* Fix several lint warnings.

### [0.18.0]
* Add the routing table to `NodeAdded` and `NodeLost` events.
* Add `NetworkStartupFailed` and `StartListeningFailed` events.
* Improve join limit to prevent damage to the network in case of many
  simultaneously joining nodes.
* Drop unneeded connections from the routing table.
* Replace node harvesting with periodic bucket polling.

### [0.17.0]
* Depend on Crust 0.11.0.

### [0.16.3]
* Add `HEARTBEAT_ATTEMPTS` constant to configure when an unresponsive peer is considered lost.
* Fix a bug that caused unneeded node harvesting attempts.

### [0.16.2]
* Reduce network traffic by including recipients in hop message that have handled the message.

### [0.16.1]
* Bug fix: DataIdentifier now correctly returns the structured data computed name in its name() function

### [0.16.0]
* Add `identifier()` method to all data elements (type + name)
* All `ImmutableData` types now concrete (not variants)

### [0.15.1]
* Fix a message handling bug.
* Add `MessageId::zero` constructor.
* Always send `NodeAdded` for a new peer, even if not in a common group.

### [0.15.0]
* Implement Rand for mock PeerId.
* Add data name to Put, Post and Delete success responses.

### [0.14.0]
* Add message id to Refresh messages
* Node numbers only increase during node addition in churn for ci_test example
* Update dependencies

### [0.13.0]
* Add tunnel nodes.
* Optimise the `GetNetworkName` message flow for quicker joining.
* Make caching optional.
* Send keepalive signals to detect lost peers.
* Implement full `Put` response flow in the example node.
* Remove digest from success responses; it has been replaced by `MessageId`.
* Migrate to Crust 0.10.0.
* Various bug fixes.

### [0.12.0]
* Make the mock_crust module public

### [0.11.1]
* Send a Disconnected event if client fails to bootstrap.

### [0.11.0]
* Replace CBOR usage with maidsafe_utilites::serialisation.
* Updated dependencies.

### [0.10.0]
* Take `MessageId`s as an argument in the Client methods.

### [0.9.0]
* Add mock Crust and network-less tests for `Core`.
* Return `MessageId`s from Client methods.
* Allow a user to connect to the same proxy node with several clients.

### [0.8.0]
* Send a Disconnected event if the network connection is lost.
* Log disconnecting clients.

### [0.7.1]
* Several bug fixes.

### [0.7.0]
* Migrate to the new Crust API.
* Add some timeouts to check for stale connections.
* Limit proxy connections to one.
* Make node discovery more efficient.
* Shorten log messages and debug formats to make the logs clearer.
* Some updates to churn handling in the example.
* Fix lots of Clippy warnings.
* Fix lots of bugs.

### [0.6.3]
* Added several tests
* Further documentation improvements
* Improved debug output of several types

### [0.6.2]
* Reject clients if the routing table is too small
* Fix computation of remaining required signatures for StructuredData
* Limit the number of concurrently joining nodes
* Remove unneeded files
* Expand documentation
* Distinct message IDs for added and lost nodes
* Ignore double puts in the example

### [0.6.1]
* Update core to send on only first connection

### [0.6.0]
* Further updates to examples
* Moved CI scripts to use Stable Rust

### [0.5.3]
* Getting examples updated
* Updating the API to expose the routing node name and close group

### [0.5.2]
* Bug fix - Blocking InterfaceError not returning
* Changing mutable to immutable for stop() function in routing.rs

### [0.5.1]
* Expose ImmutableDataType

### [0.5.0]
* Cleanup of routing API
* Exposing of success and failure event for GET, PUT, POST and DELETE
* Separating XorName and Routing Table into their own crates

### [0.4.2]
* Remove wildcard dependencies

### [0.4.1] Updated to CRUST 0.4

### [0.4.0] Updated to CRUST 0.3
* [#711](https://github.com/maidsafe/routing/pull/711) remove unneeded state on ::connect
* [MAID-1366](https://maidsafe.atlassian.net/browse/MAID-1366) update routing to crust 0.3 API
* [#369](https://github.com/maidsafe/routing/pull/369) enforce LINT checks

### [0.3.12]
* [MAID-1360](https://maidsafe.atlassian.net/browse/MAID-1360) unit tests for RoutingCore
* [MAID-1357](https://maidsafe.atlassian.net/browse/MAID-1357) unit tests for message and refresh accumulator
* [MAID-1359](https://maidsafe.atlassian.net/browse/MAID-1359) unit tests for Relay
* [MAID-1362](https://maidsafe.atlassian.net/browse/MAID-1362) more unit tests for StructuredData, Types and Utils
* [MAID-1350](https://maidsafe.atlassian.net/browse/MAID-1350) introduce simple measuring tools for establishing the threshold for the accumulators
* [MAID-1348](https://maidsafe.atlassian.net/browse/MAID-1348) ChurnNode for integration tests

### [0.3.11]
* [#699](https://github.com/maidsafe/routing/pull/699) implement debug for StructuredData
* [#696](https://github.com/maidsafe/routing/pull/696) expose NAME_TYPE_LEN and random traits
* [#695](https://github.com/maidsafe/routing/pull/695) correct style error in error.rs
* [#692](https://github.com/maidsafe/routing/pull/692) add cause and event::DoRefresh for improvements to churn
* [#691](https://github.com/maidsafe/routing/pull/691) update QA libsodium documentation
* [#690](https://github.com/maidsafe/routing/pull/690) correct failing test
* [MAID-1361](https://maidsafe.atlassian.net/browse/MAID-1361) unit tests for id, public_id, error, data, direct_messages
* [MAID-1356](https://maidsafe.atlassian.net/browse/MAID-1356) unit test filter.rs
* [MAID-1358](https://maidsafe.atlassian.net/browse/MAID-1358) unit test signed_message

### [0.3.10]
* [#685](https://github.com/maidsafe/routing/pull/685) use latest accumulator

### [0.3.9]
* [MAID-1349](https://maidsafe.atlassian.net/browse/MAID-1349) refresh_request to use authority
* [MAID-1363](https://maidsafe.atlassian.net/browse/MAID-1363) remove wake_up.rs
* [MAID-1344](https://maidsafe.atlassian.net/browse/MAID-1344) ::error::ResponseError::LowBalance
* [MAID-1364](https://maidsafe.atlassian.net/browse/MAID-1364) clean out types.rs
* [#663](https://github.com/maidsafe/routing/issues/663) only churn on QUORUM connected nodes
* [#662](https://github.com/maidsafe/routing/issues/662) enable dynamic caching
* [#670](https://github.com/maidsafe/routing/issues/670) update Travis with ElfUtils
* [#669](https://github.com/maidsafe/routing/issues/669) update Travis with install_libsodium.sh

### [0.3.8]
* [#664](https://github.com/maidsafe/routing/pull/664) update to match Crust's api change

### [0.3.7] Unique signed messages
* [#660](https://github.com/maidsafe/routing/pull/660) Unique SignedMessage with random bits and routing event loop

### [0.3.6]
*  Fixed [#560](https://github.com/maidsafe/routing/issues/560) Removed unstable features.
*  Updated "hello" messages
*  Updated cache-handling in line with current Routing requirements
*  Further work on churn handling

### [0.3.5] improvements to ResponseError and testing

* [#647](https://github.com/maidsafe/routing/pull/647) CI disallow failures on windows x86 (32bit) architecture
* [#646](https://github.com/maidsafe/routing/pull/646) correct ResponseError::HadToClearSacrificial to return NameType and u32 size
* [#645](https://github.com/maidsafe/routing/pull/645) key_value_store to test < Client | ClientManager > < ClientManager | NaeManager > behaviour

### [0.3.4] Improvements to filter and accumulator behavior

* [#642](https://github.com/maidsafe/routing/pull/642) improve filter to block resolved messages
* [#640](https://github.com/maidsafe/routing/pull/640) Enable duplicate get requests

### [0.3.3] Events and refresh

* [#638](https://github.com/maidsafe/routing/pull/638) debug formatting for Data
* [#637](https://github.com/maidsafe/routing/pull/637) our authority API update
* [#626](https://github.com/maidsafe/routing/pull/626) refresh messages
* [#636](https://github.com/maidsafe/routing/pull/636) rustfmt formatting
* [#634](https://github.com/maidsafe/routing/pull/634) rename fob to public_id in routing table
* [#628](https://github.com/maidsafe/routing/pull/628) initial handlers for cache
* [#624](https://github.com/maidsafe/routing/pull/624) remove peers from example CLI, small improvements
* [#620](https://github.com/maidsafe/routing/pull/620) event bootstrapped, connected, disconnected
* [#623](https://github.com/maidsafe/routing/pull/623) maximum allowed size for structured data

### [0.3.2] Final public API for version 0.3

* internal bug fixes
* partial restoration of unit tests
* fine-tuning public API in correspondence with user projects

### [0.3.1] Implementing internal functionality

* [#582](https://github.com/maidsafe/routing/pull/582) implement routing public api channel to routing_node
* [#580](https://github.com/maidsafe/routing/pull/580) review message_received in routing_node
* [#579](https://github.com/maidsafe/routing/pull/579) simplify example to a pure DHT (no client_managers)
* [#578](https://github.com/maidsafe/routing/pull/578) implement connect request and connect response
* [#577](https://github.com/maidsafe/routing/pull/577) implement sending events to user
* [#576](https://github.com/maidsafe/routing/pull/576) implement accumulator as stand-in for sentinel
* [#575](https://github.com/maidsafe/routing/pull/575) temporarily remove sentinel dependency
* [#574](https://github.com/maidsafe/routing/pull/574) fix sodiumoxide problems with Travis CI
* [#573](https://github.com/maidsafe/routing/pull/573) use signature as filter type, deprecating message id
* [#572](https://github.com/maidsafe/routing/pull/572) implement request network name
* [#571](https://github.com/maidsafe/routing/pull/571) refactor example to new api
* [#567](https://github.com/maidsafe/routing/pull/567) implement generic send for signed message
* [#566](https://github.com/maidsafe/routing/pull/566) implement bootstrap connections in core
* [#565](https://github.com/maidsafe/routing/pull/565) implement target nodes in core
* [#564](https://github.com/maidsafe/routing/pull/564) pruning and clean up

### [0.3.0] Unified Data and refactor for channel interface
* [MAID-1158](https://maidsafe.atlassian.net/browse/MAID-1158) Unified Data
    - [MAID-1159](https://maidsafe.atlassian.net/browse/MAID-1159) Implement PlainData
    - [MAID-1160](https://maidsafe.atlassian.net/browse/MAID-1160) Implement ImmutableData
    - [MAID-1163](https://maidsafe.atlassian.net/browse/MAID-1163) Implement StructuredData
    - [MAID-1165](https://maidsafe.atlassian.net/browse/MAID-1165) StructuredData::is_valid_successor
    - [MAID-1166](https://maidsafe.atlassian.net/browse/MAID-1166) Unit Tests for PlainData and ImmutableData
    - [MAID-1167](https://maidsafe.atlassian.net/browse/MAID-1167) Unit Tests for StructuredData
    - [MAID-1168](https://maidsafe.atlassian.net/browse/MAID-1168) Unit Test IsValidSuccessor for StructuredData
    - [MAID-1171](https://maidsafe.atlassian.net/browse/MAID-1171) Implement UnifiedData enum
    - [MAID-1172](https://maidsafe.atlassian.net/browse/MAID-1172) Update with UnifiedData: GetData and GetDataResponse
    - [MAID-1173](https://maidsafe.atlassian.net/browse/MAID-1173) Update with UnifiedData: PutData and PutDataResponse
    - [MAID-1175](https://maidsafe.atlassian.net/browse/MAID-1175) Update with UnifiedData: RoutingMembrane RoutingClient Put and Get
    - [MAID-1176](https://maidsafe.atlassian.net/browse/MAID-1176) Update with UnifiedData: Interfaces and churn
* [MAID-1179](https://maidsafe.atlassian.net/browse/MAID-1179) Implement Post and PostResponse
* [MAID-1170](https://maidsafe.atlassian.net/browse/MAID-1170) Update RoutingClient and relay node: RoutingMessage
* [MAID-1251](https://maidsafe.atlassian.net/browse/MAID-1251) Remove option first from routing node
* [MAID-1255](https://maidsafe.atlassian.net/browse/MAID-1255) RFC 0001 - Use public key for id on all messages
    - [MAID-1256](https://maidsafe.atlassian.net/browse/MAID-1256) Remove redundant field header.source.reply_to
    - [MAID-1257](https://maidsafe.atlassian.net/browse/MAID-1257) Modify Authority enum
* [MAID-1063](https://maidsafe.atlassian.net/browse/MAID-1063) replace MessageTypeTag with full enum.

* [#557](https://github.com/maidsafe/routing/pull/557) channel architecture and simplified message

### [0.2.8] - Version updates and minor fixes

* Updated dependencies' versions
* Fixed lint warnings caused by latest Rust nightly

### [0.2.7] - Activate act on churn

* [#426](https://github.com/maidsafe/routing/pull/426) close bootstrap connection
* [#426](https://github.com/maidsafe/routing/pull/426) routing acts on churn
* [#426](https://github.com/maidsafe/routing/pull/426) group size 8; quorum 6
* [#426](https://github.com/maidsafe/routing/pull/426) improve refresh routing_table
* [#426](https://github.com/maidsafe/routing/pull/426) cache on connect_response
* [#426](https://github.com/maidsafe/routing/pull/426) reflect own group: on FindGroupResponse in our range is seen, ask for FindGroup for our name.

### [0.2.6] - Temporary patch for Vault behaviour

* [#424](https://github.com/maidsafe/routing/pull/424) Patch for Vaults handle put behaviour

### [0.2.1 - 0.2.5] - debug with upper layers

* [0.2.5] [#421](https://github.com/maidsafe/routing/pull/421) Set Authority unauthorised put to ManagedNode to accommodate Vaults for now
* [0.2.4] [#419](https://github.com/maidsafe/routing/pull/419) Correct ClientInterface::HandlePutResponse
* [0.2.3] [#416](https://github.com/maidsafe/routing/pull/416) Activate HandleChurn (but don't act on the resulting MethodCall yet)
* [0.2.2] Update sodiumoxide dependency to `*`
* [0.2.2] Update crust dependency to `*`
* [0.2.1] Update sodiumoxide dependency to `0.0.5`

### [0.1.72] - documentation

* Fix master documentation url in readme
* [#406](https://github.com/maidsafe/routing/pull/406) enable handler for unauthorised put
* [#369](https://github.com/maidsafe/routing/issues/369) clean up unneeded features

### [0.1.71] - Finish Rust-2

* [#360](https://github.com/maidsafe/routing/issues/360) Fix intermittent failure in Relay
* [#372](https://github.com/maidsafe/routing/issues/372) Introduce unit tests for Routing Membrane
* [#388](https://github.com/maidsafe/routing/issues/388) Handle PutDataResponse for routing_client
* [#395](https://github.com/maidsafe/routing/issues/395) Preserve message_id

### [0.1.70] - Activate AccountTransfer

* [#354](https://github.com/maidsafe/routing/issues/354) Fix release builds
* [MAID-1069](https://maidsafe.atlassian.net/browse/MAID-1069) OurCloseGroup Authority
* [#363](https://github.com/maidsafe/routing/issues/363) Refresh message and ad-hoc accumulator
* [#290](https://github.com/maidsafe/routing/issues/290) Remove NodeInterface::handle_get_key
* [#373](https://github.com/maidsafe/routing/issues/373) Reduce group size for QA to 23

### [0.1.64] - bug fixes

* [#330](https://github.com/maidsafe/routing/issues/330) Who-Are-You / I-Am message for identifying new connections
* [#312](https://github.com/maidsafe/routing/issues/312) Fix never-connecting client
* [#343](https://github.com/maidsafe/routing/issues/343) Filter escalating number of connect requests
* [#342](https://github.com/maidsafe/routing/issues/342) Clean up overloaded debug command line printout
* [#347](https://github.com/maidsafe/routing/issues/347) Relay GetDataResponses and cached GetDataResponses back to relayed node

### [0.1.63] - bug fixes

* [#314](https://github.com/maidsafe/routing/issues/314) simple_key_value_store input validation lacking
* [#324](https://github.com/maidsafe/routing/issues/324) simple_key_value_store peer option
* [#336](https://github.com/maidsafe/routing/issues/336) Routing `0.1.62` causes API inconsistency in usage of RoutingClient

### [0.1.62] - restructure core of routing

* [MAID-1037](https://maidsafe.atlassian.net/browse/MAID-1037) Address relocation
  - [MAID-1038](https://maidsafe.atlassian.net/browse/MAID-1038) Integrate handlers with RelayMap
  - [MAID-1039](https://maidsafe.atlassian.net/browse/MAID-1039) put_public_id handler
* [MAID-1052](https://maidsafe.atlassian.net/browse/MAID-1052) Message Handling
  - [MAID-1055](https://maidsafe.atlassian.net/browse/MAID-1055) full review of implementation of handlers
  - [MAID-1057](https://maidsafe.atlassian.net/browse/MAID-1057) make event loop in routing_node internal
* [MAID-1062](https://maidsafe.atlassian.net/browse/MAID-1062) extract all_connections into a module
* [MAID-1070](https://maidsafe.atlassian.net/browse/MAID-1070) drop_bootstrap in coordination with CRUST
* [MAID-1071](https://maidsafe.atlassian.net/browse/MAID-1071) Implement relay id exchange for client node
* [MAID-1066](https://maidsafe.atlassian.net/browse/MAID-1066) Routing Example : update to internal event loop

### [0.1.61] - Relay module, relocatable Id, update NodeInterface

* [MAID-1114](https://maidsafe.atlassian.net/browse/MAID-1114) Relay module
* [MAID-1060](https://maidsafe.atlassian.net/browse/MAID-1060) update Interface for Vaults
* [MAID-1040](https://maidsafe.atlassian.net/browse/MAID-1040) enable Id, PublicId and NodeInfo with 'relocated' name

### [0.1.60] - essential logical corrections
* [MAID-1007](https://maidsafe.atlassian.net/browse/MAID-1007) limit swarm to targeted group
 - [MAID-1105](https://maidsafe.atlassian.net/browse/MAID-1105) delay RoutingTable new ConnectRequests
 - [MAID-1106](https://maidsafe.atlassian.net/browse/MAID-1106) examine Not For Us
* [MAID-1032](https://maidsafe.atlassian.net/browse/MAID-1032)
correct name calculation of pure Id
* [MAID-1034](https://maidsafe.atlassian.net/browse/MAID-1034) ConnectResponse needs to include original signed ConnectRequest
* [MAID-1043](https://maidsafe.atlassian.net/browse/MAID-1043) remove old sentinel
* [MAID-1059](https://maidsafe.atlassian.net/browse/MAID-1059) rename types::Action -> types::MessageAction; rename RoutingNodeAction -> MethodCall

### [0.1.1]
* Remove FailedToConnect Event

### [0.1.0]

* Re-expose crust::Endpoint as routing::routing_client::Endpoint

### [0.0.9]

* Move bootstrap out of routing
* Complete Routing Node Interface to accomodate churn
* Add caching to node interface
* Handle ID Caching
* Handle Cache / Get / Check calls
* Routing message handling
* Sentinel:
  - Handover existing implementation
  - Account transfer merge
  - Group response merge
  - Signature checks
* Check Authority (Ensure use and implementation of Authority is in line with the design doc / blog.)
* Implement unauthorised_put in routing_node and routing_client (this skips Sentinel checks)
* Implement routing connections management
* Added encodable/decodable for ClientIdPacket

Version 0.1.1

### [0.0.7 - 0.0.8]

* Bootstrap handler implementation
* Bootstrap handler test
* Create sort and bucket index methods
* Implement routing table
* Test routing table
* Implement sentinel (initial)
* Finalise sentinel in line with tests
* Implement client node
* Test sentinel
* Implement routing message types (Connect FindNode)
* Test message types
* Implement Get Put Post messages
* Version 0.0.8

### [0.0.6]

* Set up facade design pattern
* Test facade pattern
* Set up accumulator
* Accumulator tests
* Message header
* Message header tests
* API version 0.0.6
