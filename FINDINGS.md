# Findings of mine

(to report to boomlings.dev github)

## Level Object / Server Level Resource
- Key 9 (`difficultyNumerator` / `difficulty_numerator`) will be `-10` if the level is auto difficulty
- Key 15 (`length`) has value 5 if the level is a platformer level
- Key 16 (`dislikes`) seems to always have value 0
- The decryption/decoding process of the value of key 27 (`password`) is documented, however, this is not well explained in the documentation page for the Server Level Resource.
- Keys 36 (`extraString` / `extra_string`), 62 (`exactUploadTime` / `exact_upload_time`), 63 (`exactUpdateTime` / `exact_update_time`) are NOT returned by the `getGJLevels21.php` endpoint
- Key 41 (`dailyNumber` / `daily_number`) is not returned by `downloadGJLevel22.php` unless `levelID` parameter is "-1", "-2" or "-3"
