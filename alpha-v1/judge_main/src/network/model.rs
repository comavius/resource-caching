#[derive(serde::Deserialize, serde::Serialize)]
pub struct JudgeRequest {
    pub test_id: uuid::Uuid,
    pub code_id: String,
    pub input_id: String,
    pub expected_id: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ExecJobDirective {
    pub hook_id: uuid::Uuid,
    pub args
}