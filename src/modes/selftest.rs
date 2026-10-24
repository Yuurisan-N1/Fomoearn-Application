use crate::domain::account::Account;
use crate::net::der;
use crate::net::keybox::Keybox;
use crate::net::proxy::Proxy;
use crate::net::seal::Seal;
use crate::support::consts::net::{KEYBOX_FILE, SEAL_SERVER_PUB};
use crate::support::error::{AppError, AppResult};
use crate::support::log::format::{lg, lr};

struct Check {
    name: &'static str,
    outcome: Result<(), String>,
}

pub async fn execute() -> AppResult<()> {
    lg("Selftest mode, offline checks only, no network");

    let mut checks: Vec<Check> = Vec::new();

    checks.push(Check {
        name: "the account line splits on the pipe",
        outcome: match Account::parse_line("someone@mail.com|secret123") {
            Ok(account) if account.email == "someone@mail.com" => Ok(()),
            _ => Err("the pipe line was not read".to_string()),
        },
    });

    checks.push(Check {
        name: "the colon separator is refused",
        outcome: match Account::parse_line("someone@mail.com:secret123") {
            Err(_) => Ok(()),
            Ok(_) => Err("the colon line was accepted".to_string()),
        },
    });

    checks.push(Check {
        name: "the proxy line carries a host and a port",
        outcome: match Proxy::parse("http://user:pass@gw.example.com:823") {
            Ok(proxy) if proxy.port == 823 => Ok(()),
            _ => Err("the proxy line was not read".to_string()),
        },
    });

    checks.push(Check {
        name: "the der integer keeps a leading zero",
        outcome: if der::integer(0x80).len() == 4 && der::integer(0x80)[3] == 0x80 {
            Ok(())
        } else {
            Err("the integer came out short".to_string())
        },
    });

    checks.push(Check {
        name: "the der explicit wraps a context tag",
        outcome: if der::explicit(3, &der::integer(1))[0] == 0xA3 {
            Ok(())
        } else {
            Err("the context tag came out wrong".to_string())
        },
    });

    checks.push(Check {
        name: "the seal envelope binds the request path",
        outcome: seal_check(),
    });

    checks.push(Check {
        name: "the keybox signs a fresh attestation leaf",
        outcome: match std::fs::read_to_string(KEYBOX_FILE) {
            Ok(text) => forge_check(&text),
            Err(_) => Err(format!("{} was not found", KEYBOX_FILE)),
        },
    });

    let mut failed = 0;
    for check in &checks {
        match &check.outcome {
            Ok(()) => lg(&format!("The check passed for {}", check.name)),
            Err(reason) => {
                lr(&format!("The check failed for {} because {}", check.name, reason));
                failed += 1;
            }
        }
    }

    if failed > 0 {
        return Err(AppError::input(format!("{} checks failed", failed)));
    }

    lg(&format!("Every one of the {} checks passed", checks.len()));
    Ok(())
}

fn seal_check() -> Result<(), String> {
    let mut seal = Seal::new();
    seal.adopt("000102030405060708090a0b0c0d0e0f", SEAL_SERVER_PUB, 1_800_000_000)
        .map_err(|err| err.to_string())?;
    let envelope = seal.seal("tasks", "{}").map_err(|err| err.to_string())?;
    if !envelope.aad.starts_with("v1|000102030405060708090a0b0c0d0e0f|") {
        return Err("the version and install id are missing".to_string());
    }
    if !envelope.aad.ends_with("|api/macros/tasks") {
        return Err("the request path is missing".to_string());
    }
    if !envelope.body.contains("\"iv\"") || !envelope.body.contains("\"ct\"") {
        return Err("the envelope body is malformed".to_string());
    }
    Ok(())
}

fn forge_check(text: &str) -> Result<(), String> {
    let keybox = Keybox::load(text).map_err(|err| err.to_string())?;
    let challenge = [0x5Au8; 32];
    let forged = crate::net::attest::forge(&challenge, &keybox).map_err(|err| err.to_string())?;
    if forged.chain.len() < 2 {
        return Err("the chain is too short".to_string());
    }
    let leaf = der::head(&forged.chain[0]).ok_or_else(|| "the leaf is not der".to_string())?;
    if leaf.tag != 0x30 {
        return Err("the leaf is not a certificate".to_string());
    }
    if leaf.raw.len() < 400 {
        return Err("the leaf is suspiciously small".to_string());
    }
    Ok(())
}
