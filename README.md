# Indexed Search

Creates a trigram-based binary index which can then be searched quickly.

### Encoding

- Magic number (0x1337BEEF)
- Number of trigrams (24bit)
- Number of lines (LineNumber - 16bit)
- Array of line byte offsets (ByteOffset - 64bit) - byte offset of each line in the file
- Many 3-byte character sets followed by a `LineNumber` length value and `ByteOffset` offset value. The length and offset value are for an array of `LineNumber`s which will be found later in the file.
- Aforementioned arrays of `LineNumber` length values