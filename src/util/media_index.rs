use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// メディアの種類
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MediaType {
    SampleWav,
    Plugin,
    Other,
}

/// メディアエントリ
#[derive(Debug, Clone)]
pub struct MediaEntry {
    pub path: PathBuf,
    pub media_type: MediaType,
    pub tags: HashSet<String>,
}

/// Trie木のノード
#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    /// このノードで終わる単語に関連付けられたエントリのIDリスト
    entry_ids: HashSet<usize>,
}

/// メディアブラウザインデックス
/// 
/// サンプルWAVやプラグインを高速に検索・タグ付けするためのインメモリのTrie木やデータベース構造
#[derive(Default, Debug)]
pub struct MediaIndex {
    entries: Vec<MediaEntry>,
    trie_root: TrieNode,
    tag_index: HashMap<String, HashSet<usize>>,
}

impl MediaIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// メディアエントリを追加する
    pub fn add_media(&mut self, path: PathBuf, media_type: MediaType, tags: Vec<String>, name: &str) -> usize {
        let entry_id = self.entries.len();
        
        let mut tag_set = HashSet::new();
        for tag in tags {
            tag_set.insert(tag.clone());
            self.tag_index.entry(tag).or_default().insert(entry_id);
        }

        let entry = MediaEntry {
            path,
            media_type,
            tags: tag_set,
        };
        self.entries.push(entry);

        self.insert_to_trie(name.to_lowercase().as_str(), entry_id);

        entry_id
    }

    /// Trie木へ文字列を挿入
    fn insert_to_trie(&mut self, word: &str, entry_id: usize) {
        let mut current_node = &mut self.trie_root;
        for ch in word.chars() {
            current_node = current_node.children.entry(ch).or_default();
        }
        current_node.entry_ids.insert(entry_id);
    }

    /// 名前で前方一致検索
    pub fn search_by_prefix(&self, prefix: &str) -> Vec<&MediaEntry> {
        let mut current_node = &self.trie_root;
        let prefix_lower = prefix.to_lowercase();
        
        for ch in prefix_lower.chars() {
            if let Some(node) = current_node.children.get(&ch) {
                current_node = node;
            } else {
                return vec![];
            }
        }

        let mut result_ids = HashSet::new();
        self.collect_all_ids(current_node, &mut result_ids);

        result_ids.into_iter().map(|id| &self.entries[id]).collect()
    }

    fn collect_all_ids(&self, node: &TrieNode, result_ids: &mut HashSet<usize>) {
        for &id in &node.entry_ids {
            result_ids.insert(id);
        }
        for child in node.children.values() {
            self.collect_all_ids(child, result_ids);
        }
    }

    /// タグで検索
    pub fn search_by_tag(&self, tag: &str) -> Vec<&MediaEntry> {
        if let Some(ids) = self.tag_index.get(tag) {
            ids.iter().map(|&id| &self.entries[id]).collect()
        } else {
            vec![]
        }
    }

    /// 指定したIDのエントリにタグを追加
    pub fn add_tag_to_entry(&mut self, entry_id: usize, tag: String) {
        if let Some(entry) = self.entries.get_mut(entry_id)
            && entry.tags.insert(tag.clone()) {
                self.tag_index.entry(tag).or_default().insert(entry_id);
            }
    }
}
