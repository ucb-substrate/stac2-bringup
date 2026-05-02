# Log

## 05/02/26

- Ran on-chip BIST at 25 MHz clock frequency on a simple march pattern. All SRAMs pass except SRAM 21:

    ```rs
    >> l.basic_bist_tsi_test_sram(21)
    BistResult {
        fail: true,
        fail_cycle: 8195,
        expected: 0,
        received: 1736377928,
        signature: 799956904,
    }
    ```

## 04/30/26

- Tested all SRAMs using `BringupState::march_cm_rand_tsi_test_sram_all` at 25 MHz clock (runs March C- and random patterns)
- No errors except for SRAM 21 (8192x32), where reads seem to be delayed by a cycle
