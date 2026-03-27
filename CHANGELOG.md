# Changelog

All notable changes to this project will be documented in this file.


## [Unreleased]


### <!-- 0 -->🚀 Features

- just task runner added to project ([7a27ebc](7a27ebc1613dea78e85b741b6a71a64a10f126dc))

- test coverage report now is exclusively in terminal ([d7a1dc9](d7a1dc96fc6ce36cc7fe26a00118641071fd3760))

- add Dosage, PillId, and PillName value objects ([34f584a](34f584ac1b5c7592507a89ccb197cefcc740fcc4))

- add domain module with value objects ([241fa8a](241fa8a680efa17df6f36c08a0b5b4c431baa8c0))

- add domain errors for dosage and pill name validation ([eebe8bd](eebe8bde1743fa1f5c9bbe35d3ad5e35f53e0140))

- impl Pill entity ([4c2be9b](4c2be9bc40d714cd2334124699c11c96a5138f61))

- renamed Pill to Medication and impl DoseRecord ([1650043](165004360efdee5917bbc9538605cafc893d2259))

- rename `Dose` to `DoseRecord` and add invariants ([5bc947f](5bc947fa74b1713f0c59abf5dc86f594cd72b60d))

- define app layer errors ([7830f93](7830f9375cd22c91afa7283ba357bd525a6e11a4))

- value objects for medication schedule and dose records ([03d4ab7](03d4ab7352b39935d4e4eecdc39a3bf125cdeda1))

- move medication creation logic to mapper ([13e9b5b](13e9b5b8a0b7ff902a891a6ef1a2a690591d9489))

- introduce generic Mapper<T> domain port ([43a0552](43a0552cd72659ba83ad7e4a397a5a846dab5c1c))

- add create/update mappers and TryFrom adapters ([1bf4c12](1bf4c129df43a444426caa3748947231ddc2d097))

- add mappers module for mapping between domain and DTOs ([989a8d2](989a8d282f90a6e581fc88f40faac9bde70ecab7))

- resolve data files from ~/.config/bitpill/ ([735913e](735913e43484d863f4f50bc2a9cb5a371046720a))

- enhance coverage report with file-level details and missing lines ([0bbaaad](0bbaaad96d8e617dc4550bc5cb2366dba29f3d58))

- add StockAmount value object ([8ce4961](8ce496168fadd4fc606b63a04a98fe3f78ebd12c))

- impl StockId vo for uniquely identify Stock entities ([57116f9](57116f9ac618c53524b5045780be0c46b8ede7df))

- add MedicationRefill entity to track medication purchase history ([95a15a3](95a15a3d73d811dbdb5a2a86e5307106ba186eba))


### <!-- 1 -->🐛 Bug Fixes

- remove coverage from default task ([125d997](125d9979f498935760e383934abc09ba0351c104))

- fix parsing and comparison of medication form fields ([ba233a7](ba233a723b334a476f4984de0a5c3bc6a3ba60ee))

- fix lints in medication details presenter ([3bbe64f](3bbe64f2622e8ae7ba35676dbecb998d176b6558))

- fix status message expiration logic ([7cade74](7cade74910b07035c5d871edc4fba078c0b3a889))

- correct syntax in match expression for dose_frequency ([6c25a45](6c25a4556c4997cd57cef5ccd8f3c965183beccf))

- now handle "Custom" dose frequency in medication edition ([8c56733](8c567334767614be199f581909f29328c6bd1c51))

- ensure rustup and cargo are properly set up in CI ([b03e3ca](b03e3ca529b402710603e32bc10fd168faf7af92))

- fix cargo llvm-cov version check ([f57af53](f57af5335645986696753dc472d14956d70a971a))

- fix coverage report path normalization for nested src/ directories ([3a8d7a5](3a8d7a5832b43d8d605e751173c1841cc0ba7bae))

- mark as taken check in ui ([8343d70](8343d707f1c4df32a2c1eb7d81aaa8a9242cd5b5))

- tui openning correctlyw ([6d5db3c](6d5db3c17a549f1951af0fc690c7df38b57a15b0))

- default test-helpers ([04fbf54](04fbf542847b4fb21df1ca45d12cca7be6be6086))


### <!-- 2 -->🚜 Refactor

- split medication and dose record repositories, add clock and notification ports ([d7390ff](d7390ff7df603ea45b4dccef09696ead7e1b39ea))

- introduce ApplicationError to unify error handling ([a31018c](a31018c81ddc76bcfe90f2286736034bbfdc984a))

- switch MedicationId to UUID v7 ([dd35905](dd35905e31fe9824ef25927b746a9cd1c4b5cd29))

- switch to time-sortable UUID v7 for identifiers ([e5a47ae](e5a47aed1c6347ed63b901d4deb0c9f318e9e949))

- refactor aplication ports and services ([fefa8c7](fefa8c7010c4991ed39d7e5f155aaf845d00f2cc))

- rename ports to have consistent naming ([5fdcbe3](5fdcbe357586bf850bbb859002262284ff01a626))

- refactor app services ([fa171f0](fa171f0d08dadc2cd086ce49730f38eb83863f74))

- removed infrastructure and presentation implementations ([88e177d](88e177d8b4b93f573e7664fc12ac58a15e6f556f))

- remove presentation layer and main function ([c17f518](c17f518d6d0804f518804d1a7375dea3755d8372))

- add Send + Sync bounds to port trait ([e17c717](e17c7179a3dbc62751a95ada9ccf995474e74b7a))

- remove fake repositories from tests ([5081765](5081765b5c50327005419a1bb10eed76f700965b))

- replace from_uuid constructor with From<Uuid> impl ([49cbebe](49cbebe75ce866353d53682ea3fb6fb625575bcd))

- use fakes for testing application services ([96c15d2](96c15d2a73c7485c0b3c68c8c365e746322ed32e))

- Add DTOs for inbound ports and refactor services to use them ([45069fd](45069fd5d737cc772277f877ae5241f16ac90d99))

- add tests for container and settings repository ([491838e](491838e89461d13903557c0ddcb993112d823398))

- improve test coverage of TUI outer layers by adding integration tests that run the app with a fake in-memory service layer, and refactoring App to allow dependency injection of services and easier testing of the event loop. ([63bbe8d](63bbe8d08b2f0291c87e92618c5bf412e7548b6a))

- remove mark_medication_taken functionality and related code ([e3852b0](e3852b0a5d2b57eb7475b28ccb74466e0ec4abdb))

- remove mark_medication_taken functionality ([4704bbc](4704bbccc912c1559c1b303dc7c4740341424261))

- use Medication::try_from and add service tests ([4903ec8](4903ec8f1a5d89ae7ffb00970524c02b8d69dffa))

- make TUI input handling testable with injectable event source ([b74fb64](b74fb644526dc48e1e4c2373a1c4a4f38589cc32))

- introduce Key enum and from_code helper ([6f0fdb9](6f0fdb93772e39a88feb719f6f712e717a9b9e8b))

- make event source injectable to decouple tests from TTY ([7316ab3](7316ab35e29a13f8383d8ccfbf59170642118337))

- replace crossterm KeyEvent with Key in all handlers ([4223b55](4223b557252daa24ac4f49d4b288a49d1efc92bc))

- extract form state struct and helper methods for create/edit handlers ([8b1427f](8b1427fd773c0dcb043324ff0c6fa3c4641fadeb))

- responses to a single file ([edf104a](edf104a8f46187fd1347daddd432b5cf968f964b))

- move req dtos to single file ([076dbec](076dbec12d3acc0681b67c9f7f4f1769933f34f5))

- adjusting project to release ([651d16d](651d16d6b3797c41e65e008819684f1eda86ca72))


### <!-- 3 -->📚 Documentation

- update README with more context about the project ([16b8d49](16b8d49242d1c2df0af2d0ab4bff3691b4fd37e1))

- update example to clarify wiring ([5c534a4](5c534a42d93061c9d7184e54c104c0e09af31b7e))

- add detailed documentation for all domain entities, application services, and ports ([20f6a17](20f6a1785d52ed2a1b51736f35845669dfdc0229))

- update port and testing guidelines ([9780233](9780233f5d0e33ec6afc642af32f044c335d948d))

- update documentation to reflect current state of the project ([2d5234e](2d5234e3211bda41dc25577aca54289e966b81f8))

- update documentation with TUI keyboard shortcuts ([78e4802](78e4802f39b9b3dedf02902062d27b7a0711f855))

- add instructions for Copilot CLI agent ([509dfab](509dfabf764bf74dca01f757424146a9fb1328f0))

- clarify testing guidelines and fake usage ([a208f3f](a208f3f05563cec5f68b2de7e9b60206d80faad7))

- update overview with note about Copilot CLI instructions ([7cb164e](7cb164eb6890c15a0cd504a5f25c2fdd8100b95d))

- add comments as docs to setup.sh ([41ef524](41ef5248c27f3be227dfd22ec3967b0a509c37b6))

- docs files to rest appear as wip ([b83ee85](b83ee851a2d9759d183093c854270dfbcb25e05b))

- update README to include screenshots ([5b28f11](5b28f1124c3895b90da93852a798844913c10dbc))

- remove a rest-specific readme line ([173c84f](173c84f092070aea2a4ba4cd4ef38c7d67fddf89))

- defined license ([b0efe2d](b0efe2d5f9513dd3addd5bf086af9001fa9f970e))

- updating docs dir ([692127d](692127d6e1a7eb5304f7145333f0271e006ed80c))


### <!-- 5 -->🎨 Styling

- fix lint issues across multiple modules ([734b11a](734b11a719133e504ca08dcace08c53acc16bd12))

- fix ([51fbb0b](51fbb0b7b7690edb3c6081b8bc151f48fd478d36))

- fix lints in medication form component ([1dd763c](1dd763ca444a3d063c4b86ad0bc749a446972bea))

- move tests into service files and remove additional_tests.rs ([2dc6761](2dc67618f3ef9d489778ff314b2f0d2edb6ada0c))

- improve architecture and test coverage ([6bf09b3](6bf09b368e6674cfedfc979d455a6bfb5f935fa9))

- reformat code and fix clippy warnings ([24c68e1](24c68e1ab035ade7e77685a9a244e9f19d445172))

- remove unused import ([1ac91a7](1ac91a7f607531fb017482a90182275836aac5fb))

- define path is now defined a single time in the setup script ([f10e991](f10e991300c80ad783d8997290b52f5e3e5dde4e))

- apply cargo fmt to infrastructure and integration tests ([ef48f7e](ef48f7ebe255030bc4d4f9b33d08978dab816cfa))

- improve imports style ([1b97caf](1b97caf728680b07047eb7db70bff704bcd84898))


### <!-- 6 -->🧪 Testing

- add fake implementations for testing application services ([2b72c96](2b72c96fe77ce185759de6b3e3d1176796ac26f6))

- Add infrastructure integration tests ([1eb4463](1eb44630c78e76f4bdd56ad5e7333cf81ea05267))

- Add presentation E2E tests ([cee9629](cee9629ea513241fda222a86623328c1b0e6e689))

- deleted unused test files ([6d476cd](6d476cd8a8cb3a1411da4e50525dac04de21461e))

- improve test coverage for domain entities and value objects ([79f9002](79f90021409f6aff2e49ac00449b8c426ee90dea))

- add helper functions to create test medications ([e1b8c93](e1b8c93e870dce141a062c82ff4ea90b056f4dc7))

- move service tests to `tests/application/services` ([730e9eb](730e9eb828f47fca7ce97fa4fb9226fa752297cc))

- improve test coverage in application layer ([f2e1078](f2e1078a8665d614cb5e8327172a08f4a96f4d98))

- wire harness and fix stale dormant tests ([5366f66](5366f669dba1ba92e571bf7d7d1f6aa2c9108160))

- add AppInitializer integration tests ([d8c997b](d8c997badc90cae57771fd17754dc71c3d78a5dc))

- update integration tests to use Key instead of crossterm KeyEvent ([2af6d0a](2af6d0a0b642e6e7bda828e2e5c54b62e5410973))


### <!-- 7 -->⚙️ Miscellaneous Tasks

- add initial project structure and copilot instructions ([e73bc96](e73bc96b5104e2bda0dfd78010a2f9982223c056))

- started work on bitpill ([64a403c](64a403cc110a26114331a38433282fb01197e934))

- copilot-instructions updated ([b7db2b7](b7db2b7b3862200748bceaab64df91192686f8fa))

- update instructions for coverage command ([558252e](558252ebe95129f8da23fb54e84c20bb58e10943))

- update copilot instructions with new architecture and testing guidelines ([96a0b8c](96a0b8c37a8f10224c08957c000c8084d0a90402))

- update Cargo.lock after refactoring application ports ([c11ebd3](c11ebd3dd67da9a63458371df0053deec82765d1))

- ignore fakes in coverage ([6af4672](6af467272767edd2e3c0cbb2f985bae47fa62b80))

- removed test-one and run-both ([ad2e088](ad2e0880b0ff74fe23c14920f65a21fb3a121edc))

- add commit message check workflow ([2ea378b](2ea378bf079bc1a275f62c676df5cb23954b9a46))

- add fake inbound ports for testing ([cc5a80e](cc5a80e6fbd36003a47ef519638b5acede572782))

- add *.info to ignore list ([e6bf407](e6bf407af136576f8ce354bcb153e18b85cc12dc))

- remove Cargo.lock ([86d56d2](86d56d2dc23ecf8f3bdda46e1e21df67d2b4ed80))

- update Cargo.lock ([1ee1850](1ee1850d06b109edc4fc9153c04ba81129f26d8c))

- remove old test files that have been moved to the tests directory ([f0399f4](f0399f48a104d9ba353c454c95f295aa565b65f6))

- apply cargo fmt across application and domain layers ([eeb1e93](eeb1e93d8f3fdde08cccb67f6e9401bd2c1b2962))

- removing github actions ([8ef79d8](8ef79d897fdcde9363d07baf9e8613e7d93f55da))

- add GitHub Actions workflow for commit message and branch name checks ([d694079](d6940791ec4feab170df786204b720480933ec7b))

- add CI workflow for tests and coverage ([a6f0b89](a6f0b8968d2d0980828a1f8632374f2e2b5c8b47))

- add coverage check script ([9b00254](9b0025461f5b6dc005677bd1975cd679dbf40cfb))

- improve test runner ([f80dba0](f80dba019394ae80dd63ae4f1ef8963cc125bca3))

- update commit message check workflow ([2895650](2895650aac4016e9c9bc43efa59089724f438ad3))

- add setup and coverage check scripts ([7668c87](7668c87125836ca92c714a6966204677ce534ac6))

- test command is now using check_coverage.sh script ([810ac9d](810ac9d9b7f4f3c96baea7737fe5262fe7e79c1f))

- update commit-check and run-tests workflows ([b5cd096](b5cd096d4adadd6f19086c8809d4b69788dead9e))

- removed .ci/setup.sh as it's no longer needed ([ffcc830](ffcc830c5fc2b65d75e24645562a34f06b44e74d))

- yodate workflows for Forgejo compatibility ([6db25b6](6db25b62daf60232b619026c2b88e804d8f60c19))

- fix forgejo actions ([21b316f](21b316f46ff3024526133608d56d9f715b703568))

- forgejo actions is now using just toolsm ([3a0a938](3a0a9386e4c17154c776022a46b326410a72bfff))

- trigger forgejo actions check ([acbf5b2](acbf5b27f6b649f8caff2a368d63bdf206477406))

- add lint-workflows recipe and document local validation ([ad1cae7](ad1cae7b4efd4df91f3bdb31fd5e41c8269e450e))

- extract workflow bash logic into scripts/ ([f9ff342](f9ff34251fbde3f1680dece572e0da5fde4f2f7a))

- split workflows into focused jobs and wire scripts ([037d169](037d169d526d0d13384b61dddfe4444eb5484077))

- improve commit modularization and readability ([ea8d662](ea8d6624eb0cf30061fe5c4421e93d7b9de81d13))

- improving check_commit_messages.sh to allow merge commits ([828b828](828b828cf5b2f524b8c47fe9761e17c53606e805))

- update setup script to remove temporary directory after installation ([bbae751](bbae751cce4d167cdba721ec43d3aca5cd0f1ebf))

- update CI workflow triggers ([cdbcbd6](cdbcbd6f4ecb66c4551982606fde62456431e54c))

- add test script for running tests by file path or module filter ([1c3031c](1c3031c400ecda3aa32fb359e4b772223ba4f1e9))

- completing cargo ([4b908a5](4b908a5565b0a941c5bc04f5675de6747ab8800d))

- fix lock file ([f31e799](f31e79994102d51a7cf47dec08d9489cccee545f))

- adjusting cargo and justfile to release ([fdfc875](fdfc8753781bd162cc734f4a15290d44d329781e))

- ci/cd workflows ([5c1ebce](5c1ebce1e44ac59c06029adba1df87b54181bae4))

### Compare
