# ID-Ranges

`grep -rih "regionId" . | tr -d '[:blank:][:alpha:][:punct:]' | sort | uniq`

- `UnitID` -> 1 to 144
- `OperationID` -> 1 to 113
- `RegionID`
    - 10_000_000 to 11_000_000 -> New Eden regions
    - 11_000_000 to 12_000_000 -> Wormhole regions
    - 12_000_000 to 13_000_000 -> Abyssal regions
