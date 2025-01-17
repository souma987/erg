use erg_common::config::ErgConfig;

use super::cache::SharedModuleCache;
use super::graph::SharedModuleGraph;
use super::index::SharedModuleIndex;

#[derive(Debug, Clone, Default)]
pub struct SharedCompilerResource {
    pub mod_cache: SharedModuleCache,
    pub py_mod_cache: SharedModuleCache,
    pub index: SharedModuleIndex,
    pub graph: SharedModuleGraph,
}

impl SharedCompilerResource {
    pub fn new(cfg: ErgConfig) -> Self {
        Self {
            mod_cache: SharedModuleCache::new(cfg.copy()),
            py_mod_cache: SharedModuleCache::new(cfg),
            index: SharedModuleIndex::new(),
            graph: SharedModuleGraph::new(),
        }
    }

    pub fn clear_all(&self) {
        self.mod_cache.initialize();
        self.py_mod_cache.initialize();
        self.index.initialize();
        self.graph.initialize();
    }
}
