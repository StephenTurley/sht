// This module will implement the Save command
// To Save a snapshot, it will do the following
// Locate the files that have been changed by:
//      1. Ignore any files/dirs in .shtignore
//      2. Add any files/dirs not in the index
//      3. Remove any files/dirs that are in index but not the working dirs
//      5. Add any files/dirs that have changed from what is in the index (compare timestamp and hash)



