use directories::ProjectDirs;
use git2::Repository;
use std::fs::create_dir_all;
use std::path::PathBuf;
use xxhash_rust::xxh3::xxh3_64;

pub struct GRep {
    pub url: String,
    pub repo: Option<Repository>,
    pub branch: String,
}

impl GRep {
    pub fn init(url: &str, branch: &str) -> Self {
        let mut grep = Self {
            url: url.to_string(),
            repo: None,
            branch: branch.to_string(),
        };

        let repo_dir = grep.get_repo_dir();
        if repo_dir.join(".git").exists() {
            grep.repo = Some(Repository::open(repo_dir).expect("Failed to open existing repo"));
        }

        grep
    }

    pub fn get_repo_dir(&self) -> PathBuf {
        let project_dir =
            ProjectDirs::from("rs", "Scarjit", "tarkovdata").expect("Failed to get projectdir");
        let data_dir = project_dir.data_dir();
        let repo_dir = data_dir.join(xxh3_64(self.url.as_bytes()).to_string());
        repo_dir
    }

    fn download(&self) -> Repository {
        let repo_dir = self.get_repo_dir();
        create_dir_all(&repo_dir).expect("Failed to create repository dir");
        let r = Repository::clone(&self.url, repo_dir).expect("Failed to download repo");
        r
    }

    pub fn download_or_update(&mut self) {
        match &self.repo {
            None => self.repo = Some(self.download()),
            Some(r) => {
                r.find_remote("origin")
                    .expect("Failed to get remote/origin")
                    .fetch(&["master"], None, None)
                    .expect("Failed to fetch");

                let fetch_head = r
                    .find_reference("FETCH_HEAD")
                    .expect("Failed to find FETCH_HEAD reference");
                let fetch_commit = r
                    .reference_to_annotated_commit(&fetch_head)
                    .expect("Failed to get annotated commits from ref");
                let analysis = r
                    .merge_analysis(&[&fetch_commit])
                    .expect("Failed to execute merge analysis");
                if analysis.0.is_up_to_date() {
                    return;
                } else if analysis.0.is_fast_forward() {
                    let refname = format!("refs/heads/{}", self.branch);
                    let mut reference = r
                        .find_reference(&refname)
                        .expect("Failed to find reference");
                    reference
                        .set_target(fetch_commit.id(), "Fast-Forward")
                        .expect("Failed to set target");
                    r.set_head(&refname).expect("Failed to set head");
                    r.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
                        .expect("Failed to checkout head");
                } else {
                    panic!("Can't fast-forward");
                }
            }
        }
    }
}
