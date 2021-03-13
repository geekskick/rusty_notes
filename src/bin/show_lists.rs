extern crate clap;
extern crate diesel;
extern crate diesel_demo;
extern crate log;
extern crate simple_logger;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use self::models::*;
use clap::{App, Arg};
use diesel::debug_query;
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[derive(PartialEq)]
enum FilterType {
    Done,
    All,
    Todo,
}

struct Arguments {
    title_filter: String,
    filter: FilterType,
    verbose: bool,
}

impl Arguments {
    fn get_title_filter(matches: &clap::ArgMatches) -> String {
        matches.value_of("title_filter").unwrap_or("%").to_string()
    }

    fn get_clap() -> clap::App<'static, 'static> {
        App::new("Show Lists")
            .arg(
                Arg::with_name("title_filter")
                    .short("t")
                    .long("title_filter")
                    .help("Only show items with the title name")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("show_done")
                    .short("d")
                    .long("show_done")
                    .help("Only show done items"),
            )
            .arg(
                Arg::with_name("show_todo")
                    .short("2")
                    .long("show_todo")
                    .help("Show todo items only")
                    .conflicts_with("show_done"),
            )
            .arg(Arg::with_name("verbose").short("v").long("verbose"))
    }

    fn get_filter_type(matches: &clap::ArgMatches) -> FilterType {
        if matches.is_present("show_todo") {
            return FilterType::Todo;
        } else if matches.is_present("show_done") {
            return FilterType::Done;
        }
        FilterType::All
    }

    pub fn new() -> Arguments {
        let matches = Arguments::get_clap().get_matches();
        log::trace!("Parsed args into matches");

        Arguments {
            title_filter: Arguments::get_title_filter(&matches),
            filter: Arguments::get_filter_type(&matches),
            verbose: matches.is_present("verbose"),
        }
    }
}

fn show_results(results: &Vec<TodoItem>) {
    println!("Displaying {} items", results.len());

    for item in results {
        println!("{:?}", item);
    }
}

fn show_query<T: diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>>(query: &T) {
    let dbg = debug_query::<diesel::sqlite::Sqlite, _>(&query);
    log::trace!("{:?}", dbg);
}

fn perform<
    T: diesel::RunQueryDsl<diesel::SqliteConnection>
        + diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>
        + diesel::query_builder::QueryId
        + diesel::query_builder::Query
        + diesel::query_dsl::methods::LoadQuery<diesel::SqliteConnection, TodoItem>,
>(
    query: T,
    connection: &diesel::SqliteConnection,
) -> Vec<TodoItem> {
    show_query(&query);
    query
        .load::<TodoItem>(&connection)
        .expect("Error loading from databse")
}

fn main() {
    use self::diesel_demo::schema::lists::dsl::*;
    let args = Arguments::new();

    SimpleLogger::new()
        .with_level(match args.verbose {
            true => LevelFilter::Trace,
            false => LevelFilter::Off,
        })
        .init()
        .unwrap();
    log::trace!("Getting connection");
    let conn = establish_connection();

    log::trace!("Adding title filter to query");
    let query = lists.filter(title.like(args.title_filter));
    show_query(&query);

    let results: Vec<TodoItem>;
    if args.filter != FilterType::All {
        log::trace!("Adding done predicate to query");
        let pred = match args.filter {
            FilterType::Done => done.eq(true),
            FilterType::Todo => done.eq(false),
            FilterType::All => panic!(),
        };

        let query = lists.filter(pred);
        results = perform(query, &conn);
    } else {
        results = perform(query, &conn);
    }

    show_results(&results);
}
