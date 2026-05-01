# Log

## 04/30/26

- Tested all SRAMs using `BringupState::march_cm_rand_tsi_test_sram_all` (runs March C- and random patterns)
- No errors except for SRAM 21 (8192x32), where reads seem to be delayed by a cycle
