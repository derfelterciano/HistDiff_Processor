# HistDiff Processesor: A Local HistDiff pipeline

### **by Derfel Terciano**

## What this is

This is a localized version of the full HistDiff pipeline for Cytological Profiling.
The idea here is to make it easier for labs to process their own data locally.
This means no web servers, no middle man for data, just pure HistDiff processing
at your computer any where!

## Planned features:

- Heat map visualization!
  - See your HistDiff ASAP!
- Score distribution
  - Visualize the distribution of the HistDiff scores.
- Quality control
  - Visualize the activity levels of your controls.
  - Identify any striping or plate effects in assay plate.
  - Re-run histdiff if any striping is seen on plates.
  - See PCA loadings of controls and experiment.
- Once satisifed with results, users can export their experiements.

- _Planned fun feature:_
  - _Multiplate processing_

## TO-DO:

- [ ] Implement user input/config
  - [x] File selection
  - [x] Control definitions
    - [x] create a pop out to select negative controls
    - [x] construct plate assay replica
    - [x] implement option for 96-well plate (optional)
    - [x] Add functionality for persistent well selection state
    - [ ] TODO: fix styling
  - [x] Additional meta information to ignore
  - [x] Well name column
    - [ ] Implement drop down option
      - Use the following: [carbon components](https://svelte.carbondesignsystem.com/)
- [x] Implement Rust to Svelte API
  - [x] Rewrite HistDiff input to "mimick" histdiff binary
    - RUST SIDE IMPLEMENTATION
      - [x] Implement formatter to make data acceptable
      - **Alogrithm was re-written to histdiff_core. The input is raw cell data, processing is done on the fly**
      - ~~[ ] Output formatted data to a temp file~~
        - ~~(Alternatively, try and hold data into memory and pass it into HD?)
          This doesn't seem like a good idea but this the only way i can think of to not
          produce a temp file~~
      - ~~[ ] Modify rust HD to accept temp file OR modify HD to take in raw DataStruct~~
- [x] Implement logging
  - [x] Implement logging in rust processes
  - [x] Add global emit and listening for logging
  - [x] Prevent main UI interaction when process is running
  - [x] Implement persistent logging
  - [x] Modify rust hd core to use infos instead of traces
- [x] Implement User Output functionality
  - [x] Store the histdiff output globally in the rust backend or use a different approach
  - [x] Implement UI for user to output HistDiff data .
    - [x] Call a file dialogue for users to choose where to output data
- [ ] Implement visualization and visual analysis
  - [ ] Implement Heatmap view
    - Use new clustering library
    - [ ] Integrate clustering library to app's rust library
      - [x] Convert json tree into D3 compliant json tree
      - [x] Implement dendrogram
      - [ ] Create ability to safely transpose dataframes so both rows and cols are clustered
        - [ ] This needs to be an option if users want to see features clustered
  - [ ] Implement Score distribution
  - [ ] Implement positive control score distribution

## Major Bugs:

- [x] HistDiff panicks at only 1 cntrl well selected.
  - **_ISSUE:_** modify UI to format the control and well names to remove any leading zeroes.
  - [x] Format well control input so histdiff doesn't panic.
- [ ] Address Error handling
  - **_ISSUE:_** emit a separate error flag to soft reset app state
  - [ ] Implement error emits
