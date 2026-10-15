use crate::{FileStat, IndexEntry, PlannedFile, same_stat};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListedFile {
    pub path: String,
    pub stat: FileStat,
}

#[derive(Clone, Debug, Default)]
pub struct Plan {
    pub clean: Vec<PlannedFile>,
    pub dirty: Vec<ListedFile>,
    pub added: Vec<ListedFile>,
    pub deleted: Vec<String>,
}

pub fn plan_changes(index: &[IndexEntry], listed: &[ListedFile]) -> Plan {
    let mut index = index.to_vec();
    let mut listed = listed.to_vec();
    index.sort_by(|left, right| left.path.cmp(&right.path));
    listed.sort_by(|left, right| left.path.cmp(&right.path));
    let mut plan = Plan::default();
    let mut left = 0;
    let mut right = 0;
    while left < index.len() || right < listed.len() {
        if right == listed.len() || (left < index.len() && index[left].path < listed[right].path) {
            plan.deleted.push(index[left].path.clone());
            left += 1;
        } else if left == index.len() || listed[right].path < index[left].path {
            plan.added.push(listed[right].clone());
            right += 1;
        } else if same_stat(&index[left], &listed[right].stat) {
            plan.clean.push(PlannedFile {
                path: index[left].path.clone(),
                mode: index[left].mode,
                hash: index[left].hash,
            });
            left += 1;
            right += 1;
        } else {
            plan.dirty.push(listed[right].clone());
            left += 1;
            right += 1;
        }
    }
    plan
}
