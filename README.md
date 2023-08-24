# <p align="center">Summarize GitHub Issues with a Comment</p>

<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/createByTemplate/summarize-github--issue">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>


Can't spare the time to peruse lengthy issue posts? Let this bot lend a hand by summarizing the current issue on your GitHub repository and posting a recap as a comment. It could save you time and boost productivity.

![](summarize-github-issue.gif)

## Usage

To use the bot, comment on an issue with the trigger phrase you define. You can also specify the minimum number of comments required on an issue before the bot activates.

If you don't change the template, by default the bot will be triggered when an issue with 5 or more comments receives a comment containing @flows_summarize.

## Deploy on your own repo

1. Create a bot from the template
2. Add your OpenAI API key
3. Configure the bot to summarize issues on a specified GitHub repo

### 0 Prerequisites

You will need to bring your own [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

You will also need to sign into [flows.network](https://flows.network/) using your GitHub account. It is free to join.

### 1 Create a bot from the template

[**Just click here**](https://flows.network/flow/createByTemplate/summarize-github--issue)

Review the `trigger_phrase`  and `n_comments`variables. 
* `trigger_phrase`is the magic words you type in a issue comment to manually activate the bot. The default is `@flows_summarize`.
* `n_comments` specifies the minimum number of comments required on an issue before the bot can activate. The default is `5`.

Click on the **Create and Build** button.

### 2 Add your OpenAI API key
You will now set up OpenAI integration. Click on **Connect**, enter your key and give it a name.

[<img width="450" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">](https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png)

Once added, close the tab and return to flows.network. Click on **Continue**.

### 3 Configure the bot to summarize issues on a specified GitHub repo

Next, you will tell the bot which GitHub repo it needs to monitor for issues to review.

* `github_owner`: The GitHub org that owns the repo *you want to deploy the bot on*.
* `github_repo` : The specific repo *you want the bot to monitor*.

| Name           | Value               |
|----------------|---------------------|
| `github_owner` | WasmEdge            |
| `github_repo`  | wasmedge_hyper_demo |

> For example, to deploy this bot on the `WasmEdge/wasmedge_hyper_demo repo`, you'd set `github_owner` to `WasmEdge` and `github_repo` to `wasmedge_hyper_demo`.


Click **Connect** or **+ Add new authentication** button to grant [flows.network](https://flows.network/) access to the GitHub repo to deploy the 🤖.

[<img width="450" alt="image" src="https://github.com/flows-network/github-pr-summary/assets/45785633/6cefff19-9eeb-4533-a20b-03c6a9c89473">](https://github.com/flows-network/github-pr-summary/assets/45785633/6cefff19-9eeb-4533-a20b-03c6a9c89473)

Once done, close the popup window and return to the flow.network page. Click on **Deploy**.

### Wait for the magic!

This is it! You are now on the flow details page waiting for the flow function to build. As soon as the flow's status became `running`, the bot is ready to summarize issues! It will be summoned by commenting trigger phrase on the issues.



