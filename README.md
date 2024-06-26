# ligotab
## Tables in a few formats

ligotab is library and command line tool for formatting delimiter-separated values (CSV, etc.) as lightweight markup tables in a few formats. So far, basic [Markdown](https://www.markdownguide.org/extended-syntax/#tables), [Confluence Wiki Markup](https://confluence.atlassian.com/doc/confluence-wiki-markup-251003035.html), and [Org](https://www.gnu.org/software/emacs/manual/html_node/org/Tables.html) tables.  ligotab is a work in progress.

The command line tool, `lgt`, can read from standard input or from a single file.

### Command Line Usage
```sh
Usage: lgt [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to delimiter-separated value file [default: -]

Options:
  -d, --delimiter <DELIMITER>          Delimiter character. Expand escape characters in the shell, e.g., `$'\t'` [default: ,]
  -o, --output-format <OUTPUT_FORMAT>  Output format for the table. Valid formats are `markdown`, `confluence`, and `org` [default: markdown]
  -h, --help                           Print help
  -V, --version                        Print version
```
