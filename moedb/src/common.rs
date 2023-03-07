use rocksdb::{Options, DBCompressionType, DBCompactionStyle, DB};
use shared::toml_schema::read_sys_cfg;
use shared::util::use_available_threads;

pub fn get_cfs() -> Vec<String> {
    let sys_cfg = read_sys_cfg().unwrap();
    DB::list_cf(&get_rocksdb_cfg(), sys_cfg.moedb.path).unwrap_or(vec![])
}

pub fn check_if_cf_exists(name: &str) -> bool {
    let column_families = get_cfs();
    column_families.iter().find(|cf| cf == &name).is_none() == false
}

pub fn get_rocksdb_cfg() -> Options {
    let sys_cfg = read_sys_cfg().unwrap();
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.create_missing_column_families(true);
    opts.increase_parallelism(use_available_threads() as i32);
    opts.set_max_open_files(-1);
    opts.set_max_background_jobs(4);
    opts.set_error_if_exists(false);
    opts.set_max_file_opening_threads(-1);
    opts.set_compression_type(DBCompressionType::Snappy);
    opts.set_db_log_dir(sys_cfg.moedb.ops_log);
    opts.set_compaction_style(DBCompactionStyle::Fifo);
    opts.set_use_fsync(true);
    opts.set_allow_concurrent_memtable_write(true);
    opts.set_allow_mmap_writes(true);
    opts.set_allow_mmap_reads(true);
    opts.set_optimize_filters_for_hits(true);
    opts
}