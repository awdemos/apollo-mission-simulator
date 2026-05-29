//! Conversation history ring buffer for LLM context continuity.

const MAX_HISTORY: usize = 64;

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub text: String,
    pub from_ground: bool,
    pub mission_time: f64,
    pub speaker: String,
}

pub struct ConversationHistory {
    entries: Vec<HistoryEntry>,
}

impl ConversationHistory {
    pub fn new() -> Self {
        Self {
            entries: Vec::with_capacity(MAX_HISTORY),
        }
    }

    pub fn push(&mut self, entry: HistoryEntry) {
        if self.entries.len() >= MAX_HISTORY {
            self.entries.remove(0);
        }
        self.entries.push(entry);
    }

    pub fn recent(&self, count: usize) -> &[HistoryEntry] {
        let start = self.entries.len().saturating_sub(count);
        &self.entries[start..]
    }

    pub fn last_ground_message_for(&self, speaker_prefix: &str) -> Option<&HistoryEntry> {
        self.entries
            .iter()
            .rev()
            .find(|e| e.from_ground && e.speaker.starts_with(speaker_prefix))
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for ConversationHistory {
    fn default() -> Self {
        Self::new()
    }
}
