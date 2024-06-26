# ligotab
## Tables in a few formats

ligotab is library and command line tool for formatting delimiter-separated values (CSV, etc.) as lightweight markup tables in a few formats. So far, basic [Markdown](https://www.markdownguide.org/extended-syntax/#tables), [Confluence Wiki Markup](https://confluence.atlassian.com/doc/confluence-wiki-markup-251003035.html), and [Org](https://www.gnu.org/software/emacs/manual/html_node/org/Tables.html) tables are supported. The command line tool, `lgt`, can read from standard input or from a single file.

ligotab is an experimental work in progress in desperate need of automated testing.

### Command Line Usage
```
Usage: lgt [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to delimiter-separated value file [default: -]

Options:
  -d, --delimiter <DELIMITER>          Set the delimiter character. Expand escape characters in the shell, e.g., `$'\t'` [default: ,]
  -t, --terminator <TERMINATOR>        (Optional) Set the record terminator character
  -c, --comment-char <COMMENT_CHAR>    (Optional) Set the file commenting character
  -q, --quoting                        Disable quoting when reading file
  -u, --quote-char <QUOTE_CHAR>        Set the quoting character [default: "]
  -b, --double-quote                   Disable interpreting double quote as escape
  -o, --output-format <OUTPUT_FORMAT>  Set the output format for the table. Valid formats are `markdown`, `confluence`, and `org` [default: markdown]
  -h, --help                           Print help
  -V, --version                        Print version
```
