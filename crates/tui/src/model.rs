use acvc_app::{App, CompareReport, FrontierRow, RecallHit, WhyReport};
use acvc_domain::{Hash, Result};
use ratatui::crossterm::event::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    Campaigns,
    Frontier,
    Lineage,
    Detail,
    Diff,
    Recall,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    None,
    Quit,
    Reload,
    LoadFrontier(String),
    LoadDetail(Hash),
    LoadCompare(Hash, Hash),
    LoadFile {
        left: Hash,
        right: Hash,
        path: String,
    },
    Recall {
        campaign: String,
        query: String,
    },
}

#[derive(Clone, Debug)]
pub struct Model {
    pub screen: Screen,
    pub campaigns: Vec<String>,
    pub campaign_idx: usize,
    pub frontier: Vec<FrontierRow>,
    pub frontier_idx: usize,
    pub detail: Option<WhyReport>,
    pub lineage_idx: usize,
    pub detail_idx: usize,
    pub diff_left: Option<Hash>,
    pub diff_right: Option<Hash>,
    pub compare: Option<CompareReport>,
    pub diff_paths: Vec<String>,
    pub diff_idx: usize,
    pub file_text: String,
    pub recall_query: String,
    pub recall_hits: Vec<RecallHit>,
    pub recall_idx: usize,
    pub typing: bool,
    pub message: String,
}

impl Model {
    pub fn load(app: &App) -> Result<Self> {
        let mut model = Self::empty();
        model.campaigns = app.campaigns()?;
        if let Some(campaign) = model.campaigns.first().cloned() {
            model.frontier = app.frontier(&campaign)?;
        }
        if !model.frontier.is_empty() {
            model.screen = Screen::Frontier;
        }
        Ok(model)
    }

    pub fn empty() -> Self {
        Self {
            screen: Screen::Campaigns,
            campaigns: Vec::new(),
            campaign_idx: 0,
            frontier: Vec::new(),
            frontier_idx: 0,
            detail: None,
            lineage_idx: 0,
            detail_idx: 0,
            diff_left: None,
            diff_right: None,
            compare: None,
            diff_paths: Vec::new(),
            diff_idx: 0,
            file_text: String::new(),
            recall_query: String::new(),
            recall_hits: Vec::new(),
            recall_idx: 0,
            typing: false,
            message: String::new(),
        }
    }

    pub fn reload(&mut self, app: &App) -> Result<()> {
        let campaign = self.selected_campaign().map(str::to_string);
        self.campaigns = app.campaigns()?;
        if let Some(campaign) = campaign {
            self.frontier = app.frontier(&campaign)?;
        }
        Ok(())
    }

    pub fn selected_campaign(&self) -> Option<&str> {
        self.campaigns.get(self.campaign_idx).map(String::as_str)
    }

    pub fn on_key(&mut self, key: KeyCode) -> Command {
        if self.typing {
            return self.on_typing(key);
        }
        self.message.clear();
        match key {
            KeyCode::Char('q') => Command::Quit,
            KeyCode::Char('g') => Command::Reload,
            KeyCode::Tab | KeyCode::Right => {
                self.screen = next_screen(self.screen);
                Command::None
            }
            KeyCode::Left => {
                self.screen = prev_screen(self.screen);
                Command::None
            }
            KeyCode::Char('1') => self.jump(Screen::Campaigns),
            KeyCode::Char('2') => self.jump(Screen::Frontier),
            KeyCode::Char('3') => self.jump(Screen::Lineage),
            KeyCode::Char('4') => self.jump(Screen::Detail),
            KeyCode::Char('5') => self.jump(Screen::Diff),
            KeyCode::Char('6') => self.jump(Screen::Recall),
            KeyCode::Char('j') | KeyCode::Down => {
                self.move_selection(1);
                Command::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.move_selection(-1);
                Command::None
            }
            KeyCode::Char('/') => {
                self.typing = true;
                self.screen = Screen::Recall;
                Command::None
            }
            KeyCode::Char('d') => self.mark_diff(),
            KeyCode::Enter => self.activate(),
            _ => Command::None,
        }
    }

    fn jump(&mut self, screen: Screen) -> Command {
        self.screen = screen;
        Command::None
    }

    fn on_typing(&mut self, key: KeyCode) -> Command {
        match key {
            KeyCode::Esc => {
                self.typing = false;
                Command::None
            }
            KeyCode::Backspace => {
                self.recall_query.pop();
                Command::None
            }
            KeyCode::Enter => {
                let Some(campaign) = self.selected_campaign().map(str::to_string) else {
                    self.message = "select a campaign".into();
                    return Command::None;
                };
                Command::Recall {
                    campaign,
                    query: self.recall_query.clone(),
                }
            }
            KeyCode::Char(ch) => {
                self.recall_query.push(ch);
                Command::None
            }
            _ => Command::None,
        }
    }

    fn move_selection(&mut self, delta: isize) {
        let len = self.current_len();
        if len == 0 {
            return;
        }
        let slot = self.current_index();
        let next = (*slot as isize + delta).clamp(0, len as isize - 1) as usize;
        *slot = next;
    }

    fn current_len(&self) -> usize {
        match self.screen {
            Screen::Campaigns => self.campaigns.len(),
            Screen::Frontier => self.frontier.len(),
            Screen::Lineage => self
                .detail
                .as_ref()
                .map(|detail| detail.parents.len() + detail.children.len())
                .unwrap_or(0),
            Screen::Detail => 1,
            Screen::Diff => self.diff_paths.len(),
            Screen::Recall => self.recall_hits.len(),
        }
    }

    fn current_index(&mut self) -> &mut usize {
        match self.screen {
            Screen::Campaigns => &mut self.campaign_idx,
            Screen::Frontier => &mut self.frontier_idx,
            Screen::Lineage => &mut self.lineage_idx,
            Screen::Detail => &mut self.detail_idx,
            Screen::Diff => &mut self.diff_idx,
            Screen::Recall => &mut self.recall_idx,
        }
    }

    fn mark_diff(&mut self) -> Command {
        let Some(id) = self.selected_variant() else {
            self.message = "no variant selected".into();
            return Command::None;
        };
        if self.diff_left.is_none() {
            self.diff_left = Some(id);
            self.message = format!("diff left {}", short(&id));
            return Command::None;
        }
        self.diff_right = Some(id);
        let left = self.diff_left.unwrap();
        self.diff_left = None;
        Command::LoadCompare(left, id)
    }

    fn activate(&mut self) -> Command {
        match self.screen {
            Screen::Campaigns => {
                let Some(campaign) = self.selected_campaign().map(str::to_string) else {
                    return Command::None;
                };
                Command::LoadFrontier(campaign)
            }
            Screen::Frontier | Screen::Recall => {
                let Some(id) = self.selected_variant() else {
                    return Command::None;
                };
                Command::LoadDetail(id)
            }
            Screen::Lineage => {
                let Some(id) = self.lineage_variant() else {
                    return Command::None;
                };
                Command::LoadDetail(id)
            }
            Screen::Diff => {
                let (Some(left), Some(right)) = (
                    self.compare.as_ref().map(|item| item.left),
                    self.compare.as_ref().map(|item| item.right),
                ) else {
                    return Command::None;
                };
                let Some(path) = self.diff_paths.get(self.diff_idx).cloned() else {
                    return Command::None;
                };
                Command::LoadFile { left, right, path }
            }
            Screen::Detail => Command::None,
        }
    }

    fn selected_variant(&self) -> Option<Hash> {
        match self.screen {
            Screen::Frontier => self.frontier.get(self.frontier_idx).map(|row| row.variant),
            Screen::Recall => self.recall_hits.get(self.recall_idx).map(|hit| hit.variant),
            Screen::Detail => self.detail.as_ref().map(|detail| detail.variant),
            _ => self.frontier.get(self.frontier_idx).map(|row| row.variant),
        }
    }

    fn lineage_variant(&self) -> Option<Hash> {
        let detail = self.detail.as_ref()?;
        let mut ids = detail.parents.clone();
        ids.extend(detail.children.iter().copied());
        ids.get(self.lineage_idx).copied()
    }
}

fn next_screen(screen: Screen) -> Screen {
    match screen {
        Screen::Campaigns => Screen::Frontier,
        Screen::Frontier => Screen::Lineage,
        Screen::Lineage => Screen::Detail,
        Screen::Detail => Screen::Diff,
        Screen::Diff => Screen::Recall,
        Screen::Recall => Screen::Campaigns,
    }
}

fn prev_screen(screen: Screen) -> Screen {
    match screen {
        Screen::Campaigns => Screen::Recall,
        Screen::Frontier => Screen::Campaigns,
        Screen::Lineage => Screen::Frontier,
        Screen::Detail => Screen::Lineage,
        Screen::Diff => Screen::Detail,
        Screen::Recall => Screen::Diff,
    }
}

pub fn short(hash: &Hash) -> String {
    hash.to_hex().chars().take(12).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quit_and_move_selection() {
        let mut model = Model::empty();
        model.campaigns = vec!["one".into(), "two".into()];
        assert_eq!(model.on_key(KeyCode::Char('q')), Command::Quit);
        model.on_key(KeyCode::Char('j'));
        assert_eq!(model.campaign_idx, 1);
        model.on_key(KeyCode::Char('j'));
        assert_eq!(model.campaign_idx, 1);
        model.on_key(KeyCode::Char('k'));
        assert_eq!(model.campaign_idx, 0);
    }
}
