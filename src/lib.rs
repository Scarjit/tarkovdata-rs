pub(crate) mod git;
pub(crate) mod tarkovdata;

#[cfg(test)]
mod tests {
    use crate::git::GRep;
    
    use crate::tarkovdata::{
        ammunition, hideout, item_presets, items_en, levels, maps, quests, traders,
    };
    use crate::tarkovdata::hideout::{Name, Type};
    


    #[test]
    fn test_download() {
        let hideout = hideout::from_json();
        let _ = hideout
            .modules
            .iter()
            .filter(|s| s.module == "Air Filtering Unit")
            .next()
            .unwrap();
    }

    #[test]
    fn test_parse() {
        let mut tarkovdata =
            GRep::init("https://github.com/TarkovTracker/tarkovdata.git", "master");
        tarkovdata.download_or_update();
        let _repo_dir = tarkovdata.get_repo_dir();

        let _ammo = ammunition::from_json();
        let _hideout = hideout::from_json();
        let _item_presets = item_presets::from_json();
        let _items_en = items_en::from_json();
        let _levels = levels::from_json();
        let _maps = maps::from_json();
        let _quests = quests::from_json();
        let _traders = traders::from_json();
    }


    #[test]
    fn test_chain() {
        let _tarkovdata =
            GRep::init("https://github.com/TarkovTracker/tarkovdata.git", "master");

        let test_hideout_module = "Intelligence center";
        let level = 2;

        println!("Searching for: {} @ Level {}", test_hideout_module, level);

        let hideout = hideout::from_json();
        let hideout_module = hideout
            .modules
            .iter()
            .filter(|f| f.module == test_hideout_module && f.level == level)
            .next()
            .expect("Failed to find module");

        let _traders = traders::from_json();
        let items = items_en::from_json();

        println!("[Requirements]");

        for requirement in &hideout_module.require {
            match requirement.require_type {
                Type::Item => {
                    match &requirement.name {
                        Name::String(s) => {
                            match items.iter().filter(|p| &p.1.id == s).next(){
                                None => {
                                    panic!("Item not found ! {}", requirement.id)
                                }
                                Some(v) => {
                                    println!("\t{{Item}} {}x{}", requirement.quantity,v.1.name);
                                }
                            }
                        }
                        Name::Trader(_) => {
                            panic!("Trader as item")
                        }
                    }
                }
                Type::Module => {
                    match &requirement.name {
                        Name::String(s) => {
                            println!("\t{{Module}} {} @ Level {}", s,requirement.quantity);
                        }
                        Name::Trader(_) => {
                            panic!("Trader as module")
                        }
                    }
                }
                Type::Skill => {
                    match &requirement.name {
                        Name::String(s) => {
                            println!("\t{{Skill}} {} @ Level {}", s,requirement.quantity);
                        }
                        Name::Trader(_) => {
                            panic!("Trader as skill")
                        }
                    }

                }
                Type::Trader => {
                    match &requirement.name {
                        Name::String(_) => {
                            panic!("String as trader")
                        }
                        Name::Trader(t) => {
                            println!("\t{{Trader}} {} @ Level {}", t.name, requirement.quantity);
                        }
                    }
                }
            }
        }
    }

}
