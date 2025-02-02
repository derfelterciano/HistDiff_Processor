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
  - [ ] Control definitions
    - [ ] create a pop out to select negative controls
    - [ ] construct plate assay replica
    - [ ] implement option for 96-well plate (optional)
  - [x] Well name column
    - [ ] Implement drop down option
- [ ] Implement Rust to Svelte API
  - [ ] Rewrite HistDiff input to "mimick" histdiff binary
