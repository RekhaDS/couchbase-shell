mod addresses;
mod addresses_add;
mod addresses_drop;
mod analytics;
mod analytics_buckets;
mod analytics_datasets;
mod analytics_dataverses;
mod analytics_indexes;
mod analytics_links;
mod analytics_pending_mutations;
mod buckets;
mod buckets_builder;
mod buckets_config;
mod buckets_create;
mod buckets_drop;
mod buckets_flush;
mod buckets_get;
mod buckets_sample;
mod buckets_update;
mod cloud_json;
mod clusters;
mod clusters_health;
mod clusters_register;
mod clusters_unregister;
mod collections;
mod collections_create;
mod ctrlc_future;
mod doc;
mod doc_get;
mod doc_insert;
mod doc_remove;
mod doc_replace;
mod doc_upsert;
mod fake_data;
mod help;
mod nodes;
mod ping;
mod plugin_fetch;
mod query;
mod query_advise;
mod query_indexes;
mod scopes;
mod scopes_create;
mod search;
mod transactions;
mod transactions_list_atrs;
mod tutorial;
mod tutorial_next;
mod tutorial_page;
mod tutorial_prev;
mod use_bucket;
mod use_cluster;
mod use_cmd;
mod use_collection;
mod use_scope;
mod user_builder;
mod users;
mod users_drop;
mod users_get;
mod users_roles;
mod users_upsert;
mod util;
mod version;
mod whoami;

pub use addresses::Addresses;
pub use addresses_add::AddressesAdd;
pub use addresses_drop::AddressesDrop;
pub use analytics::Analytics;
pub use analytics_buckets::AnalyticsBuckets;
pub use analytics_datasets::AnalyticsDatasets;
pub use analytics_dataverses::AnalyticsDataverses;
pub use analytics_indexes::AnalyticsIndexes;
pub use analytics_links::AnalyticsLinks;
pub use analytics_pending_mutations::AnalyticsPendingMutations;
pub use buckets::Buckets;
pub use buckets_config::BucketsConfig;
pub use buckets_create::BucketsCreate;
pub use buckets_drop::BucketsDrop;
pub use buckets_flush::BucketsFlush;
pub use buckets_get::BucketsGet;
pub use buckets_sample::BucketsSample;
pub use buckets_update::BucketsUpdate;
pub use clusters::Clusters;
pub use clusters_health::ClustersHealth;
pub use clusters_register::ClustersRegister;
pub use clusters_unregister::ClustersUnregister;
pub use collections::Collections;
pub use collections_create::CollectionsCreate;
pub use ctrlc_future::CtrlcFuture;
pub use doc::Doc;
pub use doc_get::DocGet;
pub use doc_insert::DocInsert;
pub use doc_remove::DocRemove;
pub use doc_replace::DocReplace;
pub use doc_upsert::DocUpsert;
pub use fake_data::FakeData;
pub use help::Help;
pub use nodes::Nodes;
pub use ping::Ping;
pub use plugin_fetch::PluginFetch;
pub use query::Query;
pub use query_advise::QueryAdvise;
pub use query_indexes::QueryIndexes;
pub use scopes::Scopes;
pub use scopes_create::ScopesCreate;
pub use search::Search;
pub use transactions::Transactions;
pub use transactions_list_atrs::TransactionsListAtrs;
pub use tutorial::Tutorial;
pub use tutorial_next::TutorialNext;
pub use tutorial_page::TutorialPage;
pub use tutorial_prev::TutorialPrev;
pub use use_bucket::UseBucket;
pub use use_cluster::UseCluster;
pub use use_cmd::UseCmd;
pub use use_collection::UseCollection;
pub use use_scope::UseScope;
pub use user_builder::User;
pub use users::Users;
pub use users_drop::UsersDrop;
pub use users_get::UsersGet;
pub use users_roles::UsersRoles;
pub use users_upsert::UsersUpsert;
pub use version::Version;
pub use whoami::Whoami;

/*
mod analytics;
mod analytics_datasets;
mod analytics_dataverses;
mod analytics_indexes;
mod buckets_create;
mod buckets_drop;
mod buckets_flush;
mod buckets_sample;
mod buckets_update;
mod collections;
mod collections_create;
mod collections_get;
mod ctrlc_future;
mod data;
mod data_stats;
mod doc;
mod doc_get;
mod doc_insert;
mod doc_remove;
mod doc_replace;
mod doc_upsert;
mod nodes;
mod ping;
mod scopes;
mod scopes_create;
mod scopes_get;
mod sdk_log;
mod search;
mod tutorial;
mod tutorial_next;
mod tutorial_page;
mod tutorial_prev;
mod users;
mod users_get;
mod users_roles;
mod users_upsert;

pub use analytics::Analytics;
pub use analytics_datasets::AnalyticsDatasets;
pub use analytics_dataverses::AnalyticsDataverses;
pub use analytics_indexes::AnalyticsIndexes;
pub use buckets_create::BucketsCreate;
pub use buckets_drop::BucketsDrop;
pub use buckets_flush::BucketsFlush;
pub use buckets_sample::BucketsSample;
pub use buckets_update::BucketsUpdate;
pub use collections::Collections;
pub use collections_create::CollectionsCreate;
pub use collections_get::CollectionsGet;
use couchbase::CouchbaseError;
pub use data::Data;
pub use data_stats::DataStats;
pub use doc::Doc;
pub use doc_get::DocGet;
pub use doc_insert::DocInsert;
pub use doc_remove::DocRemove;
pub use doc_replace::DocReplace;
pub use doc_upsert::DocUpsert;
pub use nodes::Nodes;
use nu_errors::ShellError;
pub use ping::Ping;

pub use scopes::Scopes;
pub use scopes_create::ScopesCreate;
pub use scopes_get::ScopesGet;
pub use sdk_log::SDKLog;
pub use search::Search;
pub use tutorial::Tutorial;
pub use tutorial_next::TutorialNext;
pub use tutorial_page::TutorialPage;
pub use tutorial_prev::TutorialPrev;

pub use users::Users;
pub use users_get::UsersGet;
pub use users_roles::UsersRoles;
pub use users_upsert::UsersUpsert;
*/
