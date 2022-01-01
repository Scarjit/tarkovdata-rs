use crate::git::GRep;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::Once;

pub mod ammunition;
pub mod hideout;
pub mod item_presets;
pub mod items_en;
pub mod levels;
pub mod maps;
pub mod quests;
pub mod traders;

static ONCE: Once = Once::new();
static REPO_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut tarkovdata = GRep::init("https://github.com/TarkovTracker/tarkovdata.git", "master");
    ONCE.call_once(|| {
        tarkovdata.download_or_update();
    });
    let repo_dir = tarkovdata.get_repo_dir();
    repo_dir
});
