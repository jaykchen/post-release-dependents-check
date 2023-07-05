use chrono::{DateTime, Utc};
use flowsnet_platform_sdk::write_error_log;
use github_flows::{
    get_octo, listen_to_event, octocrab::models::events::payload::WorkflowRunEventAction,
    EventPayload, GithubLogin,
};
use serde::Deserialize;
use slack_flows::send_message_to_channel;
use std::fmt::format;
use store_flows::{get, set};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let owner = "jaykchen";
    let repo = "chatgpt-private-test";

    listen_to_event(
        &GithubLogin::Default,
        &owner,
        &repo,
        vec!["workflow_run"],
        |payload| handler(&owner, &repo, payload),
    )
    .await;
}

async fn handler(owner: &str, repo: &str, payload: EventPayload) {
    let octocrab = get_octo(&GithubLogin::Default);

    let success = "success".to_string();
    match payload {
        EventPayload::WorkflowRunEvent(e) => {
            if e.action == WorkflowRunEventAction::Completed {
                let workflow_run = e.workflow_run;
                match workflow_run.conclusion {
                    Some(success) => {
                        let name = workflow_run.name;
                        let run_number = workflow_run.run_number;
                        // https://github.com/second-state/microservice-rust-mysql/blob/main/.github/workflows/ci.yml
                        // https://github.com/second-state/wasmedge-quickjs/blob/main/.github/workflows/examples.yml
                        run_workflow("second-state", "microservice-rust-mysql", "ci.yml", "main")
                            .await;
                        run_workflow("second-state", "wasmedge-quickjs", "examples.yml", "main")
                            .await;

                        // let title = format!("{conclusion} executing {name} run #{run_number}");
                    }
                    None => return,
                }
            }
        }
        _ => return,
    };
}

// https://github.com/jaykchen/hacker-news-lambda/blob/main/.github/workflows/placeholder.yml

pub async fn run_workflow(owner: &str, repo: &str, workflow: &str, branch: &str) {
    let octocrab = get_octo(&GitHubLogin::Default);

    match octocrab
        .actions()
        .create_workflow_dispatch(owner, repo, workflow, branch)
        .inputs(serde_json::json!({ "inputs": {
            "logLevel": "info"
        }}))
        .send()
        .await
    {
        Ok(_) => {}
        Err(_e) => {}
    }
}

pub fn save_workflow_run_number(owner: &str, repo: &str, number: u64) -> bool {
    // let key = format("{owner}-{repo}");

    // match get(key) {
    //     Some(v) => {}
    //     None => {
    //         set(key, number);
    //     }
    // }
    true
}

/*
curl -L \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer <YOUR-TOKEN>"\
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/OWNER/REPO/actions/workflows/WORKFLOW_ID/runs

 */

pub async fn is_workflow_runs_success() -> anyhow::Result<bool> {
    let octocrab = get_octo(&GithubLogin::Default);
    // let route = format!("repos/{owner}/{repo}/actions/workflows/{workflow_id}/runs");
    let route = format!("repos/jaykchen/chatgpt-private-test/actions/workflows/rust.yml/runs");

    let res: WorkflowRunPayload = octocrab.get(route, None::<&()>).await?;
    let is_dipatch_event = res.workflow_runs[0].event == "workflow_dispatch";
    let is_completed = res.workflow_runs[0].status == "completed";
    let is_success = res.workflow_runs[0].conclusion == "success";
    Ok(is_dipatch_event && is_completed && is_success)
}

#[derive(Deserialize, Debug)]
pub struct WorkflowRun {
    id: u64,
    name: String,
    path: String,
    display_title: String,
    run_number: u32,
    event: String,
    status: String,
    conclusion: String,
    workflow_id: u64,
    url: String,
    html_url: String,
    pull_requests: Vec<String>, // Change this type based on your pull request structure
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct WorkflowRunPayload {
    total_count: u32,
    workflow_runs: Vec<WorkflowRun>,
}
