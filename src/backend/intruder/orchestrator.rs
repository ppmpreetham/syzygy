// actor pattern

use tokio::sync::{mpsc, oneshot};

enum Message {
    Update(Action),
    GetStatus(oneshot::Sender<AttackStatus>),
}

enum Action {
    Start,
    Pause,
    Resume,
    Cancel,
}

enum AttackStatus {
    Pending,
    Running,
    Paused,
    Completed(RequestResult),
    Cancelled,
}

// TODO
struct RequestResult {
//     todo!();
}
