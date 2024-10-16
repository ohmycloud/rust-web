use warp::http::StatusCode;
use crate::store::Store;
use crate::types::account::{Account, AccountId};
use argon2::{self, Config};
use paseto::v2::local_paseto;
use rand::random;
use chrono::prelude::*;

// The hash function returns a string, the hashed version of the clear-text password
pub fn hash_password(password: &[u8]) -> String {
    // The rand function creates s32 random bytes and stores theme in a slice
    let salt = random::<[u8; 32]>();
    // Argon2 depends on a configuration, and we will use the default set.
    let config = Config::default();
    // With the password, the salt, and the config, we can hash our clear-text password.
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub async fn register(
    store: Store,
    account: Account
) -> Result<impl warp::Reply, warp::Rejection> {
    // Takes the password as a byte array and passes it to the newly created hash function
    let hashed_password = hash_password(account.password.as_bytes());

    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
    };

    match store.add_account(account).await {
        Ok(_) => {
            Ok(warp::reply::with_status("Account added", StatusCode::OK))
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn verify_password(
    hash: &str,
    password: &[u8]
) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

fn issue_token(
    account_id: AccountId
) -> String {
    let current_datetime = Utc::now();
    let dt = current_datetime + chrono::Duration::days(1);

    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(
            &Vec::from("RANDOM WORDS WINTER MACINTOSH PC".as_bytes())
        )
        .set_expiration(&dt)
        .set_not_before(&Utc::now())
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct paseto token w/ builder!")
}

pub async fn login(
    store: Store,
    login: Account
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(
            &account.password,
            login.password.as_bytes()
        ) {
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("id not found"),
                    )))
                } else {
                    Err(warp::reject::custom(handle_errors::Error::WrongPassword))
                }
            }
            Err(e) => Err(warp::reject::custom(
                handle_errors::Error::ArgonLibraryError(e),
            )),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}