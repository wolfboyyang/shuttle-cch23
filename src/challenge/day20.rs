use axum::{response::IntoResponse, routing::post, Router};
use bytes::Buf;
use git2::Repository;
use tar::Archive;

pub fn task() -> Router {
    Router::new()
        .route("/archive_files", post(count_archive_files))
        .route("/archive_files_size", post(get_archive_files_size))
        .route("/cookie", post(find_cookie))
}

async fn count_archive_files(body: axum::body::Bytes) -> impl IntoResponse {
    Archive::new(body.reader())
        .entries()
        .unwrap()
        .inspect(|file| {
            let file = file.as_ref().unwrap();
            println!(
                "path:{} size:{}",
                file.path().unwrap().display(),
                file.size()
            );
        })
        .count()
        .to_string()
}

async fn get_archive_files_size(body: axum::body::Bytes) -> impl IntoResponse {
    Archive::new(body.reader())
        .entries()
        .unwrap()
        .map(|file| file.unwrap().size())
        .sum::<u64>()
        .to_string()
}

async fn find_cookie(body: axum::body::Bytes) -> impl IntoResponse {
    let temp_dir = tempfile::tempdir().unwrap();
    Archive::new(body.reader()).unpack(temp_dir.path()).unwrap();

    let repo = Repository::open(temp_dir.path()).unwrap();
    let branch = repo
        .find_branch("christmas", git2::BranchType::Local)
        .unwrap();

    let head_commit = branch.get().peel_to_commit().unwrap();

    let mut commit = head_commit;
    while commit.parent_count() > 0 {
        let mut find_cookie = false;
        commit
            .tree()
            .unwrap()
            .walk(git2::TreeWalkMode::PreOrder, |_, entry| {
                if entry.name() == Some("santa.txt")
                    && std::str::from_utf8(
                        entry.to_object(&repo).unwrap().as_blob().unwrap().content(),
                    )
                    .unwrap()
                    .contains("COOKIE")
                {
                    find_cookie = true;
                    git2::TreeWalkResult::Abort
                } else {
                    git2::TreeWalkResult::Ok
                }
            })
            .unwrap();
        if find_cookie {
            break;
        }

        commit = commit.parent(0).unwrap();
    }

    format!("{} {}", commit.author().name().unwrap(), commit.id())
}
