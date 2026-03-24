use crate::state::AppState;
use crate::weather::is_raining;
use crate::webdriver::get_webdriver;
use crate::ws_broadcast;
use model::cargo::Cargo;
use model::news::News;
use model::ws_msg::*;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use utils::db::db_backup;
use utils::runtime::rand_sleep;

fn get_period(job_name: &str) -> Option<&'static str> {
    match job_name {
        "launch_rocket" => Some("every 10 minutes"),
        "ship_cargoes" => Some("every 60 seconds"),
        "send_weather" => Some("every 5 minutes"),
        "fetch_remote_news" => Some("every 6 hours"),
        "backup_database" => Some("every 8 hours"),
        "test_short" => Some("every 20 seconds"),
        "test_long" => Some("every 1 minutes"),
        _ => panic!("Unknown job name"),
    }
}

pub async fn init(app_state: AppState) -> Result<(), JobSchedulerError> {
    let launch_rocket = Job::new_async(get_period("launch_rocket").unwrap(), {
        let app_state = app_state.clone();
        move |_, _| {
            let sender = app_state.ws_sender.clone();
            let pool = app_state.pool.clone();
            Box::pin(async move {
                tracing::info!("Launching rocket");
                let amount = Cargo::launch(&pool).await;
                let msg = WSMsg::launch(amount);
                ws_broadcast(msg, &sender);
            })
        }
    })?;

    let ship_cargoes = Job::new_async(get_period("ship_cargoes").unwrap(), {
        let app_state = app_state.clone();
        move |_, _| {
            let pool = app_state.pool.clone();
            Box::pin(async move {
                tracing::info!("Delivering cargoes");
                Cargo::deliver(&pool).await;
            })
        }
    })?;

    let send_weather = Job::new_async(get_period("send_weather").unwrap(), {
        let app_state = app_state.clone();
        move |_, _| {
            let sender = app_state.ws_sender.clone();
            Box::pin(async move {
                rand_sleep(15000).await;
                tracing::info!("Sending weather");
                let raining = is_raining().await;
                match raining {
                    Ok(raining) => {
                        tracing::info!("Is it raining outside? {raining}.");
                        let msg = WSMsg::weather(raining);
                        ws_broadcast(msg, &sender);
                    }
                    Err(error) => {
                        tracing::error!("Failed to check weather: {error:?}");
                        let msg = WSMsg::weather(false);
                        ws_broadcast(msg, &sender);
                    }
                }
            })
        }
    })?;

    let _fetch_remote_news = Job::new_async(get_period("fetch_remote_news").unwrap(), {
        let app_state = app_state.clone();

        move |_, _| {
            let pool = app_state.pool.clone();
            Box::pin(async move {
                tracing::info!("initializing webdriver");
                let webdriver = get_webdriver(app_state.config.wd_port).await;
                rand_sleep(30000).await;
                tracing::info!("Fetching remote news");
                if let Ok(driver) = webdriver {
                    if let Err(err) = News::fetch_remote(&pool, &driver).await {
                        tracing::error!("Failed to fetch remote news: {err:?}");
                    };
                    if driver.quit().await.is_err() {};
                } else {
                    tracing::error!("Failed to get webdriver {:?}", webdriver.err());
                }
            })
        }
    })?;

    let backup_database = Job::new_async(get_period("backup_database").unwrap(), {
        let app_state = app_state.clone();
        move |_, _| {
            let pool = app_state.pool.clone();
            let root_dir = app_state.config.root_dir.clone();
            Box::pin(async move {
                tracing::info!("Backing up database");
                if let Err(error) = db_backup(
                    &pool,
                    vec!["news", "cargo"],
                    &format!("{root_dir}/backend/db/backups"),
                )
                .await
                {
                    tracing::error!("Failed to backup database: {error}");
                }
            })
        }
    })?;

    // create scheduler
    let sched = JobScheduler::new().await?;

    // add jobs to scheduler
    sched.add(send_weather).await?;
    sched.add(launch_rocket).await?;
    sched.add(ship_cargoes).await?;
    // sched.add(fetch_remote_news).await?;
    sched.add(backup_database).await?;

    // start scheduler
    sched.start().await?;
    Ok(())
}
