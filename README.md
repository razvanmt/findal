# findal 

'findal' indexes all your files and makes them easier to find!
![findal-demo](https://github.com/razvanmt/findal/assets/80488375/fada066c-40e9-42c2-b23a-909d7e10f3de)

## Proof of Concept
- [x] windows support for now (developed on windows)
- [x] index all files of a hard-coded drive (for now)
- [x] search for the file path using the Search box
- [x] click the `FileName` to open the file in explorer
- [x] real-time search result behaviour

## Next steps
- [ ] system-wide file indexing (issues with windows permissions when indexing certain directories)
- [ ] better UI
- [ ] ...

## Build
1. Clone the repository locally
2. `npm run tauri build`
3. `npm run tauri dev`
