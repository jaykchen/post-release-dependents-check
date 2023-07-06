use flowsnet_platform_sdk::write_error_log;
use github_flows::{
    get_octo, listen_to_event, octocrab::models::events::payload::WorkflowRunEventAction,
    EventPayload, GithubLogin,
};

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
        handler,
    )
    .await;
}

async fn handler(payload: EventPayload) {
    let octocrab = get_octo(&GithubLogin::Default);
    // let owner_repo_workflow_id_branch_obj = vec![
    //     ("second-state", "microservice-rust-mysql", "ci.yml", "main"),
    //     ("second-state", "wasmedge-quickjs", "examples.yml", "main"),
    // ];

    // https://github.com/jaykchen/hacker-news-lambda/blob/main/.github/workflows/placeholder.yml
    let owner_repo_workflow_id_branch_obj =
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

                        for (owner, repo, workflow_id, branch) in owner_repo_workflow_id_branch_obj
                        {
                            let _ = octocrab
                                .actions()
                                .create_workflow_dispatch(owner, repo, workflow_id, branch)
                                .inputs(serde_json::json!({ "inputs": {
                                    "logLevel": "info"
                                }}))
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
