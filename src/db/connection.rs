use diesel::{Connection, PgConnection};

pub fn build_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url))
}

#[cfg(test)]
pub mod test {
    
}