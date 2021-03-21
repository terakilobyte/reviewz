use anyhow::Result;
use octocrab::{models, Octocrab};

#[tokio::main]
async fn main() -> Result<()> {
    let gh = env!("GITHUB_API_TOKEN").to_string();
    let octocrab = Octocrab::builder().personal_token(gh).build()?;
    let mut current_page = octocrab
        .pulls("mongodb", "docs-java")
        .list_comments(59)
        .await?;

    // dbg!(&current_page.items);

    let mut comments = current_page.take_items();

    while let Ok(Some(mut new_page)) = octocrab.get_page(&current_page.next).await {
        comments.extend(new_page.take_items());
        for comment in comments.drain(..) {
            println!("{}", comment.user.login);
        }
    }

    // while let Ok(Some(mut new_page)) = octocrab.get_page(&current_page.next).await {
    // prs.extend(new_page.take_items());
    //
    // for pr in prs.drain(..) {
    // let pr_num = pr.number;
    // for comment in octocrab
    // .pulls("mongodb", "docs-node")
    // .list_comments(pr_num)
    // .await?
    // {
    // println!(
    // "originator: {}, reviewer: {}",
    // pr.user.login, comment.user.login
    // );
    // }
    // }
    //
    // current_page = new_page;
    // }

    Ok(())
}
