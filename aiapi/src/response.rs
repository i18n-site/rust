use reqwest::StatusCode;
use sonic_rs::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
  #[error("{msg}\n{err}")]
  DecodeError { msg: String, err: sonic_rs::Error },
  #[error("{status}: {url}: {msg}")]
  RequestError {
    status: StatusCode,
    url: String,
    msg: String,
  },
}

// 定义响应结构体
#[derive(Debug, Deserialize)]
pub struct OpenAiResponse {
  pub id: String,
  // pub object: String,
  // pub created: u64,
  // pub model: String,
  pub choices: Vec<Choice>,
  pub usage: Usage,
  // pub system_fingerprint: String,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
  pub index: usize,
  pub message: Message,
  pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct Message {
  pub role: String,
  pub content: String,
}

#[derive(Debug, PartialEq, Eq, strum_macros::Display)]
pub enum FinishReason {
  /// 生成的文本达到了预设的停止标记（stop tokens）。
  Stop,
  /// 生成的文本达到了最大长度限制。
  Length,
  /// 生成的文本被内容过滤器拦截，可能包含不适当的内容。
  ContentFilter,
  /// 生成的文本包含了一个函数调用。
  FunctionCall,
  /// 生成的文本没有明确的结束原因，可能是由于其他原因中断的。
  Null,
  Other(String),
}

#[derive(Debug, Deserialize)]
pub struct Usage {
  pub prompt_tokens: usize,
  pub completion_tokens: usize,
  pub total_tokens: usize,
}

#[derive(Debug)]
pub struct Response {
  pub usage: Usage,
  pub finish_reason: FinishReason,
  pub txt: String,
}

impl From<OpenAiResponse> for Response {
  fn from(response: OpenAiResponse) -> Self {
    let mut finish_reason = FinishReason::Null;
    let txt: String = response
      .choices
      .into_iter()
      .map(|c| {
        let s = c.finish_reason.as_str();
        finish_reason = match s {
          "stop" => FinishReason::Stop,
          "length" => FinishReason::Length,
          "content_filter" => FinishReason::ContentFilter,
          "function_call" => FinishReason::FunctionCall,
          "null" => FinishReason::Null,
          _ => FinishReason::Other(s.into()),
        };
        c.message.content
      })
      .collect::<Vec<_>>()
      .join("\n");

    Self {
      usage: response.usage,
      finish_reason,
      txt,
    }
  }
}
