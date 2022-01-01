pub(crate) mod git;
pub(crate) mod tarkovdata;

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::git::GRep;
    use crate::tarkovdata::hideout::{Name, Type};
    use crate::tarkovdata::{
        ammunition, hideout, item_presets, items_en, levels, maps, quests, traders,
    };

    #[test]
    fn test_download() {
        let mut tarkovdata =
            GRep::init("https://github.com/TarkovTracker/tarkovdata.git", "master");
        tarkovdata.download_or_update();
    }

    #[test]
    fn test_parse() {
        let mut tarkovdata =
            GRep::init("https://github.com/TarkovTracker/tarkovdata.git", "master");
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

    #[test]
    fn test_chain() {
        let mut tarkovdata =
            GRep::init("https://github.com/TarkovTracker/tarkovdata.git", "master");
        tarkovdata.download_or_update();
        let repo_dir = tarkovdata.get_repo_dir();

        let test_hideout_module = "Intelligence center";
        let level = 2;

        println!("Searching for: {} @ Level {}", test_hideout_module, level);

        let hideout = hideout::from_json(&repo_dir);
        let hideout_module = hideout
            .modules
            .iter()
            .filter(|f| f.module == test_hideout_module && f.level == level)
            .next()
            .expect("Failed to find module");

        let traders = traders::from_json(&repo_dir);
        let items = items_en::from_json(&repo_dir);

        println!("[Requirements]");

        for requirements in &hideout_module.require {
            match requirements.require_type {
                Type::Item => match &requirements.name {
                    Name::Integer(v) => {
                        panic!("Item has integer type name ! {}", v)
                    }
                    Name::String(v) => {
                        let item = items
                            .iter()
                            .filter(|i| &i.1.id == v)
                            .next()
                            .expect("Failed to find item by id");
                        println!(
                            "\t {{Item}} {} @ Level {}",
                            item.1.name, requirements.quantity
                        )
                    }
                },
                Type::Module => match &requirements.name {
                    Name::Integer(v) => {
                        panic!("Module has integer type name ! {}", v)
                    }
                    Name::String(v) => {
                        println!("\t {{Module}} {} @ Level {}", v, requirements.quantity)
                    }
                },
                Type::Skill => match &requirements.name {
                    Name::Integer(v) => {
                        panic!("Skill has integer type name ! {}", v)
                    }
                    Name::String(v) => {
                        println!("\t {{Skill}} {} @ Level {}", v, requirements.quantity)
                    }
                },
                Type::Trader => match &requirements.name {
                    Name::Integer(v) => {
                        let trader = traders
                            .iter()
                            .filter(|t| t.1.id == *v)
                            .next()
                            .expect("Failed to get trader by id");

                        println!("\t {{Trader}} {}", trader.1.name);

                        let trader_req = &trader.1.loyalty[requirements.quantity as usize];
                        println!("\t\t\tLevel:\t{}", trader_req.level);
                        println!(
                            "\t\t\tRequired player level:\t{}",
                            trader_req.required_level
                        );
                        println!(
                            "\t\t\tRequired reputation:\t{}",
                            trader_req.required_reputation
                        );
                        println!(
                            "\t\t\tRequired sales [{}]:\t{}",
                            trader.1.sales_currency, trader_req.required_sales
                        );
                    }
                    Name::String(v) => {
                        panic!("Trader has string type name ! {}", v)
                    }
                },
            }
        }
    }
}
