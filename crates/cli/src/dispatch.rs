use std::collections::BTreeMap;
use std::env;
use std::path::{Path, PathBuf};

use acvc_app::{RecordRequest, SeedRequest};
use acvc_domain::{Direction, EdgeKind, Error, Hash, MetricSpec, Result};

use crate::args::{CampaignCommand, Cli, Command, Format};
use crate::init_at;
use crate::locate::{from_repo, from_workspace};
use crate::present::{self, emit};

pub fn dispatch(cli: Cli) -> Result<()> {
    let format = cli.format;
    let repo = cli.repo.as_deref();
    match cli.command {
        Command::Init => {
            let path = repo.unwrap_or(Path::new(".")).to_path_buf();
            let app = init_at(&path)?;
            emit(
                format,
                serde_json::json!({ "repo": app.repo() }),
                format!("initialized {}", app.repo().display()),
            );
        }
        Command::Campaign {
            action: CampaignCommand::Create { name, metrics },
        } => {
            let app = from_repo(repo)?;
            let schema = parse_schema(&metrics)?;
            app.create_campaign(&name, &schema)?;
            let labels: Vec<String> = schema
                .iter()
                .map(|spec| format!("{}:{}", spec.name, spec.direction.as_str()))
                .collect();
            emit(
                format,
                serde_json::json!({ "campaign": name, "metrics": labels }),
                format!("campaign {name}"),
            );
        }
        Command::Campaign {
            action: CampaignCommand::List,
        } => {
            let app = from_repo(repo)?;
            let names = app.campaigns()?;
            let mut schemas = Vec::new();
            for name in &names {
                let metrics = app.schema(name)?.into_iter().map(|spec| {
                    serde_json::json!({
                        "name": spec.name,
                        "direction": spec.direction.as_str(),
                    })
                });
                schemas.push(serde_json::json!({ "campaign": name, "metrics": metrics.collect::<Vec<_>>() }));
            }
            emit(
                format,
                serde_json::json!({ "campaigns": names, "schemas": schemas }),
                names.join("\n"),
            );
        }
        Command::Seed {
            campaign,
            path,
            premise,
            assumptions,
            intent,
            agent,
        } => {
            let app = from_repo(repo)?;
            let campaign = sole_campaign(&app, campaign)?;
            let (value, text) = present::seed(&app.seed(SeedRequest {
                campaign,
                path,
                claim: premise,
                assumptions,
                intent,
                agent,
            })?);
            emit(format, value, text);
        }
        Command::Materialize { variant, into } => {
            let app = from_repo(repo)?;
            let id = app.resolve(&variant)?;
            let (value, text) = present::materialize(&app.materialize(&id, &into)?);
            emit(format, value, text);
        }
        Command::Status { workspace } => {
            let workspace = workspace.unwrap_or(cwd()?);
            let app = from_workspace(repo, &workspace)?;
            let (value, text) = present::status(&app.status(&workspace)?);
            emit(format, value, text);
        }
        Command::Record {
            workspace,
            edge,
            premise,
            assumptions,
            revises,
            intent,
            agent,
            parents,
        } => {
            let workspace = workspace.unwrap_or(cwd()?);
            let app = from_workspace(repo, &workspace)?;
            let report = app.record(RecordRequest {
                workspace,
                edge: EdgeKind::parse(&edge)?,
                claim: premise,
                assumptions,
                revises: optional_hash(&app, revises)?,
                intent,
                agent,
                parents: resolve_all(&app, &parents)?,
            })?;
            let (value, text) = present::record(&report);
            emit(format, value, text);
        }
        Command::Score { variant, metrics } => {
            let (app, id) = here(repo, variant)?;
            let (value, text) = present::score(&app.score(&id, &parse_scores(&metrics)?)?);
            emit(format, value, text);
        }
        Command::Reject { variant, reason } => {
            let (app, id) = here(repo, variant)?;
            app.reject(&id, &reason)?;
            emit(
                format,
                serde_json::json!({ "variant": id.to_hex(), "rejected": true }),
                format!("rejected {}", id.to_hex()),
            );
        }
        Command::Compare { left, right } => {
            let app = from_repo(repo)?;
            let (value, text) =
                present::compare(&app.compare(&app.resolve(&left)?, &app.resolve(&right)?)?);
            emit(format, value, text);
        }
        Command::FileDiff { left, right, path } => {
            let app = from_repo(repo)?;
            let left_id = app.resolve(&left)?;
            let right_id = app.resolve(&right)?;
            let diff = app.file_diff(&left_id, &right_id, &path)?;
            let (value, text) = present::file_diff(&left_id, &right_id, &path, &diff);
            emit(format, value, text);
        }
        Command::Frontier { campaign } => {
            let app = from_repo(repo)?;
            let campaign = sole_campaign(&app, campaign)?;
            let (value, text) = present::frontier(&campaign, &app.frontier(&campaign)?);
            emit(format, value, text);
        }
        Command::Recall { campaign, query } => {
            let app = from_repo(repo)?;
            let campaign = sole_campaign(&app, campaign)?;
            let (value, text) = present::recall(&app.recall(&campaign, &query)?);
            emit(format, value, text);
        }
        Command::Why { variant } => {
            let (app, id) = here(repo, variant)?;
            let (value, text) = present::why(&app.why(&id)?);
            emit(format, value, text);
        }
        Command::Backtrack { variant, into } => {
            let app = from_repo(repo)?;
            let (value, text) =
                present::backtrack(&app.backtrack(&app.resolve(&variant)?, into.as_deref())?);
            emit(format, value, text);
        }
        Command::Pack => {
            let app = from_repo(repo)?;
            let (value, text) = present::pack(&app.pack()?);
            emit(format, value, text);
        }
        Command::Serve { bind } => crate::remote::listen(&from_repo(repo)?, &bind, format)?,
        Command::Push {
            to,
            variant,
            campaign,
        } => {
            let app = from_repo(repo)?;
            let id = app.resolve(&variant)?;
            let campaign = sole_campaign(&app, campaign)?;
            let report = crate::remote::push_variant(&app, &to, &id, &campaign)?;
            let (value, text) = present::transfer(&report);
            emit(format, value, text);
        }
        Command::Fetch {
            from,
            variant,
            campaign,
        } => {
            let app = from_repo(repo)?;
            let id = Hash::parse(&variant)?;
            let campaign = sole_campaign(&app, campaign)?;
            let report = crate::remote::fetch_variant(&app, &from, &id, &campaign)?;
            let (value, text) = present::transfer(&report);
            emit(format, value, text);
        }
        Command::Promote { variant, dest, git } => promote(format, repo, &variant, dest, git)?,
        Command::Tui => acvc_tui::run(&from_repo(repo)?)?,
        Command::Mcp => crate::mcp::serve()?,
        Command::Web { bind } => crate::web::listen(repo, &bind, format)?,
    }
    Ok(())
}

fn promote(
    format: Format,
    repo: Option<&Path>,
    variant: &str,
    dest: Option<PathBuf>,
    git: Option<PathBuf>,
) -> Result<()> {
    let app = from_repo(repo)?;
    let id = app.resolve(variant)?;
    let report = match (dest, git) {
        (Some(dest), None) => app.promote_dest(&id, &dest)?,
        (None, Some(dest)) => app.promote_git(&id, &dest)?,
        _ => return Err(Error::new("promote needs exactly one of --dest or --git")),
    };
    let (value, text) = present::promote(&report);
    emit(format, value, text);
    Ok(())
}

fn parse_schema(raw: &[String]) -> Result<Vec<MetricSpec>> {
    raw.iter()
        .map(|item| {
            let (name, direction) = item
                .split_once(':')
                .ok_or_else(|| Error::new("metric must be name:direction").ctx("metric", item))?;
            Ok(MetricSpec {
                name: name.to_string(),
                direction: Direction::parse(direction)?,
            })
        })
        .collect()
}

fn parse_scores(raw: &[String]) -> Result<BTreeMap<String, f64>> {
    let mut scores = BTreeMap::new();
    for item in raw {
        let (name, value) = item
            .split_once('=')
            .ok_or_else(|| Error::new("score must be name=value").ctx("metric", item))?;
        let parsed = value
            .parse::<f64>()
            .map_err(|_| Error::new("metric value is not a number").ctx("metric", item))?;
        scores.insert(name.to_string(), parsed);
    }
    Ok(scores)
}

fn optional_hash(app: &acvc_app::App, value: Option<String>) -> Result<Option<Hash>> {
    value.map(|prefix| app.resolve(&prefix)).transpose()
}

fn resolve_all(app: &acvc_app::App, prefixes: &[String]) -> Result<Vec<Hash>> {
    prefixes.iter().map(|prefix| app.resolve(prefix)).collect()
}

fn here(repo: Option<&Path>, variant: Option<String>) -> Result<(acvc_app::App, Hash)> {
    if let Some(prefix) = variant {
        let app = from_repo(repo)?;
        let id = app.resolve(&prefix)?;
        return Ok((app, id));
    }
    let workspace = cwd()?;
    let app = from_workspace(repo, &workspace)?;
    let id = app.workspace_variant(&workspace)?;
    Ok((app, id))
}

fn sole_campaign(app: &acvc_app::App, given: Option<String>) -> Result<String> {
    if let Some(name) = given {
        return Ok(name);
    }
    match app.campaigns()?.as_slice() {
        [name] => Ok(name.clone()),
        [] => Err(Error::new("no campaign").ctx("fix", "campaign create")),
        names => Err(Error::new("name the campaign").ctx("campaigns", names.join(" "))),
    }
}

fn cwd() -> Result<PathBuf> {
    env::current_dir().map_err(|err| Error::new(err.to_string()).ctx("op", "cwd"))
}
