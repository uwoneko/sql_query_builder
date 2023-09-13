#[cfg(feature = "sqlite")]
use crate::structure::InsertVars;
use crate::{
  behavior::{push_unique, Concat, TransactionQuery, WithQuery},
  fmt,
  structure::{Insert, InsertClause, Select},
};

impl WithQuery for Insert {}

impl TransactionQuery for Insert {}

impl Insert {
  /// Gets the current state of the [Insert] and returns it as string
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let query = sql::Insert::new()
  ///   .insert_into("users (login)")
  ///   .values("('foo')")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users (login) VALUES ('foo')
  /// ```
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the Insert into console output in a more ease to read version.
  /// This method is useful to debug complex queries or just to print the generated SQL while you type
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let insert_query = sql::Insert::new()
  ///   .insert_into("users (login, name)")
  ///   .values("('foo', 'Foo')")
  ///   .debug()
  ///   .values("('bar', 'Bar')")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users (login, name)
  /// VALUES ('foo', 'Foo')
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// The `insert into` clause. This method overrides the previous value
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let insert = sql::Insert::new()
  ///   .insert_into("users (login, name)");
  ///
  /// let insert = sql::Insert::new()
  ///   .insert_into("address (state, country)")
  ///   .insert_into("users (login, name)");
  /// ```
  #[cfg(not(feature = "sqlite"))]
  pub fn insert_into(mut self, table_name: &str) -> Self {
    self._insert_into = table_name.trim().to_owned();
    self
  }

  /// Create Insert's instance
  pub fn new() -> Self {
    Self::default()
  }

  /// The `on conflict` clause. This method overrides the previous value
  pub fn on_conflict(mut self, conflict: &str) -> Self {
    self._on_conflict = conflict.trim().to_owned();
    self
  }

  /// The `overriding` clause. This method overrides the previous value
  #[cfg(not(feature = "sqlite"))]
  pub fn overriding(mut self, option: &str) -> Self {
    self._overriding = option.trim().to_owned();
    self
  }

  /// Prints the current state of the Insert into console output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::one_line();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// The `select` clause. This method overrides the previous value
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let insert_query = sql::Insert::new()
  ///   .insert_into("users (login, name)")
  ///   .select(
  ///     sql::Select::new()
  ///       .select("login, name")
  ///       .from("users_bk")
  ///       .where_clause("active = true"),
  ///   )
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users (login, name)
  /// SELECT login, name
  /// FROM users_bk
  /// WHERE active = true
  /// ```
  pub fn select(mut self, select: Select) -> Self {
    self._select = Some(select);
    self
  }

  /// Adds at the beginning a raw SQL query.
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let raw_query = "insert into users (login, name)";
  /// let insert_query = sql::Insert::new()
  ///   .raw(raw_query)
  ///   .values("('foo', 'Foo')")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// insert into users (login, name)
  /// VALUES ('bar', 'Bar')
  /// ```
  pub fn raw(mut self, raw_sql: &str) -> Self {
    push_unique(&mut self._raw, raw_sql.trim().to_owned());
    self
  }

  /// Adds a raw SQL query after a specified clause.
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let raw = "values ('foo', 'Foo')";
  /// let insert_query = sql::Insert::new()
  ///   .insert_into("users (login, name)")
  ///   .raw_after(sql::InsertClause::InsertInto, raw)
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users (login, name)
  /// values ('foo', 'Foo')
  /// ```
  pub fn raw_after(mut self, clause: InsertClause, raw_sql: &str) -> Self {
    self._raw_after.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let raw = "insert into users (login, name)";
  /// let insert_query = sql::Insert::new()
  ///   .raw_before(sql::InsertClause::Values, raw)
  ///   .values("('bar', 'Bar')")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// insert into users (login, name)
  /// VALUES ('bar', 'Bar')
  /// ```
  pub fn raw_before(mut self, clause: InsertClause, raw_sql: &str) -> Self {
    self._raw_before.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// The `values` clause
  pub fn values(mut self, value: &str) -> Self {
    push_unique(&mut self._values, value.trim().to_owned());
    self
  }
}

#[cfg(any(doc, feature = "postgresql", feature = "sqlite"))]
impl Insert {
  /// The `returning` clause, this method can be used enabling a feature flag
  pub fn returning(mut self, output_name: &str) -> Self {
    push_unique(&mut self._returning, output_name.trim().to_owned());
    self
  }

  /// The `with` clause, this method can be used enabling a feature flag
  ///
  /// # Example
  ///
  /// ```ts
  /// use sql_query_builder as sql;
  ///
  /// let active_users = sql::Select::new().select("*").from("users_bk").where_clause("ative = true");
  /// let insert = sql::Insert::new()
  ///   .with("active_users", active_users)
  ///   .insert_into("users")
  ///   .select(sql::Select::new().select("*").from("active_users"))
  ///   .debug();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// WITH active_users AS (
  ///   SELECT *
  ///   FROM users_bk
  ///   WHERE ative = true
  /// )
  /// INSERT INTO users
  /// SELECT *
  /// FROM active_users
  /// ```
  pub fn with(mut self, name: &str, query: impl WithQuery + 'static) -> Self {
    self._with.push((name.trim().to_owned(), std::sync::Arc::new(query)));
    self
  }
}

#[cfg(any(doc, feature = "sqlite"))]
impl Insert {
  /// The `default values` clause, this method can be used enabling the feature flag `sqlite`
  pub fn default_values(mut self) -> Self {
    self._default_values = true;
    self
  }

  /// The `insert into` clause, this method can be used enabling the feature flag `sqlite`
  pub fn insert_into(mut self, expression: &str) -> Self {
    self._insert = (InsertVars::InsertInto, expression.trim().to_owned());
    self
  }

  /// The `insert or <keyword> into` clause, this method can be used enabling the feature flag `sqlite`
  pub fn insert_or(mut self, expression: &str) -> Self {
    self._insert = (InsertVars::InsertOr, expression.trim().to_owned());
    self
  }

  /// The `replace into` clause, this method can be used enabling the feature flag `sqlite`
  pub fn replace_into(mut self, expression: &str) -> Self {
    self._insert = (InsertVars::ReplaceInto, expression.trim().to_owned());
    self
  }
}

impl std::fmt::Display for Insert {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for Insert {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
