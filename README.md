## GitHub Issue Summarizer

Auto-summarizes recent issues from target repositories.

Use your own GitHub issues page as your private notebook to track.

## Usage:
1. Set environment variables (`github_owner`, `github_repo`, `trigger_phrase`), please note these are your own information. `trigger_phrase` should be one word without space.
2. Open an issue with the `target_repo_url` followed by a space, and then the `trigger_phrase`, in the title. `target_repo_url` can also be a shorter form of `some_owner/some_repo`, the `/` makes sure that the bot correctly gets the `owner` and `repo` from your input.
3. The function then posts summaries of recent issues from the target repo as comments.

note: you may use this tool in your private repo to prevent unauthorized users from using your tool or view your information.

