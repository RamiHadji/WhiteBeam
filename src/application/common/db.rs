// TODO: Modify heavily ahead of 0.2 release

#[cfg(target_os = "windows")]
use crate::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::platforms::macos as platform;
use crate::common::hash;
use crate::common::time;
use std::{env,
          error::Error,
          path::Path};
use rusqlite::{params, Connection};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct LogExecObject {
    pub program: String,
    pub hash: String,
    pub uid: u32,
    pub ts: u32,
    pub success: bool
}

pub struct HookRow {
    pub id: i64,
    pub enabled: bool,
    pub class: String,
    pub library: String,
    pub symbol: String,
    pub args: String,
}

#[derive(Clone)]
pub struct WhitelistRow {
    pub class: String,
    pub id: i64,
    pub path: String,
    pub value: String
}

#[derive(Clone)]
pub struct RuleRow {
    pub library: String,
    pub symbol: String,
    pub arg: String,
    pub actions: String
}

pub struct BaselineResult {
    pub program: String,
    pub total_blocked: u32
}

pub fn get_setting(conn: &Connection, param: String) -> Result<String, Box<dyn Error>> {
    // TODO: Log errors
    Ok(conn.query_row("SELECT value FROM Setting WHERE param = ?", params![param], |r| r.get(0))?)
}

pub fn get_whitelist(conn: &Connection) -> Result<Vec<WhitelistRow>, Box<dyn Error>> {
    // TODO: Log errors
    let mut result_vec: Vec<WhitelistRow> = Vec::new();
    let mut stmt = conn.prepare("SELECT WhitelistClass.class, Whitelist.id, Whitelist.path, Whitelist.value
                                           FROM Whitelist
                                           INNER JOIN WhitelistClass ON Whitelist.class = WhitelistClass.id")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(WhitelistRow {
            class: row.get(0)?,
            id: row.get(1)?,
            path: row.get(2)?,
            value: row.get(3)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn get_hooks_pretty(conn: &Connection) -> Result<Vec<HookRow>, Box<dyn Error>> {
    // TODO: Log errors
    let mut result_vec: Vec<HookRow> = Vec::new();
    let mut stmt = conn.prepare("SELECT Hook.id, Hook.enabled, HookClass.class, Hook.library || ' (' || HookLanguage.language || ')' AS library, Hook.symbol, GROUP_CONCAT('(' || Datatype.datatype || ') ' || Argument.name, ', ') AS args
                                 FROM Hook
                                 INNER JOIN HookClass ON Hook.class = HookClass.id
                                 INNER JOIN HookLanguage ON Hook.language = HookLanguage.id
                                 INNER JOIN Argument ON Hook.id = Argument.hook
                                 INNER JOIN Datatype ON Argument.datatype = Datatype.id
                                 WHERE Argument.parent IS NULL
                                 GROUP BY Hook.id
                                 ORDER BY Hook.id, Argument.position")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(HookRow {
            id: row.get(0)?,
            enabled: row.get(1)?,
            class: row.get(2)?,
            library: row.get(3)?,
            symbol: row.get(4)?,
            args: row.get(5)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn get_rules_pretty(conn: &Connection) -> Result<Vec<RuleRow>, Box<dyn Error>> {
    // TODO: Log errors
    let mut result_vec: Vec<RuleRow> = Vec::new();
    let mut stmt = conn.prepare("SELECT Hook.library, Hook.symbol, Argument.name AS arg, GROUP_CONCAT(Action.name, ', ') AS actions
                                 FROM Rule
                                 INNER JOIN Action ON Rule.action = Action.id
                                 INNER JOIN Argument on Rule.arg = Argument.id
                                 INNER JOIN Hook on Argument.hook = Hook.id
                                 GROUP BY Hook.id, Argument.id
                                 ORDER BY Hook.id, Rule.id")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(RuleRow {
            library: row.get(0)?,
            symbol: row.get(1)?,
            arg: row.get(2)?,
            actions: row.get(3)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn get_baseline(conn: &Connection) -> Result<Vec<BaselineResult>, Box<dyn Error>> {
    let mut result_vec: Vec<BaselineResult> = Vec::new();
    let mut stmt = conn.prepare("SELECT program, count(program) AS total_blocked
                                          FROM Log
                                          WHERE success=false
                                          GROUP BY program
                                          ORDER BY total_blocked DESC")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(BaselineResult {
            program: row.get(0)?,
            total_blocked: row.get(1)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn get_prevention(conn: &Connection) -> Result<bool, Box<dyn Error>> {
    Ok(get_setting(conn, String::from("Prevention"))? == String::from("true"))
}

pub fn get_valid_auth_string(conn: &Connection, auth: &str) -> Result<bool, Box<dyn Error>> {
    let auth_hash: String = hash::common_hash_password(auth);
    let console_secret_expiry: u32 = get_setting(conn, String::from("ConsoleSecretExpiry"))?.parse()?;
    let time_now = time::get_timestamp();
    if console_secret_expiry == 0 ||
       console_secret_expiry >= time_now {
            return Ok(get_setting(conn, String::from("ConsoleSecret"))? == String::from(auth_hash));
    }
    Ok(false)
}

pub fn get_valid_auth_env(conn: &Connection) -> Result<bool, Box<dyn Error>> {
    get_valid_auth_string(conn, &env::var("WB_AUTH")?)
}

pub fn get_seen_nonce(conn: &Connection, nonce: &str) -> Result<bool, Box<dyn Error>> {
    // TODO: Log errors
    let count: i64 = conn.query_row("SELECT count(*) FROM NonceHistory WHERE nonce = ?", params![nonce], |r| r.get(0))?;
    Ok(count > 0)
}

pub fn insert_setting(conn: &Connection, param: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let _res = conn.execute("INSERT INTO Setting (param, value) VALUES (?1, ?2)", params![param, value])?;
    Ok(())
}

pub fn insert_whitelist(conn: &Connection, param: &str, value: &str) -> Result<(), Box<dyn Error>> {
    // TODO: Verify no duplicate value exists
    let _res = conn.execute("INSERT INTO Whitelist (param, value) VALUES (?1, ?2)", params![param, value])?;
    Ok(())
}

pub fn insert_exec(conn: &Connection, exec: LogExecObject) -> Result<(), Box<dyn Error>> {
    let _res = conn.execute("INSERT INTO Log (program, hash, uid, ts, success) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![exec.program, exec.hash, exec.uid, exec.ts, exec.success])?;
    Ok(())
}

pub fn update_setting(conn: &Connection, param: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let _res = conn.execute("INSERT OR REPLACE INTO Setting (param, value) VALUES (?1, ?2)", params![param, value])?;
    Ok(())
}

pub fn delete_whitelist(conn: &Connection, id: u32) -> Result<(), Box<dyn Error>> {
    let _res = conn.execute("DELETE FROM Whitelist WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn db_open(force: bool) -> Result<Connection, String> {
    let db_path: &Path = &platform::get_data_file_path("database.sqlite");
    let no_db: bool = !db_path.exists();
    // TODO: Log instead?
    if no_db && !force {
        return Err("No database file found".to_string());
    }
    match Connection::open(db_path) {
        Ok(conn) => Ok(conn),
        Err(_e) => {
            return Err("Could not open database file".to_string());
        }
    }
}
