use nonparallel::nonparallel;
use tokio::sync::Mutex;

static MUT_A: Mutex<()> = Mutex::const_new(());

/// This test executes inside the runtime and the async wrapping,
/// and thus it can do `.await`.
/// The downside is runtime will be executing even after the mutex is unlocked,
/// which is not sound, as things that were inteded to be under the mutex can
/// still be executing until the runtime shuts down.
#[nonparallel(MUT_A.lock().await)]
#[tokio::test]
async fn inside_rt() {}

/// This test executes around the runtime and can't do `.await`. It must call
/// the `blocking_lock()` instead.
/// However, it is guaranteed to properly lock the runtime.
#[tokio::test]
#[nonparallel(MUT_A.blocking_lock())]
async fn around_rt() {}
