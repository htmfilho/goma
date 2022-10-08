use clap::{Arg, App};

mod lib;

fn main() {
    let matches = App::new("Roma")
        .version("0.6.0")
        .author("Hildeberto Mendonca <me@hildeberto.com>")
        .about("Converts a CSV file to SQL Insert Statements.")
        .arg(Arg::new("csv")
            .long("csv")
            .short('f')
            .value_name("file")
            .required(true)
            .takes_value(true)
            .help("Relative or absolute path to the CSV file. The file's name is also used as table name and sql file's name, unless specified otherwise by the arguments `--table` and `--sql` respectivelly."))
        .arg(Arg::new("sql")
            .long("sql")
            .short('q')
            .value_name("file")
            .help("Relative or absolute path to the SQL file."))
        .arg(Arg::new("delimiter")
            .long("delimiter")
            .short('d')
            .default_value("comma")
            .value_name("comma | semicolon | tab")
            .help("The supported CSV value delimiter used in the file."))
        .arg(Arg::new("table")
            .long("table")
            .short('t')
            .value_name("database_table_name")
            .help("Database table name if it is different from the name of the CSV file."))
        .arg(Arg::new("headers")
            .long("headers")
            .short('h')
            .default_value("true")
            .value_name("true | false")
            .help("Consider the first line in the file as headers to columns. They are also used as sql column names unless specified otherwise."))
        .arg(Arg::new("columns")
            .long("column")
            .short('c')
            .required_if_eq("headers", "false")
            .multiple_occurrences(true)
            .value_name("database_column_names")
            .help("Columns of the database table if different from the name of the labels."))
        .arg(Arg::new("chunk")
            .long("chunk")
            .short('k')
            .default_value("0")
            .value_name("#")
            .help("Size of the transaction chunk, indicating how many insert statements are put within a transaction scope."))
        .arg(Arg::new("chunk_insert")
            .long("chunkinsert")
            .short('i')
            .default_value("0")
            .value_name("#")
            .help("Size of the insert chunk, indicating how many lines of the CSV files are put in a single insert statement."))
        .arg(Arg::new("prefix")
            .long("prefix")
            .short('p')
            .value_name("file")
            .help("File with the content to put at the beginning of the SQL file. Example: It can be used to create the target table."))
        .arg(Arg::new("suffix")
            .long("suffix")
            .short('s')
            .value_name("file")
            .help("File with the content to put at the end of the SQL file. Example: It can be used to create indexes."))
        .arg(Arg::new("with_transaction")
            .long("with_transaction")
            .short('w')
            .default_value("false")
            .value_name("true | false")
            .help("Indicates whether SQL statements are put in a transaction block or not. This argument is ignored if the argument chunk is used."))
        .arg(Arg::new("typed")
            .long("typed")
            .short('y')
            .default_value("false")
            .value_name("true | false")
            .help("Indicates whether the values type are declared, automatically detected or everything is taken as string."))
        .get_matches();

    let args = lib::Arguments::new_from_console(matches);

    match lib::process_csv(args) {
        Ok(())   => println!("CSV file processed successfully!"),
        Err(err) => println!("Error: {}.", err)
    };
}