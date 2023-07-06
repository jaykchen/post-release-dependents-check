use flowsnet_platform_sdk::write_error_log;
use github_flows::{
    get_octo, listen_to_event, octocrab::models::events::payload::WorkflowRunEventAction,
    EventPayload, GithubLogin,
};
use dotenv::dotenv;
use std::env;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();
    let github_owner = env::var("github_owner").unwrap_or("jaykchen".to_string());
    let github_repo = env::var("github_repo").unwrap_or("chatgpt-private-test".to_string());

    listen_to_event(
        &GithubLogin::Default,
        &github_owner,
        &github_repo,
        vec!["workflow_run"],
        handler,
    )
    .await;
}

async fn handler(payload: EventPayload) {
    let octocrab = get_octo(&GithubLogin::Default);
    // let owner_repo_workflow_id_branch_vec = vec![
    //     ("second-state", "microservice-rust-mysql", "ci.yml", "main"),
    //     ("second-state", "wasmedge-quickjs", "examples.yml", "main"),
    // ];

    // https://github.com/jaykchen/hacker-news-lambda/blob/main/.github/workflows/placeholder.yml
    let owner_repo_workflow_id_branch_vec =
        vec![("jaykchen", "hacker-news-lambda", "placeholder.yml", "main")];

    let _success = "success".to_string();
    match payload {
        EventPayload::WorkflowRunEvent(e) => {
            if e.action == WorkflowRunEventAction::Completed {
                let workflow_run = e.workflow_run;
                match workflow_run.conclusion {
                    Some(_success) => {
                        // https://github.com/second-state/microservice-rust-mysql/blob/main/.github/workflows/ci.yml
                        // https://github.com/second-state/wasmedge-quickjs/blob/main/.github/workflows/examples.yml

                        for (owner, repo, workflow_id, branch) in owner_repo_workflow_id_branch_vec
                        {
                            let _ = octocrab
                                .actions()
                                .create_workflow_dispatch(owner, repo, workflow_id, branch)
                                .inputs(serde_json::json!({ 
                                    "logLevel": "info"}))
                                .send()
                                .await;
                        }
                    }
                    None => return,
                }
            }
        }
        _ => return,
    };
}
