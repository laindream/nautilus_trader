// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2024 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

use std::{
    collections::{HashMap, VecDeque},
    time::{Duration, Instant},
};

use bytes::Bytes;
use nautilus_common::{
    cache::database::CacheDatabaseAdapter, custom::CustomData, runtime::get_runtime, signal::Signal,
};
use nautilus_core::nanos::UnixNanos;
use nautilus_model::{
    accounts::any::AccountAny,
    data::{bar::Bar, quote::QuoteTick, trade::TradeTick, DataType},
    events::order::OrderEventAny,
    identifiers::{
        AccountId, ClientId, ClientOrderId, ComponentId, InstrumentId, PositionId, StrategyId,
        VenueOrderId,
    },
    instruments::{any::InstrumentAny, synthetic::SyntheticInstrument},
    orderbook::book::OrderBook,
    orders::any::OrderAny,
    position::Position,
    types::currency::Currency,
};
use sqlx::{postgres::PgConnectOptions, PgPool};
use tokio::{
    sync::mpsc::{error::TryRecvError, unbounded_channel, UnboundedReceiver, UnboundedSender},
    time::sleep,
};
use ustr::Ustr;

use crate::sql::{
    pg::{
        connect_pg, get_postgres_connect_options, PostgresConnectOptions,
        PostgresConnectOptionsBuilder,
    },
    queries::DatabaseQueries,
};

#[derive(Debug)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.infrastructure")
)]
pub struct PostgresCacheDatabase {
    pub pool: PgPool,
    tx: UnboundedSender<DatabaseQuery>,
    handle: tokio::task::JoinHandle<()>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum DatabaseQuery {
    Close,
    Add(String, Vec<u8>),
    AddCurrency(Currency),
    AddInstrument(InstrumentAny),
    AddOrder(OrderAny, Option<ClientId>, bool),
    AddAccount(AccountAny, bool),
    AddSignal(Signal),
    AddCustom(CustomData),
    AddQuote(QuoteTick),
    AddTrade(TradeTick),
    AddBar(Bar),
    UpdateOrder(OrderEventAny),
}

fn get_buffer_interval() -> Duration {
    Duration::from_millis(0)
}

async fn drain_buffer(pool: &PgPool, buffer: &mut VecDeque<DatabaseQuery>) {
    for cmd in buffer.drain(..) {
        match cmd {
            DatabaseQuery::Close => {}
            DatabaseQuery::Add(key, value) => {
                DatabaseQueries::add(pool, key, value).await.unwrap();
            }
            DatabaseQuery::AddCurrency(currency) => {
                DatabaseQueries::add_currency(pool, currency).await.unwrap();
            }
            DatabaseQuery::AddInstrument(instrument_any) => match instrument_any {
                InstrumentAny::Betting(instrument) => {
                    DatabaseQueries::add_instrument(pool, "BETTING", Box::new(instrument))
                        .await
                        .unwrap()
                }
                InstrumentAny::BinaryOption(instrument) => {
                    DatabaseQueries::add_instrument(pool, "BINARY_OPTION", Box::new(instrument))
                        .await
                        .unwrap()
                }
                InstrumentAny::CryptoFuture(instrument) => {
                    DatabaseQueries::add_instrument(pool, "CRYPTO_FUTURE", Box::new(instrument))
                        .await
                        .unwrap()
                }
                InstrumentAny::CryptoPerpetual(instrument) => {
                    DatabaseQueries::add_instrument(pool, "CRYPTO_PERPETUAL", Box::new(instrument))
                        .await
                        .unwrap()
                }
                InstrumentAny::CurrencyPair(instrument) => {
                    DatabaseQueries::add_instrument(pool, "CURRENCY_PAIR", Box::new(instrument))
                        .await
                        .unwrap()
                }
                InstrumentAny::Equity(equity) => {
                    DatabaseQueries::add_instrument(pool, "EQUITY", Box::new(equity))
                        .await
                        .unwrap()
                }
                InstrumentAny::FuturesContract(instrument) => {
                    DatabaseQueries::add_instrument(pool, "FUTURES_CONTRACT", Box::new(instrument))
                        .await
                        .unwrap()
                }
                InstrumentAny::FuturesSpread(instrument) => {
                    DatabaseQueries::add_instrument(pool, "FUTURES_SPREAD", Box::new(instrument))
                        .await
                        .unwrap()
                }
                InstrumentAny::OptionsContract(instrument) => {
                    DatabaseQueries::add_instrument(pool, "OPTIONS_CONTRACT", Box::new(instrument))
                        .await
                        .unwrap()
                }
                InstrumentAny::OptionsSpread(instrument) => {
                    DatabaseQueries::add_instrument(pool, "OPTIONS_SPREAD", Box::new(instrument))
                        .await
                        .unwrap()
                }
            },
            DatabaseQuery::AddOrder(order_any, client_id, updated) => match order_any {
                OrderAny::Limit(order) => {
                    DatabaseQueries::add_order(pool, "LIMIT", updated, Box::new(order), client_id)
                        .await
                        .unwrap()
                }
                OrderAny::LimitIfTouched(order) => DatabaseQueries::add_order(
                    pool,
                    "LIMIT_IF_TOUCHED",
                    updated,
                    Box::new(order),
                    client_id,
                )
                .await
                .unwrap(),
                OrderAny::Market(order) => {
                    DatabaseQueries::add_order(pool, "MARKET", updated, Box::new(order), client_id)
                        .await
                        .unwrap()
                }
                OrderAny::MarketIfTouched(order) => DatabaseQueries::add_order(
                    pool,
                    "MARKET_IF_TOUCHED",
                    updated,
                    Box::new(order),
                    client_id,
                )
                .await
                .unwrap(),
                OrderAny::MarketToLimit(order) => DatabaseQueries::add_order(
                    pool,
                    "MARKET_TO_LIMIT",
                    updated,
                    Box::new(order),
                    client_id,
                )
                .await
                .unwrap(),
                OrderAny::StopLimit(order) => DatabaseQueries::add_order(
                    pool,
                    "STOP_LIMIT",
                    updated,
                    Box::new(order),
                    client_id,
                )
                .await
                .unwrap(),
                OrderAny::StopMarket(order) => DatabaseQueries::add_order(
                    pool,
                    "STOP_MARKET",
                    updated,
                    Box::new(order),
                    client_id,
                )
                .await
                .unwrap(),
                OrderAny::TrailingStopLimit(order) => DatabaseQueries::add_order(
                    pool,
                    "TRAILING_STOP_LIMIT",
                    updated,
                    Box::new(order),
                    client_id,
                )
                .await
                .unwrap(),
                OrderAny::TrailingStopMarket(order) => DatabaseQueries::add_order(
                    pool,
                    "TRAILING_STOP_MARKET",
                    updated,
                    Box::new(order),
                    client_id,
                )
                .await
                .unwrap(),
            },
            DatabaseQuery::AddAccount(account_any, updated) => match account_any {
                AccountAny::Cash(account) => {
                    DatabaseQueries::add_account(pool, "CASH", updated, Box::new(account))
                        .await
                        .unwrap()
                }
                AccountAny::Margin(account) => {
                    DatabaseQueries::add_account(pool, "MARGIN", updated, Box::new(account))
                        .await
                        .unwrap()
                }
            },
            DatabaseQuery::AddSignal(signal) => {
                DatabaseQueries::add_signal(pool, &signal).await.unwrap();
            }
            DatabaseQuery::AddCustom(data) => {
                DatabaseQueries::add_custom_data(pool, &data).await.unwrap();
            }
            DatabaseQuery::AddQuote(quote) => {
                DatabaseQueries::add_quote(pool, &quote).await.unwrap();
            }
            DatabaseQuery::AddTrade(trade) => {
                DatabaseQueries::add_trade(pool, &trade).await.unwrap();
            }
            DatabaseQuery::AddBar(bar) => {
                DatabaseQueries::add_bar(pool, &bar).await.unwrap();
            }
            DatabaseQuery::UpdateOrder(event) => {
                DatabaseQueries::add_order_event(pool, event.into_boxed(), None)
                    .await
                    .unwrap();
            }
        }
    }
}

impl PostgresCacheDatabase {
    pub async fn connect(
        host: Option<String>,
        port: Option<u16>,
        username: Option<String>,
        password: Option<String>,
        database: Option<String>,
    ) -> Result<Self, sqlx::Error> {
        let pg_connect_options =
            get_postgres_connect_options(host, port, username, password, database);
        let pool = connect_pg(pg_connect_options.clone().into()).await.unwrap();
        let (tx, rx) = unbounded_channel::<DatabaseQuery>();

        // Spawn a task to handle messages
        let handle = tokio::spawn(async move {
            PostgresCacheDatabase::handle_message(rx, pg_connect_options.clone().into()).await;
        });
        Ok(PostgresCacheDatabase { pool, tx, handle })
    }

    async fn handle_message(
        mut rx: UnboundedReceiver<DatabaseQuery>,
        pg_connect_options: PgConnectOptions,
    ) {
        let pool = connect_pg(pg_connect_options).await.unwrap();
        // Buffering
        let mut buffer: VecDeque<DatabaseQuery> = VecDeque::new();
        let mut last_drain = Instant::now();
        let buffer_interval = get_buffer_interval();
        let recv_interval = Duration::from_millis(1);

        loop {
            if last_drain.elapsed() >= buffer_interval && !buffer.is_empty() {
                // Drain buffer
                drain_buffer(&pool, &mut buffer).await;
                last_drain = Instant::now();
            } else {
                // Continue to receive and handle messages until channel is hung up
                match rx.try_recv() {
                    Ok(msg) => match msg {
                        DatabaseQuery::Close => break,
                        _ => buffer.push_back(msg),
                    },
                    Err(TryRecvError::Empty) => sleep(recv_interval).await,
                    Err(TryRecvError::Disconnected) => break,
                }
            }
        }

        // Drain any remaining message
        if !buffer.is_empty() {
            drain_buffer(&pool, &mut buffer).await;
        }
    }
}

pub async fn reset_pg_database(pg_options: Option<PostgresConnectOptions>) -> anyhow::Result<()> {
    let pg_connect_options = pg_options.unwrap_or(
        PostgresConnectOptionsBuilder::default()
            .username(String::from("postgres"))
            .build()?,
    );
    let pg_pool = connect_pg(pg_connect_options.into()).await?;
    DatabaseQueries::truncate(&pg_pool).await?;
    Ok(())
}

pub async fn get_pg_cache_database() -> anyhow::Result<PostgresCacheDatabase> {
    reset_pg_database(None).await?;
    let connect_options = get_postgres_connect_options(None, None, None, None, None);
    Ok(PostgresCacheDatabase::connect(
        Some(connect_options.host),
        Some(connect_options.port),
        Some(connect_options.username),
        Some(connect_options.password),
        Some(connect_options.database),
    )
    .await?)
}

#[allow(dead_code)]
#[allow(unused)]
impl CacheDatabaseAdapter for PostgresCacheDatabase {
    fn close(&mut self) -> anyhow::Result<()> {
        let pool = self.pool.clone();
        let (tx, rx) = std::sync::mpsc::channel();

        log::debug!("Closing connection pool");
        tokio::task::block_in_place(|| {
            get_runtime().block_on(async {
                pool.close().await;
                if let Err(e) = tx.send(()) {
                    log::error!("Error closing pool: {e:?}");
                }
            });
        });

        // Cancel message handling task
        if let Err(e) = self.tx.send(DatabaseQuery::Close) {
            log::error!("Error sending close: {e:?}");
        }

        log::debug!("Awaiting task 'cache-write'"); // Naming tasks will soon be stablized
        tokio::task::block_in_place(|| {
            if let Err(e) = get_runtime().block_on(&mut self.handle) {
                log::error!("Error awaiting task 'cache-write': {e:?}");
            }
        });

        log::debug!("Closed");

        Ok(rx.recv()?)
    }

    fn flush(&mut self) -> anyhow::Result<()> {
        let pool = self.pool.clone();
        let (tx, rx) = std::sync::mpsc::channel();

        tokio::task::block_in_place(|| {
            get_runtime().block_on(async {
                if let Err(e) = DatabaseQueries::truncate(&pool).await {
                    log::error!("Error flushing pool: {e:?}");
                }
                if let Err(e) = tx.send(()) {
                    log::error!("Error sending flush result: {e:?}");
                }
            });
        });

        Ok(rx.recv()?)
    }

    fn load(&self) -> anyhow::Result<HashMap<String, Bytes>> {
        let pool = self.pool.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load(&pool).await;
            match result {
                Ok(items) => {
                    let mapping = items
                        .into_iter()
                        .map(|(k, v)| (k, Bytes::from(v)))
                        .collect();
                    if let Err(e) = tx.send(mapping) {
                        log::error!("Failed to send general items: {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load general items: {e:?}");
                    if let Err(e) = tx.send(HashMap::new()) {
                        log::error!("Failed to send empty general items: {e:?}");
                    }
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn load_currencies(&mut self) -> anyhow::Result<HashMap<Ustr, Currency>> {
        let pool = self.pool.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load_currencies(&pool).await;
            match result {
                Ok(currencies) => {
                    let mapping = currencies
                        .into_iter()
                        .map(|currency| (currency.code, currency))
                        .collect();
                    if let Err(e) = tx.send(mapping) {
                        log::error!("Failed to send currencies: {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load currencies: {e:?}");
                    if let Err(e) = tx.send(HashMap::new()) {
                        log::error!("Failed to send empty currencies: {e:?}");
                    }
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn load_instruments(&mut self) -> anyhow::Result<HashMap<InstrumentId, InstrumentAny>> {
        let pool = self.pool.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load_instruments(&pool).await;
            match result {
                Ok(instruments) => {
                    let mapping = instruments
                        .into_iter()
                        .map(|instrument| (instrument.id(), instrument))
                        .collect();
                    if let Err(e) = tx.send(mapping) {
                        log::error!("Failed to send instruments: {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load instruments: {e:?}");
                    if let Err(e) = tx.send(HashMap::new()) {
                        log::error!("Failed to send empty instruments: {e:?}");
                    }
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn load_synthetics(&mut self) -> anyhow::Result<HashMap<InstrumentId, SyntheticInstrument>> {
        todo!()
    }

    fn load_accounts(&mut self) -> anyhow::Result<HashMap<AccountId, AccountAny>> {
        let pool = self.pool.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load_accounts(&pool).await;
            match result {
                Ok(accounts) => {
                    let mapping = accounts
                        .into_iter()
                        .map(|account| (account.id(), account))
                        .collect();
                    if let Err(e) = tx.send(mapping) {
                        log::error!("Failed to send accounts: {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load accounts: {e:?}");
                    if let Err(e) = tx.send(HashMap::new()) {
                        log::error!("Failed to send empty accounts: {e:?}");
                    }
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn load_orders(&mut self) -> anyhow::Result<HashMap<ClientOrderId, OrderAny>> {
        let pool = self.pool.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load_orders(&pool).await;
            match result {
                Ok(orders) => {
                    let mapping = orders
                        .into_iter()
                        .map(|order| (order.client_order_id(), order))
                        .collect();
                    if let Err(e) = tx.send(mapping) {
                        log::error!("Failed to send orders: {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load orders: {e:?}");
                    if let Err(e) = tx.send(HashMap::new()) {
                        log::error!("Failed to send empty orders: {e:?}");
                    }
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn load_positions(&mut self) -> anyhow::Result<HashMap<PositionId, Position>> {
        todo!()
    }

    fn load_index_order_position(&self) -> anyhow::Result<HashMap<ClientOrderId, Position>> {
        todo!()
    }

    fn load_index_order_client(&self) -> anyhow::Result<HashMap<ClientOrderId, ClientId>> {
        let pool = self.pool.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load_distinct_order_event_client_ids(&pool).await;
            match result {
                Ok(currency) => {
                    if let Err(e) = tx.send(currency) {
                        log::error!("Failed to send load_index_order_client result: {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to run query load_distinct_order_event_client_ids: {e:?}");
                    if let Err(e) = tx.send(HashMap::new()) {
                        log::error!("Failed to send empty load_index_order_client result: {e:?}");
                    }
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn load_currency(&self, code: &Ustr) -> anyhow::Result<Option<Currency>> {
        let pool = self.pool.clone();
        let code = code.to_owned(); // Clone the code
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load_currency(&pool, &code).await;
            match result {
                Ok(currency) => {
                    if let Err(e) = tx.send(currency) {
                        log::error!("Failed to send currency {code}: {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load currency {code}: {e:?}");
                    if let Err(e) = tx.send(None) {
                        log::error!("Failed to send None for currency {code}: {e:?}");
                    }
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn load_instrument(
        &self,
        instrument_id: &InstrumentId,
    ) -> anyhow::Result<Option<InstrumentAny>> {
        let pool = self.pool.clone();
        let instrument_id = instrument_id.to_owned(); // Clone the instrument_id
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load_instrument(&pool, &instrument_id).await;
            match result {
                Ok(instrument) => {
                    if let Err(e) = tx.send(instrument) {
                        log::error!("Failed to send instrument {instrument_id}: {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load instrument {instrument_id}: {e:?}");
                    if let Err(e) = tx.send(None) {
                        log::error!("Failed to send None for instrument {instrument_id}: {e:?}");
                    }
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn load_synthetic(&self, instrument_id: &InstrumentId) -> anyhow::Result<SyntheticInstrument> {
        todo!()
    }

    fn load_account(&self, account_id: &AccountId) -> anyhow::Result<Option<AccountAny>> {
        let pool = self.pool.clone();
        let account_id = account_id.to_owned();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load_account(&pool, &account_id).await;
            match result {
                Ok(account) => {
                    if let Err(e) = tx.send(account) {
                        log::error!("Failed to send account {account_id}: {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load account {account_id}: {e:?}");
                    if let Err(e) = tx.send(None) {
                        log::error!("Failed to send None for account {account_id}: {e:?}");
                    }
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn load_order(&self, client_order_id: &ClientOrderId) -> anyhow::Result<Option<OrderAny>> {
        let pool = self.pool.clone();
        let client_order_id = client_order_id.to_owned();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load_order(&pool, &client_order_id).await;
            match result {
                Ok(order) => {
                    if let Err(e) = tx.send(order) {
                        log::error!("Failed to send order {client_order_id}: {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load order {client_order_id}: {e:?}");
                    let _ = tx.send(None);
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn load_position(&self, position_id: &PositionId) -> anyhow::Result<Position> {
        todo!()
    }

    fn load_actor(&self, component_id: &ComponentId) -> anyhow::Result<HashMap<String, Bytes>> {
        todo!()
    }

    fn delete_actor(&self, component_id: &ComponentId) -> anyhow::Result<()> {
        todo!()
    }

    fn load_strategy(&self, strategy_id: &StrategyId) -> anyhow::Result<HashMap<String, Bytes>> {
        todo!()
    }

    fn delete_strategy(&self, component_id: &StrategyId) -> anyhow::Result<()> {
        todo!()
    }

    fn add(&self, key: String, value: Bytes) -> anyhow::Result<()> {
        let query = DatabaseQuery::Add(key, value.into());
        self.tx
            .send(query)
            .map_err(|e| anyhow::anyhow!("Failed to send query to database message handler: {e}"))
    }

    fn add_currency(&self, currency: &Currency) -> anyhow::Result<()> {
        let query = DatabaseQuery::AddCurrency(*currency);
        self.tx.send(query).map_err(|e| {
            anyhow::anyhow!("Failed to query add_currency to database message handler: {e}")
        })
    }

    fn add_instrument(&self, instrument: &InstrumentAny) -> anyhow::Result<()> {
        let query = DatabaseQuery::AddInstrument(instrument.clone());
        self.tx.send(query).map_err(|e| {
            anyhow::anyhow!("Failed to send query add_instrument to database message handler: {e}")
        })
    }

    fn add_synthetic(&self, synthetic: &SyntheticInstrument) -> anyhow::Result<()> {
        todo!()
    }

    fn add_account(&self, account: &AccountAny) -> anyhow::Result<()> {
        let query = DatabaseQuery::AddAccount(account.clone(), false);
        self.tx.send(query).map_err(|e| {
            anyhow::anyhow!("Failed to send query add_account to database message handler: {e}")
        })
    }

    fn add_order(&self, order: &OrderAny, client_id: Option<ClientId>) -> anyhow::Result<()> {
        let query = DatabaseQuery::AddOrder(order.clone(), client_id, false);
        self.tx.send(query).map_err(|e| {
            anyhow::anyhow!("Failed to send query add_order to database message handler: {e}")
        })
    }

    fn add_position(&self, position: &Position) -> anyhow::Result<()> {
        todo!()
    }

    fn add_order_book(&self, order_book: &OrderBook) -> anyhow::Result<()> {
        todo!()
    }

    fn add_quote(&self, quote: &QuoteTick) -> anyhow::Result<()> {
        let query = DatabaseQuery::AddQuote(quote.to_owned());
        self.tx.send(query).map_err(|e| {
            anyhow::anyhow!("Failed to send query add_quote to database message handler: {e}")
        })
    }

    fn load_quotes(&self, instrument_id: &InstrumentId) -> anyhow::Result<Vec<QuoteTick>> {
        let pool = self.pool.clone();
        let instrument_id = instrument_id.to_owned();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load_quotes(&pool, &instrument_id).await;
            match result {
                Ok(quotes) => {
                    if let Err(e) = tx.send(quotes) {
                        log::error!("Failed to send quotes for instrument {instrument_id}: {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load quotes for instrument {instrument_id}: {e:?}");
                    if let Err(e) = tx.send(Vec::new()) {
                        log::error!(
                            "Failed to send empty quotes for instrument {instrument_id}: {e:?}"
                        );
                    }
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn add_trade(&self, trade: &TradeTick) -> anyhow::Result<()> {
        let query = DatabaseQuery::AddTrade(trade.to_owned());
        self.tx.send(query).map_err(|e| {
            anyhow::anyhow!("Failed to send query add_trade to database message handler: {e}")
        })
    }

    fn load_trades(&self, instrument_id: &InstrumentId) -> anyhow::Result<Vec<TradeTick>> {
        let pool = self.pool.clone();
        let instrument_id = instrument_id.to_owned();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load_trades(&pool, &instrument_id).await;
            match result {
                Ok(trades) => {
                    if let Err(e) = tx.send(trades) {
                        log::error!("Failed to send trades for instrument {instrument_id}: {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load trades for instrument {instrument_id}: {e:?}");
                    if let Err(e) = tx.send(Vec::new()) {
                        log::error!(
                            "Failed to send empty trades for instrument {instrument_id}: {e:?}"
                        );
                    }
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn add_bar(&self, bar: &Bar) -> anyhow::Result<()> {
        let query = DatabaseQuery::AddBar(bar.to_owned());
        self.tx.send(query).map_err(|e| {
            anyhow::anyhow!("Failed to send query add_bar to database message handler: {e}")
        })
    }

    fn load_bars(&self, instrument_id: &InstrumentId) -> anyhow::Result<Vec<Bar>> {
        let pool = self.pool.clone();
        let instrument_id = instrument_id.to_owned();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load_bars(&pool, &instrument_id).await;
            match result {
                Ok(bars) => {
                    if let Err(e) = tx.send(bars) {
                        log::error!("Failed to send bars for instrument {instrument_id}: {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load bars for instrument {instrument_id}: {e:?}");
                    if let Err(e) = tx.send(Vec::new()) {
                        log::error!(
                            "Failed to send empty bars for instrument {instrument_id}: {e:?}"
                        );
                    }
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn add_signal(&self, signal: &Signal) -> anyhow::Result<()> {
        let query = DatabaseQuery::AddSignal(signal.to_owned());
        self.tx.send(query).map_err(|e| {
            anyhow::anyhow!("Failed to send query add_signal to database message handler: {e}")
        })
    }

    fn load_signals(&self, name: &str) -> anyhow::Result<Vec<Signal>> {
        let pool = self.pool.clone();
        let name = name.to_owned();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load_signals(&pool, &name).await;
            match result {
                Ok(signals) => {
                    if let Err(e) = tx.send(signals) {
                        log::error!("Failed to send signals for '{name}': {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load signals for '{name}': {e:?}");
                    if let Err(e) = tx.send(Vec::new()) {
                        log::error!("Failed to send empty signals for '{name}': {e:?}");
                    }
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn add_custom_data(&self, data: &CustomData) -> anyhow::Result<()> {
        let query = DatabaseQuery::AddCustom(data.to_owned());
        self.tx.send(query).map_err(|e| {
            anyhow::anyhow!("Failed to send query add_signal to database message handler: {e}")
        })
    }

    fn load_custom_data(&self, data_type: &DataType) -> anyhow::Result<Vec<CustomData>> {
        let pool = self.pool.clone();
        let data_type = data_type.to_owned();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let result = DatabaseQueries::load_custom_data(&pool, &data_type).await;
            match result {
                Ok(signals) => {
                    if let Err(e) = tx.send(signals) {
                        log::error!("Failed to send custom data for '{data_type}': {e:?}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load custom data for '{data_type}': {e:?}");
                    if let Err(e) = tx.send(Vec::new()) {
                        log::error!("Failed to send empty custom data for '{data_type}': {e:?}");
                    }
                }
            }
        });
        Ok(rx.recv()?)
    }

    fn index_venue_order_id(
        &self,
        client_order_id: ClientOrderId,
        venue_order_id: VenueOrderId,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn index_order_position(
        &self,
        client_order_id: ClientOrderId,
        position_id: PositionId,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn update_actor(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn update_strategy(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn update_account(&self, account: &AccountAny) -> anyhow::Result<()> {
        let query = DatabaseQuery::AddAccount(account.clone(), true);
        self.tx.send(query).map_err(|e| {
            anyhow::anyhow!("Failed to send query add_account to database message handler: {e}")
        })
    }

    fn update_order(&self, event: &OrderEventAny) -> anyhow::Result<()> {
        let query = DatabaseQuery::UpdateOrder(event.clone());
        self.tx.send(query).map_err(|e| {
            anyhow::anyhow!("Failed to send query update_order to database message handler: {e}")
        })
    }

    fn update_position(&self, position: &Position) -> anyhow::Result<()> {
        todo!()
    }

    fn snapshot_order_state(&self, order: &OrderAny) -> anyhow::Result<()> {
        todo!()
    }

    fn snapshot_position_state(&self, position: &Position) -> anyhow::Result<()> {
        todo!()
    }

    fn heartbeat(&self, timestamp: UnixNanos) -> anyhow::Result<()> {
        todo!()
    }
}