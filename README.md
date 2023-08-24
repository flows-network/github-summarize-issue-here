# <p align="center">Summarize GitHub issues by an issue comment</p>

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


Can't afford the time to go through long issue posts? Let the bot help you by summarizing the current issue on the GitHub repo and post it as a comment. It could save your time and improve effiency.


## Usage

To use the bot, mention it in a comment on an issue with the trigger phrase you define. You can also define the minimum number of comments of an Issue to trigger the bot to work.

If you don't make any changes, the bot will be triggered when the issue with 5 and above comments receives the comment with @flows_summarize.

## How to deploy on your own repo

1. Create a bot from a template
2. Add your OpenAI API key
3. Configure the bot to summarize issues on a specified GitHub repo

### 0 Prerequisites

You will need to bring your own [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

You will also need to sign into [flows.network](https://flows.network/) from your GitHub account. It is free.

### 1 Create a bot from a template

[**Just click here**](https://flows.network/flow/createByTemplate/summarize-github--issue)

Review the `trigger_phrase`  and `n_comments`variable. 
* `trigger_phrase`is the magic words you type in a issue comment to manually summon the bot. Default is `@flows_summarize`.
* `n_comments` specifies the minimum number of comments of an Issue to trigger the bot to work. Default is `5`.

Click on the **Create and Build** button.

### 2 Add your OpenAI API key
You will now set up OpenAI integration. Click on **Connect**, enter your key and give it a name.

[<img width="450" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">](https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png)

Close the tab and go back to the flow.network page once you are done. Click on **Continue**.

### Configure the bot to summarize issues on a specified GitHub repo

Next, you will tell the bot which GitHub repo it needs to monitor for issues to review.

* `github_owner`: GitHub org for the repo *you want to deploy the 🤖 on*.
* `github_repo` : GitHub repo *you want to deploy the 🤖 on*.

> Let's see an example. You would like to deploy the bot to summarize issues on `WasmEdge/wasmedge_hyper_demo` repo. Here `github_owner = WasmEdge` and `github_repo = wasmedge_hyper_demo`.

Click on the **Connect** or **+ Add new authentication** button to give the function access to the GitHub repo to deploy the 🤖. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to the repo.

[<img width="450" alt="image" src="https://github.com/flows-network/github-pr-summary/assets/45785633/6cefff19-9eeb-4533-a20b-03c6a9c89473">](https://github.com/flows-network/github-pr-summary/assets/45785633/6cefff19-9eeb-4533-a20b-03c6a9c89473)

Close the tab and go back to the flow.network page once you are done. Click on **Deploy**.

### Wait for the magic!

This is it! You are now on the flow details page waiting for the flow function to build. As soon as the flow's status became `running`, the bot is ready to give issue summary! The bot is summoned by magic words (i.e., `trigger_phrase`) in issue comments.




