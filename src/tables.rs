
pub struct Table {
    pub definition: &'static str,
    pub select_sql: &'static str,
    pub insert_sql: &'static str,
}

pub static USERS: Table = Table {
    definition: "CREATE TABLE users (
        id           INTEGER PRIMARY KEY,
        name         TEXT NOT NULL,
        date_created INTEGER NOT NULL,

        games_melee_won          INTEGER NOT NULL,
        games_melee_lost         INTEGER NOT NULL,
        games_melee_disconnected INTEGER NOT NULL,
        games_settle_won         INTEGER NOT NULL,
        games_settle_lost        INTEGER NOT NULL,

        clan_id   INTEGER,
        old_names TEXT
    )",
    select_sql: "SELECT id, name, date_created, games_melee_won, games_melee_lost, games_melee_disconnected, games_settle_won, games_settle_lost, clan_id, old_names FROM lg_users WHERE is_deleted = 0",
    insert_sql: "INSERT INTO users VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
};

pub static CLANS: Table = Table {
    definition: "CREATE TABLE clans (
        id              INTEGER PRIMARY KEY,
        founder_user_id INTEGER NOT NULL,
        name            TEXT NOT NULL,
        link            TEXT NOT NULL,
        tag             TEXT NOT NULL,
        description     TEXT NOT NULL
    )",
    select_sql: "SELECT id, founder_user_id, name, link, tag, description FROM lg_clans",
    insert_sql: "INSERT INTO clans VALUES (?, ?, ?, ?, ?, ?)",
};

pub static CLAN_SCORES: Table = Table {
    definition: "CREATE TABLE clan_scores (
        clan_id         INTEGER NOT NULL,
        league_id       INTEGER NOT NULL,
        score           INTEGER NOT NULL,
        rank            INTEGER NOT NULL,
        trend           TEXT NOT NULL,
        date_last_game  INTEGER NOT NULL,
        games_count     INTEGER NOT NULL,
        favorite_scenario_id INTEGER NOT NULL,
        duration        INTEGER NOT NULL,
        PRIMARY KEY (clan_id, league_id)
    )",
    select_sql: "SELECT clan_id, league_id, score, rank, trend, date_last_game, games_count, favorite_scenario_id, duration FROM lg_clan_scores",
    insert_sql: "INSERT INTO clan_scores VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
};

pub static GAMES: Table = Table {
    definition: "CREATE TABLE games (
        id              INTEGER PRIMARY KEY,
        date_created    INTEGER NOT NULL,
        date_started    INTEGER NOT NULL,
        date_ended      INTEGER NOT NULL,
        duration        INTEGER NOT NULL,
        type            TEXT NOT NULL,
        status          TEXT NOT NULL,
        scenario_id     INTEGER NOT NULL,
        scenario_title  TEXT NOT NULL
    )",
    select_sql: "SELECT id, date_created, date_started, date_ended, duration, type, status, scenario_id, scenario_title FROM lg_games",
    insert_sql: "INSERT INTO games VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
};

pub static GAME_PLAYERS: Table = Table {
    definition: "CREATE TABLE game_players (
        game_id         INTEGER NOT NULL,
        player_id       INTEGER NOT NULL,
        team_id         INTEGER NOT NULL,
        user_id         INTEGER NOT NULL,
        color           INTEGER NOT NULL,
        name            TEXT NOT NULL,
        PRIMARY KEY (game_id, player_id)
    )",
    select_sql: "SELECT game_id, player_id, team_id, user_id, color, name FROM lg_game_players",
    insert_sql: "INSERT INTO game_players VALUES (?, ?, ?, ?, ?, ?)",
};

pub static GAME_SCORES: Table = Table {
    definition: "CREATE TABLE game_scores (
        league_id        INTEGER NOT NULL,
        game_id          INTEGER NOT NULL,
        player_id        INTEGER NOT NULL,
        score            INTEGER NOT NULL,
        old_player_score INTEGER NOT NULL,
        settle_rank      INTEGER NOT NULL,
        bonus            INTEGER NOT NULL,
        PRIMARY KEY (league_id, game_id, player_id)
    )",
    select_sql: "SELECT league_id, game_id, player_id, score, old_player_score, settle_rank, bonus FROM lg_game_scores",
    insert_sql: "INSERT INTO game_scores VALUES (?, ?, ?, ?, ?, ?, ?)",
};

pub static GAME_TEAMS: Table = Table {
    definition: "CREATE TABLE game_teams (
        team_id  INTEGER NOT NULL,
        game_id  INTEGER NOT NULL,
        name     TEXT NOT NULL,
        color    INTEGER NOT NULL,
        status   INTEGER NOT NULL,
        PRIMARY KEY (team_id, game_id)
    )",
    select_sql: "SELECT team_id, game_id, name, color, team_status FROM lg_game_teams",
    insert_sql: "INSERT INTO game_teams VALUES (?, ?, ?, ?, ?)",
};

pub static LEAGUES: Table = Table {
    definition: "CREATE TABLE leagues (
        id          INTEGER PRIMARY KEY,
        name_de     TEXT NOT NULL,
        name_en     TEXT NOT NULL,
        desc_de     TEXT NOT NULL,
        desc_en     TEXT NOT NULL,
        type        TEXT NOT NULL,
        date_start  INTEGER NOT NULL,
        date_end    INTEGER NOT NULL
    )",
    // We need to join the strings table four times to get name and desc in both languages...
    select_sql: "SELECT
                     l.id, nde.string, nen.string, dde.string, den.string, type, date_start, date_end
                 FROM lg_leagues l, lg_strings nde, lg_strings nen, lg_strings dde, lg_strings den
                 WHERE
                    l.name_sid = nde.id AND nde.language_id = 2 AND
                    l.name_sid = nen.id AND nen.language_id = 1 AND
                    l.description_sid = dde.id AND dde.language_id = 2 AND
                    l.description_sid = den.id AND den.language_id = 1",
    insert_sql: "INSERT INTO leagues VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
};

pub static SCORES: Table = Table {
    definition: "CREATE TABLE scores (
        user_id               INTEGER NOT NULL,
        league_id             INTEGER NOT NULL,
        score                 INTEGER NOT NULL,
        rank                  INTEGER NOT NULL,
        trend                 TEXT NOT NULL,
        date_last_game        INTEGER NOT NULL,
        games_won             INTEGER NOT NULL,
        games_lost            INTEGER NOT NULL,
        favorite_scenario_id  INTEGER NOT NULL,
        duration              INTEGER NOT NULL,
        bonus_account         INTEGER NOT NULL,
        PRIMARY KEY (user_id, league_id)
    )",
    select_sql: "SELECT user_id, league_id, score, rank, trend, date_last_game, games_won, games_lost, favorite_scenario_id, duration, bonus_account FROM lg_scores WHERE user_is_deleted = 0",
    insert_sql: "INSERT INTO scores VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
};
