use std::path::PathBuf;

use eyre::Context;
use regex::Regex;

use crate::{
    cli::{ContestArgs, SubmitArgs},
    commands::submit,
    App,
};

pub async fn get_problem_ids_from_html(html_code: &String) -> crate::Result<Vec<String>> {
    let re = Regex::new("contests/[a-zA-Z0-9]*/problems/([a-zA-Z0-9]+)").unwrap();
    let mut results = vec![];
    for cap in re.captures_iter(html_code) {
        println!("Found problem: {}", cap[1].to_string());
        results.push(cap[1].to_string());
    }
    Ok(results)
}

pub async fn get_problem_ids(app: &App, contest_id: String) -> crate::Result<Vec<String>> {
    let res = app.client.get(contest_id).send().await.unwrap();

    if !res.status().is_success() {
        eyre::bail!(
            "Failed to get contest problems from Kattis (http status code: {})",
            res.status()
        )
    }

    let res_body = res
        .text()
        .await
        .wrap_err("Failed to read response from Kattis")?;

    let problems = get_problem_ids_from_html(&res_body).await?;

    //print!("{:?}", res_body);
    Ok(problems)
}

pub async fn contest(app: &App, args: &ContestArgs) -> crate::Result<()> {
    let problems = get_problem_ids(app, args.contest_id.clone()).await?;

    for problem in problems {
        let mut path = PathBuf::new();
        path.push(&args.path);
        path.push(&problem);
        if !path.exists() {
            println!("No folder named {}, found skipping problem", problem);
            continue;
        }

        let submit_args = SubmitArgs {
            path: path.to_path_buf(),
            file: None,
            lang: None,
            yes: args.yes,
            open: false,
        };
        submit(app, &submit_args).await?;
    }

    Ok(())
}
