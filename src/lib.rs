use dotenv::dotenv;
use flowsnet_platform_sdk::logger;
use github_flows::{
    get_octo, listen_to_event, octocrab::models::events::payload::IssueCommentEventAction,
    EventPayload, GithubLogin,
};
use openai_flows::{
    chat::{ChatModel, ChatOptions},
    OpenAIFlows,
};
use std::env;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    dotenv().ok();
    logger::init();

    let owner = env::var("github_owner").unwrap_or("juntao".to_string());
    let repo = env::var("github_repo").unwrap_or("test".to_string());

    listen_to_event(
        &GithubLogin::Default,
        &owner,
        &repo,
        vec!["issue_comment"],
        |payload| handler(&owner, &repo, payload),
    )
    .await;

    Ok(())
}

async fn handler(owner: &str, repo: &str, payload: EventPayload) {
    let trigger_phrase = env::var("trigger_phrase").unwrap_or("@flows_summarize".to_string());
    let n_comments = env::var("n_comments").unwrap_or("5".to_string());
    let n_comments = n_comments.parse::<i64>().unwrap_or(5);

    let octocrab = get_octo(&GithubLogin::Default);
    let issue_handle = octocrab.issues(owner, repo);

    let mut summary = String::new();
    if let EventPayload::IssueCommentEvent(e) = payload {
        if e.action != IssueCommentEventAction::Created {
            return;
        }
        let trigger_person = e.comment.user.login;
        match e.comment.body {
            Some(body) => {
                if !body.starts_with(&format!("{trigger_phrase}")) {
                    std::process::exit(1);
                }
            }
            None => std::process::exit(1),
        }
        let comment_id = e.comment.id;
        let mut openai = OpenAIFlows::new();
        openai.set_retry_times(2);
        let issue_creator_name = e.issue.user.login;
        let issue_title = e.issue.title;
        let issue_number = e.issue.number;
        let issue_html_url = e.issue.html_url;
        let issue_body = match e.issue.body {
            Some(body) => squeeze_fit_remove_quoted(&body, "```", 500, 0.6),
            None => "".to_string(),
        };
        let labels = e
            .issue
            .labels
            .iter()
            .map(|lab| lab.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        let mut all_text_from_issue = format!(
            "User '{}', opened an issue titled '{}', labeled '{}', with the following post: '{}'.",
            issue_creator_name, issue_title, labels, issue_body
        );
        match issue_handle
            .list_comments(issue_number)
            .per_page(100)
            .page(1u32)
            .send()
            .await
        {
            Ok(comments_page) => {
                let count = comments_page.items.len();
                if count < n_comments as usize {
                    _ = issue_handle
                    .update_comment(
                        comment_id,
                        "Please kindly be patient enough to read an issue with less than 5 comments!",
                    )
                    .await;
                    return;
                }
                for comment in comments_page.items {
                    let comment_body = match &comment.body {
                        Some(body) => squeeze_fit_remove_quoted(body, "```", 300, 0.6),
                        None => "".to_string(),
                    };
                    let commenter = &comment.user.login;
                    let commenter_input = format!("{} commented: {}", commenter, comment_body);

                    all_text_from_issue.push_str(&commenter_input);
                }

            }

            Err(_e) => log::error!("Error getting comments from issue: {}", _e),
        };

        let all_text_from_issue = squeeze_fit_post_texts(&all_text_from_issue, 12_000, 0.4);
        let sys_prompt_1 = &format!(
                    "Given the information that user '{issue_creator_name}' opened an issue titled '{issue_title}', your task is to deeply analyze the content of the issue posts. Distill the crux of the issue, the potential solutions suggested."
                );
        let co = match all_text_from_issue.len() > 12000 {
            true => ChatOptions {
                model: ChatModel::GPT35Turbo16K,
                system_prompt: Some(sys_prompt_1),
                restart: true,
                temperature: Some(0.7),
                max_tokens: Some(192),
                ..Default::default()
            },
            false => ChatOptions {
                model: ChatModel::GPT35Turbo,
                system_prompt: Some(sys_prompt_1),
                restart: true,
                temperature: Some(0.7),
                max_tokens: Some(128),
                ..Default::default()
            },
        };
        let usr_prompt_1 = &format!(
                    "Analyze the GitHub issue content: {all_text_from_issue}. Provide a concise analysis touching upon: The central problem discussed in the issue. The main solutions proposed or agreed upon. Aim for a succinct, analytical summary that stays under 128 tokens."
                );
        match openai
            .chat_completion(&format!("issue_{issue_number}"), usr_prompt_1, &co)
            .await
        {
            Ok(r) => summary = r.choice,
            Err(_e) => log::error!("Error generating issue summary #{}: {}", issue_number, _e),
        }

        let resp = format!(
            "{}\n{}\n{}\n
            this result is generated by flows.network. Triggered by @{}",
            issue_title, issue_html_url, summary, trigger_person
        );
        // issues.update_comment(pull_number, resp).await.unwrap();
        match issue_handle.update_comment(comment_id, resp).await {
            Err(error) => {
                log::error!("Error posting resp: {}", error);
            }
            _ => {}
        }
    }
}

pub fn squeeze_fit_remove_quoted(
    inp_str: &str,
    quote_mark: &str,
    max_len: u16,
    split: f32,
) -> String {
    let mut body = String::new();
    let mut inside_quote = false;

    for line in inp_str.lines() {
        if line.contains(quote_mark) {
            inside_quote = !inside_quote;
            continue;
        }

        if !inside_quote {
            let cleaned_line = line
                .split_whitespace()
                .filter(|word| word.len() < 150)
                .collect::<Vec<&str>>()
                .join(" ");
            body.push_str(&cleaned_line);
            body.push('\n');
        }
    }

    let body_words: Vec<&str> = body.split_whitespace().collect();
    let body_len = body_words.len();
    let n_take_from_beginning = (body_len as f32 * split) as usize;
    let n_keep_till_end = body_len - n_take_from_beginning;

    let final_text = if body_len > max_len as usize {
        let mut body_text_vec = body_words.to_vec();
        let drain_start = n_take_from_beginning;
        let drain_end = body_len - n_keep_till_end;
        body_text_vec.drain(drain_start..drain_end);
        body_text_vec.join(" ")
    } else {
        body
    };

    final_text
}

pub fn squeeze_fit_post_texts(inp_str: &str, max_len: u16, split: f32) -> String {
    let bpe = tiktoken_rs::cl100k_base().unwrap();

    let input_token_vec = bpe.encode_ordinary(inp_str);
    let input_len = input_token_vec.len();
    if input_len < max_len as usize {
        return inp_str.to_string();
    }
    let n_take_from_beginning = (input_len as f32 * split).ceil() as usize;
    let n_take_from_end = max_len as usize - n_take_from_beginning;

    let mut concatenated_tokens = Vec::with_capacity(max_len as usize);
    concatenated_tokens.extend_from_slice(&input_token_vec[..n_take_from_beginning]);
    concatenated_tokens.extend_from_slice(&input_token_vec[input_len - n_take_from_end..]);

    bpe.decode(concatenated_tokens)
        .ok()
        .map_or("failed to decode tokens".to_string(), |s| s.to_string())
}
