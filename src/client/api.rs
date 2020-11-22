use super::DbClient;
use crate::acl::models::Account;
use crate::acl::schema::{accounts, roles};
use crate::acl::Auth;
use crate::modules::Modules;
use crate::rocket_factory;
use diesel::{QueryDsl, RunQueryDsl};
use rocket::http::{ContentType, Header};
use rocket::local::{Client, LocalRequest};

pub struct ApiClient {
    pub db: DbClient,
    account: Option<(String, Account)>,
    client: Client,
}

macro_rules! truncate_tables {
    ($conn: expr, $( $x:ident ),*) => {
        $(
            diesel::delete($x::table).execute($conn)?;
        )*
    };
}

impl ApiClient {
    pub fn new(modules: Option<Modules>) -> Result<ApiClient, crate::ServerError> {
        let modules = modules.unwrap_or_default();
        let client = Client::new(rocket_factory(Some("testing"), &modules)?)?;
        let db = DbClient::new(Some(&modules)).unwrap();
        truncate_tables!(db.as_ref(), accounts, roles);
        Ok(ApiClient {
            account: None,
            client,
            db,
        })
    }

    pub fn post(&mut self, route: &'static str) -> LocalRequest {
        if let Some((_, acc)) = &self.account {
            let bearer = format!("Bearer {}", acc.auth_token.as_ref().unwrap());
            self.client
                .post(route)
                .header(Header::new("Authorization", bearer))
        } else {
            self.client.post(route)
        }
        .header(ContentType::JSON)
    }

    pub fn get<'a>(&'a self, route: &'a str) -> LocalRequest<'a> {
        if let Some((_, acc)) = &self.account {
            let bearer = format!("Bearer {}", acc.auth_token.as_ref().unwrap());
            self.client
                .get(route)
                .header(Header::new("Authorization", bearer))
        } else {
            self.client.get(route)
        }
        .header(ContentType::JSON)
    }

    pub fn delete<'a>(&'a self, route: &'a str) -> LocalRequest<'a> {
        if let Some((_, acc)) = &self.account {
            let bearer = format!("Bearer {}", acc.auth_token.as_ref().unwrap());
            self.client
                .delete(route)
                .header(Header::new("Authorization", bearer))
        } else {
            self.client.delete(route)
        }
        .header(ContentType::JSON)
    }

    pub fn acting_as(&mut self, pass: &str, mut account: Account) {
        assert!(Auth::perform_login(&mut account, pass, self.db.as_ref())
            .expect("Failed to login account"));
        self.account = Some((pass.into(), account));
    }

    pub fn assert_logged_out(&self, acc: &Account) {
        let account: Account = accounts::table
            .find(acc.id)
            .first(self.db.as_ref())
            .expect("Failed to fetch test account");
        assert!(account.auth_token.is_none());
    }

    pub fn assert_logged_in(&self, acc: &Account) {
        let account: Account = accounts::table
            .find(acc.id)
            .first(self.db.as_ref())
            .expect("Failed to fetch test account");
        assert!(account.auth_token.is_some());
    }
}
