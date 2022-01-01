pub(crate) mod git;
pub(crate) mod tarkovdata;

#[cfg(test)]
mod tests {
    use serde_json::map;
    use crate::git::GRep;
    use crate::tarkovdata::{ammunition, hideout, item_presets, items_en, levels, maps, quests, traders};

    #[test]
    fn test_download() {
        let mut tarkovdata = GRep::init("https://github.com/TarkovTracker/tarkovdata.git", "master");
        tarkovdata.download_or_update();
    }

    #[test]
    fn test_parse(){
        let mut tarkovdata = GRep::init("https://github.com/TarkovTracker/tarkovdata.git", "master");
        tarkovdata.download_or_update();
        let repo_dir = tarkovdata.get_repo_dir();

        let _ammo = ammunition::from_json(&repo_dir);
        let _hideout = hideout::from_json(&repo_dir);
        let _item_presets = item_presets::from_json(&repo_dir);
        let _items_en = items_en::from_json(&repo_dir);
        let _levels = levels::from_json(&repo_dir);
        let _maps = maps::from_json(&repo_dir);
        let _quests = quests::from_json(&repo_dir);
        let _traders = traders::from_json(&repo_dir);
    }
}
